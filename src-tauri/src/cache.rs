// Cache module - Favicon cache management
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Image error: {0}")]
    Image(String),
}

pub struct FaviconCache {
    cache_dir: PathBuf,
    memory_cache: Mutex<HashMap<String, PathBuf>>,
}

impl FaviconCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        // Ensure cache directory exists
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).ok();
        }
        Self {
            cache_dir,
            memory_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Get the cache directory path
    pub fn get_cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// Check if a favicon is cached
    pub fn is_cached(&self, domain: &str) -> bool {
        let cache = self.memory_cache.lock().unwrap();
        cache.contains_key(domain) || self.file_exists(domain)
    }

    /// Get cached favicon path
    pub fn get(&self, domain: &str) -> Option<PathBuf> {
        // Check memory cache first
        {
            let cache = self.memory_cache.lock().unwrap();
            if let Some(path) = cache.get(domain) {
                return Some(path.clone());
            }
        }

        // Check file system
        if self.file_exists(domain) {
            let path = self.get_favicon_path(domain);
            let mut cache = self.memory_cache.lock().unwrap();
            cache.insert(domain.to_string(), path.clone());
            return Some(path);
        }

        None
    }

    /// Download and cache a favicon
    pub async fn fetch_and_cache(&self, domain: &str) -> Result<PathBuf, CacheError> {
        // Check if already cached
        if let Some(path) = self.get(domain) {
            return Ok(path);
        }

        // Try to fetch favicon from multiple sources
        let favicon_urls = vec![
            format!("https://www.google.com/s2/favicons?domain={}&sz=128", domain),
            format!("https://{}/favicon.ico", domain),
        ];

        let client = reqwest::Client::new();

        for url in favicon_urls {
            match self.fetch_favicon(&client, domain, &url).await {
                Ok(path) => {
                    let mut cache = self.memory_cache.lock().unwrap();
                    cache.insert(domain.to_string(), path.clone());
                    info!("Cached favicon for domain: {}", domain);
                    return Ok(path);
                }
                Err(e) => {
                    warn!("Failed to fetch favicon from {}: {}", url, e);
                    continue;
                }
            }
        }

        Err(CacheError::Image("Failed to fetch favicon from any source".to_string()))
    }

    async fn fetch_favicon(
        &self,
        client: &reqwest::Client,
        domain: &str,
        url: &str,
    ) -> Result<PathBuf, CacheError> {
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(CacheError::Image(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        // Get content type
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("image/x-icon");

        // Determine extension
        let extension = match content_type {
            "image/png" => "png",
            "image/x-icon" => "ico",
            "image/svg+xml" => "svg",
            "image/jpeg" | "image/jpg" => "jpg",
            _ => "png",
        };

        // Save to cache
        let path = self.get_favicon_path(domain);
        let bytes = response.bytes().await?;
        std::fs::write(&path, &bytes)?;

        Ok(path)
    }

    fn get_favicon_path(&self, domain: &str) -> PathBuf {
        let safe_domain = domain.replace(['/', ':', '\\'], "_");
        self.cache_dir.join(format!("{}.png", safe_domain))
    }

    fn file_exists(&self, domain: &str) -> bool {
        self.get_favicon_path(domain).exists()
    }

    /// Clear the cache
    pub fn clear(&self) -> Result<(), CacheError> {
        let mut cache = self.memory_cache.lock().unwrap();
        cache.clear();

        // Remove all files in cache directory
        for entry in std::fs::read_dir(&self.cache_dir)? {
            if let Ok(entry) = entry {
                std::fs::remove_file(entry.path()).ok();
            }
        }

        info!("Favicon cache cleared");
        Ok(())
    }

    /// Get cache size in bytes
    pub fn size(&self) -> u64 {
        std::fs::read_dir(&self.cache_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.metadata().ok())
                    .map(|m| m.len())
                    .sum()
            })
            .unwrap_or(0)
    }
}
