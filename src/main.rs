use std::env;
use teloxide::{prelude::*, utils::command::BotCommands};
use tg_gpt_bot::response;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // from .env file

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    assert!(
        env::var("TELOXIDE_TOKEN").is_ok(),
        "Environment variable TELOXIDE_TOKEN not found"
    );

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;

    println!("Bot closed...");
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Display a link to my source code.")]
    Source,
    #[command(description = "Test API connection by fetching a list of models from OpenAI")]
    TestApi,
    #[command(description = "Chat with Chat-GPT, chats are persistent for each group/DM")]
    Chat(String),
    #[command(description = "Reset Chat-GPT's conversation. Optionally include a system prompt.")]
    ChatPurge(String),
    #[command(description = "Send a prompt to generate an image")]
    Image(String),
    #[command(description = "Play some skill games")]
    Gamble(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let responder = response::Response { bot, msg };
    match cmd {
        Command::Help => {
            responder.help(Command::descriptions().to_string()).await?;
        }
        Command::Source => {
            responder.source().await?;
        }
        Command::TestApi => {
            responder.test_api().await?;
        }
        Command::Chat(prompt) => {
            responder.chat(prompt).await?;
        }
        Command::ChatPurge(prompt) => {
            responder.chat_purge(prompt).await?;
        }
        Command::Image(prompt) => {
            responder.image(prompt).await?;
        }
        Command::Gamble(prompt) => {
            responder.gamble(prompt).await?;
        }
    };
    Ok(())
}
