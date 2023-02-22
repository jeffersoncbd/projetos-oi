use crate::{
    tools::{csv, report},
    Configurations,
};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let csv_content = csv::read(&configurations.csv_path)?;
    {
        let structured_csv = csv::estruture(&csv_content);
        report::print(structured_csv);
    }
    Ok(())
}
