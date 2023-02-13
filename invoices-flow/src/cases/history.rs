use std::fs;

use chrono::{DateTime, TimeZone, Utc};

pub struct ExecutionData {
    pub date: DateTime<Utc>,
    pub content: String,
}

fn file_name_to_date(file_name: &str) -> DateTime<Utc> {
    let year = file_name[..4].parse().unwrap();
    let month = file_name[5..7].parse().unwrap();
    let day = file_name[8..10].parse().unwrap();
    let hour = file_name[11..13].parse().unwrap();
    let min = file_name[14..].parse().unwrap();
    Utc.with_ymd_and_hms(year, month, day, hour, min, 0)
        .unwrap()
}

pub fn read(path: &String) -> Result<Vec<ExecutionData>, &'static str> {
    let mut files = Vec::new();
    let dir = fs::read_dir(path).unwrap();
    for file in dir {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        let file_name: Vec<&str> = file_name.split(".").collect();
        let file_path = file.path().into_os_string().into_string().unwrap();
        let date = file_name_to_date(&file_name[0]);
        let content = fs::read_to_string(file_path).unwrap();
        files.push(ExecutionData { date, content });
    }

    files.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(files)
}
