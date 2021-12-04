use telegram_bot_raw::Message;
use worker::Result;

use super::curr_borders::send_curr_event_data;

/// ## /curr_event
///
/// This command is used to display the current event.
pub(crate) async fn handler(_: &str, msg: &Message) -> Result<bool> {
    send_curr_event_data(false, msg).await
}
