use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "calypso-sim",
    version,
    about = "MQTT simulation tool for Calypso (autonomous, interactive, scripted, or streamed)"
)]
pub struct Cli {
    /// MQTT broker host:port
    #[arg(
        short = 'u',
        long,
        env = "CALYPSO_SIREN_HOST_URL",
        default_value = "127.0.0.1:1883"
    )]
    pub siren_host_url: String,

    /// List all simulatable topics and exit
    #[arg(long)]
    pub list_topics: bool,

    /// Run the autonomous heartbeat simulator. Default ON when no other mode
    /// is chosen; explicit when paired with --key-map / --script / --stream.
    #[arg(long)]
    pub auto: bool,

    /// Disable the autonomous heartbeat for topics matching these regex
    /// patterns (blacklist mode)
    #[arg(long = "disable-topic", conflicts_with = "enable_topic")]
    pub disable_topic: Vec<String>,

    /// Run the autonomous heartbeat ONLY for topics matching these regex
    /// patterns (whitelist mode)
    #[arg(long = "enable-topic", conflicts_with = "disable_topic")]
    pub enable_topic: Vec<String>,

    /// Path to a JSON keymap file; runs the interactive raw-mode keypress
    /// injector. Press Ctrl+C to exit.
    #[arg(short = 'k', long, value_name = "FILE")]
    pub key_map: Option<String>,

    /// Run a scripted sequence of keymap keys then exit. Each line is either
    /// a single character (fires that key) or `sleep <ms>` (waits). Blank
    /// lines and lines starting with `#` are ignored. Requires `--key-map`.
    #[arg(long, value_name = "FILE", requires = "key_map")]
    pub script: Option<String>,

    /// Accept JSON-RPC 2.0 commands on stdin (one per line); replies on
    /// stdout. Tracing/diagnostics go to stderr. `--auto` defaults OFF in
    /// this mode unless explicitly set.
    #[arg(long)]
    pub stream: bool,
}

impl Cli {
    /// Whether the autonomous heartbeat should run.
    /// True if `--auto` was set, OR no other input mode was chosen and
    /// `--list-topics` wasn't requested.
    pub fn run_autonomous(&self) -> bool {
        if self.auto {
            return true;
        }
        let any_other_mode = self.stream || self.key_map.is_some() || self.script.is_some();
        !any_other_mode && !self.list_topics
    }

    /// Validate mutually-exclusive mode flags.
    pub fn validate(&self) -> Result<(), String> {
        let mut chosen = 0;
        if self.stream {
            chosen += 1;
        }
        if self.key_map.is_some() {
            chosen += 1;
        }
        if chosen > 1 {
            return Err("--stream and --key-map are mutually exclusive".into());
        }
        Ok(())
    }
}
