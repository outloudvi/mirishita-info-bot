use crate::{cmd, telegram::respond_text};
use telegram_bot_raw::Message;
use worker::Result;

/// Handler for all text messages.
pub(crate) async fn handler_text(data: &str, msg: &Message) -> Result<()> {
    let data = data.trim();
    if data.starts_with("/ping") {
        respond_text("Hi!", &msg.chat).await?;
        return Ok(());
    } else if data.starts_with("/last_event") {
        let ret = cmd::handler__last_event(data, &msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        return Ok(());
    } else if data.starts_with("/curr_event") {
        let ret = cmd::handler__curr_event(data, &msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        return Ok(());
    } else if data.starts_with("/curr_borders") {
        let ret = cmd::handler__curr_borders(data, &msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        return Ok(());
    } else if data.starts_with("/card") {
        let ret = cmd::handler__card(data, &msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        return Ok(());
    } else if data.starts_with("/list_characters") {
        let ret = cmd::handler__list_characters(data, &msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        return Ok(());
    } else {
        respond_text(&format!("Command not found: {}", data), &msg.chat).await?;
        return Ok(());
    };
}
