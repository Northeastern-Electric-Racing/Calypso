//! Unit tests for calypso-sim, gathered here (compiled only under `cfg(test)`
//! via `mod tests;` in `main.rs`) so they run under `cargo test` with full
//! access to crate internals. Integration tests that drive the real binary
//! live in the crate-root `tests/` directory instead.
//!
//! Kept deliberately small: each test guards a piece of logic that a future
//! refactor could silently break, not code that is obvious by inspection.

mod cli;
mod keymap;
