use std::{
    collections::HashMap,
    time::{Duration, UNIX_EPOCH},
};

use calypso::{
    data::{DecodeData, EncodeData},
    decode_data::*,
    encode_data::*,
    proto::{
        command_data,
        serverdata::{self, ServerData},
    },
};
use clap::Parser;
use futures_util::StreamExt;
use protobuf::Message;
use rumqttc::v5::{
    mqttbytes::v5::{Packet, Publish},
    AsyncClient, Event, MqttOptions,
};
use socketcan::{tokio::CanSocket, CanError, CanFrame, EmbeddedFrame, Frame, Id, SocketOptions};
use tokio::{
    signal,
    sync::mpsc::{self, Receiver, Sender},
};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tracing::{debug, info, level_filters::LevelFilter, trace, warn};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

const ENCODER_MAP_SUB: &str = "Calypso/Bidir/Command/#";

/// Calypso command line arguments
#[derive(Parser, Debug)]
#[command(version)]
struct CalypsoArgs {
    /// Whether to enable CAN message encoding
    #[arg(short = 'e', long, env = "CALYPSO_CAN_ENCODE")]
    encode: bool,

    /// The host url of the siren, including port and excluding protocol prefix
    #[arg(
        short = 'u',
        long,
        env = "CALYPSO_SIREN_HOST_URL",
        default_value = "localhost:1883"
    )]
    siren_host_url: String,

    /// The SocketCAN interface port
    #[arg(
        short = 'c',
        long,
        env = "CALYPSO_SOCKETCAN_IFACE",
        default_value = "vcan0"
    )]
    socketcan_iface: String,

    // Whether to enable MQTT multi-client
    #[arg(short = 'm', long, env = "CALYPSO_MQTT_MULTICLIENT")]
    mqtt_multiclient: bool,
}

/**
 * Reads the can socket and publishes the data to the given client.
 * can_interface: the socketcan interface to bind to
 * send: the channel to send protobuf messages to
 */
async fn can_manager(
    token: CancellationToken,
    can_interface: String,
    send_to_siren: Sender<(String, Option<Vec<u16>>, ServerData)>,
    mut send_over_can: Receiver<CanFrame>,
) {
    let mut socket = CanSocket::open(&can_interface).expect("Failed to open CAN socket!");
    socket
        .set_error_filter_accept_all()
        .expect("Failed to set error mask on CAN socket!");
    socket
        .set_recv_own_msgs(true)
        .expect("Cant recv own messages");
    loop {
        // Read from CAN socket
        tokio::select! {
            _ = token.cancelled() => {
                debug!("Shutting down CAN reader!");
                break;
            },
            Some(frame) = socket.next() => {
                pub_frame(frame, &send_to_siren).await;
            }
            Some(frame) = send_over_can.recv() => {
                match socket.write_frame(frame) {
                    Ok(fut) => {
                        match fut.await {
                            Ok(()) => continue,
                            Err(r) => warn!("Could not send CAN frame: {}", r),
                        }
                    },
                    Err(r) => warn!("Could not process CAN frame: {}", r),
                }
            },
        }
    }
}

async fn pub_frame(
    frame: Result<CanFrame, socketcan::Error>,
    send: &Sender<(String, Option<Vec<u16>>, ServerData)>,
) {
    let decoded_data = match frame {
        // CanDataFrame
        Ok(CanFrame::Data(data_frame)) => {
            let data = data_frame.data();
            let id: u32 = match data_frame.id() {
                socketcan::Id::Standard(std) => std.as_raw().into(),
                socketcan::Id::Extended(ext) => ext.as_raw(),
            };
            trace!("RECVED message with ID: {:#01x}", id);
            match DECODE_FUNCTION_MAP.get(&id) {
                Some(func) => func(data),
                None => vec![DecodeData::new(
                    vec![id as f32],
                    "Calypso/Unknown",
                    "ID",
                    None,
                )],
            }
        }
        // CanRemoteFrame
        Ok(CanFrame::Remote(remote_frame)) => {
            // Send frame ID for Remote
            vec![DecodeData::new(
                vec![remote_frame.raw_id() as f32],
                "Calypso/Events/RemoteFrame",
                "id",
                None,
            )]
        }
        // CanErrorFrame
        Ok(CanFrame::Error(error_frame)) => {
            // Publish enum index of error onto CAN
            // TODO: maybe look into better representation?
            let error_index: f32 = match CanError::from(error_frame) {
                CanError::TransmitTimeout => 0.0,
                CanError::LostArbitration(_) => 1.0,
                CanError::ControllerProblem(_) => 2.0,
                CanError::ProtocolViolation { .. } => 3.0,
                CanError::TransceiverError => 4.0,
                CanError::NoAck => 5.0,
                CanError::BusOff => 6.0,
                CanError::BusError => 7.0,
                CanError::Restarted => 8.0,
                CanError::DecodingFailure(_) => 9.0,
                CanError::Unknown(_) => 10.0,
            };
            vec![DecodeData::new(
                vec![error_index],
                "Calypso/Events/ErrorFrame",
                "CanError enum",
                None,
            )]
        }
        // Socket failure
        Err(err) => {
            warn!("CAN Socket failure: {}", err);
            return;
        }
    };
    let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;

    // Convert decoded CAN to Protobuf and publish over MQTT
    for data in decoded_data.iter() {
        let mut payload = serverdata::ServerData::new();
        payload.unit = data.unit.to_string();
        payload.values = data.value.clone();
        payload.time_us = timestamp;

        match send
            .send((data.topic.clone(), data.clients.clone(), payload))
            .await
        {
            Ok(()) => trace!("Sent a CAN message to SIREN manager"),
            Err(err) => warn!("Could not send CAN message to SIREN manager: {}", err),
        }
    }
}

/**
 * Manages siren
 */
async fn siren_manager(
    token: CancellationToken,
    pub_path: String,
    mut recv_messages: Receiver<(String, Option<Vec<u16>>, ServerData)>,
    send_to_manager: Sender<Publish>,
) {
    let mut mqtt_opts = MqttOptions::new(
        "Calypso-Decoder",
        pub_path.split_once(':').expect("Invalid Siren URL").0,
        pub_path
            .split_once(':')
            .unwrap()
            .1
            .parse::<u16>()
            .expect("Invalid Siren port"),
    );
    mqtt_opts
        .set_keep_alive(Duration::from_secs(20))
        .set_clean_start(false)
        .set_connection_timeout(3)
        .set_session_expiry_interval(Some(u32::MAX))
        .set_topic_alias_max(Some(600));
    let (client, mut eventloop) = rumqttc::v5::AsyncClient::new(mqtt_opts, 600);

    // subscribe for bidirectionality
    match client
        .subscribe(ENCODER_MAP_SUB, rumqttc::v5::mqttbytes::QoS::ExactlyOnce)
        .await
    {
        Ok(()) => (),
        Err(err) => warn!("Error subscribing: {}", err),
    }

    loop {
        tokio::select! {
                _ = token.cancelled() => {
                    debug!("Shutting down SIREN manager!");
                    break;
                },
                msg = eventloop.poll() => match msg {
                    Ok(Event::Incoming(Packet::Publish(msg))) => {
                            trace!("Received mqtt message: {:?}", msg);
                            match send_to_manager.send(msg).await {
                                Ok(()) => (),
                                Err(err) => warn!("Could not send MQTT message to bidir manager: {}", err),
                            }
                    },
                    Err(msg) => trace!("Received mqtt error: {:?}", msg),
                    _ => trace!("Received misc mqtt: {:?}", msg),
                },
                Some(new_msg) = recv_messages.recv() => {
                    pub_msg(new_msg, &client).await;
                },
        }
    }
}

async fn pub_msg(msg: (String, Option<Vec<u16>>, ServerData), client: &AsyncClient) {
    let Ok(bytes) = msg.2.write_to_bytes() else {
        warn!("Could not generate protobuf!");
        return;
    };
    let Ok(()) = client
        .publish(
            msg.0,
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

async fn bidir_manager(
    token: CancellationToken,
    can_push_send: Sender<CanFrame>,
    mut siren_recv: Receiver<Publish>,
) {
    let mut send_interval = tokio::time::interval(Duration::from_millis(750));

    let mut send_map = HashMap::new();
    for key in ENCODABLE_KEY_LIST {
        let encoder_func = match ENCODE_FUNCTION_MAP.get(key) {
            Some(func) => *func,
            None => panic!("An unknown message was initialized!"),
        };
        let ret = encoder_func(Vec::new());
        send_map.insert(ret.id, ret);
    }

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                debug!("Shutting down BIDIR manager!");
                break;
            },
            _ = send_interval.tick() => {
                release_commands(&send_map, &can_push_send).await;
            }
            Some(msg) = siren_recv.recv() => {
                parse_msg(msg, &mut send_map).await;
            },
        }
    }
}

async fn release_commands(send_map: &HashMap<u32, EncodeData>, can_push_send: &Sender<CanFrame>) {
    for msg in send_map.iter() {
        // let id = u32::from_str_radix((msg.1.1).trim_start_matches("0x"), 16).expect("Invalid CAN ID!");

        let id: Id = if !msg.1.is_ext {
            socketcan::StandardId::new(
                msg.1
                    .id
                    .try_into()
                    .unwrap_or_else(|_| panic!("Invalid standard ID: {}", msg.1.id)),
            )
            .unwrap_or_else(|| panic!("Invalid standard ID: {}", msg.1.id))
            .into()
        } else {
            socketcan::ExtendedId::new(msg.1.id)
                .unwrap_or_else(|| panic!("Invalid extended ID: {}", msg.1.id))
                .into()
        };

        match CanFrame::new(id, &msg.1.value) {
            Some(packet) => {
                match can_push_send.send(packet).await {
                    Ok(_) => (),
                    Err(err) => warn!("Error sending can command to can manager {}", err),
                };
            }
            None => {
                warn!("Packet is too long: {}", msg.1);
            }
        }
    }
}

async fn parse_msg(msg: Publish, send_map: &mut HashMap<u32, EncodeData>) {
    let buf = match command_data::CommandData::parse_from_bytes(&msg.payload) {
        Ok(buf) => buf,
        Err(err) => {
            println!("Could not decode command: {}", err);
            return;
        }
    };
    let Ok(topic) = std::str::from_utf8(&msg.topic) else {
        warn!("Could not parse topic, topic: {:?}", msg.topic);
        return;
    };
    let key = match topic.split('/').next_back() {
        Some(key) => key.to_owned(),
        None => {
            println!("Could not parse the key value in {}", topic);
            return;
        }
    };

    match ENCODE_FUNCTION_MAP.get(&key) {
        Some(func) => {
            let ret = func(buf.data);
            send_map.insert(ret.id, ret);
        }
        None => {
            let id: u32 = 0x7FF;
            let cnt = match send_map.get(&id) {
                Some(item) => item.value.first().unwrap_or(&0) + 1,
                None => 1,
            };
            let ret = EncodeData {
                value: vec![cnt],
                id,
                is_ext: false,
            };
            send_map.insert(ret.id, ret);
        }
    }
}

/**
 * Main Function
 * Configures the can network, retrieves the client based on the command line arguments,
 * connects the client and then reads the can socket from specified interface.
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

    // a channel to give protobuf messages to be sent out over MQTT
    let (decoder_send, decoder_recv) = mpsc::channel::<(String, Option<Vec<u16>>, ServerData)>(500);

    // a channel to give CAN messages back out (car commands)
    let (can_push_send, can_push_recv) = mpsc::channel::<CanFrame>(100);

    // a channel to give messages to the bidir manager
    let (siren_recv_send, siren_recv_recv) = mpsc::channel::<Publish>(100);

    task_tracker.spawn(siren_manager(
        token.clone(),
        cli.siren_host_url,
        decoder_recv,
        siren_recv_send,
    ));
    task_tracker.spawn(can_manager(
        token.clone(),
        cli.socketcan_iface,
        decoder_send,
        can_push_recv,
    ));
    task_tracker.spawn(bidir_manager(token.clone(), can_push_send, siren_recv_recv));

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
