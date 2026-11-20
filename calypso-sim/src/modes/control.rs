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

use crate::filter::{FilterMode, FilterTx};
use crate::runtime_topic::{self, SimCommandTx, TopicSpec, ValueSpec};

const HELP: &str = "commands: disable <regex>... | enable <regex>... | clear | \
                    add <topic> <unit> <freq_ms> <min> <max> | remove <topic> | status | help";

/// Read commands from stdin, then wait for Ctrl+C; returns on Ctrl+C.
///
/// stdin closing does *not* end the run: a sim started with no TTY (Docker,
/// `< /dev/null`) sees EOF immediately, and that must not shut the heartbeat
/// down. Ctrl+C is not watched while reading stdin; it takes the default
/// signal action and ends the process.
pub async fn run(filter_tx: FilterTx, cmd_tx: SimCommandTx) -> Result<(), String> {
    println!("Mock heartbeat running. {HELP}");

    let mut lines = BufReader::new(tokio::io::stdin()).lines();

    loop {
        match lines.next_line().await {
            Ok(Some(line)) => handle(line.trim(), &filter_tx, &cmd_tx).await,
            // stdin closed: nothing more to read, but the sim keeps running.
            Ok(None) => break,
            Err(e) => {
                eprintln!("control: stdin read error: {e}");
                break;
            }
        }
    }

    tokio::signal::ctrl_c()
        .await
        .map_err(|e| format!("ctrl+c handler failed: {e}"))
}

async fn handle(line: &str, filter_tx: &FilterTx, cmd_tx: &SimCommandTx) {
    let mut parts = line.split_whitespace();
    let Some(command) = parts.next() else {
        return;
    };
    let args: Vec<String> = parts.map(str::to_string).collect();

    match command {
        "clear" | "disable" | "enable" => match FilterMode::from_command(command, &args) {
            // The receiver is the mock task, which outlives this loop; a send
            // error only happens if it has already shut down, in which case
            // there is nothing to report to.
            Ok(filter) => {
                let _ = filter_tx.send(filter);
            }
            Err(e) => println!("error: {e}"),
        },
        "status" => println!("filter: {}", filter_tx.borrow().describe()),
        "help" => println!("{HELP}"),
        "add" => add_topic(&args, cmd_tx).await,
        "remove" => remove_topic(&args, cmd_tx).await,
        other => println!("error: unknown command '{other}'. {HELP}"),
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
    match runtime_topic::add_topic(cmd_tx, spec).await {
        Ok(name) => println!("added {name}"),
        Err(e) => println!("error: {e}"),
    }
}

/// `remove <topic>` — reports the count, which can exceed one: the in-topic
/// placeholder names repeat in the generated set.
async fn remove_topic(args: &[String], cmd_tx: &SimCommandTx) {
    let [topic] = args else {
        println!("error: usage: remove <topic>");
        return;
    };
    match runtime_topic::remove_topic(cmd_tx, topic.clone()).await {
        Ok(0) => println!("{topic} was not simulated"),
        Ok(n) => println!("removed {topic} ({n} component(s))"),
        Err(e) => println!("error: {e}"),
    }
}
