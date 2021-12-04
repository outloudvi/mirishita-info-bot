use chrono::FixedOffset;
use telegram_bot_raw::{Message, SendMessage};
use worker::Result;

use crate::matsurihi::{get_current_event_ids, get_event, get_event_borders};
use crate::telegram::respond_raw;

/// Send event data.
/// Used by both /curr_event and /curr_borders.
pub(crate) async fn send_curr_event_data(with_border: bool, msg: &Message) -> Result<bool> {
    let mut curr_event_ids = get_current_event_ids().await?;
    let text = if curr_event_ids.is_empty() {
        "No current event!".to_string()
    } else {
        curr_event_ids.sort_unstable();
        let curr_event_id = curr_event_ids.last().unwrap();
        let event_info = get_event(*curr_event_id).await?;
        let metrics = get_event_borders(*curr_event_id).await?;
        let mut ret = format!("<b>{}</b>\n", event_info.name);
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
            ret += "<i>For score borders, use /curr_borders</i>";
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

/// ## /curr_borders
///
/// This command is used to display the score metrics for current event.
pub(crate) async fn handler(_: &str, msg: &Message) -> Result<bool> {
    send_curr_event_data(true, msg).await
}
