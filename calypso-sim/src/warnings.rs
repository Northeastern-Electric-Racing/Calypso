use calypso_cangen::CANGEN_SPEC_PATH;
use definition_rs::OdysseyMsg;

/// The CAN spec in the source tree this binary was compiled from.
const BUILD_SPEC_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../Odyssey-Definitions/can-messages/"
);

/// Print a comma-separated list of CAN message topics that have no
/// `sim_freq` in the spec — these are invisible to the mock simulator
/// and can only be published via `--key-map` / `--play` / `--stream`.
///
/// Reads the spec from the source tree the binary was built from, falling back
/// to the current directory; emits nothing if neither has it (e.g. an installed
/// binary in Docker).
pub fn print_unsimulated() {
    let topics = collect_unsimulated_topics();
    if topics.is_empty() {
        return;
    }
    eprintln!("Warning topics (not simulated): {}", topics.join(", "));
}

fn collect_unsimulated_topics() -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) =
        std::fs::read_dir(BUILD_SPEC_PATH).or_else(|_| std::fs::read_dir(CANGEN_SPEC_PATH))
    else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() || path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(msgs): Result<Vec<OdysseyMsg>, _> = serde_json::from_str(&contents) else {
            continue;
        };
        for msg in msgs {
            if let OdysseyMsg::Can(canmsg) = msg
                && canmsg.sim_freq.is_none()
            {
                for field in canmsg.fields {
                    out.push(field.name);
                }
            }
        }
    }
    out.sort();
    out
}
