// API client module - HTTP client for external Favoritos API
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Message(String),
    #[error("Not configured")]
    NotConfigured,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: Option<String>,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    /// Validate credentials by attempting to fetch the current user profile
    pub async fn validate_credentials(&self) -> Result<(), ApiError> {
        let url = format!("{}/users/me", self.base_url);
        tracing::info!("Validando sesión en: {}", url);
        
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("Sesión válida");
            Ok(())
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            tracing::error!("Validación de sesión fallida: {} - {}", status, error_body);
            
            if status == 401 || status == 403 {
                Err(ApiError::Message("Sesión expirada o API Key inválida".to_string()))
            } else {
                Err(ApiError::Message(format!(
                    "Error al validar sesión ({}): {}",
                    status,
                    error_body
                )))
            }
        }
    }

    /// Login with email and password to get a new API Key
    pub async fn login(&self, email: &str, password: &str) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/auth/login", self.base_url);
        let body = serde_json::json!({
            "email": email,
            "password": password
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data.ok_or_else(|| ApiError::Message("No data returned from login".to_string()))
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let message = if status == 401 {
                "Credenciales inválidas".to_string()
            } else {
                format!("Error en login ({}): {}", status, error_body)
            };
            Err(ApiError::Message(message))
        }
    }

    /// Register a new user
    pub async fn register(&self, email: &str, password: &str, name: Option<&str>) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/auth/register", self.base_url);
        let body = serde_json::json!({
            "email": email,
            "password": password,
            "name": name
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data.ok_or_else(|| ApiError::Message("No data returned from register".to_string()))
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let message = if status == 409 {
                "El email ya está registrado".to_string()
            } else {
                format!("Error en registro ({}): {}", status, error_body)
            };
            Err(ApiError::Message(message))
        }
    }

    /// Get current user profile
    pub async fn get_profile(&self) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/users/me", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data
                .ok_or_else(|| ApiError::Message("No data returned from profile".to_string()))
        } else {
            Err(ApiError::Message(format!(
                "Failed to get profile: {}",
                response.status()
            )))
        }
    }

    /// Get all favorites with pagination
    pub async fn get_favorites(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<serde_json::Value>, ApiError> {
        let mut url = format!("{}/bookmarks", self.base_url);
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            // USAMOS pageSize SEGÚN EL API REAL
            params.push(format!("pageSize={}", l));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            if let Some(inner_data) = data.data {
                // If it's the nested structure { data: [], meta: {} }
                if let Some(list) = inner_data.get("data").and_then(|v| v.as_array()) {
                    return Ok(list.iter().cloned().collect());
                }
                // If it's a direct array
                if let Some(list) = inner_data.as_array() {
                    return Ok(list.iter().cloned().collect());
                }
            }
            Ok(Vec::new())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(ApiError::Message(format!(
                "Failed to get favorites: {} - {}",
                status, error_text
            )))
        }
    }

    /// Get a single favorite by ID
    pub async fn get_favorite(&self, id: &str) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/bookmarks/{}", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data
                .ok_or_else(|| ApiError::Message("No data returned".to_string()))
        } else {
            Err(ApiError::Message(format!(
                "Failed to get favorite: {}",
                response.status()
            )))
        }
    }

    /// Create a new favorite
    pub async fn create_favorite(
        &self,
        mut favorite: serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/bookmarks", self.base_url);
        
        // Fix: some APIs expect folderId instead of folder
        let folder_val = favorite.get("folder").cloned();
        if let Some(folder) = folder_val {
            if let Some(obj) = favorite.as_object_mut() {
                obj.remove("folder");
                if !folder.is_null() && folder.as_str().unwrap_or("") != "" {
                    obj.insert("folderId".to_string(), folder);
                }
            }
        }

        // Fix: remove empty tags if present
        if let Some(tags) = favorite.get("tags") {
            if tags.is_array() && tags.as_array().unwrap().is_empty() {
                if let Some(obj) = favorite.as_object_mut() {
                    obj.remove("tags");
                }
            }
        }

        tracing::debug!("Creando favorito en: {} con datos: {}", url, favorite);
        
        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .json(&favorite)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            tracing::debug!("Favorito creado con éxito");
            data.data
                .ok_or_else(|| ApiError::Message("No data returned".to_string()))
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            tracing::error!("Error al crear favorito: {} - {}", status, error_body);
            Err(ApiError::Message(format!(
                "Failed to create favorite: {} - {}",
                status, error_body
            )))
        }
    }

    /// Import bookmarks from an HTML file (Netscape Bookmark format)
    pub async fn import_bookmarks(
        &self,
        file_bytes: Vec<u8>,
        filename: String,
        ia: bool,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/bookmarks/import", self.base_url);

        tracing::info!(
            "[Importación] Enviando archivo HTML: {} ({} bytes, ia: {})",
            filename,
            file_bytes.len(),
            ia
        );

        let file_part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(filename.clone())
            .mime_str("text/html")
            .map_err(|e| ApiError::Message(format!("Error creando part multipart: {}", e)))?;

        let form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("ia", ia.to_string());

        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let raw_body = response.text().await.map_err(|e| {
                ApiError::Message(format!("Error al leer respuesta de importación: {}", e))
            })?;
            tracing::debug!("[Importación] Respuesta del servidor: {}", raw_body);

            let api_response: ApiResponse<serde_json::Value> =
                serde_json::from_str(&raw_body).map_err(|e| {
                    ApiError::Message(format!("Error al parsear respuesta de importación: {}", e))
                })?;
            api_response
                .data
                .ok_or_else(|| ApiError::Message("No data returned from import".to_string()))
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            tracing::error!("[Importación] Error del servidor: {} - {}", status, error_body);
            Err(ApiError::Message(format!(
                "Import failed: {} - {}",
                status, error_body
            )))
        }
    }

    /// Export bookmarks as Netscape Bookmark HTML from the server
    pub async fn export_bookmarks_html(&self) -> Result<Vec<u8>, ApiError> {
        let url = format!("{}/bookmarks/export", self.base_url);

        tracing::info!("[Exportación] Solicitando exportación HTML desde: {}", url);

        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let bytes = response.bytes().await.map_err(|e| {
                ApiError::Message(format!("Error al leer respuesta de exportación: {}", e))
            })?;
            tracing::info!(
                "[Exportación] HTML recibido: {} bytes",
                bytes.len()
            );
            Ok(bytes.to_vec())
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            tracing::error!("[Exportación] Error del servidor: {} - {}", status, error_body);
            Err(ApiError::Message(format!(
                "Export failed: {} - {}",
                status, error_body
            )))
        }
    }

    /// Trigger metadata refresh for a bookmark (async server-side process).
    /// Returns immediately with { success: true }. Use get_metadata_status to poll.
    pub async fn refresh_metadata(&self, id: &str) -> Result<(), ApiError> {
        let url = format!("{}/bookmarks/{}/metadata/refresh", self.base_url, id);

        tracing::info!("[Metadata Refresh] Disparando enriquecimiento del favorito {}", id);

        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("[Metadata Refresh] Proceso iniciado correctamente para favorito {}", id);
            Ok(())
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            tracing::error!("[Metadata Refresh] Error: {} - {}", status, error_body);
            Err(ApiError::Message(format!(
                "Metadata refresh failed: {} - {}",
                status, error_body
            )))
        }
    }

    /// Check metadata processing status for a bookmark.
    pub async fn get_metadata_status(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/bookmarks/{}/metadata/status", self.base_url, id);

        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: ApiResponse<serde_json::Value> = response.json().await?;
            api_response
                .data
                .ok_or_else(|| ApiError::Message("No data returned from metadata status".to_string()))
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            Err(ApiError::Message(format!(
                "Metadata status failed: {} - {}",
                status, error_body
            )))
        }
    }

    /// Update an existing favorite
    pub async fn update_favorite(
        &self,
        id: &str,
        mut favorite: serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/bookmarks/{}", self.base_url, id);

        // Normalización: mapear folder a folderId
        let folder_val = favorite.get("folder").cloned();
        if let Some(folder) = folder_val {
            if let Some(obj) = favorite.as_object_mut() {
                obj.remove("folder");
                if !folder.is_null() && folder.as_str().unwrap_or("") != "" {
                    obj.insert("folderId".to_string(), folder);
                }
            }
        }

        // Normalización: eliminar tags vacíos
        if let Some(tags) = favorite.get("tags") {
            if tags.is_array() && tags.as_array().unwrap().is_empty() {
                if let Some(obj) = favorite.as_object_mut() {
                    obj.remove("tags");
                }
            }
        }

        tracing::debug!("Actualizando favorito {} con datos: {}", id, favorite);

        let response = self
            .client
            .put(&url)
            .header("X-API-Key", &self.api_key)
            .json(&favorite)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data
                .ok_or_else(|| ApiError::Message("No data returned".to_string()))
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            tracing::error!("Error al actualizar favorito: {} - {}", status, error_body);
            Err(ApiError::Message(format!(
                "Failed to update favorite: {} - {}",
                status, error_body
            )))
        }
    }

    /// Delete a favorite
    pub async fn delete_favorite(&self, id: &str) -> Result<(), ApiError> {
        let url = format!("{}/bookmarks/{}", self.base_url, id);
        let response = self
            .client
            .delete(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(ApiError::Message(format!(
                "Failed to delete favorite: {}",
                response.status()
            )))
        }
    }

    /// Fetch metadata from a URL (title, description, favicon)
    pub async fn fetch_metadata(
        &self,
        url: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/metadata?url={}", self.base_url, urlencoding::encode(url));
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data
                .ok_or_else(|| ApiError::Message("No data returned".to_string()))
        } else {
            Err(ApiError::Message(format!(
                "Failed to fetch metadata: {}",
                response.status()
            )))
        }
    }

    /// Get all folders
    pub async fn get_folders(&self) -> Result<Vec<serde_json::Value>, ApiError> {
        let url = format!("{}/folders?limit=1000", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            if let Some(inner_data) = data.data {
                if let Some(list) = inner_data.get("data").and_then(|v| v.as_array()) {
                    return Ok(list.iter().cloned().collect());
                }
                if let Some(list) = inner_data.as_array() {
                    return Ok(list.iter().cloned().collect());
                }
            }
            Ok(Vec::new())
        } else {
            Err(ApiError::Message(format!(
                "Failed to get folders: {}",
                response.status()
            )))
        }
    }

    /// Create a new folder
    pub async fn create_folder(&self, name: &str) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/folders", self.base_url);
        let body = serde_json::json!({ "name": name });
        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data
                .ok_or_else(|| ApiError::Message("No data returned".to_string()))
        } else {
            Err(ApiError::Message(format!(
                "Failed to create folder: {}",
                response.status()
            )))
        }
    }

    /// Get all tags
    pub async fn get_tags(&self) -> Result<Vec<serde_json::Value>, ApiError> {
        let url = format!("{}/tags?limit=1000", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            if let Some(inner_data) = data.data {
                if let Some(list) = inner_data.get("data").and_then(|v| v.as_array()) {
                    return Ok(list.iter().cloned().collect());
                }
                if let Some(list) = inner_data.as_array() {
                    return Ok(list.iter().cloned().collect());
                }
            }
            Ok(Vec::new())
        } else {
            Err(ApiError::Message(format!(
                "Failed to get tags: {}",
                response.status()
            )))
        }
    }

    /// Create a new tag
    pub async fn create_tag(&self, name: &str, color: Option<&str>) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/tags", self.base_url);
        let mut body = serde_json::json!({ "name": name });
        if let Some(c) = color {
            body["color"] = serde_json::json!(c);
        }
        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data
                .ok_or_else(|| ApiError::Message("No data returned".to_string()))
        } else {
            Err(ApiError::Message(format!(
                "Failed to create tag: {}",
                response.status()
            )))
        }
    }
}
