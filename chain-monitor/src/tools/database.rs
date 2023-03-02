use std::{cmp::Ordering, collections::HashMap};

use chrono::{TimeZone, Utc};

use crate::{
    configurations::Configurations,
    tools::{csv::Logs, str_date_to_utc, string_to_static},
};

pub fn find_all_dates_from_job<'a>(
    logs: &'a Logs,
    configurations: &Configurations,
) -> Result<(), &'static str> {
    let mut logs: Vec<&HashMap<&str, &str>> = logs
        .iter()
        .filter(|log| log["PARAMETRO"] == configurations.job_name_in_database)
        .collect();

    logs.sort_by(|a, b| {
        let a_date = str_date_to_utc(a["INICIO"]).unwrap();
        let b_date = str_date_to_utc(b["INICIO"]).unwrap();
        a_date.cmp(&b_date)
    });

    println!("Execuções do job {}:", configurations.job_name_in_control);
    for log in logs {
        println!("- Inicio em {}, término em {}", log["INICIO"], log["FIM"])
    }

    Ok(())
}

pub fn find_job_date<'a>(
    logs: &'a Logs,
    configurations: &Configurations,
) -> Result<(), &'static str> {
    let column: &str = &configurations.column;
    let mut reference = Utc::now();
    if column == "FIM" {
        reference = Utc.with_ymd_and_hms(1995, 1, 29, 12, 0, 0).unwrap();
    }

    let mut logs: Vec<&HashMap<&str, &str>> = logs
        .iter()
        .filter(|log| {
            let column_reference = if column == "STATUS" { "INICIO" } else { column };
            if log[column_reference] == "" {
                false
            } else if let Some(execution_date) = &configurations.execution_date {
                log[column_reference].contains(execution_date)
            } else {
                true
            }
        })
        .collect();

    logs.sort_by(|a, b| {
        let column_reference = if column == "STATUS" { "INICIO" } else { column };
        let a_date = str_date_to_utc(a[column_reference]).unwrap();
        let b_date = str_date_to_utc(b[column_reference]).unwrap();
        if let Some(execution_number) = configurations.execution_number {
            if execution_number < 0 {
                return b_date.cmp(&a_date);
            }
        }
        a_date.cmp(&b_date)
    });

    let mut result: Option<&str> = None;
    let mut job_execution_index: u32 = 1;
    for log in logs {
        if log["PARAMETRO"] != configurations.job_name_in_database {
            continue;
        }
        if let Some(execution_number) = configurations.execution_number {
            let index = if execution_number < 0 {
                job_execution_index as i32 * -1
            } else {
                job_execution_index as i32
            };
            job_execution_index += 1;
            if index != execution_number {
                continue;
            }
        }

        if column == "STATUS" {
            match log[column] {
                "OK" => println!("finalizado ✅"),
                "EXEC" => println!("executando ⏩"),
                "ERRO" => println!("com erro ⚠️"),
                _ => println!("{}", log[column]),
            };
            return Ok(());
        }

        let date_of_log = str_date_to_utc(log[column])?;

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
        Some(date) => {
            println!("{date}");
            Ok(())
        }
        None => {
            return Err(string_to_static(format!(
                "O job {} não foi encontrado.",
                configurations.job_name_in_control
            )))
        }
    }
}
