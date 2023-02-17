use crate::{tools::csv, Configurations};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let _csv_content = csv::read(&configurations.csv_path)?;

    Ok(())
}
