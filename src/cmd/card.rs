use crate::constants::IDOL_ID_MAP;
use crate::{matsurihi::get_card_url, telegram::respond_img};
use telegram_bot_raw::Message;
use worker::Result;

/// /card
///
/// This command is used to spawn a card interface.
/// This command has the following signatures in matching preference:
///
/// * /card - A random card
/// * /card [idol:str] - A random card from [idol]
/// * /card [cardAssetId:str] - A card of Id [cardAssetId]
pub async fn handler(command: &str, msg: &Message) -> Result<bool> {
    let splits = command.trim().split(" ").collect::<Vec<_>>();
    if splits.len() != 2 {
        return Ok(false);
    }
    let target = splits[1];
    for (k, v) in IDOL_ID_MAP.iter() {
        if v == &target {
            // TODO
            return Ok(true);
        }
    }
    // cardId
    let url = get_card_url(target, true, true);
    respond_img(&url, &url, &msg.chat).await?;
    return Ok(true);
}
