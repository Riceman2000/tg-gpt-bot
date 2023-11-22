use super::chat_history::{ChatHistory, MessageChat, Role};
use super::config_manager::ConfigManager;

use log::{debug, info, trace};
use std::env;

use anyhow::{anyhow, Result};

use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

use serde_derive::{Deserialize, Serialize};

pub struct OpenAiApi {
    client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    uri: String,
    auth_header: String,
}

// Default is to use the constructor always
impl Default for OpenAiApi {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAiApi {
    #[must_use]
    pub fn new() -> Self {
        if env::var("OPEN_AI_TOKEN").is_err() || env::var("OPEN_AI_URI").is_err() {
            dotenv::dotenv().expect("Failed to load env vars for API.");
        }

        let https: HttpsConnector<hyper::client::HttpConnector> = HttpsConnector::new();
        let client: Client<HttpsConnector<hyper::client::HttpConnector>> =
            Client::builder().build(https);

        let uri: String = env::var("OPEN_AI_URI").expect("Open AI URI not defined!");
        let token: String = env::var("OPEN_AI_TOKEN").expect("Open AI Token not defined!");

        let auth_header: String = format!("Bearer {token}");

        Self {
            client,
            uri,
            auth_header,
        }
    }

    async fn openai_post(&self, endpoint: &str, body: &str) -> Result<String> {
        let client = reqwest::Client::new();
        Ok(client
            .post(format!("{}/{endpoint}", self.uri))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .body(body.to_string())
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn test_connection(&self) -> Result<String> {
        info!(target: "api_events", "Test connection started.");
        // Ask for list of models to check auth

        let client = reqwest::Client::new();
        let json = client
            .get(format!("{}/models", self.uri))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await?
            .json::<ModelList>()
            .await?;

        // Format the number of models and return it
        let model_names: Vec<&str> = json.data.iter().map(|m| m.id.as_ref()).collect();
        let output = format!("Connection opened with {} models found!", model_names.len());
        debug!("Connection test output: {}", output);
        Ok(output)
    }

    pub async fn completion(&self, prompt: String) -> Result<String> {
        info!(target: "api_events", "Completion gen started.");
        debug!(target: "api_events", "Completion prompt: {}", prompt);
        if prompt.is_empty() {
            info!(target: "api_events", "No prompt, stopping.");
            return Ok("Prompt is empty, usage: '/text [PROMPT HERE]'".to_string());
        }
        // Grab info from config file
        let config = ConfigManager::new()?;

        let request_data = RequestCompletion {
            prompt,
            max_tokens: config.max_tokens,
            model: config.completion_model,
        };

        let body = serde_json::to_string(&request_data)?;
        trace!("Body: {body}");
        let response = self.openai_post("completions", &body).await?;
        trace!("Completion response: {response}");
        let json: ResponseCompletion = serde_json::from_str(&response)?;

        // Return only the first response
        let output = json.choices[0].text.clone();
        debug!("Completion output: {}", output);
        Ok(output)
    }

    pub async fn chat(&self, prompt: String, chat_id: String) -> Result<String> {
        info!(target: "api_events", "Chat gen started.");
        debug!(target: "api_events", "Chat prompt: {}", prompt);
        if prompt.is_empty() {
            info!(target: "api_events", "No prompt, stopping.");
            return Ok("Prompt is empty, usage: '/chat [PROMPT HERE]'".to_string());
        }

        let config = ConfigManager::new()?;

        // Get the message history from the user that called the command
        let mut history = ChatHistory::new(&chat_id)?;
        history = history.add_entry(&chat_id, &Role::User, &prompt)?;

        // Form the request struct and convert it to a https body in json
        let messages: Vec<MessageChat> = history.messages.clone();

        let request_data = RequestChat {
            model: config.chat_model,
            messages,
        };

        let body = Body::from(serde_json::to_vec(&request_data)?);

        // Make the request
        let request = Request::builder()
            .method("POST")
            .uri(format!("{}/chat/completions", self.uri))
            .header("Content-Type", "application/json")
            .header("Authorization", &self.auth_header)
            .body(body)?;

        // Send the request and get a response
        let result = self.client.request(request).await?;
        let body_bytes = hyper::body::to_bytes(result.into_body()).await?;
        let response_string = String::from_utf8(body_bytes.to_vec()).unwrap();
        trace!("Chat response: {}", response_string);

        // Serialize the response so we can pull out what we want
        let json: ResponseChat = serde_json::from_str(&response_string)?;
        let output = json.choices[0].message.content.clone();

        // Add the response back to the history
        history.add_entry(&chat_id, &Role::Assistant, &output)?;

        debug!("Chat output: {}", output);
        Ok(output)
    }

    pub fn chat_purge(&self, chat_id: &str, prompt: &str) -> Result<String> {
        info!(target: "api_events", "Chat purge started.");
        debug!(target: "api_events", "Chat purge prompt: {}", prompt);

        // Grab info from config file
        let history = ChatHistory::new(&chat_id)?;
        history.purge(&chat_id, &prompt)?;

        if prompt.is_empty() {
            Ok("Chat history purged without a custom prompt.".to_string())
        } else {
            Ok(format!("Chat history purged with prompt '{}'.", prompt))
        }
    }

    pub async fn image(&self, prompt: String) -> Result<String> {
        info!(target: "api_events", "Image gen started.");
        debug!(target: "api_events", "Image prompt: {}", prompt);
        if prompt.is_empty() {
            return Ok("Prompt is empty, usage: '/image [PROMPT HERE]'".to_string());
        }

        let config = ConfigManager::new()?;

        // Form the request struct and convert it to a https body in json
        let request_data = OpenAiRequestImage {
            prompt,
            n: 1,
            size: config.image_size,
        };
        let body = Body::from(serde_json::to_vec(&request_data)?);

        // Make the request
        let request = Request::builder()
            .method("POST")
            .uri(format!("{}/images/generations", self.uri))
            .header("Content-Type", "application/json")
            .header("Authorization", &self.auth_header)
            .body(body)?;

        // Send the request and get a response
        let result = self.client.request(request).await?;
        let body_bytes = hyper::body::to_bytes(result.into_body()).await?;
        let response_string = String::from_utf8(body_bytes.to_vec()).unwrap();
        trace!("Image response: {}", response_string);

        // Serialize the response so we can pull out what we want
        let json: ResponseImage = serde_json::from_str(&response_string)?;

        // If we get multiple urls just return the first one
        match json.data.iter().map(|d| d.url.to_string()).next() {
            Some(s) => Ok(s),
            None => Err(anyhow!("No output found.")),
        }
    }
}

// Structs for completion generation
#[derive(Deserialize, Debug)]
struct ChoicesCompletion {
    text: String,
}

#[derive(Deserialize, Debug)]
struct ResponseCompletion {
    choices: Vec<ChoicesCompletion>,
}

#[derive(Serialize, Debug)]
struct RequestCompletion {
    prompt: String,
    max_tokens: u32,
    model: String,
}

// Structs for chat generation
#[derive(Deserialize, Debug)]
struct ResponseChat {
    choices: Vec<ChoicesChat>,
}

#[derive(Deserialize, Debug)]
struct ChoicesChat {
    message: MessageChat,
}

#[derive(Serialize, Debug)]
struct RequestChat {
    model: String,
    messages: Vec<MessageChat>,
}

// Structs for image generation
#[derive(Deserialize, Debug)]
struct ChoicesImage {
    url: String,
}

#[derive(Deserialize, Debug)]
struct ResponseImage {
    data: Vec<ChoicesImage>,
}

#[derive(Serialize, Debug)]
struct OpenAiRequestImage {
    prompt: String,
    n: u8,
    size: String,
}

// Structs for getting a list of text models
#[derive(Deserialize, Debug)]
struct ModelList {
    data: Vec<Model>,
}

#[derive(Deserialize, Debug)]
struct Model {
    id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_env_vars() {
        match dotenv::dotenv() {
            Ok(_) => debug!("Loaded .env file"),
            Err(error) => panic!("Failed to load .env: {:?}", error),
        }; // from .env file

        if env::var("OPEN_AI_TOKEN")
            .expect("OPEN_AI_TOKEN load failed")
            .is_empty()
        {
            panic!("OPEN_AI_TOKEN is empty");
        }

        if env::var("OPEN_AI_URI")
            .expect("OPEN_AI_URI load failed")
            .is_empty()
        {
            panic!("OPEN_AI_URI is empty");
        }
    }

    #[test]
    fn test_new_openaiapi() {
        let openai_api = OpenAiApi::new();
        assert!(!openai_api.uri.is_empty() && !openai_api.auth_header.is_empty());
    }

    #[tokio::test]
    async fn test_test_connection() {
        let openai_api = OpenAiApi::new();
        let response = openai_api.test_connection().await.unwrap();
        assert!(response.starts_with("Connection opened with"));
    }

    #[tokio::test]
    async fn test_completion_prompt_not_empty() {
        let openai_api = OpenAiApi::new();
        let prompt = String::from("test prompt");
        let response = openai_api.completion(prompt.clone()).await;
        assert!(response.is_ok(), "Error: {:?}", response.err());
    }

    #[tokio::test]
    async fn test_completion_prompt_empty() {
        let openai_api = OpenAiApi::new();
        let prompt = String::new();
        let response = openai_api.completion(prompt.clone()).await.unwrap();
        assert_eq!(response, "Prompt is empty, usage: '/text [PROMPT HERE]'");
    }

    #[tokio::test]
    async fn test_chat_prompt_not_empty() {
        let openai_api = OpenAiApi::new();
        let prompt = String::from("Hello!");
        let chat_id = String::from("test_chat_id");
        let response = openai_api
            .chat(prompt.clone(), chat_id.clone())
            .await
            .unwrap();
        assert!(!response.is_empty());
    }

    #[tokio::test]
    async fn test_chat_prompt_empty() {
        let openai_api = OpenAiApi::new();
        let prompt = String::new();
        let chat_id = String::from("test_chat_id");
        let response = openai_api
            .chat(prompt.clone(), chat_id.clone())
            .await
            .unwrap();
        assert_eq!(response, "Prompt is empty, usage: '/chat [PROMPT HERE]'");
    }

    #[tokio::test]
    async fn test_chat_purge_with_prompt() {
        let openai_api = OpenAiApi::new();
        let prompt = String::from("test prompt");
        let chat_id = String::from("test_purge_with_prompt");
        let response = openai_api.chat_purge(&chat_id, &prompt).unwrap();
        assert_eq!(response, "Chat history purged with prompt 'test prompt'.");
        let history = ChatHistory::new(&chat_id).unwrap();
        assert_eq!(history.messages[0].role, "system");
        assert_eq!(history.messages[0].content, "test prompt");
    }

    #[tokio::test]
    async fn test_chat_purge_without_prompt() {
        let openai_api = OpenAiApi::new();
        let prompt = String::new();
        let chat_id = String::from("test_purge_without_prompt");
        let response = openai_api.chat_purge(&chat_id, &prompt).unwrap();
        assert_eq!(response, "Chat history purged without a custom prompt.");
    }

    #[tokio::test]
    async fn test_image_prompt_not_empty() {
        let openai_api = OpenAiApi::new();
        let prompt = String::from("test prompt");
        let response = openai_api.image(prompt.clone()).await.unwrap();
        assert!(!response.is_empty());
    }

    #[tokio::test]
    async fn test_image_prompt_empty() {
        let openai_api = OpenAiApi::new();
        let prompt = String::new();
        let response = openai_api.image(prompt.clone()).await;
        assert!(response.is_ok(), "Error: {:?}", response.err());
        assert_eq!(
            response.unwrap(),
            "Prompt is empty, usage: '/image [PROMPT HERE]'"
        );
    }
}
