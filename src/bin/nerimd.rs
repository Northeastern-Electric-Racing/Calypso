// This is the NER IMD CLI, for changing IMD settings on the fly
// See the calypso commands for changing functionality programatically
// Currently supports Bender ISO175c over the CAN 2.0 Classic interface
// Written for spec Standard CAN 2025 07

use futures_util::StreamExt;
use std::time::Duration;

use clap::{Parser, Subcommand, ValueEnum};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use socketcan::{tokio::CanSocket, CanDataFrame, CanId, EmbeddedFrame, SocketOptions, StandardId};
use tracing::{debug, error};

#[derive(Parser)]
#[command(version, about = "Control the ISO175c")]
struct NerImdArgs {
    /// The SocketCAN interface port
    #[arg(
        short = 'c',
        long,
        env = "CALYPSO_SOCKETCAN_IFACE",
        default_value = "vcan0"
    )]
    socketcan_iface: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Control {
        #[command(subcommand)]
        cmd: CtrlCommands,
    },
    Set {
        #[command(subcommand)]
        cmd: SetCommands,
    },
    Get {
        #[command(subcommand)]
        cmd: GetCommands,
    },
}

// DEFINITIONS
#[derive(Subcommand)]
/// Commands to control the system
enum CtrlCommands {
    /// Reset alarm flags which are not active
    ResetAlarm {
        #[arg(default_value = "51", hide = true)]
        id: u8,
    },
    /// Selftest can only be triggered in R_iso_status = 0xFE
    TriggerSelfTest {
        option: TriggerSelfTestOpts,
        #[arg(default_value = "87", hide = true)]
        id: u8,
    },
    /// Requires Status: Lock = 0xFC (Parameter Write Enable)
    FactoryReset {
        #[arg(default_value = "111", hide = true)]
        id: u8,
    },
    /// Maximum delay time for execution: 5s
    EarthLiftStatus {
        /// Whether earth connection is opened or closed
        option: EarthLiftStatusOpts,
        #[arg(default_value = "113", hide = true)]
        id: u8,
        #[arg(default_value = "112", hide = true)]
        read_back_id: u8,
    },
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum TriggerSelfTestOpts {
    /// offline test
    OfflineTest = 0x01,
    /// offline and communication test
    OfflineAndCommsTest = 0x02,
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum EarthLiftStatusOpts {
    /// Earth connection closed
    Closed = 0x01,
    /// Earth connection opened
    Open = 0x02,
}

#[derive(Subcommand)]
/// Commands to setup the system
enum SetCommands {
    UnbalanceThreshold {
        /// Unbalance alarm threshold [%]
        #[arg(value_parser = clap::value_parser!(u8).range(15..=45))]
        threshold: u8,
        #[arg(default_value = "47", hide = true)]
        id: u8,
        #[arg(default_value = "46", hide = true)]
        read_back_id: u8,
    },
    IsoAlarmReset {
        opts: IsoAlarmResetOpts,
        #[arg(default_value = "49", hide = true)]
        id: u8,
        #[arg(default_value = "48", hide = true)]
        read_back_id: u8,
    },
    IsoActiveProfile {
        opts: IsoProfileOpts,
        #[arg(default_value = "57", hide = true)]
        id: u8,
        #[arg(default_value = "56", hide = true)]
        read_back_id: u8,
    },
    IsoPowerOnProfile {
        opts: IsoProfileOpts,
        #[arg(default_value = "59", hide = true)]
        id: u8,
        #[arg(default_value = "58", hide = true)]
        read_back_id: u8,
    },
    IsoThresholdError {
        /// Isolation error threshold [kΩ]
        #[arg(value_parser = clap::value_parser!(u16).range(30..=2000))]
        threshold: u16,
        #[arg(default_value = "71", hide = true)]
        id: u8,
        #[arg(default_value = "70", hide = true)]
        read_back_id: u8,
    },
    IsoThresholdTimeout {
        /// Threshold time [s] (0=alarm deactivated)
        #[arg(value_parser = clap::value_parser!(u16).range(0..=64255))]
        time: u16,
        #[arg(default_value = "73", hide = true)]
        id: u8,
        #[arg(default_value = "72", hide = true)]
        read_back_id: u8,
    },
    IsoThresholdWarning {
        /// Isolation warning threshold [kΩ]
        #[arg(value_parser = clap::value_parser!(u16).range(30..=2000))]
        threshold: u16,
        #[arg(default_value = "75", hide = true)]
        id: u8,
        #[arg(default_value = "74", hide = true)]
        read_back_id: u8,
    },
    SelfTestPeriod {
        /// Period [10s] (0=self test deactivated)
        #[arg(value_parser = clap::value_parser!(u16).range(1..=64255))]
        time: u16,
        #[arg(default_value = "75", hide = true)]
        id: u8,
        #[arg(default_value = "88", hide = true)]
        read_back_id: u8,
    },
    VoltageMode {
        opts: VoltageModeOpts,
        #[arg(default_value = "101", hide = true)]
        id: u8,
        #[arg(default_value = "100", hide = true)]
        read_back_id: u8,
    },
    VoltageThresholdUnderVolts {
        /// Voltage [V] (0=deactivated)
        #[arg(value_parser = clap::value_parser!(u16).range(1..=1000))]
        threshold: u16,
        #[arg(default_value = "103", hide = true)]
        id: u8,
        #[arg(default_value = "102", hide = true)]
        read_back_id: u8,
    },
    StatusLock {
        opts: StatusLockOpts,
        #[arg(default_value = "107", hide = true)]
        id: u8,
        #[arg(default_value = "106", hide = true)]
        read_back_id: u8,
    },
    IsoThresholdFirstRefEstimate {
        /// Threshold voltage for estimation reference [V]
        #[arg(value_parser = clap::value_parser!(u16).range(1..=1000))]
        threshold: u16,
        #[arg(default_value = "115", hide = true)]
        id: u8,
        #[arg(default_value = "114", hide = true)]
        read_back_id: u8,
    },
    IsoPreEstimateMaxDiff {
        /// maximum voltage difference for estimation evaluation [0.01V]
        #[arg(value_parser = clap::value_parser!(u16).range(0..=64255))]
        threshold: u16,
        #[arg(default_value = "117", hide = true)]
        id: u8,
        #[arg(default_value = "116", hide = true)]
        read_back_id: u8,
    },

    // with new FW
    InterfaceCanId {
        /// The message to change
        msg: IntefaceCanIdOpts,
        /// The new CAN-ID
        #[arg(value_parser = clap::value_parser!(u8).range(0x20..=0xFE))]
        new_id: u8,
        #[arg(default_value = "119", hide = true)]
        id: u8,
        #[arg(default_value = "118", hide = true)]
        read_back_id: u8,
    },
    InterfacePeriodicCycleTime {
        /// The message to change
        msg: InterfacePeriodicCycleTimeOpts,
        /// The cycle time [100ms] (0=deactivated)
        #[arg(value_parser = clap::value_parser!(u8).range(0x01..=0xFA))]
        time: u8,
        #[arg(default_value = "121", hide = true)]
        id: u8,
        #[arg(default_value = "120", hide = true)]
        read_back_id: u8,
    },
    InterfaceBaudrate {
        /// Sets the interface baudrate: WARNING, communication will cease until host follows
        rate: InterfaceBaudrateOpts,
        #[arg(default_value = "123", hide = true)]
        id: u8,
    },
    IsoIsoInit {
        /// [kOhms]
        #[arg(value_parser = clap::value_parser!(u16).range(0x00..=0xC350))]
        value: u16,
        #[arg(default_value = "127", hide = true)]
        id: u8,
        #[arg(default_value = "126", hide = true)]
        read_back_id: u8,
    },
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum IsoAlarmResetOpts {
    /// Automatic Iso-alarm reset
    Auto = 0xFC,
    /// Self-holding Iso-alarm (must be reset via command)
    SelfHolding = 0xFD,
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum IsoProfileOpts {
    StandardFastStart = 1,
    Standard,
    HighCapacityFastStart,
    HighCapacity,
    Disturbed,
    Service,
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum VoltageModeOpts {
    AC = 0xFD,
    DC = 0xFE,
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum StatusLockOpts {
    /// Parameter write enable
    En = 0xFC,
    /// Parameter write disable
    Dis = 0xFD,
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum IntefaceCanIdOpts {
    Request = 0x00,
    Response = 0x01,
    ImdInfoGeneral = 0x02,
    ImdInfoIsoDetail = 0x03,
    ImdInfoVoltage = 0x04,
    ImdInfoItSystem = 0x05,
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum InterfacePeriodicCycleTimeOpts {
    General = 0x00,
    IsoDetail = 0x01,
    Voltage = 0x02,
    ItSystem = 0x03,
}

#[derive(ValueEnum, IntoPrimitive, Clone)]
#[repr(u8)]
enum InterfaceBaudrateOpts {
    Mb1 = 0x01,
    Kb800,
    Kb666,
    Kb500,
    Kb250,
    Kb125,
}

#[derive(Subcommand)]
#[repr(u8)]
/// Commands to only read and retreive settings about the system
enum GetCommands {
    BootLoaderBuildNo = 0x0A,
    BootLoaderDNumber = 0x0C,
    BootLoaderVersion = 0x0E,
    HardwareIdAHHistory = 0x10,
    HardwareIdAHNumber = 0x12,
    Au8AHNumberPartB = 0x14,
    HardwareIdItemNumber = 0x16,
    Au8AHArticleNumberPartB = 0x18,
    HardwareIdSerialNumber = 0x1A,
    Au8SerialNumberPartB = 0x1C,
    SoftwareIdBuildNumber = 0x1E,
    SoftwareIdDNumber = 0x20,
    /// eg 100 --> 1.00
    SoftwareIdVersion = 0x22,
    /// 0...100: Measured value [%] (0=HV+...100=HV-)
    UnBalanceMeasuredValue = 0x2A,
    /// 0...255: Counter will be incremented with each new measured unbalance value
    UnBalanceMeasurementCounter = 0x2C,
    /// see SET
    UnbalanceThreshold = 0x2E,
    /// see SET
    IsoAlarmReset = 0x30,
    /// 0...255: Counter will be incremented with each new measured isolation resistance value
    IsoMeasurementCounter = 0x36,
    /// see SET
    IsoActiveProfile = 0x38,
    /// see SET
    IsoPowerOnProfile = 0x3A,
    /// 0...100: Quality [%]
    IsoQuality = 0x3E,
    /// 0…50000: Isolation resistance on HV_neg [kΩ]
    IsoRIsoNeg = 0x40,
    /// 0…50000: Isolation resistance on HV_pos [kΩ]
    IsoRIsoPos = 0x42,
    /**
    * 0xFC: estimated isolation value during startup
      0xFD: first measured isolation value during startup
      0xFE: Isolation value in normal operation
    */
    RIsoStatus = 0x44,
    /// see SET
    IsoThresholdError = 0x46,
    /// see SET
    IsoThresholdTimeout = 0x48,
    /// see SET
    IsoThresholdWarning = 0x4A,
    /// 0…40500: corrected isolation value [kΩ]
    IsoRIsoCorrected = 0x4C,
    /// 0…50000: original isolation value [kΩ]
    IsoRIsoOriginal = 0x4E,
    /// 1…64255: elapsed time [s]
    IsoTimeSinceLastMeasurement = 0x50,
    /// 1...200: Capacity value [0.1 uf]
    CapacityMeasuredValue = 0x52,
    /// 0...255: Counter will be incremented with each new measured capacity value
    CapacityMeasurementCounter = 0x54,
    /// see SET
    SelfTestPeriod = 0x58,
    /// 0...255: Counter will be incremented with each new measured voltage value
    VoltageMeasurementCounter = 0x5C,
    /// 0…64255: HV system voltage [0.05 V] Offset: 32128 (1606.4 V) valid range: -1606.4 V...+1606.35 V
    VoltageHVSystem = 0x5E,
    /// 0… 64255: HV_neg to Earth voltage [0.05 V] Offset: 32128 (1606.4 V) valid range: -1606.4 V...+1606.35 V
    VoltageHvNegToEarth = 0x60,
    /// 0… 64255: HV_pos to MarinaEarth voltage [0.05 V] Offset: 32128 (1606.4 V) valid range: -1606.4 V...+1606.35 V
    VoltageHvPosToEarth = 0x62,
    /// see SET
    VoltageMode = 0x64,
    /// see SET
    VoltageThresholdUnderVolts = 0x66,
    /**
    * 0: Initialization
      1: Normal operation
      2: Self test
    */
    StatusDeviceActivity = 0x68,
    /// see SET
    StatusLock = 0x6A,
    /**
         * Bit 0: true = Device error active
    Bit 1: true = HV_pos connection failure
    Bit 2: true = HV_neg connection failure
    Bit 3: true = Earth connection failure
    Bit 4: true = Iso alarm (iso value below threshold error)
    Bit 5: true = Iso warning (iso value below threshold warning)
    Bit 6: true = Iso outdated (value „time elapsed since last measurement“ > = „measurement timeout“)
    Bit 7: true = Unbalance alarm (unbalance value below threshold)
    Bit 8: true = Undervoltage alarm
    Bit 9: true = Unsafe to start
    Bit 10: true = Earthlift open
         */
    StatusWarningsAndAlarms = 0x6C,
    /// see SET
    IntefaceCanId = 0x76,
    /// see SET
    InterfacePeriodicCycleTime = 0x78,
    /// see SET
    IsoIsoInit = 0x7E,
}

// Possible error codes for set commands
#[derive(TryFromPrimitive, Debug)]
#[repr(u8)]
enum ErrorMessage {
    InvalidRequest = 0x23,
    ParameterLocked = 0x24,
    DataRangeOverflow = 0x25,
    Data2RangeOverflow = 0x26,
    CanIdAlreadyUsed = 0x27,
    WriteFailed = 0x28,
    ReadFailed = 0x29,
}

async fn write_ctrl_or_set(
    socket: &mut CanSocket,
    index: u8,
    mut data: Vec<u8>,
    read_back_index: Option<u8>,
) {
    data.insert(0, index);
    match socket
        .write_frame(socketcan::CanFrame::Data(
            CanDataFrame::new(
                CanId::Standard(StandardId::new(0x22).expect("Bad ID!")),
                &data,
            )
            .expect("Bad Data!"),
        ))
        .await
    {
        Ok(_) => {
            debug!("Successfully sent message {}", index);
            match read_back_index {
                Some(idex) => read_or_readback(socket, idex).await,
                None => debug!("Skipping readback for {}", index),
            }
        }
        Err(e) => error!("Could not send message {} for reason: {}", index, e),
    }
}

async fn read_or_readback(socket: &mut CanSocket, index: u8) {
    match socket
        .write_frame(socketcan::CanFrame::Data(
            CanDataFrame::new(
                CanId::Standard(StandardId::new(0x22).expect("Bad ID!")),
                &[index],
            )
            .expect("Bad Data!"),
        ))
        .await
    {
        Ok(_) => debug!("Successfully sent message {}", index),
        Err(e) => error!("Could not send message {} for reason: {}", index, e),
    }

    let mut timeout = tokio::time::interval(Duration::from_secs(2));
    loop {
        tokio::select! {
            _ = timeout.tick() => {
                error!("Did not recieve data within timeout!")
            }
            Some(Ok(frame)) = socket.next() => {
                match frame.id() {
                    socketcan::Id::Standard(standard_id) => if standard_id.as_raw() == 0x23 {
                        debug!("Found message!");
                        if frame.data()[0] == 0xFF {
                            println!("Data failure: {:?}", ErrorMessage::try_from_primitive(frame.data()[1]).expect("Invalid error code!"))
                        } else {
                            println!("Data returned: {:?}", frame.data());
                        }
                    },
                    socketcan::Id::Extended(_) => continue,
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args = NerImdArgs::parse();

    let mut socket = CanSocket::open(&args.socketcan_iface).expect("Failed to open CAN socket!");
    socket
        .set_error_filter_accept_all()
        .expect("Failed to set error mask on CAN socket!");

    match args.command {
        Commands::Control { cmd } => match cmd {
            CtrlCommands::ResetAlarm { id } => {
                write_ctrl_or_set(&mut socket, id, vec![0x01], None).await
            }
            CtrlCommands::TriggerSelfTest { option, id } => {
                write_ctrl_or_set(&mut socket, id, vec![option as u8], None).await
            }
            CtrlCommands::FactoryReset { id } => {
                write_ctrl_or_set(&mut socket, id, vec![0x01], None).await
            }
            CtrlCommands::EarthLiftStatus {
                option,
                id,
                read_back_id,
            } => write_ctrl_or_set(&mut socket, id, vec![option as u8], Some(read_back_id)).await,
        },
        Commands::Set { cmd } => match cmd {
            SetCommands::UnbalanceThreshold {
                threshold,
                id,
                read_back_id,
            } => write_ctrl_or_set(&mut socket, id, vec![threshold], Some(read_back_id)).await,
            SetCommands::IsoAlarmReset {
                opts,
                id,
                read_back_id,
            } => write_ctrl_or_set(&mut socket, id, vec![opts as u8], Some(read_back_id)).await,
            SetCommands::IsoActiveProfile {
                opts,
                id,
                read_back_id,
            } => write_ctrl_or_set(&mut socket, id, vec![opts as u8], Some(read_back_id)).await,
            SetCommands::IsoPowerOnProfile {
                opts,
                id,
                read_back_id,
            } => write_ctrl_or_set(&mut socket, id, vec![opts as u8], Some(read_back_id)).await,
            SetCommands::IsoThresholdError {
                threshold,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(
                    &mut socket,
                    id,
                    threshold.to_le_bytes().to_vec(),
                    Some(read_back_id),
                )
                .await
            }
            SetCommands::IsoThresholdTimeout {
                time,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(
                    &mut socket,
                    id,
                    time.to_le_bytes().to_vec(),
                    Some(read_back_id),
                )
                .await
            }
            SetCommands::IsoThresholdWarning {
                threshold,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(
                    &mut socket,
                    id,
                    threshold.to_le_bytes().to_vec(),
                    Some(read_back_id),
                )
                .await
            }
            SetCommands::SelfTestPeriod {
                time,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(
                    &mut socket,
                    id,
                    time.to_le_bytes().to_vec(),
                    Some(read_back_id),
                )
                .await
            }
            SetCommands::VoltageMode {
                opts,
                id,
                read_back_id,
            } => write_ctrl_or_set(&mut socket, id, vec![opts as u8], Some(read_back_id)).await,
            SetCommands::VoltageThresholdUnderVolts {
                threshold,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(
                    &mut socket,
                    id,
                    threshold.to_le_bytes().to_vec(),
                    Some(read_back_id),
                )
                .await
            }
            SetCommands::StatusLock {
                opts,
                id,
                read_back_id,
            } => write_ctrl_or_set(&mut socket, id, vec![opts as u8], Some(read_back_id)).await,
            SetCommands::IsoThresholdFirstRefEstimate {
                threshold,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(
                    &mut socket,
                    id,
                    threshold.to_le_bytes().to_vec(),
                    Some(read_back_id),
                )
                .await
            }
            SetCommands::IsoPreEstimateMaxDiff {
                threshold,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(
                    &mut socket,
                    id,
                    threshold.to_le_bytes().to_vec(),
                    Some(read_back_id),
                )
                .await
            }
            SetCommands::InterfaceCanId {
                msg,
                new_id,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(&mut socket, id, vec![msg as u8, new_id], Some(read_back_id))
                    .await
            }
            SetCommands::InterfacePeriodicCycleTime {
                msg,
                time,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(&mut socket, id, vec![msg as u8, time], Some(read_back_id)).await
            }
            SetCommands::InterfaceBaudrate { rate, id } => {
                write_ctrl_or_set(&mut socket, id, vec![rate as u8], None).await
            }
            SetCommands::IsoIsoInit {
                value,
                id,
                read_back_id,
            } => {
                write_ctrl_or_set(
                    &mut socket,
                    id,
                    value.to_le_bytes().to_vec(),
                    Some(read_back_id),
                )
                .await
            }
        },
        Commands::Get { cmd } => read_or_readback(&mut socket, cmd as u8).await,
    }
}
