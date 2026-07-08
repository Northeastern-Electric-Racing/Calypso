use std::collections::HashMap;
use std::io::Write;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures_util::StreamExt;
use rumqttc::v5::AsyncClient;
use tokio_util::sync::CancellationToken;

use crate::keymap::{
    KeyMode, KeyState, claim_keymap_topics, desc_suffix, load_states, publish_injection,
    unit_suffix,
};
use crate::raw_mode::{RawModeGuard, line_end};
use crate::registry::SharedRegistry;

/// Run the interactive raw-mode keypress loop. Claims every keymap topic in
/// the registry so the mock loop (if running) yields ownership.
pub async fn run(
    token: CancellationToken,
    client: AsyncClient,
    key_map_path: &str,
    registry: SharedRegistry,
) -> Result<(), String> {
    let mut states = load_states(key_map_path)?;
    claim_keymap_topics(&states, &registry).await;

    print_listing(&states);
    println!("Press mapped keys to inject. Ctrl+C to exit.");
    println!();

    let guard = RawModeGuard::new().map_err(|e| format!("Failed to enable raw mode: {e}"))?;

    let mut reader = EventStream::new();

    loop {
        tokio::select! {
            () = token.cancelled() => break,
            event = reader.next() => match event {
                Some(Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers,
                    kind: KeyEventKind::Press,
                    ..
                }))) if modifiers.contains(KeyModifiers::CONTROL) => break,
                Some(Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(ch),
                    kind: KeyEventKind::Press,
                    ..
                }))) => {
                    if let Some(state) = states.get_mut(&ch) {
                        publish_injection(ch, state, &client, &registry).await;
                    }
                }
                Some(Err(e)) => {
                    print!("Terminal event error: {e}{}", line_end());
                    let _ = std::io::stdout().flush();
                    break;
                }
                None => break,
                _ => {}
            }
        }
    }

    drop(guard);
    println!();
    println!("Shutting down...");
    Ok(())
}

fn print_listing(states: &HashMap<char, KeyState>) {
    println!("Key Mappings:");
    let mut sorted_keys: Vec<char> = states.keys().copied().collect();
    sorted_keys.sort_unstable();
    for key in &sorted_keys {
        let state = &states[key];
        let unit_s = unit_suffix(&state.unit);
        let desc_s = desc_suffix(state.desc.as_deref());
        match &state.mode {
            KeyMode::Random => {
                println!("  {key} → {} (random){unit_s}{desc_s}", state.topic);
            }
            KeyMode::Pinned { value } => {
                println!("  {key} → {} = {value}{unit_s}{desc_s}", state.topic);
            }
            KeyMode::Increment {
                current,
                step,
                min,
                max,
            } => {
                let bounds = match (min, max) {
                    (Some(lo), Some(hi)) => format!(" in [{lo}, {hi}]"),
                    (Some(lo), None) => format!(" ≥ {lo}"),
                    (None, Some(hi)) => format!(" ≤ {hi}"),
                    (None, None) => String::new(),
                };
                println!(
                    "  {key} → {} starting {current} step {step}{bounds}{unit_s}{desc_s}",
                    state.topic
                );
            }
            KeyMode::Sequence { steps } => {
                println!("  {key} → sequence ({} steps){desc_s}:", steps.len());
                for step in steps {
                    let delay = if step.delay_ms > 0 {
                        format!(" +{}ms", step.delay_ms)
                    } else {
                        String::new()
                    };
                    let step_unit_s = unit_suffix(step.unit.as_deref().unwrap_or(""));
                    println!(
                        "      {topic} = {value}{step_unit_s}{delay}",
                        topic = step.topic,
                        value = step.value
                    );
                }
            }
        }
    }
    println!();
}
