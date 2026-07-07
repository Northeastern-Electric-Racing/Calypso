//! Scenario tests that mimic what the Cerberus-2.0 VCU firmware
//! (`Core/Src/u_statemachine.c`) publishes, driving `calypso-sim --stream`.
//!
//! Ported from the former `scripts/vcu_mimic.py`. This first cut covers the
//! parts verifiable without a broker: topic **ownership** (claim / status /
//! silence / release) and the `VcuMock` **state machine** (the brake + shutdown
//! gate on entering a drive mode). The end-to-end "these exact values landed on
//! the topic" checks — Python's `assert_published` — need a live broker (Siren,
//! in the Docker compose stack) and are intentionally out of scope here; value
//! *encoding* is covered by the `encode_server_data` unit test in
//! `src/publish.rs`.
#![allow(dead_code)] // full firmware enums are mirrored; not every variant is exercised yet.
mod common;

use std::collections::HashMap;

use common::StreamHarness;

// --- constants mirrored from Cerberus-2.0 ---------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FuncState {
    Ready = 0,
    FPit = 1,
    FReverse = 2,
    FPerformance = 3,
    FEfficiency = 4,
    Faulted = 5,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NeroMenu {
    Off = 0,
    Pit = 1,
    Reverse = 2,
    Performance = 3,
    Efficiency = 4,
    Games = 5,
    Themes = 6,
    Exit = 7,
}

const MAX_NERO_STATES: u8 = 8;
const PEDAL_BRAKE_THRESH: f64 = 0.12;

// Pedal calibration from Cerberus-2.0 Core/Src/u_pedals.c
const MIN_APPS1_V: f64 = 2.15;
const MAX_APPS1_V: f64 = 3.12;
const MIN_APPS2_V: f64 = 1.15;
const MAX_APPS2_V: f64 = 2.01;
const MIN_BRAKE1_V: f64 = 0.5;
const MAX_BRAKE1_V: f64 = 1.5;
const MIN_BRAKE2_V: f64 = 0.5;
const MAX_BRAKE2_V: f64 = 1.5;

const CRITICAL_FAULTS: &[&str] = &[
    "CAN_OUTGOING_FAULT",
    "CAN_INCOMING_FAULT",
    "BMS_CAN_MONITOR_FAULT",
    "LIGHTNING_CAN_MONITOR_FAULT",
    "SHUTDOWN_FAULT",
];
const NON_CRITICAL_FAULTS: &[&str] = &[
    "ONBOARD_TEMP_FAULT",
    "IMU_ACCEL_FAULT",
    "IMU_GYRO_FAULT",
    "BSPD_PREFAULT",
    "ONBOARD_BRAKE_OPEN_CIRCUIT_FAULT",
    "ONBOARD_ACCEL_OPEN_CIRCUIT_FAULT",
    "ONBOARD_BRAKE_SHORT_CIRCUIT_FAULT",
    "ONBOARD_ACCEL_SHORT_CIRCUIT_FAULT",
    "ONBOARD_PEDAL_DIFFERENCE_FAULT",
    "RTDS_FAULT",
    "LV_LOW_VOLTAGE_FAULT",
];

/// Fixed VCU CarState/Pedals topics the mock owns (fault topics are appended —
/// see [`VcuMock::owned_topics`]).
const CARSTATE_PEDAL_TOPICS: &[&str] = &[
    "VCU/CarState/home_mode",
    "VCU/CarState/nero_index",
    "VCU/CarState/speed",
    "VCU/CarState/tsms",
    "VCU/CarState/torque_limit_percentage",
    "VCU/CarState/not_in_reverse",
    "VCU/CarState/regen_limit",
    "VCU/CarState/launch_control",
    "VCU/CarState/functional_state",
    "VCU/CarState/traction_control",
    "VCU/Pedals/Percentages/acceleration_pedal",
    "VCU/Pedals/Percentages/brake_pedal",
    "VCU/Pedals/Voltages/accel_1",
    "VCU/Pedals/Voltages/accel_2",
    "VCU/Pedals/Voltages/brake_1",
    "VCU/Pedals/Voltages/brake_2",
];

/// Firmware publishes booleans as 1.0 / 0.0 floats.
fn bit(flag: bool) -> f64 {
    if flag { 1.0 } else { 0.0 }
}

// --- VcuMock ---------------------------------------------------------------

/// A minimal mirror of the VCU state machine that drives the sim over the
/// stream protocol. Owns the spawned sim in `sim`.
struct VcuMock {
    sim: StreamHarness,
    functional: FuncState,
    nero_index: u8,
    home_mode: bool,
    accel: f64,
    brake: f64,
    tsms: bool,
    speed: f64,
}

impl VcuMock {
    fn new(sim: StreamHarness) -> Self {
        VcuMock {
            sim,
            functional: FuncState::Ready,
            nero_index: NeroMenu::Off as u8,
            home_mode: true,
            accel: 0.0,
            brake: 0.0,
            tsms: false,
            speed: 0.0,
        }
    }

    /// Every topic the mock publishes, in claim order.
    fn owned_topics() -> Vec<String> {
        let mut out: Vec<String> = CARSTATE_PEDAL_TOPICS
            .iter()
            .map(|t| (*t).to_string())
            .collect();
        for f in CRITICAL_FAULTS {
            out.push(format!("VCU/Faults/Critical/{f}"));
        }
        for f in NON_CRITICAL_FAULTS {
            out.push(format!("VCU/Faults/Non-Critical/{f}"));
        }
        out
    }

    fn claim_all(&mut self) {
        for topic in Self::owned_topics() {
            self.sim.claim(&topic);
        }
    }

    fn publish_carstate(&mut self) {
        self.sim
            .publish("VCU/CarState/home_mode", bit(self.home_mode));
        self.sim
            .publish("VCU/CarState/nero_index", f64::from(self.nero_index));
        self.sim
            .publish_unit("VCU/CarState/speed", self.speed, "mph");
        self.sim.publish("VCU/CarState/tsms", bit(self.tsms));
        self.sim
            .publish("VCU/CarState/torque_limit_percentage", 1.0);
        self.sim.publish(
            "VCU/CarState/not_in_reverse",
            bit(self.functional != FuncState::FReverse),
        );
        self.sim.publish_unit("VCU/CarState/regen_limit", 50.0, "A");
        self.sim.publish("VCU/CarState/launch_control", 0.0);
        self.sim.publish(
            "VCU/CarState/functional_state",
            f64::from(self.functional as u8),
        );
        self.sim.publish("VCU/CarState/traction_control", 0.0);
    }

    fn publish_pedals(&mut self) {
        let (a, b) = (self.accel, self.brake);
        self.sim
            .publish("VCU/Pedals/Percentages/acceleration_pedal", a);
        self.sim.publish("VCU/Pedals/Percentages/brake_pedal", b);
        self.sim.publish_unit(
            "VCU/Pedals/Voltages/accel_1",
            MIN_APPS1_V + a * (MAX_APPS1_V - MIN_APPS1_V),
            "V",
        );
        self.sim.publish_unit(
            "VCU/Pedals/Voltages/accel_2",
            MIN_APPS2_V + a * (MAX_APPS2_V - MIN_APPS2_V),
            "V",
        );
        self.sim.publish_unit(
            "VCU/Pedals/Voltages/brake_1",
            MIN_BRAKE1_V + b * (MAX_BRAKE1_V - MIN_BRAKE1_V),
            "V",
        );
        self.sim.publish_unit(
            "VCU/Pedals/Voltages/brake_2",
            MIN_BRAKE2_V + b * (MAX_BRAKE2_V - MIN_BRAKE2_V),
            "V",
        );
    }

    fn publish_faults(&mut self) {
        for f in CRITICAL_FAULTS {
            self.sim.publish(&format!("VCU/Faults/Critical/{f}"), 0.0);
        }
        for f in NON_CRITICAL_FAULTS {
            self.sim
                .publish(&format!("VCU/Faults/Non-Critical/{f}"), 0.0);
        }
    }

    /// Reset to power-on state and publish a full frame.
    fn boot(&mut self) {
        self.functional = FuncState::Ready;
        self.nero_index = NeroMenu::Off as u8;
        self.home_mode = true;
        self.accel = 0.0;
        self.brake = 0.0;
        self.tsms = false;
        self.speed = 0.0;
        self.publish_carstate();
        self.publish_pedals();
        self.publish_faults();
    }

    fn menu_increment(&mut self) {
        let next = self.nero_index + 1;
        self.nero_index = if next >= MAX_NERO_STATES { 0 } else { next };
        self.sim
            .publish("VCU/CarState/nero_index", f64::from(self.nero_index));
    }

    fn set_pedals(&mut self, accel: f64, brake: f64) {
        self.accel = accel.clamp(0.0, 1.0);
        self.brake = brake.clamp(0.0, 1.0);
        self.publish_pedals();
    }

    fn set_tsms(&mut self, closed: bool) {
        self.tsms = closed;
        self.sim.publish("VCU/CarState/tsms", bit(closed));
    }

    /// Try to enter the highlighted drive mode. Mirrors
    /// `transition_functional_state`: PIT / PERFORMANCE / EFFICIENCY require
    /// brake pressed + shutdown closed; REVERSE is gateless. Returns whether a
    /// transition happened.
    fn menu_select(&mut self) -> bool {
        let target = match self.nero_index {
            x if x == NeroMenu::Pit as u8 => Some(FuncState::FPit),
            x if x == NeroMenu::Performance as u8 => Some(FuncState::FPerformance),
            x if x == NeroMenu::Efficiency as u8 => Some(FuncState::FEfficiency),
            x if x == NeroMenu::Reverse as u8 => Some(FuncState::FReverse),
            _ => None,
        };
        let Some(target) = target else {
            return false;
        };
        // REVERSE is gateless; the others need brake + shutdown closed.
        if target != FuncState::FReverse && (self.brake < PEDAL_BRAKE_THRESH || !self.tsms) {
            return false;
        }
        self.functional = target;
        self.home_mode = false;
        self.sim.publish("VCU/CarState/home_mode", 0.0);
        self.sim.publish(
            "VCU/CarState/functional_state",
            f64::from(self.functional as u8),
        );
        self.sim.publish(
            "VCU/CarState/not_in_reverse",
            bit(self.functional != FuncState::FReverse),
        );
        true
    }
}

// --- scenarios -------------------------------------------------------------

/// S1: boot — claim every VCU topic, confirm the registry marks them all
/// `stream`-owned, then publish the initial frame (each publish must be
/// accepted).
#[test]
fn s1_boot_claims_all_topics_and_publishes_initial_frame() {
    let mut vcu = VcuMock::new(StreamHarness::spawn());
    vcu.claim_all();

    let owners: HashMap<String, String> = vcu
        .sim
        .status()
        .into_iter()
        .map(|o| {
            (
                o["topic"].as_str().unwrap_or_default().to_string(),
                o["owner"].as_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    for topic in VcuMock::owned_topics() {
        assert_eq!(
            owners.get(&topic).map(String::as_str),
            Some("stream"),
            "claim missing for {topic}"
        );
    }

    // A full boot frame publishes cleanly (publish() panics on any RPC error).
    vcu.boot();
}

/// S3: entering PIT is gated on brake pressed + shutdown closed. Mirrors
/// `transition_functional_state`. Verifiable without a broker because the gate
/// lives in the mock's state machine.
#[test]
fn s3_enter_pit_is_gated_on_brake_and_shutdown() {
    let mut vcu = VcuMock::new(StreamHarness::spawn());
    vcu.claim_all();

    vcu.menu_increment(); // OFF -> PIT
    assert_eq!(vcu.nero_index, NeroMenu::Pit as u8);

    // No brake, shutdown open: rejected, no transition.
    assert!(
        !vcu.menu_select(),
        "PIT must be refused without brake + shutdown"
    );
    assert_eq!(vcu.functional, FuncState::Ready);

    // Press brake and close shutdown: now it enters.
    vcu.set_pedals(0.0, 0.30);
    vcu.set_tsms(true);
    assert!(
        vcu.menu_select(),
        "PIT should enter once gated conditions met"
    );
    assert_eq!(vcu.functional, FuncState::FPit);
    assert!(!vcu.home_mode, "entering a drive mode leaves home_mode");
}

/// S4b: ownership isolation. With the autonomous heartbeat running, a claimed
/// topic is `stream`-owned (so `auto_may_publish` is false and the heartbeat
/// yields), a silenced topic rejects even driver publishes, and releasing
/// restores them. The end-to-end "heartbeat didn't republish" observation
/// needs a live broker (Siren) and is out of scope here.
#[test]
fn s4b_ownership_isolation_through_claim_silence_release() {
    let mut vcu = VcuMock::new(StreamHarness::spawn_with_auto());
    let topic = "VCU/CarState/torque_limit_percentage";

    vcu.sim.claim(topic);
    assert_eq!(
        vcu.sim.owner_of(topic).as_deref(),
        Some("stream"),
        "claim must mark the topic stream-owned so autonomous yields"
    );
    assert!(
        vcu.sim.publish(topic, 0.42)["ts_us"].as_u64().unwrap_or(0) > 0,
        "driver publish accepted while claimed"
    );

    vcu.sim.silence(topic);
    assert_eq!(
        vcu.sim.publish(topic, 0.99)["skipped"].as_str(),
        Some("silenced"),
        "silenced topics reject even the driver"
    );

    vcu.sim.release(topic);
    assert_eq!(
        vcu.sim.owner_of(topic),
        None,
        "release returns the topic to auto (no override)"
    );
    assert!(
        vcu.sim.publish(topic, 0.5)["ts_us"].as_u64().unwrap_or(0) > 0,
        "driver publish accepted again after release"
    );
}
