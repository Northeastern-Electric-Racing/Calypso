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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_round_trips_unit_values_and_timestamp() {
        let (bytes, ts) = encode_server_data("mph", &[1.5, -2.0, 3.25]).unwrap();
        let decoded = serverdata::ServerData::parse_from_bytes(&bytes).unwrap();
        assert_eq!(decoded.unit, "mph");
        assert_eq!(decoded.values, vec![1.5, -2.0, 3.25]);
        assert_eq!(decoded.time_us, ts);
        assert!(ts > 0, "timestamp should be a real epoch time");
    }

    #[test]
    fn encode_handles_empty_unit_and_values() {
        let (bytes, _) = encode_server_data("", &[]).unwrap();
        let decoded = serverdata::ServerData::parse_from_bytes(&bytes).unwrap();
        assert!(decoded.unit.is_empty());
        assert!(decoded.values.is_empty());
    }
}
