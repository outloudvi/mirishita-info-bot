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

use callback_types::CallbackType;
use telegram::respond_callback_query;
use telegram_bot_raw::Update;
use worker::Request as WRequest;
use worker::*;

pub mod callback_types;
pub mod cmd;
pub mod constants;
pub mod handler;
pub mod matsurihi;
pub mod telegram;
pub mod utils;

/// The message handler.
async fn handle_message(msg: telegram_bot_raw::Message) -> Result<()> {
    use telegram_bot_raw::MessageKind::*;

    match &msg.kind {
        Text { data, .. } => {
            return handler::handler_text(data, &msg).await;
        }
        _ => {
            return Ok(());
        }
    }
}

/// The callbackQuery handler.
async fn handle_callback(cb_raw: telegram_bot_raw::CallbackQuery) -> Result<()> {
    if let None = cb_raw.data {
        // No data, skipping
        return Ok(());
    }
    let callback: CallbackType =
        bincode::deserialize(cb_raw.data.unwrap().as_bytes()).map_err(|e| e.to_string())?;
    handler::handler_callback(callback, cb_raw.from).await?;
    return Ok(());
}

/// The entrypoint to the script.
#[event(fetch)]
pub async fn main(req: WRequest, env: Env) -> worker::Result<Response> {
    utils::set_panic_hook();
    let router = Router::new();

    router
        .post_async("/1b248948646a", |mut req, _| async move {
            let tg_req = req.json::<Update>().await?;
            match tg_req.kind {
                telegram_bot_raw::UpdateKind::Message(msg) => {
                    if let Err(x) = handle_message(msg).await {
                        console_log!("Err: {}", x);
                    }
                }
                telegram_bot_raw::UpdateKind::CallbackQuery(cb) => {
                    if let Err(x) = handle_callback(cb.clone()).await {
                        console_log!("Err: {}", x);
                    } else {
                        respond_callback_query(&cb).await?;
                    }
                }
                _ => {}
            }
            Response::ok("ok")
        })
        .run(req, env)
        .await
}
