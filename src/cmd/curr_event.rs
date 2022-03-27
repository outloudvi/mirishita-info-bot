//! ## /curr_event
//!
//! This command is used to display the current event.
//!
//! Use [`/last_event`](super::last_event) for historical event data.
use telegram_bot_raw::Message;
use worker::Result;

use super::last_borders::send_event_data;

pub(crate) async fn handler(_: &str, msg: &Message) -> Result<bool> {
    send_event_data(false, 0, msg).await
}
