use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool is an independent CLI flag, not state that wants a mode enum; \
              clap owns the parsing and `run_mock` owns the arbitration between them"
)]
#[command(
    name = "calypso-sim",
    version,
    about = "CAN simulation tool for Calypso over MQTT or Zenoh (mock, interactive, replay, or streamed)"
)]
pub struct Cli {
    /// MQTT broker host:port. Ignored when `--zenoh` is set.
    #[arg(
        short = 'u',
        long,
        env = "CALYPSO_SIREN_HOST_URL",
        default_value = "localhost:1883"
    )]
    pub siren_host_url: String,

    /// Publish over Zenoh instead of MQTT -- will eventually become default
    #[arg(short = 'z', long, env = "CALYPSO_ZENOH")]
    pub zenoh: bool,

    /// Zenoh conf file (JSON5). Requires `--zenoh`; when omitted the Zenoh
    /// default config is used, so the sim needs no conf file to run.
    #[arg(
        long,
        env = "CALYPSO_ZENOH_CONF",
        value_name = "FILE",
        requires = "zenoh"
    )]
    pub zenoh_conf: Option<PathBuf>,

    /// List all simulatable topics and exit
    #[arg(long)]
    pub list_topics: bool,

    /// Run the mock heartbeat simulator. Default ON when no other mode
    /// is chosen; explicit when paired with --key-map or --stream.
    #[arg(long)]
    pub mock: bool,

    /// Disable the mock heartbeat for topics matching these regex
    /// patterns (blacklist mode)
    #[arg(long = "disable-topic", conflicts_with = "enable_topic")]
    pub disable_topic: Vec<String>,

    /// Run the mock heartbeat ONLY for topics matching these regex
    /// patterns (whitelist mode)
    #[arg(long = "enable-topic", conflicts_with = "disable_topic")]
    pub enable_topic: Vec<String>,

    /// Path to a JSON scenario file; runs the interactive raw-mode keypress
    /// injector, firing each action on its bound `key`. Press Ctrl+C to exit.
    #[arg(short = 'k', long, value_name = "FILE")]
    pub key_map: Option<String>,

    /// Run one named action from the `--key-map` scenario file, then exit
    /// (deterministic replay). The action's steps run in order, including
    /// nested invokes and `sleep_ms` waits. Requires `--key-map`.
    #[arg(long, value_name = "ACTION", requires = "key_map")]
    pub play: Option<String>,

    /// Accept JSON-RPC 2.0 commands on stdin (one per line); replies on
    /// stdout. Tracing/diagnostics go to stderr. `--mock` defaults OFF in
    /// this mode unless explicitly set.
    #[arg(long, conflicts_with = "key_map")]
    pub stream: bool,
}

impl Cli {
    /// Whether the mock heartbeat should run.
    pub fn run_mock(&self) -> bool {
        let any_other_mode = self.stream || self.key_map.is_some() || self.play.is_some();
        self.mock || (!any_other_mode && !self.list_topics)
    }
}
