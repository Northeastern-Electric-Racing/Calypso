pub mod auto_script;
pub mod interactive;
pub mod mock;
pub mod stream;

use std::time::Duration;

use rumqttc::v5::EventLoop;
use tokio_util::sync::CancellationToken;

/// Background task that drives the MQTT eventloop. Required for any publish
/// to actually go through. rumqttc's `poll()` calls `clean()` internally on
/// error and reconnects on the next call, so we just keep looping.
pub async fn poll_eventloop(token: CancellationToken, mut eventloop: EventLoop) {
    loop {
        tokio::select! {
            () = token.cancelled() => break,
            result = eventloop.poll() => {
                if let Err(e) = result {
                    tracing::error!("MQTT eventloop error: {e}");
                    // Avoid tight-looping if poll() returns immediately on a
                    // local error; cooperate with cancellation during backoff.
                    tokio::select! {
                        () = token.cancelled() => break,
                        () = tokio::time::sleep(Duration::from_millis(500)) => {}
                    }
                }
            }
        }
    }
}
