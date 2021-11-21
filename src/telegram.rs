//! The part used to communicate with Telegram.
use serde::Serialize;
use telegram_bot_raw::{CallbackQuery, CallbackQueryId, MessageChat, SendMessage};
use worker::wasm_bindgen::JsValue;
use worker::Result;

use crate::constants::BOT_TOKEN;
use crate::utils::{escape, send_raw_request, send_request};

/// Send text to a chat.
///
/// It makes use of [`telegram_bot_raw::SendMessage`].
pub async fn respond_text(text: &str, chat: &MessageChat) -> Result<()> {
    let mut msg = SendMessage::new(chat, escape(text));
    msg.parse_mode(telegram_bot_raw::ParseMode::MarkdownV2);
    let req = telegram_bot_raw::Request::serialize(&msg).map_err(|e| e.to_string())?;
    send_request(req).await
}

#[derive(Serialize)]
struct ImageMessageBody {
    chat_id: String,
    photo: String,
    caption: String,
}

/// Send an image to a chat.
///
/// It does not make use of [`telegram_bot_raw::SendPhoto`], because workers-rs
/// does not support sending requests with [`worker::FormData`] yet. ([cloudflare/workers-rs#79](https://github.com/cloudflare/workers-rs/issues/79))
pub async fn respond_img(url: &str, caption: &str, chat: &MessageChat) -> Result<()> {
    let body = ImageMessageBody {
        photo: url.to_string(),
        chat_id: chat.id().to_string(),
        caption: caption.to_string(),
    };
    send_raw_request(
        &format!("https://api.telegram.org/bot{}/sendPhoto", BOT_TOKEN),
        JsValue::from_str(&serde_json::to_string(&body)?),
    )
    .await
}

#[derive(Serialize)]
pub struct AnswerCallbackQuery {
    pub callback_query_id: CallbackQueryId,
}

/// Respond to callback queries.
pub async fn respond_callback_query(query: &CallbackQuery) -> Result<()> {
    let body = AnswerCallbackQuery {
        callback_query_id: query.id.clone(),
    };
    Ok(respond_raw("answerCallbackQuery", &serde_json::to_string(&body)?).await?)
}

/// Send a raw response.
pub async fn respond_raw(method: &str, body: &str) -> Result<()> {
    send_raw_request(
        &format!("https://api.telegram.org/bot{}/{}", BOT_TOKEN, method),
        JsValue::from_str(body),
    )
    .await
}
