//! Definitions for callbacks.
use std::ops::Not;

use serde::{Deserialize, Serialize};

use crate::cmd::list_characters::IdolCategory;

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
    },
}
