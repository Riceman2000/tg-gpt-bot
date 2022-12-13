use super::config_manager::*;
use hyper::body::Buf;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::error; // Local

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
    pub fn new() -> Self {
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

    pub async fn test_connection(&self) -> Result<String, Box<dyn error::Error>> {
        // Ask for list of models to check auth
        let request = Request::builder()
            .uri(format!("{}/models", self.uri))
            .header("Authorization", &self.auth_header)
            .body(Body::from(""))
            .unwrap();

        // Send the request and collect the response
        let result = self.client.request(request).await?;
        let response_body = hyper::body::aggregate(result).await?;
        let json: OpenAiModelList = serde_json::from_reader(response_body.reader())?;

        // Format the number of models and return it
        let model_names: Vec<&str> = json.data.iter().map(|m| m.id.as_ref()).collect();
        Ok(format!(
            "Connection opened with {} models found!",
            model_names.len()
        ))
    }

    pub async fn text(&self, prompt: String) -> Result<String, Box<dyn error::Error>> {
        if prompt.is_empty() {
            return Ok("Prompt is empty, usage: '/text [PROMPT HERE]'".to_string());
        }

        // Grab info from config file
        let config = ConfigManager::new();

        // Form the request struct and convert it to a https body in json
        let request_data = OpenAiRequestText {
            prompt,
            max_tokens: config.max_tokens,
            model: config.text_model,
        };
        let body = Body::from(serde_json::to_vec(&request_data)?);

        // Make the request
        let request = Request::builder()
            .method("POST")
            .uri(format!("{}/completions", self.uri))
            .header("Content-Type", "application/json")
            .header("Authorization", &self.auth_header)
            .body(body)?;

        // Send the request and get a response
        let result = self.client.request(request).await?;
        let response_body = hyper::body::aggregate(result).await?;

        // Serialize the response so we can pull out what we want
        let json: OpenAiResponseText = serde_json::from_reader(response_body.reader())?;

        // Return only the text response
        Ok(json.choices[0].text.clone())
    }

    pub async fn image(&self, prompt: String) -> Result<String, Box<dyn error::Error>> {
        if prompt.is_empty() {
            return Ok("Prompt is empty, usage: '/image [PROMPT HERE]'".to_string());
        }

        // Grab info from config file
        let config = ConfigManager::new();

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
        let response_body = hyper::body::aggregate(result).await?;

        // Serialize the response so we can pull out what we want
        let json: OpenAiResponseImage = serde_json::from_reader(response_body.reader())?;

        // If we get multiple urls just return the first one
        let output: Vec<String> = json.data.iter().map(|d| d.url.to_string()).collect();
        Ok(output[0].clone())
    }
}

// Structs for text generation
#[derive(Deserialize, Debug)]
struct OpenAiChoicesText {
    text: String,
}

#[derive(Deserialize, Debug)]
struct OpenAiResponseText {
    choices: Vec<OpenAiChoicesText>,
}

#[derive(Serialize, Debug)]
struct OpenAiRequestText {
    prompt: String,
    max_tokens: u32,
    model: String,
}

// Structs for image generation
#[derive(Deserialize, Debug)]
struct OpenAiChoicesImage {
    url: String,
}

#[derive(Deserialize, Debug)]
struct OpenAiResponseImage {
    data: Vec<OpenAiChoicesImage>,
}

#[derive(Serialize, Debug)]
struct OpenAiRequestImage {
    prompt: String,
    n: u8,
    size: String,
}

// Structs for getting a list of text models
#[derive(Deserialize, Debug)]
struct OpenAiModelList {
    data: Vec<OpenAiModel>,
}

#[derive(Deserialize, Debug)]
struct OpenAiModel {
    id: String,
}
