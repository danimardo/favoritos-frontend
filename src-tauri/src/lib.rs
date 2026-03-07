// Favoritos Desktop - Main Library
mod api;
mod cache;
mod commands;
mod config;
mod notifications;
mod tray;

use commands::AppState;
use std::sync::Mutex;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("favoritos=info".parse().unwrap()))
        .init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();
    info!("Starting Favoritos Desktop application");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            info!("Setting up application...");

            // Initialize favicon cache
            let cache_dir = app
                .path()
                .app_cache_dir()
                .unwrap_or_else(|_| std::env::temp_dir().join("favoritos-cache"));

            let favicon_cache = cache::FaviconCache::new(cache_dir);

            app.manage(AppState {
                favicon_cache: Mutex::new(Some(favicon_cache)),
            });

            // Setup system tray
            tray::setup_tray(app.handle())?;

            info!("Application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Config commands
            commands::config_get,
            commands::config_save,
            commands::config_validate,
            commands::config_clear,
            // Favorites commands
            commands::favorites_get,
            commands::favorites_get_by_id,
            commands::favorites_create,
            commands::favorites_update,
            commands::favorites_delete,
            commands::metadata_fetch,
            commands::favorites_visit,
            commands::favorites_open_url,
            // Folder commands
            commands::folders_get,
            commands::folders_create,
            // Tag commands
            commands::tags_get,
            commands::tags_create,
            // Cache commands
            commands::cache_favicon_get,
            commands::cache_favicon_set,
            // Window commands
            commands::window_hide_main,
            commands::window_show_search,
            // Notification commands
            commands::app_show_notification,
            commands::app_quit,
            // Import commands
            commands::import_favorites,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
