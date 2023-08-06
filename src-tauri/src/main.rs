// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::RunEvent;

mod global_shortcuts;
mod tray;
mod window;

fn main() {
    tauri::Builder::default()
        .system_tray(tray::build_system_tray())
        .on_window_event(|event| window::handle_window_events(event))
        .on_system_tray_event(|app, event| tray::handle_tray_events(&app, event))
        .invoke_handler(tauri::generate_handler![window::toggle_window_js])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, e| match e {
            RunEvent::Ready => global_shortcuts::register_shortcuts(app_handle),
            _ => (),
        });
}
