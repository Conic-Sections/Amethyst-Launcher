#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_user_config])
        .run(tauri::generate_context!())
        .expect("error while running magical launcher!");
}

#[tauri::command]
fn get_user_config() -> String {
    return "".to_string();
    if !fs::metadata("/home/brokendeer/桌面/mc2/magical-launcher/test/test.json").is_ok() {
        fs::File::create("/home/brokendeer/桌面/mc2/magical-launcher/test/test.json").unwrap();
        return "".to_string();
    }
    fs::read_to_string("/home/brokendeer/桌面/mc2/magical-launcher/test/test.json").unwrap()
}
