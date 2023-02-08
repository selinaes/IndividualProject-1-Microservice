/* A library that returns back moon phase given day and location */

use chrono::{DateTime, Utc};
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MoonPhasesData {
    apiversion: String,
    day: i32,
    month: i32,
    numphases: i32,
    phasedata: Vec<MoonPhase>,
    year: i32,
}

#[derive(Deserialize, Debug)]
struct MoonPhase {
    day: i32,
    month: i32,
    phase: String,
    time: String,
    year: i32,
}

pub async fn get_moon_phase(date: DateTime<Utc>) -> Result<String, reqwest::Error> {

    let moon_phases_data: MoonPhasesData = reqwest::get(&format!(
        "https://aa.usno.navy.mil/api/moon/phases/date?date={}&nump=48",
        date.format("%Y-%m-%d")
    ))
        .await?
        .json::<MoonPhasesData>()
        .await?;


    Ok(moon_phases_data.phasedata[0].phase)
}
