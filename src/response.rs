use teloxide::prelude::*;

pub struct Response {
    pub bot: Bot,
    pub msg: Message
}

impl Response{ 
    pub async fn help_response(&self, disc: String) -> ResponseResult<()> {
        self.bot.send_message(self.msg.chat.id, disc).await?;
        Ok(())
    }
    
    pub async fn test_response(&self, input: i32) -> ResponseResult<()> {
        let resp = format!("{} x 2 = {}", input, input*2);
        self.bot.send_message(self.msg.chat.id, resp).await?;
        Ok(())
    }
}
