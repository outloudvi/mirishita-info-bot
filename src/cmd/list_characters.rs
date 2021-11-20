use crate::{callback_types::CallbackType, telegram::respond_raw};
use lazy_static::lazy_static;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use telegram_bot_raw::{InlineKeyboardButton, InlineKeyboardMarkup, Message, SendMessage};
use worker::Result;

#[derive(Clone, Hash, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IdolCategory {
    NamukoPro = 0,
    PrincessStars = 1,
    FairyStars = 2,
    AngelStars = 3,
}

lazy_static! {
    /// This is a mapping between internal idol ID and category name.
    pub static ref IDOL_CATEGORY_MAP: HashMap<IdolCategory, Vec<u8>> = {
        let mut m = HashMap::new();
        m.insert(IdolCategory::NamukoPro, vec![1,2,3,4,5,6,7,8,9,10,11,12,13]);
        m.insert(IdolCategory::PrincessStars, vec![14,17,19,21,26,27,28,29,30,32,36,37,43]);
        m.insert(IdolCategory::FairyStars, vec![15,20,25,31,33,34,38,44,46,47,49,50,51]);
        m.insert(IdolCategory::AngelStars, vec![16,18,22,23,24,32,35,39,40,41,42,45,48,52]);
        m
    };

    /// This is a mapping between category enum and category name.
    pub static ref IDOL_CATEGORY_NAMES: HashMap<IdolCategory, &'static str> = {
        let mut m = HashMap::new();
        m.insert(IdolCategory::NamukoPro, "765PRO Allstars");
        m.insert(IdolCategory::PrincessStars, "Princess Stars");
        m.insert(IdolCategory::FairyStars, "Fairy Stars");
        m.insert(IdolCategory::AngelStars, "Angel Stars");
        m
    };
}

/// ## /list_characters
///
/// This command lists all characters for card lookup.
pub async fn handler(_: &str, msg: &Message) -> Result<bool> {
    let mut kbmarkup = InlineKeyboardMarkup::new();
    for i in [
        IdolCategory::NamukoPro,
        IdolCategory::PrincessStars,
        IdolCategory::FairyStars,
        IdolCategory::AngelStars,
    ] {
        let cbt = CallbackType::ListIdolCategory(i.clone());
        let dec = bincode::serialize(&cbt).unwrap();
        let dst = std::str::from_utf8(&dec).unwrap();
        let btn = InlineKeyboardButton::callback(IDOL_CATEGORY_NAMES.get(&i).unwrap(), dst);

        kbmarkup.add_row(vec![btn]);
    }
    let mut reply_msg = SendMessage::new(&msg.chat, "Select a group...");
    reply_msg.reply_markup(kbmarkup);
    let reply_msg = serde_json::to_string(&reply_msg)?;
    respond_raw("sendMessage", &reply_msg).await?;
    Ok(true)
}
