use crate::proto::firmware_data::FirmwareData;
use crate::proto::serverdata::ServerData;
use protobuf::Message;
use tokio::{net::UdpSocket, sync::mpsc::Sender};
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};

/// the maximum UDP datagram size to read
const MAX_BUF_SIZE: usize = 1024;

/**
 * The ethernet manager to recieve ethernet packets and send them over MQTT (Input `FirmwareData`, output `ServerData`)
 *
 * # Errors
 * Errors if the UDP socket incurs an underlying IO error
 */
pub async fn eth_manager(
    token: CancellationToken,
    eth_ip: String,
    main_send_to_siren: Sender<(String, ServerData)>,
) -> Result<(), std::io::Error> {
    let sock = UdpSocket::bind(eth_ip).await?;
    let mut buf = [0; MAX_BUF_SIZE];
    loop {
        tokio::select! {
            () = token.cancelled() => {
                debug!("Shutting down ETH reader!");
                break;
            },
            Ok((len, _)) = sock.recv_from(&mut buf) => {
                if len >= MAX_BUF_SIZE {
                    warn!("Exceeded maximum ethernet buffer size, please increase!");
                    continue;
                }
                handle_eth_msg(&buf[0..len], &main_send_to_siren).await;
            }
        }
    }

    Ok(())
}

async fn handle_eth_msg(data: &[u8], main_send_to_siren: &Sender<(String, ServerData)>) {
    let Ok(data) = FirmwareData::parse_from_bytes(data) else {
        warn!("Could not parse packet, not valid protobuf!");
        return;
    };

    let mut server_data = ServerData::new();
    server_data.time_us = data.time_us;
    server_data.unit = data.unit;
    server_data.values = data.values;

    if let Err(err) = main_send_to_siren.send((data.topic, server_data)).await {
        warn!("Could not send ETH translated packet over channel: {}", err);
    }
}
