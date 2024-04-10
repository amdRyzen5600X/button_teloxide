extern crate telegram_new_features;

use telegram_new_features::parse_buttons;
use teloxide::{payloads::SendMessageSetters, prelude::*};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("starting bot...");
    dotenv().ok();

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let (text, keyboard) = parse_buttons(
            msg.text()
            .unwrap_or("send a text, not documetn, sticker and etc.")
        )
            .unwrap();

        bot.send_message(
            msg.chat.id, 
            text
        )
            .reply_markup(keyboard)
            .await?;
        Ok(())
    })
    .await;
}
