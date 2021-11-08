use crate::matsurihi::{get_current_event_ids, get_event, get_event_borders};
use crate::telegram::respond_text;
use telegram_bot_raw::Message;
use worker::Result;

/// ## /curr_event
///
/// This command is used to display the current event.
pub async fn handler(_: &str, msg: &Message) -> Result<bool> {
    let mut curr_event_ids = get_current_event_ids().await?;
    if curr_event_ids.is_empty() {
        respond_text("No current event!", &msg.chat).await?;
        return Ok(true);
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
    Ok(true)
}
