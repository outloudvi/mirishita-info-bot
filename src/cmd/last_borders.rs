use chrono::FixedOffset;
use telegram_bot_raw::{Message, SendMessage};
use worker::Result;

use crate::matsurihi::{get_current_event_ids, get_event, get_event_borders};
use crate::telegram::respond_raw;

/// Send event data.
/// Used by /{curr,last}_{event,borders}.
pub(crate) async fn send_event_data(
    with_border: bool,
    // negative -> the last x event
    // 0 -> current event
    // positive -> the #x event
    original_event_id: i32,
    msg: &Message,
) -> Result<bool> {
    let maybe_event_id = match original_event_id {
        i if i > 0 => Some(i as u32),
        i => {
            let mut curr_event_ids = get_current_event_ids().await?;
            curr_event_ids.sort_unstable();
            match curr_event_ids.last() {
                Some(curr) => Some((*curr as i32 + i) as u32),
                None => None,
            }
        }
    };

    let text = if maybe_event_id.is_none() {
        "No current event!".to_string()
    } else {
        let event_id = maybe_event_id.unwrap();
        let event_info = get_event(event_id).await?;
        let metrics = get_event_borders(event_id).await?;
        let mut ret = format!("<b>{}</b>\n", event_info.name);
        if metrics.is_none() {
            ret += "No metrics available";
        } else {
            let metrics = metrics.unwrap();
            let update_time = metrics.event_point.summary_time;
            let jst = FixedOffset::east(9 * 3600);
            ret += &format!("Updated: {}\n", update_time.with_timezone(&jst));
            if with_border {
                for k in metrics.event_point.scores {
                    if k.score.is_none() {
                        break;
                    }
                    ret += &format!("Rank #{}: {}\n", k.rank, k.score.unwrap().round());
                }
            }
            ret += &format!("Participants: {}\n", metrics.event_point.count);
            if !with_border {
                ret += &format!(
                    "<i>For score borders, use /{}</i>",
                    if original_event_id == 0 {
                        "/curr_borders".to_owned()
                    } else {
                        format!("/last_borders {}", event_id)
                    }
                );
            }
        }
        ret
    };
    let mut reply_msg = SendMessage::new(&msg.chat, text);
    reply_msg.parse_mode(telegram_bot_raw::ParseMode::Html);
    reply_msg.reply_to(msg);
    let reply_msg = serde_json::to_string(&reply_msg)?;
    respond_raw("sendMessage", &reply_msg).await?;
    Ok(true)
}

/// ## /last_borders
///
/// This command is used to display the score metrics for current event.
pub(crate) async fn handler(_: &str, msg: &Message) -> Result<bool> {
    send_event_data(true, -1, msg).await
}