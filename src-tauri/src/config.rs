// Config module - Secure configuration management using keychain
use keyring::Entry;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SERVICE_NAME: &str = "favoritos-desktop";

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Keychain error: {0}")]
    Keychain(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub api_url: String,
    pub api_key: String,
}

impl ApiConfig {
    pub fn new(api_url: String, api_key: String) -> Self {
        Self { api_url, api_key }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    /// Get API configuration from keychain
    pub fn get() -> Result<Option<ApiConfig>, ConfigError> {
        let entry = Entry::new(SERVICE_NAME, "api_config")
            .map_err(|e| ConfigError::Keychain(e.to_string()))?;

        match entry.get_password() {
            Ok(password) => {
                let config: ApiConfig = serde_json::from_str(&password)?;
                Ok(Some(config))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(ConfigError::Keychain(e.to_string())),
        }
    }

    /// Save API configuration to keychain
    pub fn save(config: &ApiConfig) -> Result<(), ConfigError> {
        let entry = Entry::new(SERVICE_NAME, "api_config")
            .map_err(|e| ConfigError::Keychain(e.to_string()))?;

        let json = serde_json::to_string(config)?;
        entry
            .set_password(&json)
            .map_err(|e| ConfigError::Keychain(e.to_string()))?;

        Ok(())
    }

    /// Clear API configuration from keychain
    pub fn clear() -> Result<(), ConfigError> {
        let entry = Entry::new(SERVICE_NAME, "api_config")
            .map_err(|e| ConfigError::Keychain(e.to_string()))?;

        match entry.delete_credential() {
            Ok(_) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(ConfigError::Keychain(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = ApiConfig::new(
            "https://api.example.com".to_string(),
            "test-key-123".to_string(),
        );
        let json = serde_json::to_string(&config).unwrap();
        let parsed: ApiConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.api_url, "https://api.example.com");
        assert_eq!(parsed.api_key, "test-key-123");
    }
}
