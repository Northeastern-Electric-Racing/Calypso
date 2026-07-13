use std::io::Write;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures_util::StreamExt;
use rumqttc::v5::AsyncClient;
use tokio_util::sync::CancellationToken;

use crate::keymap::{Scenario, key_bindings, print_listing, run_action};
use crate::raw_mode::{RawModeGuard, line_end};

/// Run the interactive raw-mode keypress loop. Each key-bound action fires on
/// its key. The heartbeat (if running) has already ceded this scenario's topics
/// up front, so keypresses never fight it (see [`crate::ownership`]).
pub async fn run(
    token: CancellationToken,
    client: AsyncClient,
    scenario: Scenario,
) -> Result<(), String> {
    let keys = key_bindings(&scenario)?;
    if keys.is_empty() {
        return Err(
            "Scenario has no key-bound actions; add a `key` to an action, or use --play".into(),
        );
    }
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
                        run_action(&scenario, name, &client).await;
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
