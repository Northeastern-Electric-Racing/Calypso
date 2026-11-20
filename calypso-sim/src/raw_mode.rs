use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

/// Whether a [`RawModeGuard`] is live.
static RAW_MODE_ACTIVE: AtomicBool = AtomicBool::new(false);

/// The line ending for terminal output: in raw mode the tty does not translate
/// `\n` to `\r\n`, so emit `\r\n` ourselves; otherwise a literal `\r` shows
/// up as `^M`, so emit `\n`.
pub fn line_end() -> &'static str {
    if RAW_MODE_ACTIVE.load(Ordering::Relaxed) {
        "\r\n"
    } else {
        "\n"
    }
}

/// RAII guard that enables raw mode on creation and restores on drop.
pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        RAW_MODE_ACTIVE.store(true, Ordering::Relaxed);
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        RAW_MODE_ACTIVE.store(false, Ordering::Relaxed);
        let _ = io::stdout().flush();
    }
}
