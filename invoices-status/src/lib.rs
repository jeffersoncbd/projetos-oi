pub mod configurations;

mod csv;
mod tools;

pub mod application {

    use crate::{configurations::Configurations, csv::read_csv};

    pub fn run(configurations: Configurations) -> Result<(), &'static str> {
        let _structured_rows = read_csv(&configurations.csv_path, &configurations.filter)?;

        Ok(())
    }
}
