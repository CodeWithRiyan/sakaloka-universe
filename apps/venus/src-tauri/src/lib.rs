//! Sakaloka Venus — Tauri desktop application library.

/// Runs the Tauri application.
///
/// # Errors
///
/// Returns an error if Tauri fails to initialise or the event loop exits
/// abnormally.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
