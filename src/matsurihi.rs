use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use worker::*;

#[derive(Deserialize)]
pub struct ScoreItem {
    pub rank: u32,
    pub score: Option<f32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointMetrics {
    pub scores: Vec<ScoreItem>,
    summary_time: DateTime<Utc>,
    pub count: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventBorderView {
    pub event_point: PointMetrics,
    high_score: PointMetrics,
    lounge_point: PointMetrics,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSchedule {
    pub begin_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    page_begin_date: DateTime<Utc>,
    page_end_date: DateTime<Utc>,
    boost_begin_date: Option<DateTime<Utc>>,
    boost_end_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct Event {
    id: u32,
    pub name: String,
    #[serde(rename = "type")]
    typ: u32,
    pub schedule: EventSchedule,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**{}**\n", self.name)?;
        write!(f, "Starts: {}\n", self.schedule.begin_date)?;
        write!(f, "Ends: {}\n", self.schedule.end_date)?;
        Ok(())
    }
}

pub async fn get_events() -> Result<Vec<Event>> {
    let ret = Fetch::Url(Url::parse("https://api.matsurihi.me/mltd/v1/events")?)
        .send()
        .await?
        .json::<Vec<Event>>()
        .await?;
    Ok(ret)
}

pub async fn get_event(event_id: u32) -> Result<Event> {
    let ret = Fetch::Url(Url::parse(&format!(
        "https://api.matsurihi.me/mltd/v1/events/{}",
        event_id
    ))?)
    .send()
    .await?
    .json::<Event>()
    .await?;
    Ok(ret)
}

pub async fn get_event_borders(event_id: u32) -> Result<EventBorderView> {
    let ret = Fetch::Url(Url::parse(&format!(
        "https://api.matsurihi.me/mltd/v1/events/{}/rankings/borderPoints",
        event_id
    ))?)
    .send()
    .await?
    .json::<EventBorderView>()
    .await?;
    Ok(ret)
}

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
