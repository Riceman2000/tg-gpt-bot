use super::open_ai_api::OpenAiApi;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use url::Url;

pub struct Response {
    pub bot: Bot,
    pub msg: Message,
}

impl Response {
    /// Send help message with the given text
    /// # Errors
    /// Telegram API failure
    pub async fn help(&self, disc: String) -> ResponseResult<()> {
        self.bot.send_message(self.msg.chat.id, disc).await?;
        Ok(())
    }

    /// Send the GitHub repo link
    /// # Errors
    /// Telegram API failure
    pub async fn source(&self) -> ResponseResult<()> {
        self.bot
            .send_message(
                self.msg.chat.id,
                "My source code can be found at: https://github.com/Riceman2000/tg-gpt-bot",
            )
            .await?;
        Ok(())
    }

    /// Test the open ai api
    /// # Errors
    /// Telegram API failure
    pub async fn test_api(&self) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();
        let response = match open_ai.test_connection().await {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API setup: {error}"),
        };
        self.bot.send_message(self.msg.chat.id, response).await?;
        Ok(())
    }

    /// Generate a chat response
    /// # Errors
    /// Telegram API failure
    pub async fn chat(&self, prompt: String) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();

        let chat_id = format!("{}", self.msg.chat.id);

        let response = match open_ai.chat(prompt, chat_id).await {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API call: {error}"),
        };

        self.bot.send_message(self.msg.chat.id, response).await?;
        Ok(())
    }

    /// Purge the chat history for a given chat ID
    /// # Errors
    /// Telegram API failure
    pub async fn chat_purge(&self, prompt: String) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();

        let chat_id = format!("{}", self.msg.chat.id);

        let response = match open_ai.chat_purge(&chat_id, &prompt) {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API call: {error}"),
        };

        self.bot.send_message(self.msg.chat.id, response).await?;
        Ok(())
    }

    /// Generate an image from a prompt, send via image URL
    /// # Errors
    /// Telegram API failure
    pub async fn image(&self, prompt: String) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();
        let response = match open_ai.image(prompt.clone()).await {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API call: {error}"),
        };

        // If the result is a properly formed URL, send it as an image
        match Url::parse(&response) {
            Ok(url) => {
                let file: InputFile = InputFile::url(url);
                self.bot.send_photo(self.msg.chat.id, file).await?;
                self.bot.send_message(self.msg.chat.id, prompt).await?;
            }
            Err(_) => {
                self.bot.send_message(self.msg.chat.id, response).await?;
            }
        };
        Ok(())
    }
}
