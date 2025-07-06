use crate::models::Server;
use crate::utils::ensure_dir_exists;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            servers: Vec::new(),
        }
    }

    pub fn add_server(&mut self, server: Server) -> Result<()> {
        // Check if server with same name already exists
        if self.find_server(&server.name).is_some() {
            return Err(anyhow::anyhow!("Server with name '{}' already exists", server.name));
        }
        
        self.servers.push(server);
        Ok(())
    }

    pub fn remove_server(&mut self, identifier: &str) -> Result<Server> {
        let index = self.servers
            .iter()
            .position(|s| s.matches(identifier))
            .ok_or_else(|| anyhow::anyhow!("Server '{}' not found", identifier))?;
        
        Ok(self.servers.remove(index))
    }

    pub fn find_server(&self, identifier: &str) -> Option<&Server> {
        self.servers.iter().find(|s| s.matches(identifier))
    }

    pub fn find_server_mut(&mut self, identifier: &str) -> Option<&mut Server> {
        self.servers.iter_mut().find(|s| s.matches(identifier))
    }

    pub fn list_servers(&self) -> &[Server] {
        &self.servers
    }

    pub fn is_empty(&self) -> bool {
        self.servers.is_empty()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_path = get_config_path()?;
        Ok(ConfigManager { config_path })
    }

    pub fn load(&self) -> Result<Config> {
        if !self.config_path.exists() {
            return Ok(Config::new());
        }

        let contents = fs::read_to_string(&self.config_path)
            .with_context(|| format!("Failed to read config file: {}", self.config_path.display()))?;

        if contents.trim().is_empty() {
            return Ok(Config::new());
        }

        let config: Config = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", self.config_path.display()))?;

        Ok(config)
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        // Ensure the parent directory exists
        if let Some(parent) = self.config_path.parent() {
            ensure_dir_exists(parent)?;
        }

        let contents = serde_json::to_string_pretty(config)
            .context("Failed to serialize config")?;

        fs::write(&self.config_path, contents)
            .with_context(|| format!("Failed to write config file: {}", self.config_path.display()))?;

        Ok(())
    }

    pub fn get_config_path(&self) -> &Path {
        &self.config_path
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ConfigManager")
    }
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

    let hop_dir = config_dir.join("hop");
    let config_path = hop_dir.join("servers.json");

    Ok(config_path)
}

/// Load configuration from file
pub fn load_config() -> Result<Config> {
    let manager = ConfigManager::new()?;
    manager.load()
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let manager = ConfigManager::new()?;
    manager.save(config)
}

/// Get the path to the configuration file
pub fn get_config_file_path() -> Result<PathBuf> {
    let manager = ConfigManager::new()?;
    Ok(manager.config_path.clone())
}

/// Initialize configuration directory and file if they don't exist
pub fn init_config() -> Result<()> {
    let manager = ConfigManager::new()?;
    
    if !manager.config_path.exists() {
        let config = Config::new();
        manager.save(&config)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_add_server() {
        let mut config = Config::new();
        let server = Server::new("test".to_string(), "user".to_string(), "192.168.1.1".to_string());
        
        assert!(config.add_server(server).is_ok());
        assert_eq!(config.servers.len(), 1);
    }

    #[test]
    fn test_config_add_duplicate_server() {
        let mut config = Config::new();
        let server1 = Server::new("test".to_string(), "user".to_string(), "192.168.1.1".to_string());
        let server2 = Server::new("test".to_string(), "user".to_string(), "192.168.1.2".to_string());
        
        assert!(config.add_server(server1).is_ok());
        assert!(config.add_server(server2).is_err());
    }

    #[test]
    fn test_config_find_server() {
        let mut config = Config::new();
        let server = Server::new("test".to_string(), "user".to_string(), "192.168.1.1".to_string());
        
        config.add_server(server).unwrap();
        
        assert!(config.find_server("test").is_some());
        assert!(config.find_server("nonexistent").is_none());
    }

    #[test]
    fn test_config_remove_server() {
        let mut config = Config::new();
        let server = Server::new("test".to_string(), "user".to_string(), "192.168.1.1".to_string());
        
        config.add_server(server).unwrap();
        assert_eq!(config.servers.len(), 1);
        
        assert!(config.remove_server("test").is_ok());
        assert_eq!(config.servers.len(), 0);
        
        assert!(config.remove_server("nonexistent").is_err());
    }
} 