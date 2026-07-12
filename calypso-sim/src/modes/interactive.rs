use std::io::Write;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures_util::StreamExt;
use rumqttc::v5::AsyncClient;
use tokio_util::sync::CancellationToken;

use crate::keymap::{claim_topics, key_bindings, load_scenario, print_listing, run_action};
use crate::raw_mode::{RawModeGuard, line_end};
use crate::registry::SharedRegistry;

/// Run the interactive raw-mode keypress loop. Each key-bound action fires on
/// its key; the action's topics are claimed in the registry so the mock loop
/// (if running) yields ownership.
pub async fn run(
    token: CancellationToken,
    client: AsyncClient,
    scenario_path: &str,
    registry: SharedRegistry,
) -> Result<(), String> {
    let scenario = load_scenario(scenario_path)?;
    let keys = key_bindings(&scenario)?;
    if keys.is_empty() {
        return Err(
            "Scenario has no key-bound actions; add a `key` to an action, or use --play".into(),
        );
    }
    claim_topics(&scenario, keys.values().map(String::as_str), &registry).await;

    print_listing(&scenario, &keys);
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
                    if let Some(name) = keys.get(&ch) {
                        run_action(&scenario, name, &client, &registry).await;
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
