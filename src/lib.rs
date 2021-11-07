use telegram_bot_raw::{MessageChat, Request, SendMessage, Update};
use utils::{escape, send_request};
use worker::Request as WRequest;
use worker::*;

use crate::matsurihi::{get_current_event_ids, get_event, get_event_borders};

mod constants;
mod matsurihi;
mod utils;

async fn respond_text(text: &str, chat: MessageChat) -> Result<()> {
    let mut msg = SendMessage::new(chat, escape(text));
    msg.parse_mode(telegram_bot_raw::ParseMode::MarkdownV2);
    let req = msg.serialize().map_err(|e| e.to_string())?;
    send_request(req).await
}

async fn handle_message(msg: telegram_bot_raw::Message) -> Result<()> {
    use telegram_bot_raw::MessageKind::Text;

    if let Text { data, .. } = msg.kind {
        let data = data.trim();
        if data.starts_with("/ping") {
            respond_text("Hi!", msg.chat).await?;
            return Ok(());
        }
        if data.starts_with("/last_event") {
            let ret = matsurihi::get_events().await?;
            if ret.len() == 0 {
                respond_text("No events found", msg.chat).await?;
                return Ok(());
            }
            let last_event = ret.last().unwrap();
            respond_text(&format!("{}", last_event), msg.chat).await?;
            return Ok(());
        }
        if data.starts_with("/curr_event") {
            let now = chrono::offset::Utc::now();
            let ret = matsurihi::get_events().await?;
            let curr_events = ret
                .into_iter()
                .filter(|x| x.schedule.begin_date <= now && x.schedule.end_date >= now)
                .collect::<Vec<_>>();
            if curr_events.len() == 0 {
                respond_text(&"No current event!", msg.chat).await?;
            } else {
                respond_text(
                    &curr_events
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("\n"),
                    msg.chat,
                )
                .await?;
            }
            return Ok(());
        }
        if data.starts_with("/curr_borders") {
            let mut curr_event_ids = get_current_event_ids().await?;
            if curr_event_ids.len() == 0 {
                respond_text(&"No current event!", msg.chat).await?;
                return Ok(());
            }
            curr_event_ids.sort();
            let curr_event_id = curr_event_ids.last().unwrap();
            let event_info = get_event(*curr_event_id).await?;
            let metrics = get_event_borders(*curr_event_id).await?;
            let mut ret = format!("**{}**\n", event_info.name);
            for k in metrics.event_point.scores {
                if let None = k.score {
                    break;
                }
                ret += &format!("Rank #{}: {}\n", k.rank, k.score.unwrap().round());
            }
            ret += &format!("Participants: {}", metrics.event_point.count);
            respond_text(&ret, msg.chat).await?;
            return Ok(());
        }
        respond_text(&format!("Command not found: {}", data), msg.chat).await?;
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
            match tg_req.kind {
                telegram_bot_raw::UpdateKind::Message(msg) => {
                    if let Err(x) = handle_message(msg).await {
                        console_log!("Err: {}", x);
                    }
                }
                _ => {}
            }
            Response::ok("ok")
        })
        .run(req, env)
        .await
}
