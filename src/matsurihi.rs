use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use worker::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSchedule {
    begin_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    page_begin_date: DateTime<Utc>,
    page_end_date: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct Event {
    id: u32,
    name: String,
    #[serde(rename = "type")]
    typ: u32,
    schedule: EventSchedule,
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
    console_log!("len: {}", ret.len());
    Ok(ret)
}
