use crate::{tools::csv, Configurations};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let csv_content = csv::read(&configurations.csv_path)?;
    let _structured_csv = csv::estruture(&csv_content);

    Ok(())
}
