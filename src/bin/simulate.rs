use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use calypso::{
    proto::serverdata::{self, ServerData},
    simulatable_message::SimComponent,
    simulate_data::create_simulated_components,
};
use clap::Parser;
use protobuf::Message;
use regex::Regex;
use rumqttc::v5::{AsyncClient, EventLoop, MqttOptions};
use tokio::{signal, sync::mpsc};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{debug, info, level_filters::LevelFilter, warn};
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
 * Build FilterMode from CLI arguments, validating regex patterns
 * Returns Err(String) if any regex pattern is invalid
 */
fn build_filter_mode(args: &CalypsoArgs) -> Result<FilterMode, String> {
    if !args.disabled_topics.is_empty() {
        let mut regexes = Vec::new();
        for pattern in &args.disabled_topics {
            match Regex::new(pattern) {
                Ok(re) => regexes.push(re),
                Err(e) => return Err(format!("Invalid regex pattern '{}': {}", pattern, e)),
            }
        }
        Ok(FilterMode::Blacklist(regexes))
    } else if !args.enabled_topics.is_empty() {
        let mut regexes = Vec::new();
        for pattern in &args.enabled_topics {
            match Regex::new(pattern) {
                Ok(re) => regexes.push(re),
                Err(e) => return Err(format!("Invalid regex pattern '{}': {}", pattern, e)),
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
           _ = token.cancelled() => {
                debug!("Shutting down sim gen!");
                break;
            },
            _ = interval.tick() => {
                for component in simulated_components.iter_mut() {
            if component.should_update() {
                component.update();
                let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
                let data: calypso::data::DecodeData = component.get_decode_data();
                let mut payload = serverdata::ServerData::new();
                payload.unit = data.unit.to_string();
                payload.values = data.value;
                payload.time_us = timestamp;

                pub_channel
                    .send((
                        data.topic.to_string(),
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
 * A thread to publish messages to a MQTT client
 * client: The client to publish to
 * recv_messages: The channel to get the messages to publish
 */
async fn publish_stub(
    token: CancellationToken,
    client: AsyncClient,
    mut recv_messages: mpsc::Receiver<(String, ServerData)>,
) {
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                debug!("Shutting down PUB stub!");
                break;
            },
             Some(new_msg) = recv_messages.recv() => {
                pub_msg(new_msg.0, new_msg.1, &client).await;
            }
        }
    }
}

/**
 * A thread to poll MQTT broker status, and relay incoming subscribed messages
 * eventloop: the eventloop to poll
 * send_to_manager: the channel to send recieved MQTT messages from (optional)
 */
async fn poll_stub(token: CancellationToken, mut eventloop: EventLoop) {
    loop {
        tokio::select! {
        _ = token.cancelled() => {
            debug!("Shutting down SIREN manager!");
            break;
        },
        _ = eventloop.poll() => {}
        }
    }
}

/**
 * Helper function to generate bytes and publish a MQTT message
 * topic: the topic to send
 * data: the data protobuf to send
 * client: the client to send data to
 */
async fn pub_msg(topic: String, data: ServerData, client: &AsyncClient) {
    let Ok(bytes) = data.write_to_bytes() else {
        warn!("Could not generate protobuf!");
        return;
    };
    let Ok(()) = client
        .publish(topic, rumqttc::v5::mqttbytes::QoS::AtMostOnce, false, bytes)
        .await
    else {
        warn!("Could not publish message");
        return;
    };
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
            eprintln!("Error: {}", err);
            exit(1);
        }
    };

    // a channel to give protobuf messages to be sent out over MQTT
    let (decoder_send, decoder_recv) = mpsc::channel::<(String, ServerData)>(500);

    let mut mqtt_opts_main = MqttOptions::new(
        format!(
            "Calypso-Simulator-{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis()
        ),
        cli.siren_host_url
            .split_once(':')
            .expect("Invalid Siren URL")
            .0,
        cli.siren_host_url
            .split_once(':')
            .unwrap()
            .1
            .parse::<u16>()
            .expect("Invalid Siren port"),
    );
    mqtt_opts_main
        .set_keep_alive(Duration::from_secs(20))
        .set_clean_start(true)
        .set_connection_timeout(3)
        .set_session_expiry_interval(Some(u32::MAX))
        .set_topic_alias_max(Some(600));
    let (client, eventloop) = rumqttc::v5::AsyncClient::new(mqtt_opts_main, 600);

    task_tracker.spawn(poll_stub(token.clone(), eventloop));

    task_tracker.spawn(publish_stub(token.clone(), client, decoder_recv));

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
