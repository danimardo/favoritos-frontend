// Tauri Commands - IPC interface between frontend and backend
use crate::api::{ApiClient, ApiError};
use crate::cache::FaviconCache;
use crate::config::{ApiConfig, ConfigManager};
use crate::notifications;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub favicon_cache: Mutex<Option<FaviconCache>>,
}

// ==================== Config Commands ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub configured: bool,
    pub api_url: Option<String>,
}

#[tauri::command]
pub fn config_get() -> Result<ConfigResponse, String> {
    match ConfigManager::get() {
        Ok(config) => Ok(ConfigResponse {
            configured: config.is_some(),
            api_url: config.map(|c| c.api_url),
        }),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn config_save(api_url: String, api_key: String) -> Result<(), String> {
    let config = ApiConfig::new(api_url, api_key);
    ConfigManager::save(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn config_validate(api_url: String, api_key: String) -> Result<bool, String> {
    let client = ApiClient::new(api_url, api_key);
    match client.health().await {
        Ok(_) => Ok(true),
        Err(ApiError::Message(msg)) => Err(msg),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn config_clear() -> Result<(), String> {
    ConfigManager::clear().map_err(|e| e.to_string())
}

// ==================== Favorites Commands ====================

fn get_api_client() -> Result<ApiClient, String> {
    let config = ConfigManager::get()
        .map_err(|e| e.to_string())?
        .ok_or("API not configured")?;
    Ok(ApiClient::new(config.api_url, config.api_key))
}

#[tauri::command]
pub async fn favorites_get(
    page: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<serde_json::Value>, String> {
    let client = get_api_client()?;
    client
        .get_favorites(page, limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_get_by_id(id: String) -> Result<serde_json::Value, String> {
    let client = get_api_client()?;
    client
        .get_favorite(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_create(favorite: serde_json::Value) -> Result<serde_json::Value, String> {
    let client = get_api_client()?;
    client
        .create_favorite(favorite)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_update(
    id: String,
    favorite: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let client = get_api_client()?;
    client
        .update_favorite(&id, favorite)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_delete(id: String) -> Result<(), String> {
    let client = get_api_client()?;
    client
        .delete_favorite(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn metadata_fetch(url: String) -> Result<serde_json::Value, String> {
    let client = get_api_client()?;
    client
        .fetch_metadata(&url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorites_visit(app: AppHandle, id: String) -> Result<serde_json::Value, String> {
    let client = get_api_client()?;

    // Get favorite to get URL
    let favorite = client.get_favorite(&id).await.map_err(|e| e.to_string())?;

    // Open URL in default browser
    if let Some(url) = favorite.get("url").and_then(|v| v.as_str()) {
        open::that(url).ok();
    }

    // Return updated favorite (with incremented visit count)
    client.get_favorite(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn favorites_open_url(url: String) -> Result<(), String> {
    open::that(&url).map_err(|e| e.to_string())
}

// ==================== Folder Commands ====================

#[tauri::command]
pub async fn folders_get() -> Result<Vec<serde_json::Value>, String> {
    let client = get_api_client()?;
    client.get_folders().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn folders_create(name: String) -> Result<serde_json::Value, String> {
    let client = get_api_client()?;
    client
        .create_folder(&name)
        .await
        .map_err(|e| e.to_string())
}

// ==================== Tag Commands ====================

#[tauri::command]
pub async fn tags_get() -> Result<Vec<serde_json::Value>, String> {
    let client = get_api_client()?;
    client.get_tags().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn tags_create(name: String, color: Option<String>) -> Result<serde_json::Value, String> {
    let client = get_api_client()?;
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
    let cache_guard = state.favicon_cache.lock().map_err(|e| e.to_string())?;
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
    let cache_guard = state.favicon_cache.lock().map_err(|e| e.to_string())?;
    if let Some(cache) = cache_guard.as_ref() {
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
pub async fn import_favorites(data: String) -> Result<ImportResult, String> {
    let client = get_api_client()?;

    // Parse JSON data
    let favorites: Vec<serde_json::Value> =
        serde_json::from_str(&data).map_err(|e| e.to_string())?;

    let mut imported = 0;
    let mut skipped = 0;
    let mut errors = Vec::new();

    for favorite in favorites {
        match client.create_favorite(favorite).await {
            Ok(_) => imported += 1,
            Err(e) => {
                skipped += 1;
                errors.push(e.to_string());
            }
        }
    }

    Ok(ImportResult {
        imported,
        skipped,
        errors,
    })
}
