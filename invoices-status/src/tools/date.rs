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
impl Clone for Date {
    fn clone(&self) -> Self {
        Self {
            string: self.string.clone(),
            utc: self.utc.clone(),
        }
    }
}

const MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

fn get_month_number(month: &str) -> Result<u32, &'static str> {
    let month = month.to_lowercase();
    for (i, &name) in MONTHS.iter().enumerate() {
        if name == month {
            return Ok((i + 1) as u32);
        }
    }
    Ok(match month.parse() {
        Ok(month) => month,
        Err(_) => return Err(""),
    })
}

pub fn parse(date: String) -> Result<Date, &'static str> {
    let mut separator = '-';

    if date.contains('/') {
        separator = '/'
    }

    let parts: Vec<&str> = date.split(separator).collect();

    if parts.len() != 3 {
        return Err("Formato diferente de \"DD/MM/AA\" ou \"DD-MM-AA\"");
    }

    let day = match parts[0].parse() {
        Ok(day) => day,
        Err(_) => {
            return Err("Dia não é um número válido");
        }
    };
    if day > 31 || day < 1 {
        return Err("Dia não é um número válido");
    }
    let month = match get_month_number(parts[1]) {
        Ok(month) => month,
        Err(_) => {
            return Err("Mês não é um número válido");
        }
    };
    if month > 12 || day < 1 {
        return Err("Mês não é um número válido");
    }
    let year = match format!("20{}", parts[2]).parse() {
        Ok(year) => year,
        Err(_) => {
            return Err("Ano não é um número válido");
        }
    };
    if year > 2099 || year < 2000 {
        return Err("Ano não é um número válido");
    }
    Ok(Date {
        string: UTC
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .unwrap()
            .format("%d/%m/%y")
            .to_string(),
        utc: UTC.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap(),
    })
}
