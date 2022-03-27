//! ## /curr_borders
//!
//! This command is used to display the score metrics for current event.
//!
//! Use [`/last_borders`](super::last_borders) for historical event data.
use telegram_bot_raw::Message;
use worker::Result;

use super::last_borders::send_event_data;

pub(crate) async fn handler(_: &str, msg: &Message) -> Result<bool> {
    send_event_data(true, 0, msg).await
}
