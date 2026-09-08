use std::time::UNIX_EPOCH;

use crate::proto::serverdata;
use protobuf::Message;
use rumqttc::v5::AsyncClient;
use rumqttc::v5::mqttbytes::QoS;
use zenoh::{Session, bytes::Encoding};

/// The wire the simulator publishes on. Chosen once at startup (`--zenoh`) and
/// then cloned into every mode, so no mode has to know which one it got.
///
/// Both variants are cheap to clone: `AsyncClient` clones an internal channel
/// handle, and `Session` is reference-counted.
#[derive(Clone)]
pub enum Transport {
    Mqtt(AsyncClient),
    Zenoh(Session),
}

impl Transport {
    /// Hand `bytes` to the underlying wire under `topic`.
    ///
    /// Note the two are not symmetric in delivery: the MQTT arm only *enqueues*
    /// (the eventloop task does the socket write, hence the shutdown drain in
    /// `main`), while the Zenoh arm has published by the time it returns.
    async fn send(&self, topic: &str, bytes: Vec<u8>) -> Result<(), String> {
        match self {
            Self::Mqtt(client) => client
                .publish(topic, QoS::AtMostOnce, false, bytes)
                .await
                .map_err(|e| format!("publish: {e}")),
            Self::Zenoh(session) => session
                .put(topic, bytes)
                .encoding(Encoding::APPLICATION_PROTOBUF)
                .await
                .map_err(|e| format!("publish: {e}")),
        }
    }
}

/// Encode a `ServerData` payload for `unit`/`values`, stamped with the current
/// time (microseconds since the UNIX epoch). Returns the serialized bytes and
/// that timestamp. Split out from [`publish_data`] so the encoding can be unit
/// tested without a broker or client.
fn encode_server_data(unit: &str, values: &[f32]) -> Result<(Vec<u8>, u64), String> {
    let timestamp = UNIX_EPOCH.elapsed().map_or(0, |d| d.as_micros() as u64);

    let mut payload = serverdata::ServerData::new();
    payload.unit = unit.to_string();
    payload.values = values.to_vec();
    payload.time_us = timestamp;

    let bytes = payload
        .write_to_bytes()
        .map_err(|e| format!("serialize: {e}"))?;

    Ok((bytes, timestamp))
}

/// Encode a `ServerData` payload and publish it on `transport`. Returns the
/// timestamp (microseconds since UNIX epoch) embedded in the payload.
pub async fn publish_data(
    transport: &Transport,
    topic: &str,
    unit: &str,
    values: &[f32],
) -> Result<u64, String> {
    let (bytes, timestamp) = encode_server_data(unit, values)?;

    transport.send(topic, bytes).await?;

    Ok(timestamp)
}

/// Resolve a publish request's `value`/`values` into the concrete values to send.
///
/// One rule for every input path (scenario load, replay, and the stream RPC):
/// exactly one of `value` (scalar) or `values` (array) must be set, and `values`
/// must be non-empty. On violation returns a bare reason; callers add context.
pub fn resolve_values(value: Option<f32>, values: Option<&[f32]>) -> Result<Vec<f32>, String> {
    match (value, values) {
        (Some(_), Some(_)) => Err("set exactly one of `value` or `values`, not both".into()),
        (Some(v), None) => Ok(vec![v]),
        (None, Some(vs)) if !vs.is_empty() => Ok(vs.to_vec()),
        (None, Some(_)) => Err("`values` must be non-empty".into()),
        (None, None) => Err("set `value` or `values`".into()),
    }
}
