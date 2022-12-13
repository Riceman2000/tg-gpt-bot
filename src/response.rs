use super::open_ai_api::*;
use teloxide::prelude::*; // Local

pub struct Response {
    pub bot: Bot,
    pub msg: Message,
}

impl Response {
    pub async fn help_response(&self, disc: String) -> ResponseResult<()> {
        self.bot.send_message(self.msg.chat.id, disc).await?;
        Ok(())
    }

    pub async fn source_response(&self) -> ResponseResult<()> {
        self.bot
            .send_message(
                self.msg.chat.id,
                "My source code can be found at: https://github.com/Riceman2000/tg-gpt-bot",
            )
            .await?;
        Ok(())
    }

    pub async fn test_api_response(&self) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();
        let response = match open_ai.test_connection().await {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API setup: {error}"),
        };
        self.bot.send_message(self.msg.chat.id, response).await?;
        Ok(())
    }

    pub async fn prompt_response(&self, prompt: String) -> ResponseResult<()> {
        let open_ai = OpenAiApi::new();
        let response = match open_ai.prompt(prompt).await {
            Ok(resp_string) => resp_string,
            Err(error) => format!("Error during API call: {error}"),
        };

        self.bot.send_message(self.msg.chat.id, response).await?;
        Ok(())
    }
}
