use crate::{csv, Configurations};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let backlog_csv = {
        let csv = csv::read(&configurations.csv_path)?;
        csv::structure_backlog(&csv)?
    };
    csv::save(backlog_csv, &configurations.destiny_path)?;
    Ok(())
}
