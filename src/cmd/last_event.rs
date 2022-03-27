use telegram_bot_raw::Message;
use worker::Result;

use super::last_borders::last_something;

/// ## /last_event
///
/// This command is used to display the latest event (not necessarily active).
pub(crate) async fn handler(command: &str, msg: &Message) -> Result<bool> {
    last_something(command, msg, false).await
}
