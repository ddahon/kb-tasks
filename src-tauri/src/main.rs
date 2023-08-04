// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};

use gtk::traits::GtkWindowExt;
use tauri::{
    AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, Menu, RunEvent, Submenu, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTraySubmenu, Window,
};

const TOGGLE_SHORTCUT: &str = "Ctrl+Shift+Space";

fn main() {
    tauri::Builder::default()
        .system_tray(build_system_tray())
        .on_system_tray_event(|app, event| register_tray_events(&app, event))
        .invoke_handler(tauri::generate_handler![toggle_window_js])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, e| match e {
            // Application is ready (triggered only once)
            RunEvent::Ready => register_shortcut(app_handle),
            _ => (),
        });
}

fn register_tray_events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { .. } => toggle_window(app),
        _ => {}
    }
}

fn build_system_tray() -> SystemTray {
    let menu_item = CustomMenuItem::new("0", "show");
    let system_tray_menu = SystemTrayMenu::new().add_item(menu_item);
    SystemTray::new().with_menu(system_tray_menu)
}

fn register_shortcut(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    app_handle
        .global_shortcut_manager()
        .register(TOGGLE_SHORTCUT, move || toggle_window(&app_handle))
        .unwrap();
}

#[tauri::command]
async fn toggle_window_js(app_handle: AppHandle) {
    toggle_window(&app_handle)
}

fn toggle_window(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    if let Some(window) = app_handle.get_window("main") {
        match window.is_focused() {
            Ok(false) => {
                show_window_gtk(&window);
                //window.show().unwrap();
                //window.set_focus().unwrap();
            }
            Ok(true) => {
                window.hide().unwrap();
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn show_window_gtk(window: &Window) {
    match window.gtk_window() {
        Ok(gtk_window) => {
            let timestamp: u32 = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                .try_into()
                .unwrap();
            gtk_window.present();
            gtk_window.present_with_time(timestamp);
        }
        Err(_) => (),
    }
}
