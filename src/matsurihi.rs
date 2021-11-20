//! This is the bindings and typings from Matsurihime.
//!
//! Here is onlay a part of the APIs and schemas. Read [Princess API Docs](https://api.matsurihi.me/docs/) for details.
use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::Display;
use worker::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SkillItem {
    id: u32,
    description: String,
    effect_id: u8,
    evaluation: u8,
    evaluation2: u8,
    duration: u32,
    interval: u32,
    probability: u32,
    value: Vec<u32>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ConsumeItem {
    id: u32,
    name: String,
    description: String,
    resource_id: String,
    model_id: String,
    sort_id: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CenterEffectItem {
    id: u32,
    description: String,
    idol_type: u8,
    specific_idol_type: Option<u8>,
    attribute: u8,
    value: u32,
    song_type: Option<u8>,
    attribute2: Option<u32>,
    value2: Option<u32>,
}

#[derive(Serialize_repr, Deserialize_repr, Display)]
#[repr(u8)]
pub enum Rarity {
    N = 1,
    R = 2,
    SR = 3,
    SSR = 4,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CardItem {
    pub id: u32,
    pub name: String,
    idol_id: u32,
    pub resource_id: String,
    pub rarity: Rarity,
    event_id: Option<u32>,
    category: String,
    extra_type: u32,
    costume: Option<ConsumeItem>,
    bonus_costume: Option<ConsumeItem>,
    rank5_costune: Option<ConsumeItem>,
    flavor_text: String,
    flavor_text_awakened: String,
    level_max: u32,
    level_max_awakened: u32,
    center_effect: Option<CenterEffectItem>,
    center_effect_name: String,
    skill: Option<Vec<SkillItem>>,
    // Different from docs
    skill_name: String,
    add_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct ScoreItem {
    pub rank: u32,
    pub score: Option<f32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointMetrics {
    pub scores: Vec<ScoreItem>,
    #[allow(dead_code)]
    summary_time: DateTime<Utc>,
    pub count: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventBorderView {
    pub event_point: PointMetrics,
    #[allow(dead_code)]
    high_score: PointMetrics,
    #[allow(dead_code)]
    lounge_point: PointMetrics,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSchedule {
    pub begin_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    #[allow(dead_code)]
    page_begin_date: DateTime<Utc>,
    #[allow(dead_code)]
    page_end_date: DateTime<Utc>,
    #[allow(dead_code)]
    boost_begin_date: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    boost_end_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct Event {
    id: u32,
    pub name: String,
    /// See `EVENT_TYPE_MAP` in constants for the mapping relationships.
    #[serde(rename = "type")]
    #[allow(dead_code)]
    typ: u32,
    pub schedule: EventSchedule,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "**{}**", self.name)?;
        writeln!(f, "Starts: {}", self.schedule.begin_date)?;
        writeln!(f, "Ends: {}", self.schedule.end_date)?;
        Ok(())
    }
}

/// Get all events.
pub async fn get_events() -> Result<Vec<Event>> {
    let ret = Fetch::Url(Url::parse(
        "https://api.matsurihi.me/mltd/v1/events?prettyPrint=false",
    )?)
    .send()
    .await?
    .json::<Vec<Event>>()
    .await?;
    Ok(ret)
}

/// Get an event by its ID.
pub async fn get_event(event_id: u32) -> Result<Event> {
    let ret = Fetch::Url(Url::parse(&format!(
        "https://api.matsurihi.me/mltd/v1/events/{}?prettyPrint=false",
        event_id
    ))?)
    .send()
    .await?
    .json::<Event>()
    .await?;
    Ok(ret)
}

/// Get the metrics for an event by its ID.
pub async fn get_event_borders(event_id: u32) -> Result<EventBorderView> {
    let ret = Fetch::Url(Url::parse(&format!(
        "https://api.matsurihi.me/mltd/v1/events/{}/rankings/borderPoints?prettyPrint=false",
        event_id
    ))?)
    .send()
    .await?
    .json::<EventBorderView>()
    .await?;
    Ok(ret)
}

/// Get the IDs for all ongoing events.
pub async fn get_current_event_ids() -> Result<Vec<u32>> {
    let now = chrono::Utc::now();
    let evts = get_events().await?;
    let ret = evts
        .into_iter()
        .filter(|x| x.schedule.begin_date <= now && x.schedule.end_date >= now)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| x.id)
        .collect();
    Ok(ret)
}

/// Get a card's URL.
pub fn get_card_url(card_res_id: &str, plus: bool, with_annotation: bool) -> String {
    format!(
        "https://storage.matsurihi.me/mltd/card/{}_{}_{}.png",
        card_res_id,
        if plus { "1" } else { "0" },
        if with_annotation { "a" } else { "b" }
    )
}

/// Get a card background's URL.
#[allow(dead_code)]
pub fn get_card_bg_url(card_res_id: &str, plus: bool) -> String {
    format!(
        "https://storage.matsurihi.me/mltd/card_bg/{}_{}.png",
        card_res_id,
        if plus { "1" } else { "0" },
    )
}

/// Get the metadata for a card.
pub async fn get_card(card_id: u32) -> Result<CardItem> {
    let mut ret = Fetch::Url(Url::parse(&format!(
        "https://api.matsurihi.me/mltd/v1/cards/{}?prettyPrint=false",
        card_id
    ))?)
    .send()
    .await?
    .json::<Vec<CardItem>>()
    .await?;
    if ret.is_empty() {
        Err(worker::Error::RustError("No card found".to_string()))
    } else {
        Ok(ret.remove(0))
    }
}

/// Get cards for an idol.
pub async fn get_idol_cards(idol_id: u32) -> Result<Vec<CardItem>> {
    let ret = Fetch::Url(Url::parse(&format!(
        "https://api.matsurihi.me/mltd/v1/cards?prettyPrint=false&idolId={}",
        idol_id
    ))?)
    .send()
    .await?
    .json::<Vec<CardItem>>()
    .await?;
    Ok(ret)
}
