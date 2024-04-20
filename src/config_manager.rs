use anyhow::Result;
use log::warn;
use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigManager {
    pub chat_model: String,
    pub chat_base_prompt: String,
    pub max_tokens: u32,
    pub image_size: String,
}

// Default config values
impl Default for ConfigManager {
    fn default() -> Self {
        ConfigManager {
            chat_model: "gpt-4".to_string(),
            chat_base_prompt: "You are an assistant that is built into a Telegram bot. Only respond with plaintext and if you are writing code begin with CODE-START and end with CODE-END.".to_string(),
            max_tokens: 1024,
            image_size: "512x512".to_string(),
        }
    }
}

impl ConfigManager {
    /// Grab values from the system config file
    /// # Errors
    /// If there is an OS read and write error
    pub fn new() -> Result<Self> {
        let serialized_data = match Self::read_file(None) {
            Ok(data) => data,
            Err(error) => {
                warn!("Using default values due to error in reading config file: {error}");
                ConfigManager::default().write_file(None)?;
                ConfigManager::default()
            }
        };

        Ok(serialized_data)
    }

    fn read_file(path_in: Option<&Path>) -> Result<Self> {
        let path = path_in.unwrap_or(Path::new("config.json"));

        let mut file = File::open(path)?;

        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;

        let serialized_data: ConfigManager = serde_json::from_str(&json_string)?;

        Ok(serialized_data)
    }

    fn write_file(&self, path_in: Option<&Path>) -> Result<()> {
        let path = path_in.unwrap_or(Path::new("config.json"));

        let json_string = serde_json::to_string_pretty(&self)?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .truncate(true)
            .open(path)?;

        file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_read_file_valid_file() {
        let config_data = r#"
        {
            "chat_model": "gpt-3.5-turbo",
            "chat_base_prompt": "Test prompt",
            "max_tokens": 1024,
            "image_size": "512x512"
        }
        "#;
        fs::write("test_config.json", config_data).unwrap();
        let path = Path::new("test_config.json");
        let result = ConfigManager::read_file(Some(path)).unwrap();
        assert_eq!(result.chat_base_prompt, "Test prompt");
        fs::remove_file("test_config.json").unwrap();
    }

    #[test]
    fn test_write_file() {
        let config = ConfigManager {
            chat_model: "gpt-3.5-turbo".to_string(),
            chat_base_prompt: "Test write".to_string(),
            max_tokens: 1024,
            image_size: "512x512".to_string(),
        };

        let path = Path::new("test_config_write.json");
        config.write_file(Some(path)).unwrap();
        let read_config = ConfigManager::read_file(Some(path)).unwrap();
        assert_eq!(read_config.chat_base_prompt, "Test write");
        fs::remove_file("test_config_write.json").unwrap();
    }

    #[test]
    fn test_read_file_invalid_file() {
        fs::write("test_malformed_config.json", "invalid json content").unwrap();
        let path = Path::new("test_malformed_config.json");
        assert!(ConfigManager::read_file(Some(path)).is_err());
        fs::remove_file("test_malformed_config.json").unwrap();
    }

    #[test]
    fn test_read_file_incomplete_structure() {
        let config_data = r#"
        {
            "chat_model": "gpt-3.5-turbo",
            "max_tokens": 1024,
            "image_size": "512x512"
        }
        "#;
        fs::write("test_incomplete_config.json", config_data).unwrap();
        let path = Path::new("test_incomplete_config.json");
        assert!(ConfigManager::read_file(Some(path)).is_err());
        fs::remove_file("test_incomplete_config.json").unwrap();
    }
}
