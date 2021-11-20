use crate::constants::IDOL_ID_MAP;
use crate::matsurihi::get_card;
use crate::{matsurihi::get_card_url, telegram::respond_img};
use telegram_bot_raw::Message;
use worker::Result;

/// ## /card
///
/// This command is used to display a card.
///
/// This command has the following signatures in matching preference:
/// * /card - A random card
/// * /card [id:int] - A card of id [id]
/// * /card [idol:str] - A random card from [idol]
/// * /card [cardAssetId:str] - A card of assetId [cardAssetId]
pub(crate) async fn handler(command: &str, msg: &Message) -> Result<bool> {
    let splits = command.trim().split(' ').collect::<Vec<_>>();
    if splits.len() != 2 {
        return Ok(false);
    }
    let target = splits[1];

    // /card [id]
    if let Ok(id) = target.parse::<u32>() {
        let card = get_card(id).await?;
        respond_img(
            &get_card_url(&card.resource_id, true, true),
            &card.name,
            &msg.chat,
        )
        .await?;
        return Ok(true);
    }

    for (_k, v) in IDOL_ID_MAP.iter() {
        if v == &target {
            // /card [idol]
            // TODO
            return Ok(true);
        }
    }

    // /card [cardAssetId]
    let url = get_card_url(target, true, true);
    respond_img(&url, &url, &msg.chat).await?;
    Ok(true)
}
