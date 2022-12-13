# tg-gpt-bot
Telegram bot written in Rust to access the GPT API.

## Core components
- [Teloxide](https://github.com/teloxide/teloxide)
- [OpenAI API](https://beta.openai.com/docs/api-reference/introduction)

## Self-host quickstart
- Clone the repository
- [Install rustup](https://rustup.rs/) and/or update your Rust install
- Create a `.env` file in the root of the repository and have it follow this format:
```
TELOXIDE_TOKEN=[TELEGRAM BOT API TOKEN]
OPEN_AI_TOKEN=[OPEN AI API TOKEN]
OPEN_AI_URI=https://api.openai.com/v1
```
  - Get the Telegram API key from the @BotFather bot on Telegram
  - Get the OpenAI API key from an OpenAI account
    - NOTE: Large scale usage costs money, there is a free trial but do not let this bot out into a large croud unless you are ready to pay
  - The URI listed here will work, only change it if you know what you are doing
- Run the program using `cargo run`
- Test on Telegram by starting a conversation with the bot and sending it `/help`
- After your first run of the program a `config.json` file will be generated, this file can be edited while the bot is running to change it's operating parameters

## Still need help?
- Submit an issue and I will get back to you ASAP!

## Interested in contributing?
- Pick an issue or make your own for a feature you'd like to see added and submit a pull request! I am open to new ideas!
- Please follow these guidelines before submitting a pull request:
  - Run `cargo fmt` before submittal
  - Resolve any and all Clippy warnings before submittal, do not mute any warnings without first discussing it in the issue page for your particular pull-request.

Thanks!
