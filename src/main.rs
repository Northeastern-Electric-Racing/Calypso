use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, UNIX_EPOCH},
};

use calypso::{
    data::{DecodeData, EncodeData},
    decode_data::*,
    encode_data::*,
    mqtt::MqttClient,
    proto::{
        command_data,
        serverdata::{self, ServerData},
    },
};
use clap::Parser;
use futures_util::stream::StreamExt;
use protobuf::Message;
use socketcan::{tokio::CanSocket, CanError, CanFrame, EmbeddedFrame, Frame, Id, SocketOptions};
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
    time::sleep,
};

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

    /// The priorty url to use for critical messages that cannot be buffered, including port and excluding protocol prefix
    #[arg(
        short = 'u',
        long,
        env = "CALYPSO_PRIORITY_HOST_URL",
        default_value = "localhost:1882"
    )]
    priority_host_url: String,

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

    // The number of consumers to receive can messages from the can line reader
    #[arg(short = 'n', long, env = "CALYPSO_NUM_CAN_CONSUMERS")]
    num_can_consumers: usize,

    // The number of mqtt clients to send to siren to publish decoded data
    #[arg(short = 's', long, env = "CALYPSO_NUM_SIREN_SENDERS")]
    num_siren_senders: usize,
}

async fn can_frame_consumer(
    rx: Arc<Mutex<mpsc::Receiver<Result<CanFrame, socketcan::Error>>>>,
    clients: &HashMap<u16, mpsc::Sender<(String, ServerData)>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(frame) = rx.lock().await.recv().await {
            let decoded_data = match frame {
                Ok(CanFrame::Data(data_frame)) => {
                    let data = data_frame.data();
                    let id: u32 = match data_frame.id() {
                        socketcan::Id::Standard(std) => std.as_raw().into(),
                        socketcan::Id::Extended(ext) => ext.as_raw(),
                    };
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
                    println!("CAN Socket failure: {}", err);
                    continue;
                }
            };

            let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;

            // Convert decoded CAN to Protobuf and publish over MQTT
            for data in decoded_data.iter() {
                let mut payload = serverdata::ServerData::new();
                payload.unit = data.unit.to_string();
                payload.values = data.value.clone();
                payload.time_us = timestamp;

                if let Some(additional_clients) = &data.clients {
                    for port in additional_clients.iter() {
                        if let Some(client) = clients.get_mut(port) {
                            if client
                                .send((data.topic.clone(), payload.clone()))
                                .await
                                .is_err()
                            {
                                println!("Failed to send to client, {}", port);
                            }
                        }
                    }
                }

                // Publish to Siren.
                if let Some(client) = clients.get_mut(&1883) {
                    if client.send((data.topic.clone(), payload)).await.is_err() {
                        println!("Failed to send to siren");
                    }
                }
            }
        }
    })
}

/**
 * Reads the can socket and publishes the data to the given client.
 */
fn read_can(
    pub_path: &str,
    priority_path: &str,
    can_interface: &str,
    mqtt_multiclient: bool,
    num_can_consumers: usize,
    num_mqtt_senders: usize,
) -> Vec<JoinHandle<()>> {
    // TODO: Look into channel size, just mirroring broadcast size from scylla
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let (siren_send, siren_recv) = mpsc::channel::<(String, ServerData)>(10000);
    siren_mut = Arc::new(Mutex::new(siren_recv));
    for i in 0..num_mqtt_senders {
        let mut rx = siren_mut.clone();

        let handle = MqttClient::new(pub_path, format!("calypso-decoder-{}", i).as_str())
            .sending_loop(rx, 1883);
        handles.push(handle);
    }

    let mut clients: HashMap<u16, mpsc::Sender<(String, ServerData)>> =
        HashMap::from([(1883, siren_send)]);

    // Add 1882 client if multi-client is enabled
    if mqtt_multiclient {
        let (priority_send, priority_recv) = mpsc::channel::<(String, ServerData)>(1000);
        MqttClient::new(priority_path, "calypso-priority")
            .sending_loop(Arc::new(Mutex::new(priority_recv)), 1882);
        clients.insert(1882, priority_send);
    }

    let mut socket = CanSocket::open(can_interface).expect("Failed to open CAN socket!");
    socket
        .set_error_filter_accept_all()
        .expect("Failed to configure CAN socket!");

    let (can_tx, can_rx) = mpsc::channel(100);

    let read_handle = tokio::spawn(async move {
        while let Some(frame) = socket.next().await {
            can_tx
                .send(frame)
                .await
                .expect("Failed to send can frame to processor")
        }
    });
    handles.push(read_handle);

    let can_rx = Arc::new(Mutex::new(can_rx));

    for _i in 0..num_can_consumers {
        let rx = can_rx.clone();
        let client_ref = &clients;
        let handle = tokio::spawn(async move {
            can_frame_consumer(rx, client_ref).await;
        });
        handles.push(handle);
    }
    handles
}

/**
 * Reads the mqtt incoming messages and sends can messages based off of that
 */
fn read_siren(pub_path: &str, send_map: Arc<RwLock<HashMap<u32, EncodeData>>>) -> JoinHandle<()> {
    let mut client = MqttClient::new(pub_path, "calypso-encoder");

    let _ = client.connect();
    while !client.is_connected() {
        println!("[read_siren] Unable to connect to Siren, going into reconnection mode.");
        if client.reconnect().is_ok() {
            println!("[read_siren] Reconnected to Siren!");
        }
    }

    let reciever = client.start_consumer().expect("Could not begin consuming");
    client
        .subscribe(ENCODER_MAP_SUB)
        .expect("Could not subscribe!");

    // do the default initialization for all, do outside of the thread to save time negotiating when send_can comes up
    let mut writable_send_map = send_map.write().expect("Could not modify send messages!");
    for key in ENCODABLE_KEY_LIST {
        let key_owned = key.to_owned();
        let encoder_func = match ENCODE_FUNCTION_MAP.get(&key_owned) {
            Some(func) => *func,
            None => panic!("An unknown message was initialized!"),
        };
        let ret = encoder_func(Vec::new());
        writable_send_map.insert(ret.id, ret);
    }
    drop(writable_send_map);

    tokio::spawn(async move {
        for msg in reciever.iter() {
            if let Some(msg) = msg {
                let buf = match command_data::CommandData::parse_from_bytes(msg.payload()) {
                    Ok(buf) => buf,
                    Err(err) => {
                        println!("Could not decode command: {}", err);
                        continue;
                    }
                };
                let key = match msg.topic().split('/').next_back() {
                    Some(key) => key.to_owned(),
                    None => {
                        println!("Could not parse the key value in {}", msg.topic());
                        continue;
                    }
                };

                match ENCODE_FUNCTION_MAP.get(&key) {
                    Some(func) => {
                        let ret = func(buf.data);
                        send_map
                            .write()
                            .expect("Could not modify send messages!")
                            .insert(ret.id, ret);
                    }
                    None => {
                        let id: u32 = 0x7FF;
                        let mut send_map_writable =
                            send_map.write().expect("Could not modify send messages!");
                        let cnt = match send_map_writable.get(&id) {
                            Some(item) => item.value.first().unwrap_or(&0) + 1,
                            None => 1,
                        };
                        let ret = EncodeData {
                            value: vec![cnt],
                            id,
                            is_ext: false,
                        };
                        send_map_writable.insert(ret.id, ret);
                    }
                }
            } else {
                while !client.is_connected() {
                    println!(
                        "[read_siren] Unable to connect to Siren, going into reconnection mode."
                    );
                    if client.reconnect().is_ok() {
                        println!("[read_siren] Reconnected to Siren!");
                    }
                }
                client
                    .subscribe(ENCODER_MAP_SUB)
                    .expect("Could not subscribe!");
            }
        }
    })
}

fn send_out(
    can_interface: &str,
    send_map: Arc<RwLock<HashMap<u32, EncodeData>>>,
) -> JoinHandle<()> {
    let socket = CanSocket::open(can_interface).expect("Failed to open CAN socket!");

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(750)).await;
            let sender = send_map.read().expect("Cannot read map of sendables!");
            for msg in sender.iter() {
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
                        match socket.write_frame(packet) {
                            Ok(_) => (),
                            Err(err) => println!("Error sending can message {}", err),
                        };
                    }
                    None => {
                        println!("Packet is too long: {}", msg.1);
                    }
                }
            }
        }
    })
}

/**
 * Main Function
 * Configures the can network, retrieves the client based on the command line arguments,
 * connects the client and then reads the can socket from specified interface.
 */
#[tokio::main]
async fn main() {
    let cli = CalypsoArgs::parse();
    let can_handles = read_can(
        &cli.siren_host_url,
        &cli.priority_host_url,
        &cli.socketcan_iface,
        cli.mqtt_multiclient,
        cli.num_can_consumers,
        cli.num_siren_senders,
    );

    // use a arc for mutlithread, and a rwlock to enforce one writer
    if cli.encode {
        let send_map: Arc<RwLock<HashMap<u32, EncodeData>>> = Arc::new(RwLock::new(HashMap::new()));

        let siren_handle = read_siren(&cli.siren_host_url, Arc::clone(&send_map));

        let send_handle = send_out(&cli.socketcan_iface, Arc::clone(&send_map));

        siren_handle.await.expect("Encoder failed with ");
        println!("Encoder ended");

        send_handle.await.expect("Sender failed with ");
        println!("Sender ended");
    }

    for handle in can_handles {
        handle.await.expect("Decoder Thread Failed");
    }
    println!("Decoder ended");
}
