use std::cmp::Ordering;

use chrono::{TimeZone, Utc};

use crate::{
    configurations::Configurations,
    tools::{csv::Logs, str_date_to_utc, string_to_static},
};

pub fn find_job_date<'a>(
    logs: &'a Logs,
    configurations: &Configurations,
) -> Result<&'a str, &'static str> {
    let mut reference = Utc::now();
    if configurations.column == "FIM" {
        reference = Utc.with_ymd_and_hms(1995, 1, 29, 12, 0, 0).unwrap();
    }
    let mut result: Option<&str> = None;

    for log in logs {
        if let Some(chain) = &configurations.chain {
            if log["CADEIA"].trim() != chain {
                continue;
            }
        }
        let column: &str = &configurations.column;
        if log[column] == "" || log["PARAMETRO"] != configurations.job_name_in_database {
            continue;
        }

        if column == "STATUS" {
            match log[column] {
                "OK" => return Ok("finalizado ✅"),
                "EXEC" => return Ok("executando ⏩"),
                "ERRO" => return Ok("com erro ⚠️"),
                _ => return Ok(log[column]),
            }
        }

        let date_of_log = str_date_to_utc(log[column]);

        match date_of_log.cmp(&reference) {
            Ordering::Less => {
                if column == "INICIO" {
                    reference = date_of_log;
                    result = Some(log[column].trim());
                }
            }
            Ordering::Greater => {
                if column == "FIM" {
                    reference = date_of_log;
                    result = Some(log[column].trim());
                }
            }
            Ordering::Equal => (),
        };
    }

    match result {
        Some(date) => Ok(date),
        None => {
            return Err(string_to_static(format!(
                "O job {} não foi encontrado.",
                configurations.job_name_in_control
            )))
        }
    }
}
