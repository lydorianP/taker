#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use taker_lib::db::Database;

fn main() {
    let app_data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.taker.app");
    
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
    
    let db_path = app_data_dir.join("taker.db");
    let db = Database::new(&db_path).expect("Failed to initialize database");
    let db = Arc::new(db);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            taker_lib::commands::notes::create_note,
            taker_lib::commands::notes::get_notes,
            taker_lib::commands::notes::get_note,
            taker_lib::commands::notes::update_note,
            taker_lib::commands::notes::delete_note,
            taker_lib::commands::notes::create_vault,
            taker_lib::commands::notes::get_vaults,
            taker_lib::commands::notes::delete_vault,
            taker_lib::commands::models::get_models,
            taker_lib::commands::models::get_cloud_backends,
            taker_lib::commands::models::add_cloud_backend,
            taker_lib::commands::models::delete_cloud_backend,
            taker_lib::commands::models::search_huggingface,
            taker_lib::commands::models::download_model,
            taker_lib::commands::models::delete_model,
            taker_lib::commands::settings::get_setting,
            taker_lib::commands::settings::set_setting,
            taker_lib::commands::settings::get_all_settings,
            taker_lib::commands::plugins::get_plugins,
            taker_lib::commands::plugins::enable_plugin,
            taker_lib::commands::plugins::disable_plugin,
            taker_lib::commands::plugins::install_plugin,
            taker_lib::commands::plugins::uninstall_plugin,
            taker_lib::commands::plugins::get_marketplace_plugins,
            taker_lib::commands::ai::summarize_note,
            taker_lib::commands::ai::generate_flashcards,
            taker_lib::commands::ai::generate_slideshow,
            taker_lib::commands::ai::get_flashcards,
            taker_lib::commands::ai::get_slideshows,
            taker_lib::commands::audio::text_to_speech,
            taker_lib::commands::audio::transcribe_audio,
            taker_lib::commands::audio::generate_podcast,
            taker_lib::commands::audio::get_audio_files,
            taker_lib::commands::io::export_note,
            taker_lib::commands::io::import_note,
            taker_lib::commands::io::export_vault,
        ])
        .run(tauri::generate_context!())
        .expect("error while running taker application");
}
