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

        let serialized_data = match default_values.read_file() {
            Ok(data) => data,
            Err(error) => {
                println!("Using default values due to error in reading config file: {error}");
                default_values
            }
        };

        match serialized_data.write_file() {
            Ok(_) => (),
            Err(error) => panic!("Cannot write config data: {error}"),
        };

        serialized_data
    }

    pub fn read_file(&self) -> Result<Self, Box<dyn error::Error>> {
        let path = Path::new("config.json");

        let mut file = File::open(path)?;

        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;

        let serialized_data: ConfigManager = serde_json::from_str(&json_string)?;

        Ok(serialized_data)
    }

    pub fn write_file(&self) -> Result<(), Box<dyn error::Error>> {
        let path = Path::new("config.json");

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
