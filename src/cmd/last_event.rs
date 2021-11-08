use crate::matsurihi::get_events;
use crate::telegram::respond_text;
use telegram_bot_raw::Message;
use worker::Result;

/// ## /last_event
///
/// This command is used to display the latest event (not necessarily active).
pub async fn handler(_: &str, msg: &Message) -> Result<bool> {
    let ret = get_events().await?;
    if ret.is_empty() {
        respond_text("No events found", &msg.chat).await?;
        return Ok(true);
    }
    let last_event = ret.last().unwrap();
    respond_text(&format!("{}", last_event), &msg.chat).await?;
    Ok(true)
}
