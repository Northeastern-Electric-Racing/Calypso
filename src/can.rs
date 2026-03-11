use crate::data::{DecodeData, EncodeData};
use crate::decode_data::DECODE_FUNCTION_MAP;
use crate::proto::serverdata::ServerData;
use futures_util::StreamExt;
use socketcan::Id;
use socketcan::tokio::CanSocket;
use socketcan::{CanDataFrame, CanError, CanFrame, EmbeddedFrame, Frame, SocketOptions};
use std::collections::HashMap;
use std::time::UNIX_EPOCH;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, trace, warn};
/**
 * Helper function to create a `CanFrame`
 * msg: (id, `EncodeData`), the message to send
 * # Panics
 * If an invaid ID is sent by the `EncodeData`
 */
#[must_use]
pub fn create_frame(msg: (&u32, &EncodeData)) -> Option<CanFrame> {
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
pub async fn release_commands<S: ::std::hash::BuildHasher>(
    send_map: &HashMap<u32, EncodeData, S>,
    can_push_send: &Sender<CanFrame>,
) {
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
 * Reads the can socket and publishes the data to siren channel
 * `can_interface`: the socketcan interface to bind to
 * `send_to_siren`: the channel to send protobuf messages to
 * `alt_send_to_siren`: the channel to send priority queue alt messages to
 * `send_over_can`: can messages to be sent over CAN
 *
 * # Panics
 * Panics if can socket could not be opened
 */
pub async fn can_manager(
    token: CancellationToken,
    can_interface: String,
    main_send_to_siren: Sender<(String, ServerData)>,
    alt_send_to_siren: Option<Sender<(String, ServerData)>>,
    send_raw_can: Option<Sender<CanDataFrame>>,
    mut send_over_can: Receiver<CanFrame>,
) {
    let mut socket = CanSocket::open(&can_interface).expect("Failed to open CAN socket!");
    socket
        .set_error_filter_accept_all()
        .expect("Failed to set error mask on CAN socket!");
    socket
        .set_recv_own_msgs(true) // important to get the bidir messages
        .expect("Cant recv own messages");

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
            Some(frame) = socket.next() => {
                frame_cnt += 1;
                pub_frame(frame, &main_send_to_siren, alt_send_to_siren.as_ref(), send_raw_can.as_ref(), &mut mqtt_cnt, ).await;
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
    frame: Result<CanFrame, socketcan::Error>,
    main_send: &Sender<(String, ServerData)>,
    alt_send: Option<&Sender<(String, ServerData)>>,
    raw_send: Option<&Sender<CanDataFrame>>,
    cnt: &mut u64,
) {
    let decoded_data = match frame {
        // CanDataFrame
        Ok(CanFrame::Data(data_frame)) => {
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
    // TODO switch to hardware timestamps
    let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;

    // Convert decoded CAN to Protobuf and publish over MQTT
    for data in &decoded_data {
        *cnt += 1;
        let mut payload = ServerData::new();
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
