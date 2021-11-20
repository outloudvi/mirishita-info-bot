//! Definitions for callbacks.

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
    ListIdolCategory(IdolCategory),

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
    IdolCard(u32),
}