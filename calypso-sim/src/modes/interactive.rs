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
            event = reader.next() => match classify(event) {
                Input::Quit => break,
                Input::Error(e) => {
                    print!("Terminal event error: {e}{}", line_end());
                    let _ = std::io::stdout().flush();
                    break;
                }
                Input::Key(ch) => {
                    if let Some(name) = keys.get(&ch) {
                        run_action(&scenario, name, &client).await;
                    }
                }
                Input::Ignore => {}
            }
        }
    }

    drop(guard);
    println!();
    println!("Shutting down...");
    Ok(())
}

/// What the interactive loop should do with one terminal event.
enum Input {
    /// A printable key was pressed — run its bound action, if any.
    Key(char),
    /// Ctrl+C or end-of-stream: leave the loop.
    Quit,
    /// The event stream errored; report and leave.
    Error(String),
    /// Anything else (key release, resize, non-char key): ignore.
    Ignore,
}

/// Classify one [`EventStream`] item into the loop's vocabulary, hiding the
/// crossterm `KeyEvent` destructuring.
fn classify(event: Option<std::io::Result<Event>>) -> Input {
    match event {
        None => Input::Quit,
        Some(Err(e)) => Input::Error(e.to_string()),
        Some(Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(ch),
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }))) => {
            if ch == 'c' && modifiers.contains(KeyModifiers::CONTROL) {
                Input::Quit
            } else {
                Input::Key(ch)
            }
        }
        _ => Input::Ignore,
    }
}
