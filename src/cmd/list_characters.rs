use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use telegram_bot_raw::{
    EditMessageText, InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup, SendMessage,
    User,
};
use worker::{console_log, Result};

use crate::callback_types::CallbackType;
use crate::constants::{IDOL_ID_MAP, PAGE_SIZE};
use crate::matsurihi::get_idol_cards;
use crate::telegram::respond_raw;
use crate::MessageIdentifier;

/// These are the categories of idols.
#[derive(Clone, Hash, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum IdolCategory {
    NamukoPro = 0,
    PrincessStars = 1,
    FairyStars = 2,
    AngelStars = 3,
}

lazy_static! {
    /// This is a mapping between internal idol ID and category name.
    ///
    /// You can have all idol IDs at [`IDOL_ID_MAP`].
    pub(crate) static ref IDOL_CATEGORY_MAP: HashMap<IdolCategory, Vec<u32>> = {
        let mut m = HashMap::new();
        m.insert(IdolCategory::NamukoPro, vec![1,2,3,4,5,6,7,8,9,10,11,12,13]);
        m.insert(IdolCategory::PrincessStars, vec![14,17,19,21,26,27,28,29,30,32,36,37,43]);
        m.insert(IdolCategory::FairyStars, vec![15,20,25,31,33,34,38,44,46,47,49,50,51]);
        m.insert(IdolCategory::AngelStars, vec![16,18,22,23,24,35,39,40,41,42,45,48,52]);
        m
    };

    /// This is a mapping between category enum and category name.
    pub(crate) static ref IDOL_CATEGORY_NAMES: HashMap<IdolCategory, &'static str> = {
        let mut m = HashMap::new();
        m.insert(IdolCategory::NamukoPro, "765PRO Allstars");
        m.insert(IdolCategory::PrincessStars, "Princess Stars");
        m.insert(IdolCategory::FairyStars, "Fairy Stars");
        m.insert(IdolCategory::AngelStars, "Angel Stars");
        m
    };
}

fn find_idol_category(idol_id: &u32) -> Result<IdolCategory> {
    for (ic, ids) in IDOL_CATEGORY_MAP.iter() {
        if ids.contains(idol_id) {
            return Ok(ic.clone());
        }
    }
    Err(worker::Error::RustError("Idol not found".to_string()))
}

/// ## /list_characters
///
/// This command lists all characters for card lookup.
pub(crate) async fn handler(_: &str, msg: &Message) -> Result<bool> {
    let mut kbmarkup = InlineKeyboardMarkup::new();
    for i in [
        IdolCategory::NamukoPro,
        IdolCategory::PrincessStars,
        IdolCategory::FairyStars,
        IdolCategory::AngelStars,
    ] {
        let btn = InlineKeyboardButton::callback(
            IDOL_CATEGORY_NAMES.get(&i).unwrap(),
            serde_json::to_string(&CallbackType::ListIdolCategory(i.clone())).unwrap(),
        );

        kbmarkup.add_row(vec![btn]);
    }
    let mut reply_msg = SendMessage::new(&msg.chat, "Select a group...");
    reply_msg.reply_markup(kbmarkup);
    let reply_msg = serde_json::to_string(&reply_msg)?;
    respond_raw("sendMessage", &reply_msg).await?;
    Ok(true)
}

/// Callback for /list_characters.
///
/// This shall be the step 2 (character selection) of /list_characters.
pub(crate) async fn respond_step_2(idol_category: IdolCategory, from: User) -> Result<bool> {
    if let Some(cat) = IDOL_CATEGORY_MAP.get(&idol_category) {
        let kbmarkup = cat
            .chunks(3)
            .map(|iarr| {
                iarr.iter()
                    .map(|i| {
                        InlineKeyboardButton::callback(
                            IDOL_ID_MAP.get(i).unwrap(),
                            serde_json::to_string(&CallbackType::ListIdol {
                                idol_id: *i,
                                page_id: 1,
                            })
                            .unwrap(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .fold(InlineKeyboardMarkup::new(), |mut kbd, vek| {
                kbd.add_row(vek);
                kbd
            });

        let text = format!(
            "You've selected: {}\nNow select an idol...",
            IDOL_CATEGORY_NAMES.get(&idol_category).unwrap()
        );
        let mut reply_msg = SendMessage::new(&from, &text);
        reply_msg.reply_markup(kbmarkup);
        let reply_msg = serde_json::to_string(&reply_msg)?;
        respond_raw("sendMessage", &reply_msg).await?;
    }
    Ok(true)
}

/// This exists because
/// [`telegram_bot_raw::requests::edit_message_reply_markup::
/// EditMessageReplyMarkup`] is not up to date. We need to only indicate
/// inline_message_id, while this property just does not exist there.
///
/// https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize)]
pub(crate) struct OurEditMessageReplyMarkup {
    inline_message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Callback for /list_characters.
///
/// This shall be the step 3 (card selection) of /list_characters.
pub(crate) async fn respond_step_3(
    idol_id: u32,
    page_id: u32,
    chat: Option<MessageIdentifier>,
    from: User,
) -> Result<bool> {
    if page_id == 0 {
        return Err(worker::Error::RustError("Bad page_id".to_string()));
    }
    if let Some(idol_name) = IDOL_ID_MAP.get(&idol_id) {
        let cards = get_idol_cards(idol_id).await?;
        let len = cards.len();
        let page_from = (page_id as usize - 1) * PAGE_SIZE;

        let mut kbmarkup = cards
            .into_iter()
            .skip(page_from)
            .take(PAGE_SIZE)
            .map(|x| {
                vec![InlineKeyboardButton::callback(
                    x.name,
                    serde_json::to_string(&CallbackType::IdolCard(x.id)).unwrap(),
                )]
            })
            .fold(InlineKeyboardMarkup::new(), |mut kbd, ikb| {
                kbd.add_row(ikb);
                kbd
            });

        let mut pagination_row = vec![];
        if page_id > 1 {
            pagination_row.push(InlineKeyboardButton::callback(
                "Prev".to_string(),
                serde_json::to_string(&CallbackType::ListIdol {
                    idol_id,
                    page_id: page_id - 1,
                })
                .unwrap(),
            ));
        }

        let idol_category = find_idol_category(&idol_id)?;
        pagination_row.push(InlineKeyboardButton::callback(
            "Up".to_string(),
            serde_json::to_string(&CallbackType::ListIdolCategory(idol_category)).unwrap(),
        ));

        console_log!("from {} ~ SIZE {} * len {}", page_from, PAGE_SIZE, len);
        if page_id as usize * PAGE_SIZE < len {
            pagination_row.push(InlineKeyboardButton::callback(
                "Next".to_string(),
                serde_json::to_string(&CallbackType::ListIdol {
                    idol_id,
                    page_id: page_id + 1,
                })
                .unwrap(),
            ));
        }
        kbmarkup.add_row(pagination_row);

        let title = format!(
            "Showing cards of {}...\nPage {} / {}",
            idol_name,
            page_id,
            (len as f32 / PAGE_SIZE as f32).ceil() as usize
        );
        let reply_json = match chat {
            Some(msg) => {
                let mut m = EditMessageText::new(msg.chat, msg.id, title);
                m.reply_markup(kbmarkup);
                serde_json::to_string(&m)?
            }
            None => {
                let mut m = SendMessage::new(from, title);
                m.reply_markup(kbmarkup);
                serde_json::to_string(&m)?
            }
        };
        respond_raw("editMessageText", &reply_json).await?;
    }
    Ok(true)
}
