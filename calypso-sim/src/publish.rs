use std::time::UNIX_EPOCH;

use crate::proto::serverdata;
use protobuf::Message;
use rumqttc::v5::AsyncClient;
use rumqttc::v5::mqttbytes::QoS;

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

/// Encode a `ServerData` payload and publish it to the broker. Returns the
/// timestamp (microseconds since UNIX epoch) embedded in the payload.
pub async fn publish_data(
    client: &AsyncClient,
    topic: &str,
    unit: &str,
    values: &[f32],
) -> Result<u64, String> {
    let (bytes, timestamp) = encode_server_data(unit, values)?;

    client
        .publish(topic, QoS::AtMostOnce, false, bytes)
        .await
        .map_err(|e| format!("publish: {e}"))?;

    Ok(timestamp)
}

/// Resolve a publish request's `value`/`values` into the concrete values to send.
///
/// One rule for every input path (scenario load, replay, and the stream RPC):
/// exactly one of `value` (scalar) or `values` (array) must be set, and `values`
/// must be non-empty. On violation returns a bare reason; callers add context.
pub fn resolve_values(value: Option<f32>, values: Option<Vec<f32>>) -> Result<Vec<f32>, String> {
    match (value, values) {
        (Some(_), Some(_)) => Err("set exactly one of `value` or `values`, not both".into()),
        (Some(v), None) => Ok(vec![v]),
        (None, Some(vs)) if !vs.is_empty() => Ok(vs),
        (None, Some(_)) => Err("`values` must be non-empty".into()),
        (None, None) => Err("set `value` or `values`".into()),
    }
}
