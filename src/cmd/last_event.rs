use telegram_bot_raw::Message;
use worker::Result;

use super::last_borders::send_event_data;

/// ## /last_event
///
/// This command is used to display the latest event (not necessarily active).
pub(crate) async fn handler(_: &str, msg: &Message) -> Result<bool> {
    send_event_data(false, -1, msg).await
}
