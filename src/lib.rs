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

use telegram::respond_callback_query;
use telegram_bot_raw::Update;
use worker::{
    console_log, event, wasm_bindgen, wasm_bindgen_futures, worker_sys, Request as WRequest, Result,
};

pub(crate) mod callback_types;
pub(crate) mod cmd;
pub mod constants;
pub(crate) mod handler;
pub mod matsurihi;
pub mod telegram;
pub(crate) mod types;
pub mod utils;

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
    let query_id = cb_raw.id;

    if cb_raw.data.is_none() {
        // No data, skipping
        return Ok(());
    }

    let callback_result =
        serde_json::from_str(&cb_raw.data.clone().unwrap()).map_err(|e| e.to_string());

    // If cannot decode callback...
    if callback_result.is_err() {
        respond_callback_query(
            query_id,
            Some("Cannot extract intent. Please start again from the commands.".to_string()),
            true,
        )
        .await?;
        return Ok(());
    }

    // We need the original message (& chat) to reply
    let maybe_original_message = match cb_raw.message {
        Some(mocp) => {
            if let telegram_bot_raw::MessageOrChannelPost::Message(m) = mocp {
                Some(m)
            } else {
                None
            }
        }
        None => None,
    };

    match maybe_original_message {
        Some(om) => {
            if let Err(e) = handler::handler_callback(callback_result.unwrap(), om).await {
                // Have original message, but error somewhere inside
                respond_callback_query(query_id, Some(e.to_string()), true).await?;
            } else {
                // Success
                respond_callback_query(query_id, None, false).await?;
            }
        }
        None => {
            respond_callback_query(
                query_id,
                Some("Cannot find original message or chat.".to_string()),
                false,
            )
            .await?;
        }
    };

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
                    }
                }
                _ => {}
            };
            Response::ok("ok")
        })
        .run(req, env)
        .await
}
