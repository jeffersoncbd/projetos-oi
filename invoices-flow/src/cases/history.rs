use std::fs;

use chrono::{DateTime, TimeZone, Utc};

use crate::tools::string_to_static;

pub struct ExecutionData {
    pub date: DateTime<Utc>,
    pub content: String,
}
impl Clone for ExecutionData {
    fn clone(&self) -> Self {
        Self {
            date: self.date.clone(),
            content: self.content.clone(),
        }
    }
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

pub fn read(path: &String, executions_amount: u32) -> Result<Vec<ExecutionData>, &'static str> {
    let mut files = Vec::new();
    let dir = match fs::read_dir(path) {
        Ok(path) => path,
        Err(error) => {
            let feedback = format!("Falha ao ler arquivos na pasta \"{}\": {}", path, error);
            return Err(string_to_static(feedback));
        }
    };
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

    let mut executions = Vec::new();
    for (i, execution) in files.iter().enumerate() {
        if i as i32 > files.len() as i32 - executions_amount as i32 - 1 {
            executions.push(execution.clone())
        }
    }

    Ok(executions)
}
