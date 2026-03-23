//! Sakaloka Venus — Tauri desktop entry point.

// Prevents an additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Application entry point.
fn main() {
    sakaloka_venus_lib::run();
}
