// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, GlobalShortcutManager, Manager, RunEvent};

const TOGGLE_SHORTCUT: &str = "Ctrl+Shift+,";

fn main() {
    tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, e| match e {
            // Application is ready (triggered only once)
            RunEvent::Ready => register_shortcut(app_handle),
            _ => (),
        });
}

fn register_shortcut(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    app_handle
        .global_shortcut_manager()
        .register(TOGGLE_SHORTCUT, move || toggle_window(&app_handle))
        .unwrap();
}

fn toggle_window(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    if let Some(window) = app_handle.get_window("main") {
        match window.is_focused() {
            Ok(false) => window.set_focus().unwrap(),
            Ok(true) => window.hide().unwrap(),
            Err(e) => eprintln!("{}", e),
        }
    }
}
