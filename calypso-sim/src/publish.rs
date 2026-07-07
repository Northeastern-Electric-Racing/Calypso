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
    let timestamp = UNIX_EPOCH
        .elapsed()
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0);

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
