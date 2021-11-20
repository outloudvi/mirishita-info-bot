//! # mirishita_info_bot
//!
//! This is a Telegram bot written with the help of [workers-rs](https://github.com/cloudflare/workers-rs)
//! and [telegram-bot-raw](https://lib.rs/crates/telegram-bot-raw).
//!
//! The data source is Matsurihi.me's [Princess](https://api.matsurihi.me/docs/).
//!
//! You can play with the bot at Telegram [@mirishita_info_bot](https://t.me/@mirishita_info_bot).
//!
//! It may also bring about some inspiration for anyone who want to run a
//! telegram bot written with Rust on Cloudflare Workers.
//!
//! For bot command documentaion, see [`cmd`].

use telegram::{respond_callback_query, respond_text};
use telegram_bot_raw::{ChatRef, MessageId, ToChatRef, Update};
use worker::{
    console_log, event, wasm_bindgen, wasm_bindgen_futures, worker_sys, Request as WRequest, Result,
};

pub(crate) mod callback_types;
pub(crate) mod cmd;
pub mod constants;
pub(crate) mod handler;
pub mod matsurihi;
pub mod telegram;
pub mod utils;

pub(crate) struct MessageIdentifier {
    id: MessageId,
    chat: ChatRef,
}

/// The message handler.
async fn handle_message(msg: telegram_bot_raw::Message) -> Result<()> {
    use telegram_bot_raw::MessageKind::*;

    match &msg.kind {
        Text { data, .. } => handler::handler_text(data, &msg).await,
        _ => Ok(()),
    }
}

/// The callbackQuery handler.
async fn handle_callback(cb_raw: telegram_bot_raw::CallbackQuery) -> Result<()> {
    if cb_raw.data.is_none() {
        // No data, skipping
        return Ok(());
    }
    let callback_result = serde_json::from_str(&cb_raw.data.unwrap()).map_err(|e| e.to_string());
    if callback_result.is_err() {
        let chat = cb_raw.message.and_then(|x| match x {
            telegram_bot_raw::MessageOrChannelPost::Message(m) => Some(m.chat),
            telegram_bot_raw::MessageOrChannelPost::ChannelPost(_) => None,
        });
        if chat.is_none() {
            // Bad data & nowhere to notify
            return Ok(());
        }
        respond_text(
            "Message data expired. Please try using the commands.",
            &chat.unwrap(),
        )
        .await?;
        return Ok(());
    }
    handler::handler_callback(
        callback_result.unwrap(),
        cb_raw.message.and_then(|x| match x {
            telegram_bot_raw::MessageOrChannelPost::Message(m) => Some(MessageIdentifier {
                id: m.id,
                chat: m.chat.to_chat_ref(),
            }),
            telegram_bot_raw::MessageOrChannelPost::ChannelPost(_) => None,
        }),
        cb_raw.from,
    )
    .await?;
    Ok(())
}

/// The entrypoint to the script.
#[event(fetch)]
pub async fn main(req: WRequest, env: worker::Env) -> worker::Result<worker::Response> {
    use worker::*;

    utils::set_panic_hook();
    let router = Router::new();

    router
        .post_async("/1b248948646a", |mut req, _| async move {
            let tg_req = req.json::<Update>().await?;
            match tg_req.kind {
                telegram_bot_raw::UpdateKind::Message(msg) => {
                    if let Err(x) = handle_message(msg).await {
                        console_log!("Message handling error: {}", x);
                    }
                }
                telegram_bot_raw::UpdateKind::CallbackQuery(cb) => {
                    if let Err(x) = handle_callback(cb.clone()).await {
                        console_log!("Callback handling error: {}", x);
                    } else {
                        respond_callback_query(&cb).await?;
                    }
                }
                _ => {}
            };
            Response::ok("ok")
        })
        .run(req, env)
        .await
}
