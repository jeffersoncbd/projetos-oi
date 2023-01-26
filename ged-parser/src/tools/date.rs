const MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "ago", "set", "out", "nov", "dec",
];

pub fn get_month_number(month: &str) -> u32 {
    let month = month.to_lowercase();
    for (i, &name) in MONTHS.iter().enumerate() {
        if name == month {
            return (i + 1) as u32;
        }
    }
    return month
        .parse()
        .expect(&format!("The month \"{}\" is not accepted.", month));
}
