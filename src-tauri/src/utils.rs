#[tauri::command]
pub fn print_rust(msg: String) {
    println!("{}", msg)
}
