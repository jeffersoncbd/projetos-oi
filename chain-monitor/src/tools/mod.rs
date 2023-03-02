use chrono::{DateTime, TimeZone, Utc};

pub mod csv;
pub mod database;
pub mod names_relationship;

pub fn string_to_static(value: String) -> &'static str {
    let value: &'static str = Box::leak(value.into_boxed_str());
    value
}

pub fn str_date_to_utc(date: &str) -> Result<DateTime<Utc>, &'static str> {
    if date.len() != 19 {
        let feedback = format!("A data deve ter 19 dígitos ({date})");
        return Err(string_to_static(feedback));
    }

    let error = Err("Não foi possível converter a data.");

    let year: i32 = match date[6..10].parse() {
        Ok(value) => value,
        Err(_) => return error,
    };
    let month: u32 = match date[3..5].parse() {
        Ok(value) => value,
        Err(_) => return error,
    };
    let day: u32 = match date[0..2].parse() {
        Ok(value) => value,
        Err(_) => return error,
    };
    let hour: u32 = match date[11..13].parse() {
        Ok(value) => value,
        Err(_) => return error,
    };
    let min: u32 = match date[14..16].parse() {
        Ok(value) => value,
        Err(_) => return error,
    };
    let sec: u32 = match date[17..19].parse() {
        Ok(value) => value,
        Err(_) => return error,
    };

    match Utc.with_ymd_and_hms(year, month, day, hour, min, sec) {
        chrono::LocalResult::Single(date) => Ok(date),
        _ => return error,
    }
}
