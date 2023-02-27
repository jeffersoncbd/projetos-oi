use crate::{
    configurations::Configurations,
    tools::{csv, database},
};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let csv = csv::load(&configurations.csv_path)?;
    let logs = csv::struct_csv(&csv)?;
    let job_date = database::find_job_date(&logs, &configurations)?;

    println!("{job_date}");

    Ok(())
}
