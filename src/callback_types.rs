use crate::cmd::list_characters::IdolCategory;
use serde::{Deserialize, Serialize};

/// This is the one used to determine callback.
/// Callback is limited to 64 bytes so the size should be well controlled.
///
/// Currently `serde` will take care of everything here.
#[derive(Serialize, Deserialize)]
pub enum CallbackType {
    /// Listing idols in a category.
    /// See [`crate::cmd::list_characters::respond_step_2`] for the logic.
    #[serde(rename = "LIC")]
    ListIdolCategory(IdolCategory),

    /// Listing cards for an idol.
    #[serde(rename = "LI")]
    ListIdol(u32),
}
