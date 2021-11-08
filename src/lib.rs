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

use crate::matsurihi::{get_current_event_ids, get_event, get_event_borders};
use crate::telegram::respond_text;
use telegram_bot_raw::Update;
use worker::Request as WRequest;
use worker::*;

mod cmd;
mod constants;
mod matsurihi;
mod telegram;
mod utils;

/// The message handler.
async fn handle_message(msg: telegram_bot_raw::Message) -> Result<()> {
    use telegram_bot_raw::MessageKind::Text;

    if let Text { ref data, .. } = msg.kind {
        let data = data.trim();
        if data.starts_with("/ping") {
            respond_text("Hi!", &msg.chat).await?;
            return Ok(());
        } else if data.starts_with("/last_event") {
            let ret = matsurihi::get_events().await?;
            if ret.is_empty() {
                respond_text("No events found", &msg.chat).await?;
                return Ok(());
            }
            let last_event = ret.last().unwrap();
            respond_text(&format!("{}", last_event), &msg.chat).await?;
        } else if data.starts_with("/curr_event") {
            let now = chrono::offset::Utc::now();
            let ret = matsurihi::get_events().await?;
            let curr_events = ret
                .into_iter()
                .filter(|x| x.schedule.begin_date <= now && x.schedule.end_date >= now)
                .collect::<Vec<_>>();
            if curr_events.is_empty() {
                respond_text("No current event!", &msg.chat).await?;
            } else {
                respond_text(
                    &curr_events
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("\n"),
                    &msg.chat,
                )
                .await?;
            }
        } else if data.starts_with("/curr_borders") {
            let mut curr_event_ids = get_current_event_ids().await?;
            if curr_event_ids.is_empty() {
                respond_text("No current event!", &msg.chat).await?;
                return Ok(());
            }
            curr_event_ids.sort_unstable();
            let curr_event_id = curr_event_ids.last().unwrap();
            let event_info = get_event(*curr_event_id).await?;
            let metrics = get_event_borders(*curr_event_id).await?;
            let mut ret = format!("**{}**\n", event_info.name);
            for k in metrics.event_point.scores {
                if k.score.is_none() {
                    break;
                }
                ret += &format!("Rank #{}: {}\n", k.rank, k.score.unwrap().round());
            }
            ret += &format!("Participants: {}", metrics.event_point.count);
            respond_text(&ret, &msg.chat).await?;
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

/// The function bound to "fetch" event.
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
