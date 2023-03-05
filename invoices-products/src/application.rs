use crate::{csv, data, Configurations};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let _data = {
        let csv_content = csv::read(&configurations.csv_path)?;
        data::Structure::from(csv_content)
    };

    Ok(())
}
