use url::Url;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn parse_buttons(text: &str) -> Option<(String, InlineKeyboardMarkup)> {
    let mut splited_text: Vec<_> = text.split("\n").collect();
    if !text.contains("\n\n") {
        return Some((text.to_string(), InlineKeyboardMarkup::default()));
    }
    let mut buttons = splited_text.pop().unwrap().to_string();
    buttons.pop();
    let text: String = splited_text.join("\n");
    let mut keyboard = InlineKeyboardMarkup::default();

    println!("{:?}", buttons.split("]").collect::<Vec<_>>());
    for (i, button) in buttons.split("]").enumerate() {
        let mut button = button.split(" [");
        let button_text = button.next()?.trim();
        let button_url = match Url::parse(button.next().unwrap_or("")) {
            Ok(v) => v,
            Err(_) => {
                return Some((text, InlineKeyboardMarkup::default()))
            },
        };
        keyboard = keyboard.append_to_row(i/3, InlineKeyboardButton::url(button_text, button_url));
    }
    Some((text, keyboard))
}
