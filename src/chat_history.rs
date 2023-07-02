use super::config_manager::*;
use log::{debug, error};
use serde_derive::{Deserialize, Serialize};
use std::error;
use std::fs::{create_dir, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatHistory {
    pub messages: Vec<MessageChat>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageChat {
    pub role: String,
    pub content: String,
}

pub enum Role {
    User,
    System,
    Assistant,
}

// Default config values
impl Default for ChatHistory {
    fn default() -> Self {
        let config = ConfigManager::new();
        ChatHistory {
            messages: vec![MessageChat {
                role: "system".to_string(),
                content: config.chat_base_prompt,
            }],
        }
    }
}

impl ChatHistory {
    pub fn new(chat_id: &String) -> Self {
        let default_values = ChatHistory::default();

        let serialized_data = match default_values.read_file(chat_id) {
            Ok(data) => data,
            Err(error) => {
                error!("Using default values due to error in reading history file: {error}");
                default_values
            }
        };

        match serialized_data.write_file(chat_id) {
            Ok(_) => (),
            Err(error) => panic!("Cannot write history data: {error}"),
        };

        serialized_data
    }

    pub fn add_entry(
        mut self,
        chat_id: &String,
        role: &Role,
        content: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        let role_string = match role {
            Role::User => "user".to_string(),
            Role::System => "system".to_string(),
            Role::Assistant => "assistant".to_string(),
        };

        self.messages.push(MessageChat {
            role: role_string,
            content: content.to_string(),
        });

        self.write_file(chat_id)?;
        Ok(self)
    }

    pub fn purge(
        mut self,
        chat_id: &String,
        prompt: &String,
    ) -> Result<Self, Box<dyn error::Error>> {
        let init_prompt = if prompt.is_empty() {
            let config = ConfigManager::new();
            config.chat_base_prompt
        } else {
            prompt.clone()
        };

        debug!("Init prompt: {}", init_prompt);

        self.messages = vec![MessageChat {
            role: "system".to_string(),
            content: init_prompt,
        }];

        debug!("Post-purge struct: {:?}", self);

        self.write_file(chat_id)?;
        Ok(self)
    }

    fn read_file(&self, chat_id: &String) -> Result<Self, Box<dyn error::Error>> {
        // If the chat history directory does not exist make it
        if !Path::new("./chat-history").is_dir() {
            create_dir("./chat-history")?;
        }
        let user_history_file = format!("chat-history/{}-history.json", chat_id);
        let path = Path::new(&user_history_file);

        let mut file = File::open(path)?;

        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;

        let serialized_data: ChatHistory = serde_json::from_str(&json_string)?;

        Ok(serialized_data)
    }

    fn write_file(&self, chat_id: &String) -> Result<(), Box<dyn error::Error>> {
        let user_history_file = format!("chat-history/{}-history.json", chat_id);
        let path = Path::new(&user_history_file);

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
