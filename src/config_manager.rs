use serde_derive::{Deserialize, Serialize};
use std::error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigManager {
    pub completion_model: String,
    pub chat_model: String,
    pub chat_base_prompt: String,
    pub max_tokens: u32,
    pub image_size: String,
}

// Default config values
impl Default for ConfigManager {
    fn default() -> Self {
        ConfigManager {
            completion_model: "text-davinci-003".to_string(),
            chat_model: "gpt-3.5-turbo".to_string(),
            chat_base_prompt: "You are an assistant that is built into a Telegram bot. Only respond with plaintext and if you are writing code begin with CODE-START and end with CODE-END.".to_string(),
            max_tokens: 1024,
            image_size: "512x512".to_string(),
        }
    }
}

impl ConfigManager {
    pub fn new() -> Self {
        let default_values = ConfigManager::default();

        let serialized_data = match default_values.read_file(None) {
            Ok(data) => data,
            Err(error) => {
                println!("Using default values due to error in reading config file: {error}");
                default_values
            }
        };

        match serialized_data.write_file(None) {
            Ok(_) => (),
            Err(error) => panic!("Cannot write config data: {error}"),
        };

        serialized_data
    }

    pub fn read_file(&self, path_in: Option<&Path>) -> Result<Self, Box<dyn error::Error>> {
        let path = path_in.unwrap_or(Path::new("config.json"));

        let mut file = File::open(path)?;

        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;

        let serialized_data: ConfigManager = serde_json::from_str(&json_string)?;

        Ok(serialized_data)
    }

    pub fn write_file(&self, path_in: Option<&Path>) -> Result<(), Box<dyn error::Error>> {
        let path = path_in.unwrap_or(Path::new("config.json"));

        let json_string = serde_json::to_string_pretty(&self)?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
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
            "completion_model": "text-davinci-003",
            "chat_model": "gpt-3.5-turbo",
            "chat_base_prompt": "Test prompt",
            "max_tokens": 1024,
            "image_size": "512x512"
        }
        "#;
        fs::write("test_config.json", config_data).unwrap();
        let config = ConfigManager::default();
        let path = Path::new("test_config.json");
        let result = config.read_file(Some(path)).unwrap();
        assert_eq!(result.chat_base_prompt, "Test prompt");
        fs::remove_file("test_config.json").unwrap();
    }

    #[test]
    fn test_write_file() {
        let config = ConfigManager {
            completion_model: "text-davinci-003".to_string(),
            chat_model: "gpt-3.5-turbo".to_string(),
            chat_base_prompt: "Test write".to_string(),
            max_tokens: 1024,
            image_size: "512x512".to_string(),
        };

        let path = Path::new("test_config_write.json");
        config.write_file(Some(path)).unwrap();
        let read_config = config.read_file(Some(path)).unwrap();
        assert_eq!(read_config.chat_base_prompt, "Test write");
        fs::remove_file("test_config_write.json").unwrap();
    }

    #[test]
    fn test_read_file_invalid_file() {
        fs::write("test_malformed_config.json", "invalid json content").unwrap();
        let config = ConfigManager::default();
        let path = Path::new("test_malformed_config.json");
        assert!(config.read_file(Some(path)).is_err());
        fs::remove_file("test_malformed_config.json").unwrap();
    }

    #[test]
    fn test_read_file_incomplete_structure() {
        let config_data = r#"
        {
            "completion_model": "text-davinci-003",
            "chat_model": "gpt-3.5-turbo",
            "max_tokens": 1024,
            "image_size": "512x512"
        }
        "#;
        fs::write("test_incomplete_config.json", config_data).unwrap();
        let config = ConfigManager::default();
        let path = Path::new("test_incomplete_config.json");
        assert!(config.read_file(Some(path)).is_err());
        fs::remove_file("test_incomplete_config.json").unwrap();
    }
}
