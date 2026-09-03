mod cli;
mod keymap;
mod modes;
mod ownership;
#[allow(clippy::all, clippy::pedantic)]
mod proto;
mod publish;
mod raw_mode;
mod simulatable_message;
mod simulate_data;
mod warnings;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::publish::Transport;
use crate::simulate_data::create_simulated_components;
use clap::Parser;
use rumqttc::v5::{AsyncClient, EventLoop, MqttOptions};
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    init_tracing();

    if cli.list_topics {
        list_topics_and_exit();
    }

    warnings::print_unsimulated();

    // Load the scenario once, up front (for --key-map / --play). This fails fast
    // on a bad file, and lets us reserve the scenario's topics from the mock
    // heartbeat before anything publishes (ownership is a startup partition, not
    // a runtime negotiation — see `ownership`).
    let scenario = cli.key_map.as_deref().map(|path| {
        keymap::load_scenario(path).unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            exit(1);
        })
    });

    let token = CancellationToken::new();

    let (transport, eventloop) = connect_transport(&cli).await.unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        exit(1);
    });

    // MQTT only: publishes are enqueued by the client and written to the socket
    // by this task. Zenoh publishes inline, so it has no eventloop to drive.
    let poll_handle = eventloop.map(|el| tokio::spawn(modes::poll_eventloop(token.clone(), el)));

    let mock_handle = spawn_mock(&cli, scenario.as_ref(), &transport, &token);

    let foreground = run_foreground(&cli, &token, &transport, scenario).await;

    if let Some(poll_handle) = poll_handle {
        // Let the MQTT eventloop drain any just-enqueued publishes before we
        // cancel it. `AsyncClient::publish` only enqueues; the eventloop's
        // `poll()` is what writes to the socket. Cancelling first drops the
        // eventloop with the queue unflushed — best-effort for QoS0, but this
        // lets the last messages of a clean stream-EOF / Ctrl+C shutdown
        // actually land.
        tokio::time::sleep(Duration::from_millis(50)).await;

        token.cancel();
        if let Err(e) = poll_handle.await {
            tracing::error!("MQTT eventloop task panicked: {e}");
        }
    } else {
        token.cancel();
    }

    if let Some(h) = mock_handle
        && let Err(e) = h.await
    {
        tracing::error!("mock task panicked: {e}");
    }

    if let Err(err) = foreground {
        eprintln!("Error: {err}");
        exit(1);
    }
}

async fn run_foreground(
    cli: &Cli,
    token: &CancellationToken,
    transport: &Transport,
    scenario: Option<keymap::Scenario>,
) -> Result<(), String> {
    if cli.stream {
        modes::stream::run(token.clone(), transport.clone()).await
    } else if let Some(action) = &cli.play {
        // A missing scenario here is an impossible state, not a runtime error.
        let scenario = scenario.expect("clap enforces --play requires --key-map");
        modes::replay::run(transport.clone(), scenario, action).await
    } else if cli.key_map.is_some() {
        let scenario = scenario.expect("--key-map implies main loaded the scenario");
        modes::interactive::run(token.clone(), transport.clone(), scenario).await
    } else {
        // Pure --mock: wait for SIGINT, then exit.
        tokio::signal::ctrl_c()
            .await
            .map_err(|e| format!("ctrl+c handler failed: {e}"))
    }
}

/// If the mock heartbeat is enabled, resolve its share of the topic space
/// against the driver (a scenario's topics, if any), print the split, and spawn
/// the task. Exits on a bad enable/disable pattern — fail fast, before spawning
/// (rather than silently disabling the heartbeat inside the spawned task).
fn spawn_mock(
    cli: &Cli,
    scenario: Option<&keymap::Scenario>,
    transport: &Transport,
    token: &CancellationToken,
) -> Option<tokio::task::JoinHandle<()>> {
    if !cli.run_mock() {
        return None;
    }
    let filter = ownership::FilterMode::build(&cli.enable_topic, &cli.disable_topic)
        .unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            exit(1);
        });
    let driver_owned = scenario.map(keymap::scenario_topics).unwrap_or_default();
    let partition = ownership::Partition::resolve(&filter, driver_owned);
    partition.print_summary();
    Some(tokio::spawn(modes::mock::run(
        token.clone(),
        transport.clone(),
        partition.heartbeat,
    )))
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

/// Open the transport chosen on the command line.
///
/// Returns the [`Transport`] plus, for MQTT only, the `EventLoop` that `main`
/// must drive for publishes to reach the socket. `None` means there is nothing
/// to drive (Zenoh publishes inline).
async fn connect_transport(cli: &Cli) -> Result<(Transport, Option<EventLoop>), String> {
    if cli.zenoh {
        let session = connect_zenoh(cli.zenoh_conf.as_deref()).await?;
        return Ok((Transport::Zenoh(session), None));
    }
    if let Some(path) = &cli.zenoh_conf {
        tracing::warn!(
            "Ignoring --zenoh-conf '{}': the transport is MQTT. Pass --zenoh to use it.",
            path.display()
        );
    }
    let (client, eventloop) = connect_mqtt(&cli.siren_host_url)?;
    Ok((Transport::Mqtt(client), Some(eventloop)))
}

/// Open a Zenoh session. With no `--zenoh-conf` the Zenoh defaults are used, so
/// the sim runs without a conf file; an explicit path must load and parse.
async fn connect_zenoh(conf_path: Option<&Path>) -> Result<zenoh::Session, String> {
    zenoh::init_log_from_env_or("warn");

    let config = match conf_path {
        Some(path) => zenoh::Config::from_file(path)
            .map_err(|e| format!("Invalid Zenoh conf '{}': {e}", path.display()))?,
        None => zenoh::Config::default(),
    };

    zenoh::open(config)
        .await
        .map_err(|e| format!("Could not open Zenoh session: {e}"))
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
            .map_or(0, |d| d.as_millis())
    );

    let mut mqtt_opts = MqttOptions::new(client_id, host, port);
    mqtt_opts
        .set_keep_alive(Duration::from_secs(20))
        .set_clean_start(true)
        .set_connection_timeout(3)
        .set_session_expiry_interval(Some(u32::MAX));
    Ok(AsyncClient::new(mqtt_opts, 600))
}
