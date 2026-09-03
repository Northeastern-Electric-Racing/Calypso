use std::time::{Duration, SystemTime};

use protobuf::Message;
use rumqttc::v5::{AsyncClient, Event, EventLoop, MqttOptions, mqttbytes::v5::Packet};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::sync::CancellationToken;
use tracing::{debug, trace, warn};

use crate::proto::{
    command_data::{self, CommandData},
    serverdata::ServerData,
};

const ENCODER_MAP_SUB: &str = "Calypso/Bidir/Command/#";

/**
 * Inits siren communication, returning the main (1st) and priority (2nd) structs
 * `pub_path`:  The base URL (and port for main)
 *
 * # Panics
 *  Panics if time went backwards
 */
pub async fn siren_creator(pub_path: String) -> (AsyncClient, EventLoop) {
    let mut mqtt_opts_main = MqttOptions::new(
        format!(
            "Calypso-Decoder-{}",
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
        .set_clean_start(false)
        .set_connection_timeout(3)
        .set_session_expiry_interval(Some(u32::MAX));
    let (main_client, main_eventloop) = rumqttc::v5::AsyncClient::new(mqtt_opts_main, 600);

    // subscribe for bidirectionality
    match main_client
        .subscribe(ENCODER_MAP_SUB, rumqttc::v5::mqttbytes::QoS::ExactlyOnce)
        .await
    {
        Ok(()) => (),
        Err(err) => warn!("Error subscribing: {}", err),
    }

    // here we split into two threads, one owns the client the other owns the eventloop

    (main_client, main_eventloop)
}

/**
 * A thread to publish messages to a MQTT client
 * client: The client to publish to
 * `recv_messages`: The channel to get the messages to publish
 */
pub async fn publish_stub(
    token: CancellationToken,
    client: AsyncClient,
    mut recv_messages: Receiver<(String, ServerData)>,
) {
    loop {
        tokio::select! {
            () = token.cancelled() => {
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
 * `send_to_manager`: the channel to send recieved MQTT messages from (optional)
 */
pub async fn poll_stub(
    token: CancellationToken,
    mut eventloop: EventLoop,
    send_to_manager: Sender<(String, CommandData)>,
) {
    loop {
        tokio::select! {
            () = token.cancelled() => {
                debug!("Shutting down SIREN manager!");
                break;
            },
            msg = eventloop.poll() => match msg {
                Ok(Event::Incoming(Packet::Publish(msg))) => {
                    debug!("Received mqtt message: {:?}", msg);
                    let buf = match command_data::CommandData::parse_from_bytes(&msg.payload) {
                        Ok(buf) => buf,
                        Err(err) => {
                            warn!("Could not decode command: {}", err);
                            continue;
                        }
                    };
                    let Ok(topic) = std::str::from_utf8(&msg.topic) else {
                        warn!("Could not parse topic, topic: {:?}", msg.topic);
                        continue;
                    };
                    match send_to_manager.send((topic.to_string(), buf)).await {
                        Ok(()) => (),
                        Err(err) => warn!("Could not send MQTT message to bidir manager: {}", err),
                    }
                },
                Err(msg) => trace!("Received mqtt error: {:?}", msg),
                _ => trace!("Received misc mqtt: {:?}", msg),
            },
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
        .publish(
            topic,
            rumqttc::v5::mqttbytes::QoS::ExactlyOnce,
            false,
            bytes,
        )
        .await
    else {
        warn!("Could not publish message");
        return;
    };
}
