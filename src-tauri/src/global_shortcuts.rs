use tauri::{AppHandle, GlobalShortcutManager};

const TOGGLE_SHORTCUT: &str = "Ctrl+Alt+Space";

pub fn register_shortcuts(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    app_handle
        .global_shortcut_manager()
        .register(TOGGLE_SHORTCUT, move || {
            crate::window::toggle_window(&app_handle)
        })
        .unwrap();
}
