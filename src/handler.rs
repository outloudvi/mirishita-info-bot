//! Various handlers.

use telegram_bot_raw::Message;
use worker::Result;

use crate::callback_types::CallbackType;
use crate::cmd;
use crate::cmd::list_characters::{respond_step_2, respond_step_3, respond_step_4};
use crate::telegram::{can_edit_photo, respond_text};
use crate::types::MessageIdentifier;

/// Handler for all callback items.
///
/// @return Ok, or the error message.
pub(crate) async fn handler_callback(cb: CallbackType, om: Message) -> Result<()> {
    let msg_id = MessageIdentifier::from_message(&om);
    match cb {
        // Edit a message, w/o photo
        CallbackType::ListIdolCategory(idol_cat) => match respond_step_2(idol_cat, msg_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        // Edit a message, w/o photo
        CallbackType::ListIdol { idol_id, page_id } => {
            match respond_step_3(idol_id, page_id, msg_id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        // Send a new message or edit a message, w/ photo
        CallbackType::IdolCard {
            card_id,
            with_annotation,
            with_plus,
        } => match respond_step_4(
            card_id,
            with_annotation,
            with_plus,
            msg_id,
            can_edit_photo(&om),
        )
        .await
        {
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
        Ok(())
    } else if data.starts_with("/last_event") {
        let ret = cmd::handler__last_event(data, msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        Ok(())
    } else if data.starts_with("/curr_event") {
        let ret = cmd::handler__curr_event(data, msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        Ok(())
    } else if data.starts_with("/curr_borders") {
        let ret = cmd::handler__curr_borders(data, msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        Ok(())
    } else if data.starts_with("/card") {
        let ret = cmd::handler__card(data, msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        Ok(())
    } else if data.starts_with("/list_characters") {
        let ret = cmd::handler__list_characters(data, msg).await?;
        if !ret {
            respond_text("Bad command usage", &msg.chat).await?;
        }
        Ok(())
    } else {
        respond_text(&format!("Command not found: {}", data), &msg.chat).await?;
        Ok(())
    }
}
