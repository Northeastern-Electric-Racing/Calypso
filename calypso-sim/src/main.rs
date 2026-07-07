mod cli;
mod data;
mod keymap;
mod modes;
#[allow(clippy::all, clippy::pedantic)]
mod proto;
mod publish;
mod raw_mode;
mod registry;
mod simulatable_message;
mod simulate_data;
mod warnings;

#[cfg(test)]
mod tests;

use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::simulate_data::create_simulated_components;
use clap::Parser;
use rumqttc::v5::{AsyncClient, EventLoop, MqttOptions};
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

use cli::Cli;
use registry::TopicRegistry;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    init_tracing();

    if cli.list_topics {
        list_topics_and_exit();
    }

    warnings::print_unsimulated();

    let (client, eventloop) = connect_mqtt(&cli.siren_host_url).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        exit(1);
    });

    let token = CancellationToken::new();
    let poll_handle = tokio::spawn(modes::poll_eventloop(token.clone(), eventloop));

    let registry = TopicRegistry::shared();

    let auto_handle = if cli.run_autonomous() {
        // Validate the enable/disable regex patterns up front so a bad pattern
        // fails fast with a non-zero exit instead of silently disabling the
        // entire autonomous heartbeat inside the spawned task.
        let filter = modes::autonomous::FilterMode::build(&cli.enable_topic, &cli.disable_topic)
            .unwrap_or_else(|err| {
                eprintln!("Error: {err}");
                exit(1);
            });
        Some(tokio::spawn(modes::autonomous::run(
            token.clone(),
            client.clone(),
            registry.clone(),
            filter,
        )))
    } else {
        None
    };

    let foreground = run_foreground(&cli, &token, &client, &registry).await;

    // Let the MQTT eventloop drain any just-enqueued publishes before we cancel
    // it. `AsyncClient::publish` only enqueues; the eventloop's `poll()` is what
    // writes to the socket. Cancelling first drops the eventloop with the queue
    // unflushed — best-effort for QoS0, but this lets the last messages of a
    // clean stream-EOF / Ctrl+C shutdown actually land.
    tokio::time::sleep(Duration::from_millis(50)).await;

    token.cancel();
    if let Some(h) = auto_handle
        && let Err(e) = h.await
    {
        tracing::error!("autonomous task panicked: {e}");
    }
    if let Err(e) = poll_handle.await {
        tracing::error!("MQTT eventloop task panicked: {e}");
    }

    if let Err(err) = foreground {
        eprintln!("Error: {err}");
        exit(1);
    }
}

async fn run_foreground(
    cli: &Cli,
    token: &CancellationToken,
    client: &AsyncClient,
    registry: &registry::SharedRegistry,
) -> Result<(), String> {
    if cli.stream {
        modes::stream::run(token.clone(), client.clone(), registry.clone()).await
    } else if let Some(script_path) = &cli.script {
        // clap enforces `--script requires --key-map` (see cli.rs), so a missing
        // key map here is an impossible state, not a reachable runtime error.
        let key_map_path = cli
            .key_map
            .as_deref()
            .expect("clap enforces --script requires --key-map");
        modes::auto_script::run(client.clone(), key_map_path, script_path, registry.clone()).await
    } else if let Some(key_map_path) = &cli.key_map {
        modes::interactive::run(
            token.clone(),
            client.clone(),
            key_map_path,
            registry.clone(),
        )
        .await
    } else {
        // Pure --auto: wait for SIGINT, then exit.
        tokio::signal::ctrl_c()
            .await
            .map_err(|e| format!("ctrl+c handler failed: {e}"))
    }
}

fn init_tracing() {
    // Tracing always writes to stderr so stdout stays clean for stream mode
    // and keymap-mode logs.
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}

fn list_topics_and_exit() -> ! {
    let components = create_simulated_components();
    println!("Available topics ({} total):", components.len());
    for c in &components {
        println!("  {} [{}]", c.name, c.unit);
    }
    exit(0);
}

fn connect_mqtt(host_url: &str) -> Result<(AsyncClient, EventLoop), String> {
    let (host, port_str) = host_url
        .split_once(':')
        .ok_or_else(|| format!("Invalid broker URL '{host_url}', expected host:port"))?;
    let port: u16 = port_str
        .parse()
        .map_err(|_| format!("Invalid port: {port_str}"))?;

    let client_id = format!(
        "Calypso-Sim-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    );

    let mut mqtt_opts = MqttOptions::new(client_id, host, port);
    mqtt_opts
        .set_keep_alive(Duration::from_secs(20))
        .set_clean_start(true)
        .set_connection_timeout(3)
        .set_session_expiry_interval(Some(u32::MAX));
    Ok(AsyncClient::new(mqtt_opts, 600))
}
