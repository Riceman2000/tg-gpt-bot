use super::open_ai_api::*;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use url::Url;

pub struct Response {
    pub bot: Bot,
    pub msg: Message,
}

impl Response {
    pub async fn help(&self, disc: String) -> ResponseResult<()> {
        self.bot.send_message(self.msg.chat.id, disc).await?;
        Ok(())
    }

    pub async fn source(&self) -> ResponseResult<()> {
        self.bot
            .send_message(
                self.msg.chat.id,
                "My source code can be found at: https://github.com/Riceman2000/tg-gpt-bot",
            )
            .await?;
        Ok(())
    }

    pub async fn test_api(&self) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();
        let response = match open_ai.test_connection().await {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API setup: {error}"),
        };
        self.bot.send_message(self.msg.chat.id, response).await?;
        Ok(())
    }

    pub async fn completion(&self, prompt: String) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();
        let response = match open_ai.completion(prompt).await {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API call: {error}"),
        };

        self.bot.send_message(self.msg.chat.id, response).await?;
        Ok(())
    }

    pub async fn chat(&self, prompt: String) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();
        let response = match open_ai.chat(prompt).await {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API call: {error}"),
        };

        self.bot.send_message(self.msg.chat.id, response).await?;
        Ok(())
    }

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
