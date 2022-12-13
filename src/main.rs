use teloxide::{prelude::*, utils::command::BotCommands};
use tg_gpt_bot::*; // Local

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    dotenv::dotenv().ok(); // from .env file
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
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
    #[command(description = "Send a prompt")]
    Prompt(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let responder = response::Response { bot, msg };
    match cmd {
        Command::Help => {
            responder
                .help_response(Command::descriptions().to_string())
                .await?;
        }
        Command::Source => {
            responder.source_response().await?;
        }
        Command::TestApi => {
            responder.test_api_response().await?;
        }
        Command::Prompt(prompt) => {
            responder.prompt_response(prompt).await?;
        }
    };
    Ok(())
}
