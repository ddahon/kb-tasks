// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};

use gtk::{
    traits::{GtkWindowExt, WidgetExt},
    ApplicationWindow,
};
use tauri::{
    AppHandle, CustomMenuItem, GlobalShortcutManager, GlobalWindowEvent, Manager, Menu, RunEvent,
    Submenu, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTraySubmenu, Window,
};

const TOGGLE_SHORTCUT: &str = "Ctrl+Alt+Space";
const TOGGLE_MENU_ITEM_ID: &str = "toggle";

fn main() {
    tauri::Builder::default()
        .system_tray(build_system_tray())
        .on_window_event(|event| handle_window_events(event))
        .on_system_tray_event(|app, event| handle_tray_events(&app, event))
        .invoke_handler(tauri::generate_handler![toggle_window_js])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, e| match e {
            RunEvent::Ready => register_shortcuts(app_handle),
            _ => (),
        });
}

fn handle_window_events(event: GlobalWindowEvent) {
    match (event.event(), event.window().gtk_window()) {
        (tauri::WindowEvent::Focused(focused), Ok(gtk_window)) => {
            if !focused && !gtk_window.is_visible() {
                hide_window(&event.window().app_handle());
            }
        }
        (_, _) => (),
    }
}

fn handle_tray_events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { .. } => toggle_window(app),
        _ => {}
    }
}

fn build_system_tray() -> SystemTray {
    let menu_item = CustomMenuItem::new(TOGGLE_MENU_ITEM_ID, "Show");
    let system_tray_menu = SystemTrayMenu::new().add_item(menu_item);
    SystemTray::new().with_menu(system_tray_menu)
}

fn register_shortcuts(app_handle: &AppHandle) {
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
            Ok(false) => show_window(&app_handle),
            Ok(true) => hide_window(&app_handle),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn show_window(app_handle: &AppHandle) {
    println!("show");
    if let Some(window) = app_handle.get_window("main") {
        show_window_gtk(&window);
        app_handle
            .tray_handle()
            .get_item(TOGGLE_MENU_ITEM_ID)
            .set_title("Hide")
            .unwrap();
    }
}

fn hide_window(app_handle: &AppHandle) {
    println!("hide");
    if let Some(window) = app_handle.get_window("main") {
        window.hide().unwrap();
        app_handle
            .tray_handle()
            .get_item(TOGGLE_MENU_ITEM_ID)
            .set_title("Show")
            .unwrap()
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

fn gtk_is_visible(window: &Window) -> Result<bool, tauri::Error> {
    match window.gtk_window() {
        Ok(gtk_window) => Ok(gtk_window.is_visible()),
        Err(e) => Err(e),
    }
}
