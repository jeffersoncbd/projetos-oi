use crate::{csv, data, Configurations};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let data = {
        let csv_content = csv::read(&configurations.csv_path)?;
        data::Structure::from(csv_content, &configurations.filtering_period)
    };

    for row in data {
        println!("{row:?}");
    }

    Ok(())
}
