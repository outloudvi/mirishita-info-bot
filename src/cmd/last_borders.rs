//! ## /last_borders
//!
//! This command is used to display the score metrics for current event.
//!
//! This command (and [`/last_event`](super::last_event)) accepts the following
//! types of inputs:
//! * `/last_borders` - Borders for last event
//! * `/last_borders -1` - Borders for last event (== `/last_borders`)
//! * `/last_borders 0` - Borders for current event (if any)
//! * `/last_borders 3` - Borders for event #3
//! * `/last_borders -2` - Borders for the previous of last event
use chrono::FixedOffset;
use telegram_bot_raw::{Message, SendMessage};
use worker::Result;

use crate::matsurihi::{get_current_event_ids, get_event, get_event_borders, get_events};
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
        i => 'blk: {
            let mut curr_event_ids = get_current_event_ids().await?;
            curr_event_ids.sort_unstable();
            if curr_event_ids.last().is_some() {
                break 'blk Some((*curr_event_ids.last().unwrap() as i32 + i) as u32);
            }
            if i == 0 {
                // No current event
                break 'blk None;
            }
            let events = get_events().await?;
            let last_event_id = events
                .into_iter()
                .map(|ev| ev.id)
                .reduce(|a, b| a.max(b))
                .unwrap_or(0);
            Some(((last_event_id as i32) + 1 + i) as u32)
        }
    };

    let text = if maybe_event_id.is_none() {
        "No current event!".to_string()
    } else {
        let event_id = maybe_event_id.unwrap();
        let maybe_event_info = get_event(event_id).await;
        match maybe_event_info {
            Ok(event_info) => {
                let metrics = get_event_borders(event_id).await?;
                let mut ret = format!("<b>{}</b> (#{})\n", event_info.name, event_info.id);
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
                            "<i>For score borders, use {}</i>",
                            if original_event_id == 0 {
                                "/curr_borders".to_owned()
                            } else {
                                format!("/last_borders {}", event_id)
                            }
                        );
                    }
                }
                ret
            }
            Err(_) => "Event not found!".to_string(),
        }
    };
    let mut reply_msg = SendMessage::new(&msg.chat, text);
    reply_msg.parse_mode(telegram_bot_raw::ParseMode::Html);
    reply_msg.reply_to(msg);
    let reply_msg = serde_json::to_string(&reply_msg)?;
    respond_raw("sendMessage", &reply_msg).await?;
    Ok(true)
}

pub(crate) async fn last_something(
    command: &str,
    msg: &Message,
    with_border: bool,
) -> Result<bool> {
    let splits = command.trim().split(' ').collect::<Vec<_>>();
    let event_id = match splits.get(1) {
        Some(x) => match x.parse::<i32>() {
            Ok(v) => v,
            Err(_) => -1,
        },
        None => -1,
    };
    send_event_data(with_border, event_id, msg).await
}

pub(crate) async fn handler(command: &str, msg: &Message) -> Result<bool> {
    last_something(command, msg, true).await
}
