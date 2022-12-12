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
    #[command(description = "display this text.")]
    Help,
    #[command(description = "simple test")]
    Test(i32),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let responder = response::Response { bot, msg };
    match cmd {
        Command::Help => {
            responder.help_response(Command::descriptions().to_string()).await?;
        }
        Command::Test(input) => {
            responder.test_response(input).await?;
        }
    };
    Ok(())
}
