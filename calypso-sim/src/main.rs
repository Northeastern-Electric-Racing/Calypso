mod cli;
mod filter;
mod keymap;
mod modes;
#[allow(clippy::all, clippy::pedantic)]
mod proto;
mod publish;
mod raw_mode;
mod runtime_topic;
mod simulatable_message;
mod simulate_data;
mod warnings;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::process::exit;

use crate::filter::{FilterMode, FilterRx, FilterTx};
use crate::keymap::Scenario;
use crate::runtime_topic::{SimCommandRx, SimCommandTx};
use crate::simulate_data::create_simulated_components;
use clap::Parser;
use tokio::task::JoinHandle;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use zenoh::Session;

use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    init_tracing();

    if cli.list_topics {
        list_topics_and_exit();
    }

    if let Err(err) = run(&cli).await {
        eprintln!("Error: {err}");
        exit(1);
    }
}

async fn run(cli: &Cli) -> Result<(), String> {
    warnings::print_unsimulated();

    // Load the scenario once, up front (for --key-map / --play), so a bad file
    // is a startup error rather than a failure part-way through a run.
    let scenario = cli
        .key_map
        .as_deref()
        .map(keymap::load_scenario)
        .transpose()?;

    // The heartbeat's topic filter lives in a watch channel so `--stream`'s
    // `set_filter` RPC and the plain-`--mock` stdin commands can retune it
    // while the sim runs. Built before anything spawns so a bad regex is a
    // startup error, not a surprise mid-run.
    let initial = FilterMode::build(&cli.enable_topic, &cli.disable_topic)?;

    let session = connect_zenoh(cli.zenoh_conf.as_deref()).await?;

    let (filter_tx, filter_rx) = tokio::sync::watch::channel(initial);

    // Add/remove are events rather than state, so they get an mpsc: a watch
    // would coalesce two adds into one. When the heartbeat is off nothing holds
    // the receiver, and senders get a clear error instead of a silent no-op.
    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(16);

    let mock_handle = spawn_mock(cli, &session, filter_rx, cmd_rx);

    let foreground = run_foreground(cli, session, scenario, filter_tx, cmd_tx).await;

    if let Some(handle) = mock_handle {
        handle.abort();
        // Cancellation is the expected outcome; only a panic is worth reporting.
        if let Err(e) = handle.await
            && e.is_panic()
        {
            tracing::error!("mock task panicked: {e}");
        }
    }

    foreground
}

async fn run_foreground(
    cli: &Cli,
    session: Session,
    scenario: Option<Scenario>,
    filter_tx: FilterTx,
    cmd_tx: SimCommandTx,
) -> Result<(), String> {
    if cli.stream {
        return modes::stream::run(session, filter_tx, cmd_tx).await;
    }
    // clap makes `--play` require `--key-map`, so a play action always comes
    // with a scenario.
    match (scenario, cli.play.as_deref()) {
        (Some(scenario), Some(action)) => modes::replay::run(session, scenario, action).await,
        (Some(scenario), None) => modes::interactive::run(session, scenario).await,
        // Pure --mock: stdin is free, so take live control commands on it.
        (None, _) => modes::control::run(filter_tx, cmd_tx).await,
    }
}

/// Spawn the mock heartbeat over every simulatable topic, if it is enabled.
/// Which of those topics it actually publishes is the filter's call, and the
/// filter can change while running — so the task takes the whole set.
fn spawn_mock(
    cli: &Cli,
    session: &Session,
    filter_rx: FilterRx,
    cmd_rx: SimCommandRx,
) -> Option<JoinHandle<()>> {
    if !cli.run_mock() {
        return None;
    }
    Some(tokio::spawn(modes::mock::run(
        session.clone(),
        create_simulated_components(),
        filter_rx,
        cmd_rx,
    )))
}

fn init_tracing() {
    // Tracing always writes to stderr so stdout stays clean for stream mode
    // and keymap-mode logs. Without RUST_LOG, Zenoh (chatty at INFO) logs at
    // WARN; the default directive covers a RUST_LOG with no valid directives.
    let directives = std::env::var(EnvFilter::DEFAULT_ENV)
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "info,zenoh=warn".to_string());
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(directives),
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

/// Open a Zenoh session. With no `--zenoh-conf` the Zenoh defaults are used, so
/// the sim runs without a conf file; an explicit path must load and parse.
async fn connect_zenoh(conf_path: Option<&Path>) -> Result<Session, String> {
    let config = match conf_path {
        Some(path) => zenoh::Config::from_file(path)
            .map_err(|e| format!("Invalid Zenoh conf '{}': {e}", path.display()))?,
        None => zenoh::Config::default(),
    };

    zenoh::open(config)
        .await
        .map_err(|e| format!("Could not open Zenoh session: {e}"))
}
