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

use crate::filter::{FilterMode, FilterTx};
use crate::runtime_topic::SimCommandTx;
use crate::simulate_data::create_simulated_components;
use clap::Parser;
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};
use zenoh::Session;

use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    init_tracing();

    if cli.list_topics {
        list_topics_and_exit();
    }

    warnings::print_unsimulated();

    // Load the scenario once, up front (for --key-map / --play), so a bad file
    // is a startup error rather than a failure part-way through a run.
    let scenario = cli.key_map.as_deref().map(|path| {
        keymap::load_scenario(path).unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            exit(1);
        })
    });

    let token = CancellationToken::new();

    let session = connect_zenoh(cli.zenoh_conf.as_deref())
        .await
        .unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            exit(1);
        });

    // The heartbeat's topic filter lives in a watch channel so `--stream`'s
    // `set_filter` RPC and the plain-`--mock` stdin commands can retune it
    // while the sim runs. Built before anything spawns so a bad regex is a
    // startup error, not a surprise mid-run.
    let initial = FilterMode::build(&cli.enable_topic, &cli.disable_topic).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        exit(1);
    });
    let (filter_tx, filter_rx) = tokio::sync::watch::channel(initial);

    // Add/remove are events rather than state, so they get an mpsc: a watch
    // would coalesce two adds into one. When the heartbeat is off nothing holds
    // the receiver, and senders get a clear error instead of a silent no-op.
    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(16);

    let mock_handle = spawn_mock(&cli, &session, &token, filter_rx, cmd_rx);

    let foreground = run_foreground(&cli, &token, &session, scenario, filter_tx, cmd_tx).await;

    token.cancel();

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
    session: &Session,
    scenario: Option<keymap::Scenario>,
    filter_tx: FilterTx,
    cmd_tx: SimCommandTx,
) -> Result<(), String> {
    if cli.stream {
        modes::stream::run(token.clone(), session.clone(), filter_tx, cmd_tx).await
    } else if let Some(action) = &cli.play {
        // A missing scenario here is an impossible state, not a runtime error.
        let scenario = scenario.expect("clap enforces --play requires --key-map");
        modes::replay::run(session.clone(), scenario, action).await
    } else if cli.key_map.is_some() {
        let scenario = scenario.expect("--key-map implies main loaded the scenario");
        modes::interactive::run(token.clone(), session.clone(), scenario).await
    } else {
        // Pure --mock: stdin is free, so take live control commands on it.
        modes::control::run(token.clone(), filter_tx, cmd_tx).await
    }
}

/// Spawn the mock heartbeat over every simulatable topic, if it is enabled.
/// Which of those topics it actually publishes is the filter's call, and the
/// filter can change while running — so the task takes the whole set.
fn spawn_mock(
    cli: &Cli,
    session: &Session,
    token: &CancellationToken,
    filter_rx: filter::FilterRx,
    cmd_rx: runtime_topic::SimCommandRx,
) -> Option<tokio::task::JoinHandle<()>> {
    if !cli.run_mock() {
        return None;
    }
    Some(tokio::spawn(modes::mock::run(
        token.clone(),
        session.clone(),
        create_simulated_components(),
        filter_rx,
        cmd_rx,
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

/// Open a Zenoh session. With no `--zenoh-conf` the Zenoh defaults are used, so
/// the sim runs without a conf file; an explicit path must load and parse.
async fn connect_zenoh(conf_path: Option<&Path>) -> Result<Session, String> {
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
