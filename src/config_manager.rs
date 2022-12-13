use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigManager {
    pub text_model: String,
    pub max_tokens: u32,
}

// Default config values
impl Default for ConfigManager {
    fn default() -> Self {
        ConfigManager{
            text_model: "text-davinci-003".to_string(),
            max_tokens: 32,
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
            Err(error) => panic!("Cannot write config data: {error}")
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

    pub fn write_file(&self) -> Result<(), Box<dyn error::Error>>{ 
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
