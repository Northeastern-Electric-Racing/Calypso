use std::path::PathBuf;

use protobuf::Message;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tracing::{debug, trace, warn};
use zenoh::{Config, Session, bytes::Encoding, sample::Sample};

use crate::proto::{
    command_data::{self, CommandData},
    serverdata::ServerData,
};

/// The chief processor of incoming zenoh data, this handles
/// - zenoh state
/// - reception via mqtt and subsequent parsing
///   Takes in many channels:
/// - `zenoh_sender_rx`: A receiver of any messages, it then publishes them
/// - `zenoh_recv_tx`: Optional, a sender of all zenoh messages, if None no messages sent
pub struct ZenohProcessor {
    cancel_token: CancellationToken,
    zenoh_sender_rx: Receiver<(String, ServerData)>,
    zenoh_recv_tx: Option<Sender<(String, CommandData)>>,
    session: Session,
}

impl ZenohProcessor {
    /// Creates a new Zenoh receiver and sender
    /// # Panics
    ///  Panics if zenoh conf invalid
    pub async fn new(
        cancel_token: CancellationToken,
        mqtt_sender_rx: Receiver<(String, ServerData)>,
        mqtt_recv_tx: Option<Sender<(String, CommandData)>>,
        conf_path: PathBuf,
    ) -> ZenohProcessor {
        zenoh::init_log_from_env_or("info");

        let session = zenoh::open(Config::from_file(conf_path).expect("Could not find Zenoh conf"))
            .await
            .expect("Invalid zenoh conf");

        ZenohProcessor {
            cancel_token,
            zenoh_sender_rx: mqtt_sender_rx,
            zenoh_recv_tx: mqtt_recv_tx,
            session,
        }
    }

    fn convert_to_mqtt(sample: &zenoh::sample::Sample) -> Option<CommandData> {
        command_data::CommandData::parse_from_reader(&mut sample.payload().reader()).ok()
    }

    async fn handle_recv(&self, sample: Sample) {
        let Some(msg) = Self::convert_to_mqtt(&sample) else {
            warn!("Could not deserialize Zenoh incoming!");
            return;
        };
        if let Some(ref recv_tx) = self.zenoh_recv_tx
            && let Err(e) = recv_tx.send((sample.key_expr().to_string(), msg)).await
        {
            warn!("Error putting received command into queue! {}", e);
        }
    }

    /// This handles the reception of mqtt messages, will not return
    /// # Panics
    ///  Panics if subscription fails
    pub async fn process_zenoh(mut self) {
        debug!("Subscribing to siren, all topics");
        let subscriber = self
            .session
            .declare_subscriber("Calypso/Bidir/Command/**")
            .await
            .expect("Could not subscribe to MQTT");

        loop {
            tokio::select! {
                () = self.cancel_token.cancelled() => {
                    debug!("Shutting down Zenoh processor!");
                    break;
                },
                Ok(msg) = subscriber.recv_async() => {
                    self.handle_recv(msg).await;
                },
                Some(sendable) = self.zenoh_sender_rx.recv() => {
                    trace!("Sending {:?}", sendable);
                    let Ok(bytes) = protobuf::Message::write_to_bytes(&sendable.1) else {
                        warn!("Failed to serialize protobuf message!");
                        continue;
                    };

                    if let Err(err)= self.session.put(sendable.0, bytes).encoding(Encoding::APPLICATION_PROTOBUF).await {
                        warn!("Error sending zenoh message: {}", err);
                    }
                }
            }
        }
    }
}
