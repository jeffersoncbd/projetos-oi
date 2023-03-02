use crate::{
    configurations::Configurations,
    tools::{csv, database},
};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let csv = csv::load(&configurations.csv_path)?;
    let logs = csv::struct_csv(&csv)?;
    if configurations.column == "ALL" {
        database::find_all_dates_from_job(&logs, &configurations)?;
    } else {
        database::find_job_date(&logs, &configurations)?;
    }

    Ok(())
}
