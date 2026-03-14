// Config module - Reliable configuration management using tauri-plugin-store
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Wry};
use tauri_plugin_store::{StoreBuilder, StoreExt};

const CONFIG_FILE: &str = "settings.bin";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub api_url: String,
    pub api_key: String,
    pub log_level: String,
}

impl ApiConfig {
    pub fn new(api_url: String, api_key: String, log_level: String) -> Self {
        Self {
            api_url,
            api_key,
            log_level,
        }
    }

    /// Obfuscate/encrypt key
    fn encrypt_key(key: &str) -> String {
        urlencoding::encode(key).to_string()
    }

    fn decrypt_key(encrypted: &str) -> String {
        urlencoding::decode(encrypted)
            .unwrap_or_default()
            .to_string()
    }
}

pub struct ConfigManager;

impl ConfigManager {
    /// Internal helper to ensure store is loaded and available
    fn get_or_load_store(
        app: &AppHandle,
    ) -> Result<std::sync::Arc<tauri_plugin_store::Store<Wry>>, String> {
        let path = PathBuf::from(CONFIG_FILE);
        if let Some(store) = app.get_store(&path) {
            Ok(store)
        } else {
            // Load it if not in memory
            StoreBuilder::new(app, path)
                .build()
                .map_err(|e| e.to_string())
        }
    }

    pub fn get(app: &AppHandle) -> Option<ApiConfig> {
        tracing::info!("Recuperando configuración del Store...");
        let store = Self::get_or_load_store(app).ok()?;

        let api_url = store.get("api_url")?.as_str()?.to_string();
        let api_key = store
            .get("api_key")
            .and_then(|v| v.as_str().map(|s| ApiConfig::decrypt_key(s)))
            .unwrap_or_default();
        let log_level = store
            .get("log_level")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "info".to_string());

        tracing::info!("Configuración recuperada con éxito");
        Some(ApiConfig::new(api_url, api_key, log_level))
    }

    pub fn save(app: &AppHandle, config: &ApiConfig) -> Result<(), String> {
        tracing::info!("Guardando configuración en el Store...");
        let store = Self::get_or_load_store(app)?;

        let encrypted_key = ApiConfig::encrypt_key(&config.api_key);

        store.set("api_url", serde_json::json!(config.api_url));
        store.set("api_key", serde_json::json!(encrypted_key));
        store.set("log_level", serde_json::json!(config.log_level));

        store.save().map_err(|e| e.to_string())?;

        tracing::info!("Configuración persistida correctamente");
        Ok(())
    }

    pub fn clear(app: &AppHandle) -> Result<(), String> {
        let store = Self::get_or_load_store(app)?;
        // Solo borrar la sesión (api_key), conservando api_url y log_level
        store.delete("api_key");
        store.save().map_err(|e| e.to_string())
    }

    /// Static version for initialization only (without AppHandle)
    pub fn get_initial_log_level() -> String {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("favoritos-desktop");
        path.push(CONFIG_FILE);

        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    return json
                        .get("log_level")
                        .and_then(|v| v.as_str())
                        .unwrap_or("info")
                        .to_string();
                }
            }
        }
        "info".to_string()
    }
}
