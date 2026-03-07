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

    /// Validate credentials by attempting to fetch a small list of favorites
    pub async fn validate_credentials(&self) -> Result<(), ApiError> {
        let url = format!("{}/bookmarks", self.base_url);
        tracing::info!("Validando conexión a: {}", url);
        
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("Validación exitosa");
            Ok(())
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            tracing::error!("Validación fallida: {} - {}", status, error_body);
            
            if status == 401 || status == 403 {
                Err(ApiError::Message("API Key inválida o sin permisos".to_string()))
            } else {
                Err(ApiError::Message(format!(
                    "Error de validación ({}): {}",
                    status,
                    error_body
                )))
            }
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
            params.push(format!("limit={}", l));
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
            let data: ApiResponse<Vec<serde_json::Value>> = response.json().await?;
            Ok(data.data.unwrap_or_default())
        } else {
            Err(ApiError::Message(format!(
                "Failed to get favorites: {}",
                response.status()
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
        favorite: serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/favorites", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .json(&favorite)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<serde_json::Value> = response.json().await?;
            data.data
                .ok_or_else(|| ApiError::Message("No data returned".to_string()))
        } else {
            Err(ApiError::Message(format!(
                "Failed to create favorite: {}",
                response.status()
            )))
        }
    }

    /// Update an existing favorite
    pub async fn update_favorite(
        &self,
        id: &str,
        favorite: serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/bookmarks/{}", self.base_url, id);
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
            Err(ApiError::Message(format!(
                "Failed to update favorite: {}",
                response.status()
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
        let url = format!("{}/folders", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<Vec<serde_json::Value>> = response.json().await?;
            Ok(data.data.unwrap_or_default())
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
        let url = format!("{}/tags", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ApiResponse<Vec<serde_json::Value>> = response.json().await?;
            Ok(data.data.unwrap_or_default())
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
