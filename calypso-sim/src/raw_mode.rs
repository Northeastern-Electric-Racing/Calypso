use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

/// In raw mode the tty driver does not translate `\n` to `\r\n`, so we must
/// emit `\r\n` ourselves. In cooked mode (script / mock / stream) a
/// literal `\r` renders as staircase output / `^M`, so we must emit `\n`.
static RAW_MODE_ACTIVE: AtomicBool = AtomicBool::new(false);

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
