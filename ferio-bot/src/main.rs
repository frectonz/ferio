use ferio::{get_holidays, HolidayDate};
use std::error::Error;
use teloxide::{
    prelude::*,
    utils::{
        command::BotCommands,
        html::{bold, link},
    },
};

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "list holidays and observances being celebrated today")]
    Today,
    #[command(description = "send me holidays and observances everyday")]
    Start,
    #[command(description = "stop sending me holidays and observances")]
    Stop,
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Today => {
            let date = HolidayDate::Today;
            let holidays = get_holidays(&date).await?;
            let mut msg = format!(
                "There are {} holidays and observence on {}.\n",
                bold(&holidays.len().to_string()),
                bold(&date.get_date())
            );
            let holidays = holidays.iter().fold(String::new(), |mut acc, h| {
                acc.push('\n');
                acc.push_str(&link(&h.wikipedia_url, &h.get_greeting()));
                acc
            });
            msg.push_str(&holidays);
            bot.send_message(message.chat.id, msg)
                .parse_mode(teloxide::types::ParseMode::Html)
                .send()
                .await?
        }
        Command::Start => {
            bot.send_message(message.chat.id, "Start".to_string())
                .await?
        }
        Command::Stop => {
            bot.send_message(message.chat.id, "Stop".to_string())
                .await?
        }
    };

    Ok(())
}
