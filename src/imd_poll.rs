use std::time::Duration;
use std::time::UNIX_EPOCH;

use socketcan::{CanDataFrame, CanFrame, EmbeddedFrame, Id};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};

use crate::proto::serverdata::ServerData;

enum ImdScaling {
    Default,
    TimesPoint1,
    Voltage,
}
struct ImdPacket {
    pub id: u8,
    pub size: usize,
    pub topic: &'static str,
    pub unit: &'static str,
    pub scaling: ImdScaling,
}

impl ImdPacket {
    pub fn convert(&self, input: &mut Vec<u8>) -> (String, ServerData) {
        // extend to id plus size of u32.  All this fuckery needed as from_le_bytes requires exactly 4 bytes
        input.truncate(self.size + 1);
        input.resize(5, 0);
        let out = u32::from_le_bytes(
            input[1..=size_of::<u32>()]
                .try_into()
                .expect("Critical failure to decode IMD message"),
        );
        let send: f32 = match self.scaling {
            ImdScaling::Default => out as f32,
            ImdScaling::TimesPoint1 => out as f32 * 0.1,
            ImdScaling::Voltage => (out as f32 * 0.05) - 1606.4,
        };

        let mut payload = ServerData::new();
        payload.unit = self.unit.to_string();
        payload.values = vec![send];
        payload.time_us = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;

        (self.topic.to_string(), payload)
    }
}

// To add a supported field from the datasheet, add it here.
// Note that for now we are max 4 byte fields, a field 5 bytes or more cannot be read by the driver and will be truncated
const SENDABLES: [ImdPacket; 9] = [
    ImdPacket {
        id: 0x40,
        size: 2,
        topic: "IMD/Info/Iso_Detail/R_iso_neg",
        unit: "kOhm",
        scaling: ImdScaling::Default,
    },
    ImdPacket {
        id: 0x42,
        size: 2,
        topic: "IMD/Info/Iso_Detail/R_iso_pos",
        unit: "kOhm",
        scaling: ImdScaling::Default,
    },
    ImdPacket {
        id: 0x4E,
        size: 2,
        topic: "IMD/Info/Iso_Detail/R_iso_original",
        unit: "kOhm",
        scaling: ImdScaling::Default,
    },
    ImdPacket {
        id: 0x3E,
        size: 1,
        topic: "IMD/Info/Iso_Detail/Iso_quality",
        unit: "%",
        scaling: ImdScaling::Default,
    },
    ImdPacket {
        id: 0x5E,
        size: 2,
        topic: "IMD/Info/Voltage/HV_system",
        unit: "V",
        scaling: ImdScaling::Voltage,
    },
    ImdPacket {
        id: 0x60,
        size: 2,
        topic: "IMD/Info/Voltage/HV_neg_to_earth",
        unit: "V",
        scaling: ImdScaling::Voltage,
    },
    ImdPacket {
        id: 0x62,
        size: 2,
        topic: "IMD/Info/Voltage/HV_pos_to_earth",
        unit: "V",
        scaling: ImdScaling::Voltage,
    },
    ImdPacket {
        id: 0x52,
        size: 2,
        topic: "IMD/Info/It_System/Capacity_measured_value",
        unit: "uF",
        scaling: ImdScaling::TimesPoint1,
    },
    ImdPacket {
        id: 0x2A,
        size: 1,
        topic: "IMD/Info/It_System/Unbalance_measured_value",
        unit: "%",
        scaling: ImdScaling::Default,
    },
];

/// This thread polls the IMD for data as our firmware version is too old to get the data automatically (RIP)
/// It 1. asks for a message, 2. waits for the message, 3. sends out said message, and 4. sends out another request
///
/// # Panics
/// Panics if it doesnt work
pub async fn imd_poll_main(
    cancel_token: CancellationToken,
    can_send: mpsc::Sender<CanFrame>,
    mut can_recv: mpsc::Receiver<CanDataFrame>,
    mqtt_send: mpsc::Sender<(String, ServerData)>,
) {
    debug!("Starting IMD manager");
    // send/recieve cyclically
    // pub frame
    // recv frame
    // pub mqtt

    // pre-compute starter ID
    let id: Id = socketcan::StandardId::new(0x22).expect("Tf").into();

    // we cannot exceed 10Hz.  Some extra leeway here. This missed tick burst will guarrantee we send at the next available moment after 100ms has passed
    let mut min_time: tokio::time::Interval = tokio::time::interval(Duration::from_millis(120));

    let mut lock: bool = false;
    let mut curr_idex: usize = 0;
    loop {
        tokio::select! {
            () = cancel_token.cancelled() => {
                debug!("Shutting down IMD manager!");
                break;
            },
            Some(val) = can_recv.recv() => {
                // check ID is Ox23
                match val.id() {
                    Id::Standard(id) => {
                        if id.as_raw() != 0x23 {
                            continue;
                        }
                    },
                    Id::Extended(_) => {
                        // ignore message
                        continue;
                    }
                }

                // check message identifier equals current index, if not warn and bail
                if val.data()[0] != SENDABLES[curr_idex].id {
                    warn!("Detected unknown request response id for IMD!");
                    continue;
                }

                let mut data = val.data().to_owned();
                let to_send = SENDABLES[curr_idex].convert(&mut data);
                if let Err(err) = mqtt_send.send(to_send).await {
                    warn!("Could not send MQTT message from IMD {}", err);
                }

                // we can now send the next message on the next loop
                lock = false;

            },
            // this is fancy ass code to basically say if its been 100ms since last message, and we arent locked, go for it
            _ = min_time.tick(), if !lock => {
                curr_idex += 1;
                if curr_idex >= SENDABLES.len() {
                    curr_idex = 0;
                }
                // send can frame.  Byte 1 is the ID of the data we want
                if let Err(err) = can_send.send(CanFrame::new(id, &[SENDABLES[curr_idex].id]).expect("Failed to create IMD frame")).await {
                    warn!("Error sending IMD frame: {}", err);
                }

                // lets reset our interval, (i.e the 100ms restarts now)
                min_time.reset();
                // lets also lock the thread so we dont try and send another
                lock = true;
            }
        };
    }
}
