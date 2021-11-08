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

use crate::telegram::respond_text;
use telegram_bot_raw::Update;
use worker::Request as WRequest;
use worker::*;

pub mod cmd;
pub mod constants;
pub mod matsurihi;
pub mod telegram;
pub mod utils;

/// The message handler.
async fn handle_message(msg: telegram_bot_raw::Message) -> Result<()> {
    use telegram_bot_raw::MessageKind::Text;

    if let Text { ref data, .. } = msg.kind {
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
        } else {
            respond_text(&format!("Command not found: {}", data), &msg.chat).await?;
        }
    }

    Ok(())
}

#[event(fetch)]
pub async fn main(req: WRequest, env: Env) -> worker::Result<Response> {
    utils::set_panic_hook();
    let router = Router::new();

    router
        .post_async("/1b248948646a", |mut req, _| async move {
            let tg_req = req.json::<Update>().await?;
            if let telegram_bot_raw::UpdateKind::Message(msg) = tg_req.kind {
                if let Err(x) = handle_message(msg).await {
                    console_log!("Err: {}", x);
                }
            }
            Response::ok("ok")
        })
        .run(req, env)
        .await
}
