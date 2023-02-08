/* A library that returns back moon phase given day and location */
use reqwest;

pub fn get_moon_phase(day: u8, month: u8, year: u16, lat: f32, lon: f32) -> String {
    let url = format!(
        "https://api.usno.navy.mil/rstt/oneday?date={}/{}/{}&coords={},{}",
        month, day, year, lat, lon
    );
    let body = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let phase = body.split("Phase: ").collect::<Vec<&str>>()[1]
        .split("")
        .collect::<Vec<&str>>()[0];
    phase.to_string()
}
