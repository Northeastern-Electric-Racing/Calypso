use std::collections::HashMap;
use std::io::{self, Write};
use std::process::exit;
use std::time::{Duration, UNIX_EPOCH};

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
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use tokio_util::sync::CancellationToken;

#[derive(Parser, Debug)]
#[command(version, about = "Interactive MQTT injection tool for manual testing")]
struct InjectArgs {
    /// Path to JSON key mapping file (maps single characters to MQTT topics)
    #[arg(short = 'k', long, required_unless_present = "list_topics")]
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

/// RAII guard that enables raw mode on creation and restores on drop.
struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

fn parse_key_map(content: &str) -> Result<HashMap<char, String>, String> {
    let raw: HashMap<String, String> =
        serde_json::from_str(content).map_err(|e| format!("Invalid key map JSON: {e}"))?;
    let mut map = HashMap::new();
    for (key_str, topic) in raw {
        let mut chars = key_str.chars();
        let (Some(ch), None) = (chars.next(), chars.next()) else {
            return Err(format!(
                "Key mapping keys must be single characters, got: '{key_str}'"
            ));
        };
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
                *current = current.clamp(*min, *max);
            }
            SimValue::Discrete {
                options, current, ..
            } => {
                *current = options.choose(&mut rng).unwrap().0;
            }
        }
    }
}

async fn poll_stub(token: CancellationToken, mut eventloop: rumqttc::v5::EventLoop) {
    loop {
        tokio::select! {
            () = token.cancelled() => break,
            result = eventloop.poll() => {
                if let Err(e) = result {
                    print!("MQTT connection error: {e}\r\n");
                    io::stdout().flush().ok();
                }
            }
        }
    }
}

/// Randomize a component's value and publish it to the MQTT broker.
async fn publish_injection(ch: char, component: &mut SimComponent, client: &AsyncClient) {
    randomize_component(component);
    let data = component.get_decode_data();

    let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
    let mut payload = serverdata::ServerData::new();
    payload.unit.clone_from(&data.unit);
    payload.values.clone_from(&data.value);
    payload.time_us = timestamp;

    let Ok(bytes) = payload.write_to_bytes() else {
        print!("[{ch}] serialization error for {}\r\n", data.topic);
        io::stdout().flush().ok();
        return;
    };
    match client
        .publish(&data.topic, QoS::AtMostOnce, false, bytes)
        .await
    {
        Ok(()) => {
            let values_str: Vec<String> = data.value.iter().map(|v| format!("{v:.2}")).collect();
            print!(
                "[{ch}] {} = [{}] {}\r\n",
                data.topic,
                values_str.join(", "),
                data.unit
            );
        }
        Err(e) => {
            print!("[{ch}] publish error: {e}\r\n");
        }
    }
    io::stdout().flush().ok();
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

    // clap enforces key_map is present when list_topics is absent
    let key_map_path = cli.key_map.unwrap();

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
        UNIX_EPOCH.elapsed().expect("Time went backwards").as_millis()
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

    println!("Key Mappings:");
    let mut sorted_keys: Vec<_> = components.keys().copied().collect();
    sorted_keys.sort_unstable();
    for key in &sorted_keys {
        let component = &components[key];
        println!("  {key} → {} [{}]", component.name, component.unit);
    }
    println!();
    println!("Press mapped keys to inject. Ctrl+C to exit.");
    println!();

    let guard = RawModeGuard::new().expect("Failed to enable raw mode");

    let mut reader = EventStream::new();

    loop {
        match reader.next().await {
            Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }))) if modifiers.contains(KeyModifiers::CONTROL) => break,
            Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                kind: KeyEventKind::Press,
                ..
            }))) => {
                if let Some(component) = components.get_mut(&ch) {
                    publish_injection(ch, component, &client).await;
                }
            }
            Some(Err(e)) => {
                print!("Terminal event error: {e}\r\n");
                io::stdout().flush().ok();
                break;
            }
            None => break,
            _ => {}
        }
    }

    drop(guard);
    println!("\r\nShutting down...");

    token.cancel();
    poll_handle.await.ok();
}