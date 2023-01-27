pub mod date {
    use chrono::{prelude::Utc as UTC, DateTime, TimeZone, Utc};

    pub struct Date {
        pub string: String,
        pub utc: DateTime<Utc>,
    }
    impl PartialEq for Date {
        fn eq(&self, other: &Self) -> bool {
            self.string == other.string
        }
    }

    const MONTHS: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "ago", "set", "out", "nov", "dec",
    ];

    fn mount_error(date: String) -> &'static str {
        format_args!(
            "A data {} não está no formato esperado (DD/MM/AA ou DD-MM-AA).",
            date
        )
        .as_str()
        .unwrap()
    }

    fn get_month_number(month: &str) -> Result<u32, &'static str> {
        let month = month.to_lowercase();
        for (i, &name) in MONTHS.iter().enumerate() {
            if name == month {
                return Ok((i + 1) as u32);
            }
        }
        Ok(match month.parse() {
            Ok(month) => month,
            Err(_) => return Err("month"),
        })
    }

    pub fn parse(date: String) -> Result<Date, &'static str> {
        let mut separator = '-';

        if date.contains('/') {
            separator = '/'
        }

        let parts: Vec<&str> = date.split(separator).collect();

        if parts.len() != 3 {
            return Err(mount_error(date.clone()));
        }

        let day = match parts[0].parse() {
            Ok(day) => day,
            Err(_) => {
                return Err(mount_error(date.clone()));
            }
        };
        let month = match get_month_number(parts[1]) {
            Ok(month) => month,
            Err(_) => {
                return Err(mount_error(date.clone()));
            }
        };
        let year = match format!("20{}", parts[2]).parse() {
            Ok(year) => year,
            Err(_) => {
                return Err(mount_error(date.clone()));
            }
        };
        Ok(Date {
            string: UTC
                .with_ymd_and_hms(year, month, day, 0, 0, 0)
                .unwrap()
                .format("%d/%m/%y")
                .to_string(),
            utc: UTC.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap(),
        })
    }
}
