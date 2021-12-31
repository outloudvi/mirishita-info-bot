use telegram_bot_raw::{Message, SendMessage};
use worker::Result;

use crate::matsurihi::get_events;
use crate::telegram::respond_raw;

/// ## /last_event
///
/// This command is used to display the latest event (not necessarily active).
pub(crate) async fn handler(_: &str, msg: &Message) -> Result<bool> {
    let ret = get_events().await?;
    let text = match ret.last() {
        Some(evt) => format!("{}", evt),
        None => "No events found".to_string(),
    };

    let mut reply_msg = SendMessage::new(&msg.chat, text);
    reply_msg.reply_to(msg);
    reply_msg.parse_mode(telegram_bot_raw::ParseMode::Html);
    let reply_msg = serde_json::to_string(&reply_msg)?;
    respond_raw("sendMessage", &reply_msg).await?;
    Ok(true)
}
