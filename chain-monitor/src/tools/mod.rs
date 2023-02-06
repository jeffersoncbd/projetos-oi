use chrono::{DateTime, TimeZone, Utc};

pub mod chains;
pub mod csv;
pub mod database;

pub fn string_to_static(value: String) -> &'static str {
    let value: &'static str = Box::leak(value.into_boxed_str());
    value
}

pub fn str_date_to_utc(date: &str) -> DateTime<Utc> {
    let hour: u32 = date[11..13].parse().unwrap();
    let min: u32 = date[14..16].parse().unwrap();
    let sec: u32 = date[17..19].parse().unwrap();
    let day: u32 = date[0..2].parse().unwrap();
    let month: u32 = date[3..5].parse().unwrap();
    let year: i32 = date[6..10].parse().unwrap();

    Utc.with_ymd_and_hms(year, month, day, hour, min, sec)
        .unwrap()
}
