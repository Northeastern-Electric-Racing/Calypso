//! Live filter control on stdin, for plain `--mock` runs.
//!
//! Only wired up when no foreground mode owns the terminal: `--stream` parses
//! stdin as JSON-RPC (it exposes `set_filter` instead) and `--key-map` puts the
//! terminal in raw mode for keypresses.
//!
//! Commands, one per line:
//! ```text
//! disable <regex>...   heartbeat publishes everything EXCEPT these
//! enable  <regex>...   heartbeat publishes ONLY these
//! clear                remove the filter (all topics)
//! status               print the current filter
//! help                 print this list
//! ```
//! `enable` and `disable` replace the filter rather than accumulating, so each
//! command fully describes the resulting state — the same mutual exclusion the
//! `--enable-topic` / `--disable-topic` flags have.

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::sync::CancellationToken;

use crate::filter::{FilterMode, FilterTx};

const HELP: &str = "commands: disable <regex>... | enable <regex>... | clear | status | help";

/// Read filter commands from stdin until EOF, Ctrl+C, or cancellation.
///
/// Returns on EOF *without* ending the run: a sim started with no TTY (Docker,
/// `< /dev/null`) sees stdin close immediately, and that must not shut the
/// heartbeat down. In that case we fall back to waiting for Ctrl+C.
pub async fn run(token: CancellationToken, filter_tx: FilterTx) -> Result<(), String> {
    println!("Mock heartbeat running. {HELP}");

    let mut lines = BufReader::new(tokio::io::stdin()).lines();

    loop {
        tokio::select! {
            () = token.cancelled() => return Ok(()),
            line = lines.next_line() => match line {
                Ok(Some(line)) => handle(line.trim(), &filter_tx),
                // stdin closed: nothing more to read, but the sim keeps running.
                Ok(None) => break,
                Err(e) => {
                    eprintln!("control: stdin read error: {e}");
                    break;
                }
            }
        }
    }

    tokio::select! {
        () = token.cancelled() => Ok(()),
        result = tokio::signal::ctrl_c() => {
            result.map_err(|e| format!("ctrl+c handler failed: {e}"))
        }
    }
}

fn handle(line: &str, filter_tx: &FilterTx) {
    if line.is_empty() {
        return;
    }
    let mut parts = line.split_whitespace();
    let Some(command) = parts.next() else {
        return;
    };
    let args: Vec<String> = parts.map(str::to_string).collect();

    let next = match command {
        "clear" => Some(FilterMode::Disabled),
        "status" => {
            println!("filter: {}", filter_tx.borrow().describe());
            None
        }
        "help" => {
            println!("{HELP}");
            None
        }
        "disable" | "enable" if args.is_empty() => {
            println!("error: `{command}` needs at least one regex. {HELP}");
            None
        }
        "disable" => build(&[], &args),
        "enable" => build(&args, &[]),
        other => {
            println!("error: unknown command '{other}'. {HELP}");
            None
        }
    };

    if let Some(filter) = next {
        // The receiver is the mock task, which outlives this loop; a send error
        // only happens if it has already shut down, in which case there is
        // nothing to report to.
        let _ = filter_tx.send(filter);
    }
}

/// Compile a new filter, reporting a bad regex instead of applying it.
fn build(enable: &[String], disable: &[String]) -> Option<FilterMode> {
    match FilterMode::build(enable, disable) {
        Ok(filter) => Some(filter),
        Err(e) => {
            println!("error: {e}");
            None
        }
    }
}
