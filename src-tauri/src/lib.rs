// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod files;
mod shizuku;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(shizuku::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_haptics::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            files::list_folder,
            files::toggle_mod,
            files::trash_folder,
            files::paste_to_dir,
            shizuku::shizuku_status,
            shizuku::shizuku_request_permission,
            shizuku::shizuku_list_folder,
            shizuku::shizuku_make_dirs,
            shizuku::shizuku_rename,
            shizuku::shizuku_delete,
            shizuku::shizuku_open_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
