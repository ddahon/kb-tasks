use tauri::{AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

use crate::window::toggle_window;

pub const TOGGLE_MENU_ITEM_ID: &str = "toggle";

pub fn build_system_tray() -> SystemTray {
    let menu_item = CustomMenuItem::new(TOGGLE_MENU_ITEM_ID, "Show");
    let system_tray_menu = SystemTrayMenu::new().add_item(menu_item);
    SystemTray::new().with_menu(system_tray_menu)
}

pub fn handle_tray_events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { .. } => toggle_window(app),
        _ => {}
    }
}
