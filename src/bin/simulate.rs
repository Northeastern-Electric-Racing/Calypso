use std::{
    thread::{self},
    time::{Duration, UNIX_EPOCH},
};

use calypso::{
    mqtt::MqttClient, serverdata, simulatable_message::SimulatedComponent, simulate_data::create_simulated_components
};
use clap::Parser;


/// Calypso command line arguments
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


fn simulate_out(pub_path: &str) {
    let mut client = MqttClient::new(pub_path, "calypso-simulator");
    let _ = client.connect(); // todo: add error handling
    let sleep_time = Duration::from_millis(10);

    // todo: a way to turn individual components on and off
    let mut simulated_components: Vec<SimulatedComponent> = create_simulated_components();

    // loop through the simulated components, if they should update, update them and publish the data
    loop {
        for component in simulated_components.iter_mut() {
            if component.should_update() {
                component.update();
                let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
                let data: calypso::data::DecodeData = component.get_data();
                let mut payload = serverdata::ServerData::new();
                payload.unit = data.unit.to_string();
                payload.values = data.value;
                payload.time_us = timestamp;

                client
                    .publish(
                        data.topic.to_string(),
                        protobuf::Message::write_to_bytes(&payload).unwrap_or_else(|e| {
                            format!("failed to serialize {}", e).as_bytes().to_vec()
                        }),
                    )
                    .expect("Could not publish!");
            }
        }
        // sleep for a bit
        thread::sleep(sleep_time);
    }
}



/**
 * Main Function
 * Calls the `simulate_out` function with the siren host URL from the command line arguments.
 */
fn main() {
    let cli = CalypsoArgs::parse();
    simulate_out(&cli.siren_host_url);
}
