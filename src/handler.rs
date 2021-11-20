//! Various handlers.

use telegram_bot_raw::{Message, User};
use worker::Result;

use crate::callback_types::CallbackType;
use crate::cmd::list_characters::{respond_step_2, respond_step_3, respond_step_4};
use crate::telegram::respond_text;
use crate::{cmd, MessageIdentifier};

/// Handler for all callback items.
pub(crate) async fn handler_callback(
    cb: CallbackType,
    chat: Option<MessageIdentifier>,
    from: User,
) -> Result<()> {
    match cb {
        CallbackType::ListIdolCategory(idol_cat) => match respond_step_2(idol_cat, from).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        CallbackType::ListIdol { idol_id, page_id } => {
            match respond_step_3(idol_id, page_id, chat, from).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        CallbackType::IdolCard {
            card_id,
            with_annotation,
            with_plus,
        } => match respond_step_4(card_id, with_annotation, with_plus, chat, from).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
    }
}

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
