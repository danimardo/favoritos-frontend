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
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse {
    pub items: Vec<serde_json::Value>,
    pub total: u32,
    pub total_pages: u32,
    pub page: u32,
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

    /// Get favorites with server-side pagination, search, and filtering
    pub async fn get_favorites(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        search: Option<String>,
        folder: Option<String>,
        tags: Option<String>,
    ) -> Result<PaginatedResponse, ApiError> {
        let mut url = format!("{}/bookmarks", self.base_url);
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("pageSize={}", l));
        }
        if let Some(ref s) = search {
            if !s.is_empty() {
                params.push(format!("search={}", urlencoding::encode(s)));
            }
        }
        if let Some(ref f) = folder {
            if !f.is_empty() {
                params.push(format!("folder={}", urlencoding::encode(f)));
            }
        }
        if let Some(ref t) = tags {
            if !t.is_empty() {
                params.push(format!("tagId={}", urlencoding::encode(t)));
            }
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        tracing::info!("[API] get_favorites URL: {}", url);

        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let raw_text = response.text().await?;
            let data: ApiResponse<serde_json::Value> = serde_json::from_str(&raw_text)
                .map_err(|e| ApiError::Message(format!("JSON parse error: {}", e)))?;

            // Log first ~500 chars of raw response (safe UTF-8 boundary)
            let preview_end = raw_text.char_indices()
                .take_while(|(i, _)| *i < 500)
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(raw_text.len());
            tracing::info!("[API] get_favorites raw response preview: {}", &raw_text[..preview_end]);

            if let Some(inner_data) = data.data {
                // Log the keys available in inner_data
                if let Some(obj) = inner_data.as_object() {
                    let keys: Vec<&String> = obj.keys().collect();
                    tracing::info!("[API] get_favorites inner_data keys: {:?}", keys);
                }

                // Extract pagination metadata (API uses "meta" or "pagination")
                let pagination = inner_data.get("pagination")
                    .or_else(|| inner_data.get("meta"));
                if let Some(p) = &pagination {
                    tracing::info!("[API] get_favorites pagination/meta raw: {}", p);
                } else {
                    tracing::warn!("[API] get_favorites: no pagination/meta field in inner_data");
                }
                let total = pagination
                    .and_then(|p| p.get("total"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                let page_size = pagination
                    .and_then(|p| p.get("pageSize").or_else(|| p.get("limit")))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(50) as u32;
                // totalPages may not exist; calculate from total/pageSize
                let total_pages = pagination
                    .and_then(|p| p.get("totalPages").or_else(|| p.get("pages")))
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u32)
                    .unwrap_or_else(|| {
                        if total == 0 { 1 } else { (total + page_size - 1) / page_size }
                    });
                let current_page = pagination
                    .and_then(|p| p.get("page"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as u32;

                // Extract items from nested { data: [...] } or direct array
                let items = if let Some(list) = inner_data.get("data").and_then(|v| v.as_array()) {
                    list.iter().cloned().collect()
                } else if let Some(list) = inner_data.as_array() {
                    list.iter().cloned().collect()
                } else {
                    Vec::new()
                };

                tracing::info!("[API] get_favorites page={}: {} items (total: {}, pages: {})",
                    current_page, items.len(), total, total_pages);

                return Ok(PaginatedResponse {
                    items,
                    total,
                    total_pages,
                    page: current_page,
                });
            }

            Ok(PaginatedResponse {
                items: Vec::new(),
                total: 0,
                total_pages: 0,
                page: 1,
            })
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

    /// Paginate an endpoint automatically until all items are fetched
    async fn paginate_all(&self, endpoint: &str) -> Result<Vec<serde_json::Value>, ApiError> {
        let mut all_items = Vec::new();
        let mut page = 1u32;
        let page_size = 100;

        loop {
            let url = format!("{}/{}?page={}&pageSize={}", self.base_url, endpoint, page, page_size);
            let response = self
                .client
                .get(&url)
                .header("X-API-Key", &self.api_key)
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(ApiError::Message(format!(
                    "Failed to get {}: {}",
                    endpoint,
                    response.status()
                )));
            }

            let raw_text = response.text().await?;
            let data: ApiResponse<serde_json::Value> = serde_json::from_str(&raw_text)
                .map_err(|e| ApiError::Message(format!("JSON parse error for {}: {}", endpoint, e)))?;

            let mut page_items = Vec::new();
            let mut total_pages = 1u32;

            if let Some(inner_data) = data.data {
                // Format: { data: [...], pagination|meta: { totalPages|pages, ... } }
                if let Some(list) = inner_data.get("data").and_then(|v| v.as_array()) {
                    page_items = list.iter().cloned().collect();
                    if let Some(pagination) = inner_data.get("pagination").or_else(|| inner_data.get("meta")) {
                        let total_items = pagination
                            .get("total")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0) as u32;
                        let ps = pagination
                            .get("pageSize")
                            .or_else(|| pagination.get("limit"))
                            .and_then(|v| v.as_u64())
                            .unwrap_or(page_size as u64) as u32;
                        total_pages = pagination
                            .get("totalPages")
                            .or_else(|| pagination.get("pages"))
                            .and_then(|v| v.as_u64())
                            .map(|v| v as u32)
                            .unwrap_or_else(|| {
                                if total_items == 0 { 1 } else { (total_items + ps - 1) / ps }
                            });
                    }
                }
                // Format: direct array (no pagination wrapper)
                else if let Some(list) = inner_data.as_array() {
                    page_items = list.iter().cloned().collect();
                    total_pages = 1;
                }
            }

            tracing::info!(
                "[API] paginate_all {}: page {}/{}, got {} items",
                endpoint,
                page,
                total_pages,
                page_items.len()
            );

            let is_empty = page_items.is_empty();
            all_items.extend(page_items);

            if page >= total_pages || is_empty {
                break;
            }
            page += 1;
        }

        tracing::info!("[API] paginate_all {}: total {} items loaded", endpoint, all_items.len());
        Ok(all_items)
    }

    /// Fetch all items from a /stats endpoint (returns complete list, no pagination needed)
    async fn fetch_all_from_stats(&self, endpoint: &str) -> Result<Vec<serde_json::Value>, ApiError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        tracing::info!("[API] fetch_all_from_stats: {}", url);

        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            tracing::warn!("[API] {} returned status {}, falling back to paginate_all",
                endpoint, response.status());
            // Fallback to paginated endpoint if /stats is not available
            let base_endpoint = endpoint.replace("/stats", "");
            return self.paginate_all(&base_endpoint).await;
        }

        let raw_text = response.text().await?;
        let data: ApiResponse<serde_json::Value> = serde_json::from_str(&raw_text)
            .map_err(|e| ApiError::Message(format!("JSON parse error for {}: {}", endpoint, e)))?;

        let items = if let Some(inner_data) = data.data {
            // Format: { data: [...], pagination: {...} } or direct array
            if let Some(list) = inner_data.get("data").and_then(|v| v.as_array()) {
                list.iter().cloned().collect()
            } else if let Some(list) = inner_data.as_array() {
                list.iter().cloned().collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        tracing::info!("[API] fetch_all_from_stats {}: {} items loaded", endpoint, items.len());
        Ok(items)
    }

    /// Get all folders with bookmark counts via /folders/stats
    pub async fn get_folders(&self) -> Result<Vec<serde_json::Value>, ApiError> {
        self.fetch_all_from_stats("folders/stats").await
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

    /// Get all tags with bookmark counts via /tags/stats
    pub async fn get_tags(&self) -> Result<Vec<serde_json::Value>, ApiError> {
        self.fetch_all_from_stats("tags/stats").await
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

    /// Delete a tag by ID
    pub async fn delete_tag(&self, id: &str) -> Result<(), ApiError> {
        let url = format!("{}/tags/{}", self.base_url, id);
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
                "Failed to delete tag: {}",
                response.status()
            )))
        }
    }
}
