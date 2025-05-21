//! A thread-safe configuration management library with JSON file support.
//!
//! This module provides a simple way to load and access configuration values from JSON files
//! stored in a `configs/` directory. Configurations are cached globally for efficient access.
//!
//! # Examples
//!
//! ```
//! use config_ro::Config;
//!
//! // Load the "database" configuration (reads from "configs/database.json")
//! let config = Config::new("database");
//!
//! // Access nested values using dot notation
//! let port: u16 = config.get("database.port").unwrap();
//! let host: String = config.get("database.host").unwrap();
//! ```
use std::fs;

use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;
use serde::de::DeserializeOwned;

pub trait ConfigModule {
    fn get(&self, name: &str) -> Option<&str>;
}

// Global config cache
lazy_static! {
    static ref CONFIGS: RwLock<HashMap<String, Value>> = RwLock::new(HashMap::new());
}

/// Configuration instance that provides access to cached configuration values
///
/// Each `Config` instance is associated with a specific configuration file
/// and provides type-safe access to its values.
pub struct Config {
    name: String,
}

impl Config {
    /// Creates or retrieves a cached configuration instance
    ///
    /// The configuration is loaded from `configs/{name}.json`. The file is parsed
    /// only once and then cached for subsequent accesses.
    ///
    /// # Arguments
    /// * `name` - Name of the configuration file (without extension)
    ///
    /// # Panics
    /// - If the configuration file doesn't exist in `configs/` directory
    /// - If the file contains invalid JSON
    ///
    /// # Examples
    /// ```
    /// use config_ro::Config;
    /// let config = Config::new("app_settings");
    /// ```
    pub fn new(name: &str) -> Self {
        let has = {
            let configs = CONFIGS.read().unwrap();
            configs.get(name).is_some()
        };
        if !has {
            let mut configs = CONFIGS.write().unwrap();
            configs.insert(name.to_string(), from_name(name));
        }
        Config {
            name: name.to_string(),
        }
    }

    /// Retrieves a configuration value by its path, supporting nested structures
    ///
    /// Uses dot notation to access nested values (e.g., "database.connection.port").
    /// The value is automatically deserialized to the requested type.
    ///
    /// # Arguments
    /// * `path` - Dot-separated path to the configuration value
    ///
    /// # Returns
    /// `Some(T)` if the value exists and can be deserialized, `None` otherwise
    ///
    /// # Examples
    /// ```
    /// use config_ro::Config;
    /// let config = Config::new("app");
    ///
    /// // Flat structure
    /// let timeout: u32 = config.get("timeout").unwrap();
    ///
    /// // Nested structure
    /// let db_port: u16 = config.get("database.connection.port").unwrap();
    ///
    /// // Optional values
    /// let retry_count: Option<u8> = config.get("retries.count");
    /// ```
    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Option<T> {
        let configs = CONFIGS.read().unwrap();
        let value = configs.get(&self.name)?;

        let mut current = value;
        for key in path.split('.') {
            current = match current.get(key) {
                Some(v) => v,
                None => return None,
            };
        }

        serde_json::from_value(current.clone()).ok()
    }
}

fn from_name(name: &str) -> Value {
    let filename = format!("configs/{}.json", name);
    let content = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read config file: {}", filename));

    serde_json::from_str(&content).unwrap_or_else(|_| panic!("Invalid JSON format in {}", filename))
}

