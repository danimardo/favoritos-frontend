// Favoritos Desktop - Main Library
mod api;
mod cache;
mod commands;
mod config;
mod notifications;
mod tray;

use commands::AppState;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, ShortcutState};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

/// Alterna la visibilidad de la ventana de búsqueda global.
fn toggle_search_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("search") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

/// Indica si los argumentos de línea de comandos piden abrir la búsqueda global.
fn wants_search(args: &[String]) -> bool {
    args.iter().any(|a| a == "--search")
}

fn setup_logging() {
    let log_level_str = config::ConfigManager::get_initial_log_level();

    let filter = format!("favoritos_app_lib={}", log_level_str);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(filter.parse().unwrap()))
        .init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();
    info!("Starting Favoritos Desktop application");

    tauri::Builder::default()
        // Una sola instancia: una segunda ejecución (p. ej. `--search` lanzado desde un
        // atajo del sistema en Wayland) se redirige a la instancia ya abierta.
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if wants_search(&args) {
                toggle_search_window(app);
            } else if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        if shortcut.matches(Modifiers::ALT, Code::Space)
                            || shortcut.matches(Modifiers::CONTROL | Modifiers::ALT, Code::Space)
                        {
                            toggle_search_window(app);
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            info!("Setting up application...");

            // Register global shortcut con fallback a Ctrl+Alt+Space. En Wayland (y en algunos
            // entornos Linux) el registro puede fallar: no es fatal, se usa
            // `favoritos-frontend --search` desde un atajo del sistema.
            if let Err(e) = app.global_shortcut().register("Alt+Space") {
                warn!("No se pudo registrar Alt+Space (¿en uso?): {e}");
                match app.global_shortcut().register("Ctrl+Alt+Space") {
                    Ok(()) => info!("Atajo de respaldo registrado: Ctrl+Alt+Space"),
                    Err(e2) => warn!("No se pudo registrar Ctrl+Alt+Space: {e2}"),
                }
            }

            // Initialize favicon cache
            let cache_dir = app
                .path()
                .app_cache_dir()
                .unwrap_or_else(|_| std::env::temp_dir().join("favoritos-cache"));

            let favicon_cache = cache::FaviconCache::new(cache_dir);

            app.manage(AppState {
                favicon_cache: tokio::sync::Mutex::new(Some(favicon_cache)),
            });

            // Setup system tray
            // En Linux puede no haber bandeja (p. ej. GNOME sin extensión): no es fatal.
            if let Err(e) = tray::setup_tray(app.handle()) {
                warn!("No se pudo crear el icono de bandeja: {e}");
            }

            // Ensure search window is hidden on start
            if let Some(search_window) = app.get_webview_window("search") {
                let _ = search_window.hide();
            }

            // Arranque con `--search`: abrir la búsqueda global directamente
            let args: Vec<String> = std::env::args().collect();
            if wants_search(&args) {
                toggle_search_window(app.handle());
            }

            info!("Application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Config commands
            commands::config_get,
            commands::config_save,
            commands::config_update_settings,
            commands::config_validate,
            commands::config_clear,
            // UI Preferences commands
            commands::preference_get,
            commands::preference_set,
            // Auth & User commands
            commands::auth_login,
            commands::auth_register,
            commands::user_get_profile,
            // Favorites commands
            commands::favorites_get,
            commands::favorites_get_by_id,
            commands::favorites_create,
            commands::favorites_update,
            commands::favorites_delete,
            commands::metadata_fetch,
            commands::metadata_refresh,
            commands::favorites_visit,
            commands::favorites_open_url,
            commands::favorites_export,
            commands::favorites_export_html,
            // Folder commands
            commands::folders_get,
            commands::folders_create,
            // Tag commands
            commands::tags_get,
            commands::tags_create,
            commands::tags_delete,
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
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // For the main window, we want to hide it instead of closing
                if window.label() == "main" {
                    window.hide().unwrap();
                    api.prevent_close();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
