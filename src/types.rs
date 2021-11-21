use telegram_bot_raw::{ChatId, Message, MessageId};

pub(crate) struct MessageIdentifier {
    pub(crate) id: MessageId,
    pub(crate) chat: ChatId,
}

impl MessageIdentifier {
    pub(crate) fn from_message(m: &Message) -> Self {
        Self {
            id: m.id,
            chat: m.chat.id(),
        }
    }
}
