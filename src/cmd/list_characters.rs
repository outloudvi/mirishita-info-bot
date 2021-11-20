use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use telegram_bot_raw::{
    ChatId, EditMessageText, InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup,
    SendMessage, User,
};
use worker::{console_log, Result};

use crate::callback_types::CallbackType;
use crate::constants::{IDOL_ID_MAP, PAGE_SIZE};
use crate::matsurihi::{get_card, get_card_url, get_idol_cards};
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
                    &format!("{} [{}]", x.name, x.rarity),
                    serde_json::to_string(&CallbackType::IdolCard {
                        card_id: x.id,
                        with_annotation: true,
                        with_plus: true,
                    })
                    .unwrap(),
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
        match chat {
            Some(msg) => {
                let mut m = EditMessageText::new(msg.chat, msg.id, title);
                m.reply_markup(kbmarkup);
                respond_raw("editMessageText", &serde_json::to_string(&m)?).await?;
            }
            None => {
                let mut m = SendMessage::new(from, title);
                m.reply_markup(kbmarkup);
                respond_raw("sendMessage", &serde_json::to_string(&m)?).await?;
            }
        };
    }
    Ok(true)
}

#[derive(Serialize)]
pub(crate) struct SendPhotoItem {
    chat_id: ChatId,
    photo: String,
    caption: Option<String>,
    reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize)]
pub(crate) struct InputMedia {
    /// Must be "photo"
    #[serde(rename = "type")]
    typ: String,
    media: String,
    caption: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct EditMessageMedia {
    chat_id: String,
    message_id: String,
    media: InputMedia,
    reply_markup: Option<ReplyMarkup>,
}

/// Callback for /list_characters.
///
/// This shall be the step 4 (card display) of /list_characters.
pub(crate) async fn respond_step_4(
    card_id: u32,
    with_annotation: bool,
    with_plus: bool,
    chat: Option<MessageIdentifier>,
    from: User,
) -> Result<bool> {
    let card = get_card(card_id).await?;

    let mut kbmarkup = InlineKeyboardMarkup::new();
    let mut kbvec = vec![];

    // Plus or Plusless
    kbvec.push(InlineKeyboardButton::callback(
        format!("Toggle {}/{}+", card.rarity, card.rarity),
        serde_json::to_string(&CallbackType::IdolCard {
            card_id,
            with_annotation,
            with_plus: !with_plus,
        })
        .unwrap(),
    ));

    // Annotation / Annotationless
    kbvec.push(InlineKeyboardButton::callback(
        "Toggle annotation".to_string(),
        serde_json::to_string(&CallbackType::IdolCard {
            card_id,
            with_annotation: !with_annotation,
            with_plus,
        })
        .unwrap(),
    ));

    kbmarkup.add_row(kbvec);

    let card_url = get_card_url(&card.resource_id, with_plus, with_annotation);
    let caption = Some(format!(
        "{} [{}{}]",
        card.name,
        card.rarity,
        if with_plus { "+" } else { "" }
    ));

    match chat {
        Some(msg) => {
            // let mut m = EditMessageText::new(msg.chat, msg.id, title);
            // m.reply_markup(kbmarkup);
            let m = EditMessageMedia {
                chat_id: msg.chat.to_string(),
                message_id: msg.id.to_string(),
                media: InputMedia {
                    typ: "photo".to_string(),
                    media: card_url,
                    caption,
                },
                reply_markup: Some(kbmarkup.into()),
            };
            respond_raw("editMessageMedia", &serde_json::to_string(&m)?).await?;
        }
        None => {
            let photo = SendPhotoItem {
                chat_id: from.id.into(),
                photo: card_url,
                caption,
                reply_markup: Some(kbmarkup.into()),
            };
            respond_raw("sendPhoto", &serde_json::to_string(&photo)?).await?;
        }
    };

    Ok(true)
}
