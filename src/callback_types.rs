use crate::cmd::list_characters::IdolCategory;
use serde::{Deserialize, Serialize};

/// This is the one used to determine callback.
/// Callback is limited to 64 bytes so the size should be well controlled.
/// Currently `serde` will take care of everything here.
#[derive(Serialize, Deserialize)]
pub enum CallbackType {
    #[serde(rename = "LIC")]
    ListIdolCategory(IdolCategory),
    #[serde(rename = "LI")]
    ListIdol(u32),
}
