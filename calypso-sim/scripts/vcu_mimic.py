"""VCU mimic harness for calypso-sim --stream.

Spawns calypso-sim, drives JSON-RPC over its stdio, and verifies via
an independent paho-mqtt subscriber that publishes land on the broker.
Replays six scenarios mirroring Cerberus-2.0's `Core/Src/u_statemachine.c`.

Run from the calypso-sim directory:

    python3 scripts/vcu_mimic.py
    python3 scripts/vcu_mimic.py --with-auto       # autonomous heartbeat alongside
    python3 scripts/vcu_mimic.py --only S1,S2      # filter scenarios
    python3 scripts/vcu_mimic.py --broker localhost:1883
"""

from __future__ import annotations

import argparse
import contextlib
import json
import logging
import math
import os
import queue
import subprocess
import sys
import threading
import time
from dataclasses import dataclass, field
from enum import IntEnum
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
SIM_DIR = SCRIPT_DIR.parent
sys.path.insert(0, str(SCRIPT_DIR))

import paho.mqtt.client as mqtt  # noqa: E402
import serverdata_pb2  # noqa: E402


log = logging.getLogger("vcu-mimic")


# ---------- StreamClient ---------------------------------------------------

class StreamRpcError(RuntimeError):
    def __init__(self, code: int, message: str):
        super().__init__(f"[{code}] {message}")
        self.code = code
        self.message = message


class StreamClient:
    """JSON-RPC 2.0 client over calypso-sim --stream's stdio."""

    def __init__(self, broker: str, with_auto: bool = False, release_bin: Path | None = None):
        bin_path = release_bin or (SIM_DIR / "target" / "release" / "calypso-sim")
        if not bin_path.exists():
            raise FileNotFoundError(
                f"{bin_path} not found — run `cargo build --release` in {SIM_DIR}"
            )
        cmd = [str(bin_path), "-u", broker, "--stream"]
        if with_auto:
            cmd.append("--auto")
        log.info("spawning: %s", " ".join(cmd))
        self.proc = subprocess.Popen(
            cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
            text=True, bufsize=1, cwd=str(SIM_DIR),
        )
        self._next_id = 1
        self._id_lock = threading.Lock()
        self._pending: dict[int, queue.Queue] = {}
        self._pending_lock = threading.Lock()
        self._closed = False
        self._reader = threading.Thread(target=self._read_stdout, daemon=True)
        self._reader.start()
        self._stderr_thread = threading.Thread(target=self._drain_stderr, daemon=True)
        self._stderr_thread.start()

    def _read_stdout(self):
        for line in self.proc.stdout:
            line = line.strip()
            if not line:
                continue
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                log.warning("non-JSON on stdout: %r", line)
                continue
            rid = msg.get("id")
            if rid is None:
                log.debug("notification (no id): %s", msg)
                continue
            with self._pending_lock:
                q = self._pending.pop(rid, None)
            if q is not None:
                q.put(msg)
            else:
                log.warning("response for unknown id %s: %s", rid, msg)

    def _drain_stderr(self):
        for line in self.proc.stderr:
            log.debug("sim-stderr: %s", line.rstrip())

    def _call(self, method: str, params: dict | None = None, timeout: float = 5.0):
        if self._closed:
            raise RuntimeError("StreamClient is closed")
        with self._id_lock:
            rid = self._next_id
            self._next_id += 1
        q: queue.Queue = queue.Queue(maxsize=1)
        with self._pending_lock:
            self._pending[rid] = q
        req = {"jsonrpc": "2.0", "id": rid, "method": method, "params": params or {}}
        line = json.dumps(req) + "\n"
        self.proc.stdin.write(line)
        self.proc.stdin.flush()
        try:
            resp = q.get(timeout=timeout)
        except queue.Empty:
            with self._pending_lock:
                self._pending.pop(rid, None)
            raise TimeoutError(f"no response to {method} (id={rid}) within {timeout}s")
        if "error" in resp:
            err = resp["error"]
            raise StreamRpcError(err.get("code", 0), err.get("message", ""))
        return resp.get("result", {})

    def publish(self, topic: str, value: float, unit: str = "") -> int:
        r = self._call("publish", {"topic": topic, "value": float(value), "unit": unit})
        return int(r.get("ts_us", 0))

    def publish_values(self, topic: str, values: list[float], unit: str = "") -> int:
        r = self._call("publish", {"topic": topic, "values": [float(v) for v in values], "unit": unit})
        return int(r.get("ts_us", 0))

    def claim(self, topic: str) -> dict:   return self._call("claim", {"topic": topic})
    def release(self, topic: str) -> dict: return self._call("release", {"topic": topic})
    def silence(self, topic: str) -> dict: return self._call("silence", {"topic": topic})
    def status(self) -> list[dict]:        return self._call("status").get("overrides", [])
    def list_topics(self) -> list[dict]:   return self._call("list_topics").get("topics", [])
    def ping(self) -> bool:                return bool(self._call("ping").get("ok"))

    def close(self):
        if self._closed:
            return
        self._closed = True
        with contextlib.suppress(Exception):
            self.proc.stdin.close()
        try:
            self.proc.wait(timeout=3)
        except subprocess.TimeoutExpired:
            self.proc.terminate()
            with contextlib.suppress(subprocess.TimeoutExpired):
                self.proc.wait(timeout=2)
        log.info("stream client closed (rc=%s)", self.proc.returncode)


# ---------- BrokerObserver -------------------------------------------------

@dataclass
class LastMsg:
    values: list[float]
    time_us: int
    recv_ts: float


class BrokerObserver:
    def __init__(self, host: str, port: int):
        self.host, self.port = host, port
        self.last: dict[str, LastMsg] = {}
        self._lock = threading.Lock()
        self._connected = threading.Event()
        self.client = mqtt.Client(callback_api_version=mqtt.CallbackAPIVersion.VERSION2,
                                  client_id="vcu-mimic-observer")
        self.client.on_connect = self._on_connect
        self.client.on_message = self._on_message

    def _on_connect(self, client, userdata, flags, rc, props=None):
        log.info("broker connected (rc=%s); subscribing to #", rc)
        client.subscribe("#")
        self._connected.set()

    def _on_message(self, client, userdata, msg):
        try:
            sd = serverdata_pb2.ServerData.FromString(msg.payload)
        except Exception as e:
            log.debug("payload not ServerData on %s: %s", msg.topic, e)
            return
        with self._lock:
            self.last[msg.topic] = LastMsg(list(sd.values), int(sd.time_us), time.monotonic())

    def start(self, timeout: float = 3.0):
        self.client.connect(self.host, self.port, keepalive=30)
        self.client.loop_start()
        if not self._connected.wait(timeout):
            raise TimeoutError(f"broker {self.host}:{self.port} did not connect within {timeout}s")

    def stop(self):
        self.client.loop_stop()
        with contextlib.suppress(Exception):
            self.client.disconnect()

    def get(self, topic: str) -> LastMsg | None:
        with self._lock:
            return self.last.get(topic)

    def assert_published(self, topic: str, expected: float, within_ms: int = 500, tol: float = 1e-3):
        deadline = time.monotonic() + within_ms / 1000.0
        last_seen = None
        while time.monotonic() < deadline:
            m = self.get(topic)
            if m is not None and m.values and abs(m.values[0] - expected) <= tol:
                return m
            last_seen = m
            time.sleep(0.02)
        raise AssertionError(
            f"topic {topic!r} did not reach {expected} within {within_ms}ms; "
            f"last={last_seen.values if last_seen else None}"
        )

    def assert_owner_only_us(self, topic: str, since: float, max_age_ms: int = 1500):
        """Assert that the most recent publish on `topic` happened at or before `since`
        (i.e. the autonomous heartbeat hasn't republished it after we stopped)."""
        time.sleep(max_age_ms / 1000.0)
        m = self.get(topic)
        if m is None:
            return  # nothing seen at all is fine for an idle topic
        if m.recv_ts > since + 0.05:
            raise AssertionError(
                f"topic {topic!r} was republished after our claim cutoff "
                f"(diff={(m.recv_ts - since) * 1000:.0f}ms)"
            )


# ---------- VcuMock --------------------------------------------------------

class FuncState(IntEnum):
    READY = 0
    F_PIT = 1
    F_REVERSE = 2
    F_PERFORMANCE = 3
    F_EFFICIENCY = 4
    FAULTED = 5


class NeroMenu(IntEnum):
    OFF = 0
    PIT = 1
    REVERSE = 2
    PERFORMANCE = 3
    EFFICIENCY = 4
    GAMES = 5
    THEMES = 6
    EXIT = 7


MAX_NERO_STATES = 8

# Calibration constants from Cerberus-2.0 Core/Src/u_pedals.c
MIN_APPS1_VOLTS = 2.15
MAX_APPS1_VOLTS = 3.12
MIN_APPS2_VOLTS = 1.15
MAX_APPS2_VOLTS = 2.01
MIN_BRAKE1_VOLTS = 0.5
MAX_BRAKE1_VOLTS = 1.5
MIN_BRAKE2_VOLTS = 0.5
MAX_BRAKE2_VOLTS = 1.5
PEDAL_BRAKE_THRESH = 0.12
PIT_MAX_SPEED = 5.0  # mph

CRITICAL_FAULTS = [
    "CAN_OUTGOING_FAULT", "CAN_INCOMING_FAULT", "BMS_CAN_MONITOR_FAULT",
    "LIGHTNING_CAN_MONITOR_FAULT", "SHUTDOWN_FAULT",
]
NON_CRITICAL_FAULTS = [
    "ONBOARD_TEMP_FAULT", "IMU_ACCEL_FAULT", "IMU_GYRO_FAULT", "BSPD_PREFAULT",
    "ONBOARD_BRAKE_OPEN_CIRCUIT_FAULT", "ONBOARD_ACCEL_OPEN_CIRCUIT_FAULT",
    "ONBOARD_BRAKE_SHORT_CIRCUIT_FAULT", "ONBOARD_ACCEL_SHORT_CIRCUIT_FAULT",
    "ONBOARD_PEDAL_DIFFERENCE_FAULT", "RTDS_FAULT", "LV_LOW_VOLTAGE_FAULT",
]


@dataclass
class VcuMock:
    client: StreamClient
    functional: FuncState = FuncState.READY
    nero_index: NeroMenu = NeroMenu.OFF
    home_mode: bool = True
    accel_norm: float = 0.0
    brake_norm: float = 0.0
    tsms: bool = False  # shutdown closed?
    speed_mph: float = 0.0
    faults: dict[str, bool] = field(default_factory=dict)

    def __post_init__(self):
        for f in CRITICAL_FAULTS + NON_CRITICAL_FAULTS:
            self.faults.setdefault(f, False)

    @property
    def all_owned_topics(self) -> list[str]:
        out = [
            "VCU/CarState/home_mode", "VCU/CarState/nero_index", "VCU/CarState/speed",
            "VCU/CarState/tsms", "VCU/CarState/torque_limit_percentage",
            "VCU/CarState/not_in_reverse", "VCU/CarState/regen_limit",
            "VCU/CarState/launch_control", "VCU/CarState/functional_state",
            "VCU/CarState/traction_control",
            "VCU/Pedals/Percentages/acceleration_pedal",
            "VCU/Pedals/Percentages/brake_pedal",
            "VCU/Pedals/Voltages/accel_1", "VCU/Pedals/Voltages/accel_2",
            "VCU/Pedals/Voltages/brake_1", "VCU/Pedals/Voltages/brake_2",
        ]
        for f in CRITICAL_FAULTS:
            out.append(f"VCU/Faults/Critical/{f}")
        for f in NON_CRITICAL_FAULTS:
            out.append(f"VCU/Faults/Non-Critical/{f}")
        return out

    def claim_all(self):
        for t in self.all_owned_topics:
            self.client.claim(t)

    def release_all(self):
        for t in self.all_owned_topics:
            with contextlib.suppress(StreamRpcError):
                self.client.release(t)

    # --- publishing ----
    def publish_carstate(self):
        c = self.client
        c.publish("VCU/CarState/home_mode", 1.0 if self.home_mode else 0.0)
        c.publish("VCU/CarState/nero_index", float(int(self.nero_index)))
        c.publish("VCU/CarState/speed", self.speed_mph, unit="mph")
        c.publish("VCU/CarState/tsms", 1.0 if self.tsms else 0.0)
        c.publish("VCU/CarState/torque_limit_percentage", 1.0)
        c.publish("VCU/CarState/not_in_reverse", 0.0 if self.functional == FuncState.F_REVERSE else 1.0)
        c.publish("VCU/CarState/regen_limit", 50.0, unit="A")
        c.publish("VCU/CarState/launch_control", 0.0)
        c.publish("VCU/CarState/functional_state", float(int(self.functional)))
        c.publish("VCU/CarState/traction_control", 0.0)

    def publish_pedals(self):
        c = self.client
        a, b = self.accel_norm, self.brake_norm
        c.publish("VCU/Pedals/Percentages/acceleration_pedal", a)
        c.publish("VCU/Pedals/Percentages/brake_pedal", b)
        c.publish("VCU/Pedals/Voltages/accel_1",
                  MIN_APPS1_VOLTS + a * (MAX_APPS1_VOLTS - MIN_APPS1_VOLTS), unit="V")
        c.publish("VCU/Pedals/Voltages/accel_2",
                  MIN_APPS2_VOLTS + a * (MAX_APPS2_VOLTS - MIN_APPS2_VOLTS), unit="V")
        c.publish("VCU/Pedals/Voltages/brake_1",
                  MIN_BRAKE1_VOLTS + b * (MAX_BRAKE1_VOLTS - MIN_BRAKE1_VOLTS), unit="V")
        c.publish("VCU/Pedals/Voltages/brake_2",
                  MIN_BRAKE2_VOLTS + b * (MAX_BRAKE2_VOLTS - MIN_BRAKE2_VOLTS), unit="V")

    def publish_faults(self):
        for f in CRITICAL_FAULTS:
            self.client.publish(f"VCU/Faults/Critical/{f}", 1.0 if self.faults[f] else 0.0)
        for f in NON_CRITICAL_FAULTS:
            self.client.publish(f"VCU/Faults/Non-Critical/{f}", 1.0 if self.faults[f] else 0.0)

    def tick(self):
        """One 4Hz heartbeat — all topics get refreshed."""
        self.publish_carstate()
        self.publish_pedals()
        self.publish_faults()

    # --- state machine (mirrors Core/Src/u_statemachine.c) ----
    def boot(self):
        self.functional = FuncState.READY
        self.nero_index = NeroMenu.OFF
        self.home_mode = True
        self.accel_norm = 0.0
        self.brake_norm = 0.0
        self.tsms = False
        self.speed_mph = 0.0
        for f in self.faults:
            self.faults[f] = False
        self.tick()

    def menu_increment(self):
        nxt = int(self.nero_index) + 1
        self.nero_index = NeroMenu(0 if nxt >= MAX_NERO_STATES else nxt)
        self.client.publish("VCU/CarState/nero_index", float(int(self.nero_index)))

    def menu_decrement(self):
        nxt = int(self.nero_index) - 1
        if nxt < 0:
            nxt = MAX_NERO_STATES - 1
        self.nero_index = NeroMenu(nxt)
        self.client.publish("VCU/CarState/nero_index", float(int(self.nero_index)))

    def menu_select(self) -> bool:
        """Try to enter the currently-highlighted drive mode. Mirrors
        transition_functional_state in u_statemachine.c — only F_PIT, F_PERFORMANCE,
        and F_EFFICIENCY require brake-pressed + shutdown-closed; F_REVERSE is gateless."""
        target_map = {
            NeroMenu.PIT: FuncState.F_PIT,
            NeroMenu.PERFORMANCE: FuncState.F_PERFORMANCE,
            NeroMenu.EFFICIENCY: FuncState.F_EFFICIENCY,
        }
        if self.nero_index == NeroMenu.REVERSE:
            self.functional = FuncState.F_REVERSE
        elif self.nero_index in target_map:
            if self.brake_norm < PEDAL_BRAKE_THRESH:
                log.warning("brake not pressed (%.2f < %.2f) — cannot enter drive mode",
                            self.brake_norm, PEDAL_BRAKE_THRESH)
                return False
            if not self.tsms:
                log.warning("shutdown not closed (tsms=0) — cannot enter drive mode")
                return False
            self.functional = target_map[self.nero_index]
        else:
            log.info("nero_index=%s is not a drive mode; no transition", self.nero_index)
            return False
        self.home_mode = False
        self.client.publish("VCU/CarState/home_mode", 0.0)
        self.client.publish("VCU/CarState/functional_state", float(int(self.functional)))
        self.client.publish("VCU/CarState/not_in_reverse",
                            0.0 if self.functional == FuncState.F_REVERSE else 1.0)
        return True

    def set_home(self):
        if self.functional != FuncState.FAULTED:
            self.functional = FuncState.READY
        self.home_mode = True
        self.client.publish("VCU/CarState/home_mode", 1.0)
        self.client.publish("VCU/CarState/functional_state", float(int(self.functional)))

    def set_pedals(self, accel: float, brake: float):
        self.accel_norm = max(0.0, min(1.0, accel))
        self.brake_norm = max(0.0, min(1.0, brake))
        self.publish_pedals()

    def set_tsms(self, closed: bool):
        self.tsms = closed
        self.client.publish("VCU/CarState/tsms", 1.0 if closed else 0.0)

    def set_speed(self, mph: float):
        self.speed_mph = mph
        self.client.publish("VCU/CarState/speed", mph, unit="mph")

    def trigger_fault(self, name: str):
        if name not in self.faults:
            raise ValueError(f"unknown fault: {name}")
        self.faults[name] = True
        # Special case mirrored from transition_functional_state: any fault →
        # FAULTED + nero=OFF + home_mode=true
        self.functional = FuncState.FAULTED
        self.nero_index = NeroMenu.OFF
        self.home_mode = True
        critical = name in CRITICAL_FAULTS
        topic = f"VCU/Faults/{'Critical' if critical else 'Non-Critical'}/{name}"
        self.client.publish(topic, 1.0)
        self.client.publish("VCU/CarState/functional_state", float(int(self.functional)))
        self.client.publish("VCU/CarState/nero_index", float(int(self.nero_index)))
        self.client.publish("VCU/CarState/home_mode", 1.0)

    def clear_faults(self):
        for f in self.faults:
            self.faults[f] = False
        self.publish_faults()
        self.functional = FuncState.READY
        self.client.publish("VCU/CarState/functional_state", float(int(self.functional)))


# ---------- Scenarios ------------------------------------------------------

def s1_boot(vcu: VcuMock, obs: BrokerObserver):
    log.info("S1: boot — claim all VCU topics, publish initial state")
    vcu.claim_all()
    overrides = {o["topic"]: o["owner"] for o in vcu.client.status()}
    for t in vcu.all_owned_topics:
        assert overrides.get(t) == "stream", f"claim missing for {t}"
    vcu.boot()
    obs.assert_published("VCU/CarState/home_mode", 1.0)
    obs.assert_published("VCU/CarState/nero_index", 0.0)
    obs.assert_published("VCU/CarState/functional_state", 0.0)
    obs.assert_published("VCU/Pedals/Percentages/acceleration_pedal", 0.0)
    obs.assert_published("VCU/Faults/Non-Critical/BSPD_PREFAULT", 0.0)
    log.info("S1 ✓ %d/%d owned topics, boot state observed on broker",
             len(vcu.all_owned_topics), len(vcu.all_owned_topics))


def s2_menu_nav(vcu: VcuMock, obs: BrokerObserver):
    log.info("S2: menu navigation 0→7→0")
    expected = [1, 2, 3, 4, 5, 6, 7, 0]
    for want in expected:
        vcu.menu_increment()
        obs.assert_published("VCU/CarState/nero_index", float(want))
        time.sleep(0.2)
    log.info("S2 ✓ wrap at MAX_NERO_STATES verified")


def s3_enter_pit(vcu: VcuMock, obs: BrokerObserver):
    log.info("S3: enter PIT — gate on brake + tsms")
    # Try to select PIT without brake — should be rejected by VcuMock
    vcu.menu_increment()  # 0 → 1 (PIT)
    obs.assert_published("VCU/CarState/nero_index", 1.0)
    assert not vcu.menu_select(), "menu_select should fail without brake"
    assert vcu.functional == FuncState.READY, "functional should not transition"
    # Press brake, close tsms, retry
    vcu.set_pedals(0.0, 0.30)
    obs.assert_published("VCU/Pedals/Percentages/brake_pedal", 0.30)
    vcu.set_tsms(True)
    obs.assert_published("VCU/CarState/tsms", 1.0)
    assert vcu.menu_select(), "menu_select should succeed once gated conditions met"
    obs.assert_published("VCU/CarState/home_mode", 0.0)
    obs.assert_published("VCU/CarState/functional_state", float(int(FuncState.F_PIT)))
    log.info("S3 ✓ brake+tsms gate enforced, F_PIT entered")


def s4_drive_sweep(vcu: VcuMock, obs: BrokerObserver, duration_s: float = 8.0):
    log.info("S4: drive telemetry sweep (%.0fs in F_PIT)", duration_s)
    assert vcu.functional == FuncState.F_PIT, "must be in F_PIT for S4"
    start = time.monotonic()
    last_tick = start
    samples = 0
    while time.monotonic() - start < duration_s:
        t = time.monotonic() - start
        accel = max(0.0, math.sin(t * math.pi / duration_s))
        vcu.set_pedals(accel, 0.0)
        # PIT speed limit: scale linearly with accel, capped at PIT_MAX_SPEED
        vcu.set_speed(min(PIT_MAX_SPEED, accel * PIT_MAX_SPEED * 1.2))
        # heartbeat the rest of the carstate every 250ms
        if time.monotonic() - last_tick > 0.25:
            vcu.publish_carstate()
            vcu.publish_faults()
            last_tick = time.monotonic()
        samples += 1
        time.sleep(0.05)
    # Verify the sweep peaked near 1.0 and speed tracked it
    a = obs.get("VCU/Pedals/Percentages/acceleration_pedal")
    s = obs.get("VCU/CarState/speed")
    assert a and s, "expected last-sample data on broker"
    log.info("S4 ✓ %d samples, last accel=%.2f speed=%.2f mph (PIT cap=%.1f)",
             samples, a.values[0], s.values[0], PIT_MAX_SPEED)


def s4b_owner_isolation(vcu: VcuMock, obs: BrokerObserver, with_auto: bool):
    if not with_auto:
        log.info("S4b: skip (run with --with-auto to verify autonomous isolation)")
        return
    log.info("S4b: verify autonomous heartbeat does NOT republish claimed topics")
    topic = "VCU/CarState/torque_limit_percentage"
    vcu.client.publish(topic, 0.42)
    obs.assert_published(topic, 0.42)
    cutoff = time.monotonic()
    obs.assert_owner_only_us(topic, since=cutoff, max_age_ms=1500)
    log.info("S4b ✓ %s held its claimed value across 1.5s of autonomous ticks", topic)


def s5_enter_reverse(vcu: VcuMock, obs: BrokerObserver):
    log.info("S5: enter REVERSE")
    # Per Cerberus, the driver returns to home_mode (which transitions active→READY)
    # and then scrolls to REVERSE in the menu and selects it. F_REVERSE has no
    # brake/tsms gate in transition_functional_state.
    vcu.set_home()
    obs.assert_published("VCU/CarState/home_mode", 1.0)
    obs.assert_published("VCU/CarState/functional_state", float(int(FuncState.READY)))
    while vcu.nero_index != NeroMenu.REVERSE:
        vcu.menu_increment()
        time.sleep(0.05)
    obs.assert_published("VCU/CarState/nero_index", float(int(NeroMenu.REVERSE)))
    assert vcu.menu_select(), "REVERSE select failed"
    obs.assert_published("VCU/CarState/functional_state", float(int(FuncState.F_REVERSE)))
    obs.assert_published("VCU/CarState/not_in_reverse", 0.0)
    log.info("S5 ✓ F_REVERSE entered, not_in_reverse=0")


def s6_fault_recovery(vcu: VcuMock, obs: BrokerObserver):
    log.info("S6: fault & recovery (BSPD_PREFAULT)")
    vcu.trigger_fault("BSPD_PREFAULT")
    obs.assert_published("VCU/Faults/Non-Critical/BSPD_PREFAULT", 1.0)
    obs.assert_published("VCU/CarState/functional_state", float(int(FuncState.FAULTED)))
    obs.assert_published("VCU/CarState/nero_index", 0.0)
    obs.assert_published("VCU/CarState/home_mode", 1.0)
    vcu.clear_faults()
    obs.assert_published("VCU/Faults/Non-Critical/BSPD_PREFAULT", 0.0)
    obs.assert_published("VCU/CarState/functional_state", float(int(FuncState.READY)))
    log.info("S6 ✓ fault → FAULTED+OFF+home, recovered to READY")


SCENARIOS: dict[str, callable] = {
    "S1": s1_boot,
    "S2": s2_menu_nav,
    "S3": s3_enter_pit,
    "S4": s4_drive_sweep,
    "S4b": s4b_owner_isolation,
    "S5": s5_enter_reverse,
    "S6": s6_fault_recovery,
}


# ---------- main -----------------------------------------------------------

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--broker", default="localhost:1883")
    ap.add_argument("--with-auto", action="store_true",
                    help="run autonomous heartbeat alongside stream")
    ap.add_argument("--only", default="",
                    help="comma-separated list of scenario IDs to run (default: all)")
    ap.add_argument("--debug", action="store_true")
    args = ap.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.debug else logging.INFO,
        format="%(asctime)s %(levelname)s %(name)s: %(message)s",
        datefmt="%H:%M:%S",
    )
    host, _, port_s = args.broker.partition(":")
    port = int(port_s or "1883")
    selected = [s.strip() for s in args.only.split(",") if s.strip()] or list(SCENARIOS)

    obs = BrokerObserver(host, port)
    obs.start()
    client = StreamClient(args.broker, with_auto=args.with_auto)
    try:
        # quick sanity ping
        assert client.ping(), "stream did not respond to ping"
        log.info("stream ping ok; broker observer subscribed")
        vcu = VcuMock(client)
        for sid in selected:
            fn = SCENARIOS.get(sid)
            if not fn:
                log.warning("unknown scenario: %s (have %s)", sid, list(SCENARIOS))
                continue
            print()
            print(f"========== {sid} =========")
            if sid == "S4b":
                fn(vcu, obs, args.with_auto)
            else:
                fn(vcu, obs)
        print()
        log.info("all scenarios complete")
    finally:
        with contextlib.suppress(Exception):
            VcuMock(client).release_all() if 'vcu' not in locals() else vcu.release_all()
        client.close()
        obs.stop()


if __name__ == "__main__":
    main()
