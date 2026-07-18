use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use calypso::{
    data::{DecodeData, EncodeData},
    decode_data::DECODE_FUNCTION_MAP,
    encode_data::{ENCODABLE_KEY_LIST, ENCODE_FUNCTION_MAP},
    imd_poll::imd_poll_main,
    mqtt_handler::{poll_stub, publish_stub, siren_creator},
    proto::{
        command_data::CommandData,
        serverdata::{self, ServerData},
    },
    zenoh_handler::ZenohProcessor,
};
use calypso_cangen::can_types::BidirMode;
use clap::Parser;
use socketcan::{
    CanDataFrame, CanError, CanFrame, EmbeddedFrame, Frame, Id, SocketOptions, tokio::CanSocket,
};
use tokio::{
    signal,
    sync::mpsc::{self, Receiver, Sender},
};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tracing::{debug, info, level_filters::LevelFilter, trace, warn};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

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

    /// The `SocketCAN` interface port
    #[arg(
        short = 'c',
        long,
        env = "CALYPSO_SOCKETCAN_IFACE",
        default_value = "vcan0"
    )]
    socketcan_iface: String,

    /// Whether to use
    #[arg(long, env = "CALYPSO_CAN_ENCODE")]
    imd: bool,

    /// Use Zenoh instead of MQTT -- will eventually become default
    #[arg(short = 'z', long, env = "CALYPSO_ZENOH")]
    zenoh: bool,

    /// Zenoh conf file
    #[arg(long, env = "CALYPSO_ZENOH_CONF", default_value_os = "./zenoh.json5")]
    zenoh_conf: PathBuf,
}

/**
 * Reads the can socket and publishes the data to siren channel
 * `can_interface`: the socketcan interface to bind to
 * `send_to_siren`: the channel to send protobuf messages to
 * `alt_send_to_siren`: the channel to send priority queue alt messages to
 * `send_over_can`: can messages to be sent over CAN
 */
async fn can_manager(
    token: CancellationToken,
    can_interface: String,
    main_send_to_siren: Sender<(String, ServerData)>,
    alt_send_to_siren: Option<Sender<(String, ServerData)>>,
    send_raw_can: Option<Sender<CanDataFrame>>,
    mut send_over_can: Receiver<CanFrame>,
) {
    let socket = CanSocket::open(&can_interface).expect("Failed to open CAN socket!");
    socket
        .set_error_filter_accept_all()
        .expect("Failed to set error mask on CAN socket!");
    socket
        .set_recv_own_msgs(true) // important to get the bidir messages
        .expect("Cant recv own messages");
    socket
        .set_recv_timestamp(true)
        .expect("Cant set fetch timestamp");
    // socket
    //     .set_timestamping(SOF_TIMESTAMPING_RX_SOFTWARE | SOF_TIMESTAMPING_SOFTWARE)
    //     .expect("Cant set timestamping flags");

    // the rate variables, updated every 3 seconds to the user
    let mut mqtt_cnt: u64 = 0u64;
    let mut frame_cnt: u64 = 0u64;
    let mut disp_interval = tokio::time::interval(Duration::from_secs(3));
    let mut time_interval = tokio::time::Instant::now();

    loop {
        tokio::select! {
            () = token.cancelled() => {
                debug!("Shutting down CAN reader!");
                break;
            },
            res = socket.read_frame_with_timestamp() => {
                match res {
                    Ok((frame, time)) => {
                        frame_cnt += 1;
                        pub_frame(frame, time, &main_send_to_siren, alt_send_to_siren.as_ref(), send_raw_can.as_ref(), &mut mqtt_cnt, ).await;
                    },
                    Err(err) => {
                        warn!("CAN Socket failure: {}", err);
                    }
                }
            }
            Some(frame) = send_over_can.recv() => {
                match socket.write_frame(frame).await {
                    Ok(()) => (),
                    Err(r) => warn!("Could not send CAN frame: {}", r),
                }
            },
            _ = disp_interval.tick() => {
                info!("{:.3} msgs/sec and {:.3} frames/sec", (mqtt_cnt as f64
                / (tokio::time::Instant::now() - time_interval).as_millis() as f64) * 1000f64,
                (frame_cnt as f64 / (tokio::time::Instant::now() - time_interval).as_millis() as f64) * 1000f64);
                time_interval = tokio::time::Instant::now();
                frame_cnt = 0;
                mqtt_cnt = 0;
            }
        }
    }
}

/**
 * Handles reception of a frame or error
 * frame: the frame
 * `main_send`: the siren receiver
 * `alt_send`: the priority siren receiver
 * cnt: a variable incremented per MQTT message sent over `main_send`
 */
async fn pub_frame(
    frame: CanFrame,
    timestamp: SystemTime,
    main_send: &Sender<(String, ServerData)>,
    alt_send: Option<&Sender<(String, ServerData)>>,
    raw_send: Option<&Sender<CanDataFrame>>,
    cnt: &mut u64,
) {
    let decoded_data = match frame {
        // CanDataFrame
        CanFrame::Data(data_frame) => {
            let data = data_frame.data();
            let id: u32 = match data_frame.id() {
                socketcan::Id::Standard(std) => std.as_raw().into(),
                socketcan::Id::Extended(ext) => ext.as_raw(),
            };
            if let Some(send) = raw_send {
                // for now just hardcode IMD
                if id == 0x23
                    && let Err(err) = send.send(data_frame).await
                {
                    warn!("Could not send IMD code the response! {}", err);
                }
            }
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
        CanFrame::Remote(remote_frame) => {
            // Send frame ID for Remote
            vec![DecodeData::new(
                vec![remote_frame.raw_id() as f32],
                "Calypso/Events/RemoteFrame",
                "id",
                None,
            )]
        }
        // CanErrorFrame
        CanFrame::Error(error_frame) => {
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
    };
    // TODO switch to hardware timestamps
    let timestamp = timestamp.duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;

    // Convert decoded CAN to Protobuf and publish over MQTT
    for data in &decoded_data {
        *cnt += 1;
        let mut payload = serverdata::ServerData::new();
        payload.unit.clone_from(&data.unit);
        payload.values.clone_from(&data.value);
        payload.time_us = timestamp;

        if let Some(alt_send) = alt_send
            && let Some(clients) = &data.clients
            && clients.first().unwrap_or(&0) == &1882
        {
            match alt_send.send((data.topic.clone(), payload.clone())).await {
                Ok(()) => trace!("Sent a CAN message to SIREN manager alt"),
                Err(err) => {
                    warn!("Could not send CAN message to SIREN manager alt: {}", err);
                }
            }
        }

        match main_send.send((data.topic.clone(), payload)).await {
            Ok(()) => trace!("Sent a CAN message to SIREN manager"),
            Err(err) => warn!("Could not send CAN message to SIREN manager: {}", err),
        }
    }
}

/**
 * Thread to manage bidirectionality, both sending can messages and receiving MQTT messages from respective channels
 * `can_push_send`: the channel to send out CAN messages
 * `siren_recv`: the MQTT messages to receive
 * encode: actually sends out the CAN messages
 */
async fn bidir_manager(
    token: CancellationToken,
    can_push_send: Sender<CanFrame>,
    mut siren_recv: Receiver<(String, CommandData)>,
    encode: bool,
) {
    let mut send_interval = tokio::time::interval(Duration::from_millis(750));

    let mut send_map = HashMap::new();
    // build an initial map
    for key in ENCODABLE_KEY_LIST {
        let encoder_func = match ENCODE_FUNCTION_MAP.get(key) {
            Some(func) => func.0,
            None => panic!("An unknown message was initialized!"),
        };
        let ret = encoder_func(Vec::new());
        send_map.insert(ret.id, ret);
    }

    loop {
        tokio::select! {
            () = token.cancelled() => {
                debug!("Shutting down BIDIR manager!");
                break;
            },
            _ = send_interval.tick() => {
                if encode {
                    release_commands(&send_map, &can_push_send).await;
                } else {
                    trace!("Not releasing commands, upload disabled");
                }
            }
            Some(msg) = siren_recv.recv() => {
                if let Some(packet) = parse_msg(msg, &mut send_map) { match can_push_send.send(packet).await {
                Ok(()) => (),
                Err(err) => warn!("Error sending can command to can manager {}", err),
                } }
            },
        }
    }
}

/**
 * Helper function to create a `CanFrame`
 * msg: (id, `EncodeData`), the message to send
 */
fn create_frame(msg: (&u32, &EncodeData)) -> Option<CanFrame> {
    let id: Id = if msg.1.is_ext {
        socketcan::ExtendedId::new(msg.1.id)
            .unwrap_or_else(|| panic!("Invalid extended ID: {}", msg.1.id))
            .into()
    } else {
        socketcan::StandardId::new(
            msg.1
                .id
                .try_into()
                .unwrap_or_else(|_| panic!("Invalid standard ID: {}", msg.1.id)),
        )
        .unwrap_or_else(|| panic!("Invalid standard ID: {}", msg.1.id))
        .into()
    };

    CanFrame::new(id, &msg.1.value)
}

/**
 * Helper function to dump the current bidir commands into CAN
 * `send_map`: A map of CAN IDs and data to be sent to the car
 * `can_push_send`: A channel to send CAN messages
 */
async fn release_commands(send_map: &HashMap<u32, EncodeData>, can_push_send: &Sender<CanFrame>) {
    for msg in send_map {
        // let id = u32::from_str_radix((msg.1.1).trim_start_matches("0x"), 16).expect("Invalid CAN ID!");

        match create_frame(msg) {
            Some(packet) => match can_push_send.send(packet).await {
                Ok(()) => (),
                Err(err) => warn!("Error sending can command to can manager {}", err),
            },
            None => {
                warn!("Packet is too long: {}", msg.1);
            }
        }
    }
}

/**
 * Helper function to parse a MQTT message to create the corresponding bidir update
 * msg: The raw MQTT message
 * `send_map`: The map of CAN IDs and encodable data to modify
 *
 * Will return the can frame to send immediately, if available
 */
fn parse_msg(
    msg: (String, CommandData),
    send_map: &mut HashMap<u32, EncodeData>,
) -> Option<CanFrame> {
    let key = if let Some(key) = msg.0.split('/').next_back() {
        key.to_owned()
    } else {
        warn!("Could not parse the key value in {}", msg.0);
        return None;
    };

    debug!("Parsing message with key {}", key);

    if let Some(func) = ENCODE_FUNCTION_MAP.get(&key) {
        let ret = func.0(msg.1.data);
        if func.1 == BidirMode::Broadcast {
            send_map.insert(ret.id, ret);
            None
        } else if let Some(packet) = create_frame((&ret.id, &ret)) {
            Some(packet)
        } else {
            warn!("Oneshot encodable packet is too long: {}", ret.id);
            None
        }
    } else {
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
        None
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
    let (decoder_send, decoder_recv) = mpsc::channel::<(String, ServerData)>(500);

    // a channel to hijack certain raw CAN messages, right now only used for IMD
    let (can_decoder_send, can_decoder_recv) = if cli.imd {
        let ch = mpsc::channel::<CanDataFrame>(50);
        (Some(ch.0), Some(ch.1))
    } else {
        (None, None)
    };

    // a channel to give CAN messages back out (car commands)
    let (can_push_send, can_push_recv) = mpsc::channel::<CanFrame>(100);

    // a channel to give messages to the bidir manager
    let (siren_recv_send, siren_recv_recv) = mpsc::channel::<(String, CommandData)>(100);

    if cli.zenoh {
        let zenoh =
            ZenohProcessor::new(token.clone(), decoder_recv, siren_recv_send, cli.zenoh_conf).await;
        task_tracker.spawn(zenoh.process_zenoh());
    } else {
        // the actual client and eventloop handlers
        let main_broker = siren_creator(cli.siren_host_url).await;

        task_tracker.spawn(poll_stub(token.clone(), main_broker.1, siren_recv_send));
        task_tracker.spawn(publish_stub(token.clone(), main_broker.0, decoder_recv));
    }

    task_tracker.spawn(can_manager(
        token.clone(),
        cli.socketcan_iface,
        decoder_send.clone(),
        None,
        can_decoder_send,
        can_push_recv,
    ));

    task_tracker.spawn(bidir_manager(
        token.clone(),
        can_push_send.clone(),
        siren_recv_recv,
        cli.encode,
    ));

    if let Some(can_decoder_recv) = can_decoder_recv {
        task_tracker.spawn(imd_poll_main(
            token.clone(),
            can_push_send,
            can_decoder_recv,
            decoder_send,
        ));
    }

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
