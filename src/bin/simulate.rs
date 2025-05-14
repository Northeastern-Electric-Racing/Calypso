use std::time::{Duration, SystemTime, UNIX_EPOCH};

use calypso::{
    proto::serverdata, simulatable_message::SimComponent,
    simulate_data::create_simulated_components,
};
use clap::Parser;
use rumqttc::v5::MqttOptions;

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
}

async fn simulate_out(pub_path: &str) {
    let mut mqtt_opts_main = MqttOptions::new(
        format!(
            "Calypso-Simulator-{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis()
        ),
        pub_path.split_once(':').expect("Invalid Siren URL").0,
        pub_path
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
    let (client, mut eventloop) = rumqttc::v5::AsyncClient::new(mqtt_opts_main, 600);

    // todo: a way to turn individual components on and off
    // note: components are pre-initialized within the function
    let mut simulated_components: Vec<SimComponent> = create_simulated_components();

    let mut interval = tokio::time::interval(Duration::from_millis(5));

    loop {
        tokio::select! {
            _ = eventloop.poll() => {},
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

                client
                    .publish(
                        data.topic.to_string(),
                        rumqttc::v5::mqttbytes::QoS::ExactlyOnce,
                        false,
                        protobuf::Message::write_to_bytes(&payload).unwrap_or_else(|e| {
                            format!("failed to serialize {}", e).as_bytes().to_vec()
                        }),
                    )
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
    simulate_out(&cli.siren_host_url).await;
}
