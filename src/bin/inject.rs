use std::collections::HashMap;
use std::io::{self, Write};
use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use calypso::{
    proto::serverdata,
    simulatable_message::{SimComponent, SimValue},
    simulate_data::create_simulated_components,
};
use clap::Parser;
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures_util::StreamExt;
use protobuf::Message;
use rand::prelude::*;
use rumqttc::v5::{AsyncClient, MqttOptions};
use tokio_util::sync::CancellationToken;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(version, about = "Interactive MQTT injection tool for manual testing")]
struct InjectArgs {
    /// Path to JSON key mapping file (maps single characters to MQTT topics)
    #[arg(short = 'k', long)]
    key_map: Option<String>,

    /// Siren broker host:port
    #[arg(
        short = 'u',
        long,
        env = "CALYPSO_SIREN_HOST_URL",
        default_value = "127.0.0.1:1883"
    )]
    siren_host_url: String,

    /// List all available topics and exit
    #[arg(long)]
    list_topics: bool,
}

fn parse_key_map(content: &str) -> Result<HashMap<char, String>, String> {
    let raw: HashMap<String, String> =
        serde_json::from_str(content).map_err(|e| format!("Invalid key map JSON: {e}"))?;
    let mut map = HashMap::new();
    for (key_str, topic) in raw {
        if key_str.len() != 1 {
            return Err(format!(
                "Key mapping keys must be single characters, got: '{key_str}'"
            ));
        }
        let ch = key_str.chars().next().unwrap();
        map.insert(ch, topic);
    }
    Ok(map)
}

fn load_key_map(path: &str) -> HashMap<char, String> {
    let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read key map file '{path}': {e}");
        exit(1);
    });
    parse_key_map(&content).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(1);
    })
}

fn build_topic_components(key_map: &HashMap<char, String>) -> HashMap<char, SimComponent> {
    let mut components = create_simulated_components();
    let mut result = HashMap::new();
    for (&key, topic) in key_map {
        if let Some(pos) = components.iter().position(|c| c.name == *topic) {
            result.insert(key, components.swap_remove(pos));
        } else {
            eprintln!("Warning: no simulated component for topic '{topic}' (key '{key}')");
        }
    }
    result
}

/// Generate a fresh random value within each point's defined bounds.
fn randomize_component(component: &mut SimComponent) {
    let mut rng = rand::rng();
    for point in &mut component.points {
        match &mut point.value {
            SimValue::Range {
                min,
                max,
                inc_min,
                round,
                current,
                ..
            } => {
                *current = rng.random_range(*min..*max);
                if *inc_min != 0.0 {
                    *current = (*current / *inc_min).round() * *inc_min;
                }
                if *round {
                    *current = current.round();
                }
            }
            SimValue::Discrete {
                options, current, ..
            } => {
                let idx = rng.random_range(0..options.len());
                *current = options[idx].0;
            }
        }
    }
}

async fn poll_stub(token: CancellationToken, mut eventloop: rumqttc::v5::EventLoop) {
    loop {
        tokio::select! {
            () = token.cancelled() => {
                debug!("MQTT poll shutting down");
                break;
            },
            _ = eventloop.poll() => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = InjectArgs::parse();

    if cli.list_topics {
        let components = create_simulated_components();
        println!("Available topics ({} total):", components.len());
        for component in &components {
            println!("  {} [{}]", component.name, component.unit);
        }
        return;
    }

    let key_map_path = cli.key_map.unwrap_or_else(|| {
        eprintln!("--key-map is required (use --list-topics to see available topics)");
        exit(1);
    });

    let key_map = load_key_map(&key_map_path);
    if key_map.is_empty() {
        eprintln!("Key map is empty");
        exit(1);
    }

    let mut components = build_topic_components(&key_map);
    if components.is_empty() {
        eprintln!("No matching topics found for any key mapping");
        exit(1);
    }

    // Set up MQTT connection
    let (host, port_str) = cli.siren_host_url.split_once(':').unwrap_or_else(|| {
        eprintln!("Invalid siren URL format, expected host:port");
        exit(1);
    });
    let port: u16 = port_str.parse().unwrap_or_else(|_| {
        eprintln!("Invalid port: {port_str}");
        exit(1);
    });

    let client_id = format!(
        "Calypso-Inject-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    );
    let mut mqtt_opts = MqttOptions::new(client_id, host, port);
    mqtt_opts
        .set_keep_alive(Duration::from_secs(20))
        .set_clean_start(true)
        .set_connection_timeout(3)
        .set_session_expiry_interval(Some(u32::MAX))
        .set_topic_alias_max(Some(600));
    let (client, eventloop) = AsyncClient::new(mqtt_opts, 600);

    let token = CancellationToken::new();
    let poll_handle = tokio::spawn(poll_stub(token.clone(), eventloop));

    // Print key mappings
    println!("Key Mappings:");
    let mut sorted_keys: Vec<_> = components.keys().copied().collect();
    sorted_keys.sort_unstable();
    for key in &sorted_keys {
        if let Some(component) = components.get(key) {
            println!("  {key} → {} [{}]", component.name, component.unit);
        }
    }
    println!();
    println!("Press mapped keys to inject. Ctrl+C to exit.");
    println!();

    enable_raw_mode().expect("Failed to enable raw mode");

    let mut reader = EventStream::new();

    loop {
        match reader.next().await {
            Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }))) if modifiers.contains(KeyModifiers::CONTROL) => {
                break;
            }
            Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                kind: KeyEventKind::Press,
                ..
            }))) => {
                if let Some(component) = components.get_mut(&ch) {
                    randomize_component(component);
                    let data = component.get_decode_data();

                    let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
                    let mut payload = serverdata::ServerData::new();
                    payload.unit.clone_from(&data.unit);
                    payload.values.clone_from(&data.value);
                    payload.time_us = timestamp;

                    let topic = &data.topic;
                    let values_str: Vec<String> =
                        data.value.iter().map(|v| format!("{v:.2}")).collect();

                    if let Ok(bytes) = payload.write_to_bytes() {
                        match client
                            .publish(
                                topic.as_str(),
                                rumqttc::v5::mqttbytes::QoS::AtMostOnce,
                                false,
                                bytes,
                            )
                            .await
                        {
                            Ok(()) => {
                                print!(
                                    "[{ch}] {topic} = [{}] {}\r\n",
                                    values_str.join(", "),
                                    data.unit
                                );
                                io::stdout().flush().ok();
                            }
                            Err(e) => {
                                print!("[{ch}] publish error: {e}\r\n");
                                io::stdout().flush().ok();
                            }
                        }
                    }
                }
            }
            Some(Err(_)) | None => break,
            _ => {}
        }
    }

    disable_raw_mode().expect("Failed to disable raw mode");
    println!("\r\nShutting down...");

    token.cancel();
    poll_handle.await.ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_key_map() {
        let map = parse_key_map(r#"{"a": "BMS/Pack/Voltage", "b": "BMS/Pack/Current"}"#).unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map[&'a'], "BMS/Pack/Voltage");
        assert_eq!(map[&'b'], "BMS/Pack/Current");
    }

    #[test]
    fn parse_key_map_rejects_multi_char_keys() {
        let result = parse_key_map(r#"{"ab": "topic"}"#);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("single characters"));
    }

    #[test]
    fn parse_key_map_rejects_invalid_json() {
        let result = parse_key_map("not json");
        assert!(result.is_err());
    }

    #[test]
    fn parse_key_map_empty() {
        let map = parse_key_map("{}").unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn build_topic_components_filters_to_mapped_keys() {
        let components = create_simulated_components();
        if components.is_empty() {
            return;
        }
        let target_name = components[0].name.clone();
        let mut key_map = HashMap::new();
        key_map.insert('x', target_name.clone());

        let filtered = build_topic_components(&key_map);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[&'x'].name, target_name);
    }

    #[test]
    fn build_topic_components_skips_unknown_topics() {
        let mut key_map = HashMap::new();
        key_map.insert('z', "NonExistent/Topic/Name".to_string());

        let filtered = build_topic_components(&key_map);
        assert!(filtered.is_empty());
    }

    #[test]
    fn randomize_component_stays_within_bounds() {
        let mut components = create_simulated_components();
        for component in &mut components {
            for _ in 0..10 {
                randomize_component(component);
                for point in &component.points {
                    match &point.value {
                        SimValue::Range {
                            min, max, current, ..
                        } => {
                            assert!(
                                *current >= *min,
                                "value {current} below min {min} for topic {}",
                                component.name
                            );
                            assert!(
                                *current <= *max,
                                "value {current} above max {max} for topic {}",
                                component.name
                            );
                        }
                        SimValue::Discrete {
                            options, current, ..
                        } => {
                            assert!(
                                options.iter().any(|(v, _)| (*v - *current).abs() < 0.001),
                                "discrete value {current} not in options for topic {}",
                                component.name
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn key_resolves_to_correct_topic() {
        let components = create_simulated_components();
        if components.len() < 2 {
            return;
        }
        let topic_a = components[0].name.clone();
        let topic_b = components[1].name.clone();

        let mut key_map = HashMap::new();
        key_map.insert('a', topic_a.clone());
        key_map.insert('b', topic_b.clone());

        let filtered = build_topic_components(&key_map);
        assert_eq!(filtered[&'a'].name, topic_a);
        assert_eq!(filtered[&'b'].name, topic_b);
    }
}
