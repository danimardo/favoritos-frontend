// Tauri Commands - IPC interface between frontend and backend
use crate::api::{ApiClient, ApiError};
use crate::cache::FaviconCache;
use crate::config::{ApiConfig, ConfigManager};
use crate::notifications;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_opener::OpenerExt;

pub struct AppState {
    pub favicon_cache: tokio::sync::Mutex<Option<FaviconCache>>,
}

// ==================== Config Commands ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub configured: bool,
    pub api_url: Option<String>,
    pub log_level: String,
}

#[tauri::command]
pub fn config_get(app: AppHandle) -> Result<ConfigResponse, String> {
    let config = ConfigManager::get(&app);
    Ok(ConfigResponse {
        configured: config.is_some(),
        api_url: config.as_ref().map(|c| c.api_url.clone()),
        log_level: config
            .map(|c| c.log_level)
            .unwrap_or_else(|| "info".to_string()),
    })
}

#[tauri::command]
pub fn config_save(
    app: AppHandle,
    api_url: String,
    api_key: String,
    log_level: String,
) -> Result<(), String> {
    let config = ApiConfig::new(api_url, api_key, log_level);
    ConfigManager::save(&app, &config)
}

#[tauri::command]
pub fn config_update_settings(
    app: AppHandle,
    api_url: String,
    api_key: Option<String>,
    log_level: String,
) -> Result<(), String> {
    let config = match ConfigManager::get(&app) {
        Some(mut existing) => {
            existing.api_url = api_url;
            if let Some(key) = api_key {
                existing.api_key = key;
            }
            existing.log_level = log_level;
            existing
        }
        None => ApiConfig::new(api_url, api_key.unwrap_or_default(), log_level),
    };

    ConfigManager::save(&app, &config)
}

#[tauri::command]
pub async fn config_validate(api_url: String, api_key: String) -> Result<bool, String> {
    let client = ApiClient::new(api_url, api_key);
    match client.validate_credentials().await {
        Ok(_) => Ok(true),
        Err(ApiError::Message(msg)) => Err(msg),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn config_clear(app: AppHandle) -> Result<(), String> {
    ConfigManager::clear(&app)
}

// ==================== Auth & User Commands ====================

#[tauri::command]
pub async fn auth_login(
    api_url: String,
    email: String,
    password: String,
) -> Result<serde_json::Value, String> {
    let client = ApiClient::new(api_url, "".to_string());
    client
        .login(&email, &password)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn auth_register(
    api_url: String,
    email: String,
    password: String,
    name: Option<String>,
) -> Result<serde_json::Value, String> {
    let client = ApiClient::new(api_url, "".to_string());
    client
        .register(&email, &password, name.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn user_get_profile(app: AppHandle) -> Result<serde_json::Value, String> {
    let client = get_api_client(&app)?;
    client.get_profile().await.map_err(|e| e.to_string())
}

// ==================== Internal Helpers ====================

fn get_api_client(app: &AppHandle) -> Result<ApiClient, String> {
    match ConfigManager::get(app) {
        Some(config) => Ok(ApiClient::new(config.api_url, config.api_key)),
        None => Err("API no configurada. Por favor, ve a Ajustes.".to_string()),
    }
}

// ==================== Favorites Commands ====================

#[tauri::command]
pub async fn favorites_get(
    app: AppHandle,
    page: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<serde_json::Value>, String> {
    let client = get_api_client(&app)?;
    client
        .get_favorites(page, limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_get_by_id(app: AppHandle, id: String) -> Result<serde_json::Value, String> {
    let client = get_api_client(&app)?;
    client.get_favorite(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_create(
    app: AppHandle,
    favorite: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let client = get_api_client(&app)?;
    client
        .create_favorite(favorite)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_update(
    app: AppHandle,
    id: String,
    favorite: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let client = get_api_client(&app)?;
    client
        .update_favorite(&id, favorite)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_delete(app: AppHandle, id: String) -> Result<(), String> {
    let client = get_api_client(&app)?;
    client.delete_favorite(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn metadata_fetch(app: AppHandle, url: String) -> Result<serde_json::Value, String> {
    let client = get_api_client(&app)?;
    client.fetch_metadata(&url).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_visit(app: AppHandle, id: String) -> Result<serde_json::Value, String> {
    let client = get_api_client(&app)?;

    // Get favorite to get URL
    let favorite = client.get_favorite(&id).await.map_err(|e| e.to_string())?;

    // Open URL in default browser
    if let Some(url) = favorite.get("url").and_then(|v| v.as_str()) {
        app.opener().open_url(url, None::<&str>).ok();
    }

    // Return updated favorite (with incremented visit count)
    client.get_favorite(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn favorites_open_url(app: AppHandle, url: String) -> Result<(), String> {
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn favorites_export(app: AppHandle, json_data: String) -> Result<bool, String> {
    use tauri_plugin_dialog::DialogExt;
    
    tracing::info!("favorites_export command called (sync). JSON size: {} bytes", json_data.len());
    
    let file_path = app.dialog()
        .file()
        .add_filter("JSON", &["json"])
        .set_file_name("favoritos_export.json")
        .set_title("Guardar Exportación de Favoritos")
        .blocking_save_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            tracing::info!("Saving export to: {}", path_str);
            std::fs::write(&path_str, json_data).map_err(|e| {
                tracing::error!("Failed to write file: {}", e);
                e.to_string()
            })?;
            tracing::info!("Export successful");
            Ok(true)
        }
        None => {
            tracing::info!("Export cancelled by user");
            Ok(false)
        }
    }
}

// ==================== Folder Commands ====================

#[tauri::command]
pub async fn folders_get(app: AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let client = get_api_client(&app)?;
    client.get_folders().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn folders_create(app: AppHandle, name: String) -> Result<serde_json::Value, String> {
    let client = get_api_client(&app)?;
    client.create_folder(&name).await.map_err(|e| e.to_string())
}

// ==================== Tag Commands ====================

#[tauri::command]
pub async fn tags_get(app: AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let client = get_api_client(&app)?;
    client.get_tags().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn tags_create(
    app: AppHandle,
    name: String,
    color: Option<String>,
) -> Result<serde_json::Value, String> {
    let client = get_api_client(&app)?;
    client
        .create_tag(&name, color.as_deref())
        .await
        .map_err(|e| e.to_string())
}

// ==================== Cache Commands ====================

#[tauri::command]
pub async fn cache_favicon_get(
    domain: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let cache_guard = state.favicon_cache.lock().await;
    if let Some(cache) = cache_guard.as_ref() {
        Ok(cache.get(&domain).map(|p| p.to_string_lossy().to_string()))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn cache_favicon_set(
    domain: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut cache_guard = state.favicon_cache.lock().await;
    if let Some(cache) = cache_guard.as_mut() {
        let path = cache
            .fetch_and_cache(&domain)
            .await
            .map_err(|e| e.to_string())?;
        Ok(path.to_string_lossy().to_string())
    } else {
        Err("Cache not initialized".to_string())
    }
}

// ==================== Window Commands ====================

#[tauri::command]
pub fn window_hide_main(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
pub fn window_show_search(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("search") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())
    } else {
        Err("Search window not found".to_string())
    }
}

// ==================== Notification Commands ====================

#[tauri::command]
pub fn app_show_notification(app: AppHandle, title: String, body: String) -> Result<(), String> {
    notifications::show_notification(&app, &title, &body)
}

#[tauri::command]
pub fn app_quit(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_window_state::{AppHandleExt, StateFlags};
    let _ = app.save_window_state(StateFlags::all());
    app.exit(0);
    Ok(())
}

// ==================== Import Commands ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub imported: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

#[tauri::command]
pub async fn import_favorites(app: AppHandle, data: String) -> Result<ImportResult, String> {
    let client = get_api_client(&app)?;

    // Parse JSON data to ensure it's valid
    let payload: serde_json::Value =
        serde_json::from_str(&data).map_err(|e| format!("Formato JSON inválido: {}", e))?;

    // Call the bulk endpoint
    match client.create_bookmarks_bulk(payload).await {
        Ok(result_data) => {
            // Map the API response to our ImportResult struct
            let imported = result_data
                .get("imported")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;
            let skipped = result_data
                .get("skipped")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;
            let mut errors = Vec::new();

            if let Some(skipped_urls) = result_data.get("skippedUrls").and_then(|v| v.as_array()) {
                for url in skipped_urls {
                    errors.push(format!(
                        "Saltado: {}",
                        url.as_str().unwrap_or("URL desconocida")
                    ));
                }
            }

            Ok(ImportResult {
                imported,
                skipped,
                errors,
            })
        }
        Err(e) => Err(format!(
            "Error en el servidor durante la importación: {}",
            e
        )),
    }
}
