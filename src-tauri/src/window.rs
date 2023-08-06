use std::time::{SystemTime, UNIX_EPOCH};

use gtk::traits::{GtkWindowExt, WidgetExt};
use tauri::{AppHandle, GlobalWindowEvent, Manager, Window};

use crate::tray::TOGGLE_MENU_ITEM_ID;

#[tauri::command]
pub async fn toggle_window_js(app_handle: AppHandle) {
    toggle_window(&app_handle)
}

pub fn toggle_window(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    if let Some(window) = app_handle.get_window("main") {
        match window.is_focused() {
            Ok(false) => show_window(&app_handle),
            Ok(true) => hide_window(&app_handle),
            Err(e) => eprintln!("{}", e),
        }
    }
}

pub fn show_window(app_handle: &AppHandle) {
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

pub fn hide_window(app_handle: &AppHandle) {
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

pub fn show_window_gtk(window: &Window) {
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

pub fn handle_window_events(event: GlobalWindowEvent) {
    match (event.event(), event.window().gtk_window()) {
        (tauri::WindowEvent::Focused(focused), Ok(gtk_window)) => {
            if !focused && !gtk_window.is_visible() {
                hide_window(&event.window().app_handle());
            }
        }
        (_, _) => (),
    }
}
