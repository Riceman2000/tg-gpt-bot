use super::config_manager::ConfigManager;

use anyhow::Result;

use log::{debug, warn};

use serde_derive::{Deserialize, Serialize};

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
        let config = match ConfigManager::new() {
            Ok(c) => c,
            Err(e) => panic!("Failed to get config with {e}"),
        };
        ChatHistory {
            messages: vec![MessageChat {
                role: "system".to_string(),
                content: config.chat_base_prompt,
            }],
        }
    }
}

impl ChatHistory {
    /// Process a new message, will create a new chat or add to an existing one
    /// # Errors
    /// OS file write errors
    pub fn new(chat_id: &str) -> Result<Self> {
        let serialized_data = match ChatHistory::read_file(chat_id) {
            Ok(data) => data,
            Err(error) => {
                warn!("Using default values due to error in reading history file: {error}");
                ChatHistory::default()
            }
        };

        serialized_data.write_file(chat_id)?;

        Ok(serialized_data)
    }

    /// Add an entry to the selected `chat_id` with the given role
    /// Also writes to the history file
    /// # Errors
    /// OS file write errors
    pub fn add_entry(mut self, chat_id: &str, role: &Role, content: &str) -> Result<Self> {
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

    /// Wipes a chat history, if the user provides a system prompt it will be used otherwise it
    /// will use the system prompt in the system config file
    /// # Errors
    /// OS file write errors
    pub fn purge(mut self, chat_id: &str, prompt: &str) -> Result<Self> {
        let init_prompt = if prompt.is_empty() {
            let config = ConfigManager::new()?;
            config.chat_base_prompt
        } else {
            prompt.to_string()
        };

        debug!("Init prompt: {}", init_prompt);

        self.messages = vec![MessageChat {
            role: "system".to_string(),
            content: init_prompt.to_string(),
        }];

        debug!("Post-purge struct: {:?}", self);

        self.write_file(chat_id)?;
        Ok(self)
    }

    fn read_file(chat_id: &str) -> Result<Self> {
        // If the chat history directory does not exist make it
        if !Path::new("./chat-history").is_dir() {
            create_dir("./chat-history")?;
        }
        let user_history_file = format!("chat-history/{chat_id}-history.json");
        let path = Path::new(&user_history_file);

        let mut file = File::open(path)?;

        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;

        let serialized_data: ChatHistory = serde_json::from_str(&json_string)?;

        Ok(serialized_data)
    }

    fn write_file(&self, chat_id: &str) -> Result<()> {
        let user_history_file = format!("chat-history/{chat_id}-history.json");
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
