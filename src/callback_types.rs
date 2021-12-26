//! Definitions for callbacks.
use std::ops::Not;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::cmd::list_characters::IdolCategory;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum BgStatus {
    None = 0,
    No = 1,
    Yes = 2,
}

impl BgStatus {
    fn default() -> Self {
        BgStatus::None
    }
}

/// This is the one used to determine callback.
/// Callback is limited to 64 bytes so the size should be well controlled.
///
/// Currently `serde` will take care of everything here.
#[derive(Serialize, Deserialize)]
pub(crate) enum CallbackType {
    /// Listing idols in a category.
    /// See [`crate::cmd::list_characters::respond_step_2`] for the logic.
    #[serde(rename = "LIC")]
    ListIdolCategory {
        #[serde(rename = "c")]
        category: IdolCategory,
        #[serde(rename = "f")]
        #[serde(skip_serializing_if = "Not::not")]
        #[serde(default)]
        force_new_msg: bool,
    },

    /// Listing cards for an idol.
    #[serde(rename = "LI")]
    ListIdol {
        #[serde(rename = "i")]
        idol_id: u32,
        #[serde(rename = "p")]
        page_id: u32,
    },

    /// Indicating a card.
    #[serde(rename = "IC")]
    IdolCard {
        #[serde(rename = "c")]
        card_id: u32,
        #[serde(rename = "a")]
        with_annotation: bool,
        #[serde(rename = "p")]
        with_plus: bool,
        #[serde(rename = "b", default = "BgStatus::default")]
        bg: BgStatus,
    },
}
