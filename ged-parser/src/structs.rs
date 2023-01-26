use chrono::{prelude::Utc as UTC, DateTime, TimeZone, Utc};

use crate::tools::date::get_month_number;

pub struct DueDate {
    pub string: String,
    pub utc: DateTime<Utc>,
}
impl PartialEq for DueDate {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

pub struct DataUnity<'a> {
    pub due_date: DueDate,
    pub status: &'a str,
    pub amount: &'a str,
}
impl<'a> DataUnity<'a> {
    pub fn new(due_date: &'a str, status: &'a str, amount: &'a str) -> DataUnity<'a> {
        let mut separator = '-';

        if due_date.contains('/') {
            separator = '/'
        }

        let parts: Vec<&str> = due_date.split(separator).collect();

        if parts.len() != 3 {
            panic!(
                "The due date \"{}\" is in a format I don't is accepted.",
                due_date
            );
        }

        let day = parts[0].parse().expect("The day should it a number.");
        let month = get_month_number(parts[1]);
        let year = format!("20{}", parts[2])
            .parse()
            .expect("The year should it a number.");
        return DataUnity {
            due_date: DueDate {
                string: UTC
                    .with_ymd_and_hms(year, month, day, 0, 0, 0)
                    .unwrap()
                    .format("%d/%m/%y")
                    .to_string(),
                utc: UTC.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap(),
            },
            status,
            amount,
        };
    }
}

pub enum StructuredCSVRow<'a> {
    Whatsapp(DataUnity<'a>),
    Email(DataUnity<'a>),
}
