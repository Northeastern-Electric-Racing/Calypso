use std::path::PathBuf;
use std::process::exit;
use std::time::{Duration, UNIX_EPOCH};

use calypso::mqtt_handler::{poll_stub, publish_stub, siren_creator};
use calypso::zenoh_handler::ZenohProcessor;
use calypso::{
    proto::serverdata::{self, ServerData},
    simulatable_message::SimComponent,
    simulate_data::create_simulated_components,
};
use clap::Parser;
use regex::Regex;
use tokio::{signal, sync::mpsc};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{debug, info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

/**
* The command line arguments for the simulator.
*/
#[derive(Parser, Debug)]
#[command(version)]
struct CalypsoArgs {
    /// The host url of the siren, including port and excluding protocol prefix
    #[arg(
        short = 'u',
        long,
        env = "CALYPSO_SIREN_HOST_URL",
        default_value = "localhost:1883"
    )]
    siren_host_url: String,

    /// Disable topics matching regex patterns (blacklist mode)
    #[arg(long = "disable-topic", conflicts_with = "enabled_topics")]
    disabled_topics: Vec<String>,

    /// Enable ONLY topics matching regex patterns (whitelist mode)
    #[arg(long = "enable-topic", conflicts_with = "disabled_topics")]
    enabled_topics: Vec<String>,

    /// Use Zenoh instead of MQTT -- will eventually become default
    #[arg(short = 'z', long, env = "CALYPSO_ZENOH")]
    zenoh: bool,

    /// Zenoh conf file
    #[arg(long, env = "CALYPSO_ZENOH_CONF", default_value_os = "./zenoh.json5")]
    zenoh_conf: PathBuf,
}

/**
 * Filter mode for topic filtering
 */
#[derive(Debug, Clone)]
enum FilterMode {
    /// Publish all topics except those matching patterns (blacklist)
    Blacklist(Vec<Regex>),
    /// Publish only topics matching patterns (whitelist)
    Whitelist(Vec<Regex>),
    /// No filtering, publish all topics
    Disabled,
}

/**
 * Build `FilterMode` from CLI arguments, validating regex patterns
 * Returns Err(String) if any regex pattern is invalid
 */
fn build_filter_mode(args: &CalypsoArgs) -> Result<FilterMode, String> {
    if !args.disabled_topics.is_empty() {
        let mut regexes = Vec::new();
        for pattern in &args.disabled_topics {
            match Regex::new(pattern) {
                Ok(re) => regexes.push(re),
                Err(e) => return Err(format!("Invalid regex pattern '{pattern}': {e}")),
            }
        }
        Ok(FilterMode::Blacklist(regexes))
    } else if !args.enabled_topics.is_empty() {
        let mut regexes = Vec::new();
        for pattern in &args.enabled_topics {
            match Regex::new(pattern) {
                Ok(re) => regexes.push(re),
                Err(e) => return Err(format!("Invalid regex pattern '{pattern}': {e}")),
            }
        }
        Ok(FilterMode::Whitelist(regexes))
    } else {
        Ok(FilterMode::Disabled)
    }
}

/**
 * Check if a topic should be published based on the filter mode
 */
fn should_publish(topic: &str, filter: &FilterMode) -> bool {
    match filter {
        FilterMode::Disabled => true,
        FilterMode::Blacklist(patterns) => {
            // Publish if topic does NOT match any blacklist pattern
            !patterns.iter().any(|re| re.is_match(topic))
        }
        FilterMode::Whitelist(patterns) => {
            // Publish if topic matches at least one whitelist pattern
            patterns.iter().any(|re| re.is_match(topic))
        }
    }
}

async fn simulate_out(
    token: CancellationToken,
    pub_channel: mpsc::Sender<(String, ServerData)>,
    filter_mode: FilterMode,
) {
    // todo: a way to turn individual components on and off
    // note: components are pre-initialized within the function
    let all_components = create_simulated_components();

    // Filter components based on filter mode
    let mut simulated_components: Vec<SimComponent> = all_components
        .into_iter()
        .filter(|component| should_publish(&component.name, &filter_mode))
        .collect();

    if simulated_components.is_empty() {
        info!("No components to simulate after filtering. All topics filtered out.");
    } else {
        info!("Simulating {} components", simulated_components.len());
    }

    let mut interval = tokio::time::interval(Duration::from_millis(5));

    loop {
        tokio::select! {
           () = token.cancelled() => {
                debug!("Shutting down sim gen!");
                break;
            },
            _ = interval.tick() => {
                for component in &mut simulated_components {
            if component.should_update() {
                component.update();
                let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
                let data: calypso::data::DecodeData = component.get_decode_data();
                let mut payload = serverdata::ServerData::new();
                payload.unit.clone_from(&data.unit);
                payload.values = data.value;
                payload.time_us = timestamp;

                pub_channel
                    .send((
                        data.topic.clone(),
                        payload
                    ))
                    .await
                    .expect("Could not publish!");
            }
        }
            }
        }
    }
}

/**
 * Main Function
 * Calls the `simulate_out` function with the siren host URL from the command line arguments.
 */
#[tokio::main]
async fn main() {
    let cli = CalypsoArgs::parse();

    println!("Initializing fmt subscriber");
    // construct a subscriber that prints formatted traces to stdout
    // if RUST_LOG is not set, defaults to loglevel INFO
    let subscriber = tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_ansi(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("Could not init tracing");

    // the below two threads need to cancel cleanly to ensure all queued messages are sent.  therefore they are part of the a task tracker group.
    // create a task tracker and cancellation token
    let task_tracker = TaskTracker::new();
    let token = CancellationToken::new();

    // Build filter mode and validate regex patterns
    let filter_mode = match build_filter_mode(&cli) {
        Ok(mode) => mode,
        Err(err) => {
            eprintln!("Error: {err}");
            exit(1);
        }
    };

    // a channel to give protobuf messages to be sent out over MQTT
    let (decoder_send, decoder_recv) = mpsc::channel::<(String, ServerData)>(500);

    if cli.zenoh {
        let zenoh = ZenohProcessor::new(token.clone(), decoder_recv, None, cli.zenoh_conf).await;
        task_tracker.spawn(zenoh.process_zenoh());
    } else {
        // the actual client and eventloop handlers
        let main_broker = siren_creator(cli.siren_host_url, "Calypso-Simulator".to_string()).await;

        task_tracker.spawn(poll_stub(token.clone(), main_broker.1, None));
        task_tracker.spawn(publish_stub(token.clone(), main_broker.0, decoder_recv));
    }

    task_tracker.spawn(simulate_out(token.clone(), decoder_send, filter_mode));

    task_tracker.close();

    info!("Initialization complete, ready...");
    info!("Use Ctrl+C or SIGINT to exit cleanly!");

    signal::ctrl_c()
        .await
        .expect("Could not read cancellation trigger (ctr+c)");
    info!("Received exit signal, shutting down!");
    token.cancel();

    task_tracker.wait().await;
}
