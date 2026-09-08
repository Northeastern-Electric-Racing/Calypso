//! Live filter control on stdin, for plain `--mock` runs.
//!
//! Only wired up when no foreground mode owns the terminal: `--stream` parses
//! stdin as JSON-RPC (it exposes `set_filter` instead) and `--key-map` puts the
//! terminal in raw mode for keypresses.
//!
//! Commands, one per line:
//! ```text
//! disable <regex>...                        publish everything EXCEPT these
//! enable  <regex>...                        publish ONLY these
//! clear                                     remove the filter (all topics)
//! add <topic> <unit> <freq_ms> <min> <max>  start simulating a new topic
//! remove <topic>                            stop simulating a topic
//! status                                    print the current filter
//! help                                      print this list
//! ```
//!
//! `add` covers the continuous-range case only; a weighted option set is too
//! unwieldy for a command line, so `--stream`'s `add_topic` handles that.
//! `enable` and `disable` replace the filter rather than accumulating, so each
//! command fully describes the resulting state — the same mutual exclusion the
//! `--enable-topic` / `--disable-topic` flags have.

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::sync::CancellationToken;

use crate::filter::{FilterMode, FilterTx};
use crate::runtime_topic::{self, SimCommand, SimCommandTx, TopicSpec, ValueSpec};

const HELP: &str = "commands: disable <regex>... | enable <regex>... | clear | \
                    add <topic> <unit> <freq_ms> <min> <max> | remove <topic> | status | help";

/// Read filter commands from stdin until EOF, Ctrl+C, or cancellation.
///
/// Returns on EOF *without* ending the run: a sim started with no TTY (Docker,
/// `< /dev/null`) sees stdin close immediately, and that must not shut the
/// heartbeat down. In that case we fall back to waiting for Ctrl+C.
pub async fn run(
    token: CancellationToken,
    filter_tx: FilterTx,
    cmd_tx: SimCommandTx,
) -> Result<(), String> {
    println!("Mock heartbeat running. {HELP}");

    let mut lines = BufReader::new(tokio::io::stdin()).lines();

    loop {
        tokio::select! {
            () = token.cancelled() => return Ok(()),
            line = lines.next_line() => match line {
                Ok(Some(line)) => handle(line.trim(), &filter_tx, &cmd_tx).await,
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

async fn handle(line: &str, filter_tx: &FilterTx, cmd_tx: &SimCommandTx) {
    if line.is_empty() {
        return;
    }
    let mut parts = line.split_whitespace();
    let Some(command) = parts.next() else {
        return;
    };
    let args: Vec<String> = parts.map(str::to_string).collect();

    let next = match command {
        "clear" | "disable" | "enable" => match FilterMode::from_command(command, &args) {
            Ok(filter) => Some(filter),
            Err(e) => {
                println!("error: {e}. {HELP}");
                None
            }
        },
        "status" => {
            println!("filter: {}", filter_tx.borrow().describe());
            None
        }
        "help" => {
            println!("{HELP}");
            None
        }
        "add" => {
            add_topic(&args, cmd_tx).await;
            None
        }
        "remove" => {
            remove_topic(&args, cmd_tx).await;
            None
        }
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

/// `add <topic> <unit> <freq_ms> <min> <max>` — the flat range case. Increments
/// are derived from the span rather than asked for: a five-argument command is
/// already at the limit of what is comfortable to type.
async fn add_topic(args: &[String], cmd_tx: &SimCommandTx) {
    let [topic, unit, freq, min, max] = args else {
        println!("error: usage: add <topic> <unit> <freq_ms> <min> <max>");
        return;
    };
    let (Ok(sim_freq), Ok(min), Ok(max)) = (freq.parse(), min.parse::<f32>(), max.parse::<f32>())
    else {
        println!("error: freq_ms, min and max must all be numbers");
        return;
    };

    let spec = TopicSpec {
        topic: topic.clone(),
        unit: unit.clone(),
        sim_freq,
        sim: ValueSpec::Range {
            min,
            max,
            inc_min: 0.0,
            // Step up to a twentieth of the span per tick, so the value walks
            // the range rather than jumping across it.
            inc_max: (max - min) / 20.0,
            round: false,
        },
    };
    let component = match runtime_topic::build(spec) {
        Ok(c) => c,
        Err(e) => {
            println!("error: {e}");
            return;
        }
    };

    let added = runtime_topic::request(cmd_tx, |reply| SimCommand::Add {
        component: Box::new(component),
        reply,
    })
    .await;
    match added {
        Err(e) | Ok(Err(e)) => println!("error: {e}"),
        Ok(Ok(())) => println!("added {topic}"),
    }
}

/// `remove <topic>` — reports the count, which can exceed one: the in-topic
/// placeholder names repeat in the generated set.
async fn remove_topic(args: &[String], cmd_tx: &SimCommandTx) {
    let [topic] = args else {
        println!("error: usage: remove <topic>");
        return;
    };

    let removed = runtime_topic::request(cmd_tx, |reply| SimCommand::Remove {
        name: topic.clone(),
        reply,
    })
    .await;
    match removed {
        Ok(0) => println!("{topic} was not simulated"),
        Ok(n) => println!("removed {topic} ({n} component(s))"),
        Err(e) => println!("error: {e}"),
    }
}
