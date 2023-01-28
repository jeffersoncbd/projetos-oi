pub mod configurations;

mod csv;
mod spreadsheet;
mod tools;

pub mod application {

    use crate::{configurations::Configurations, csv::read_csv, spreadsheet};

    pub fn run(configurations: Configurations) -> Result<(), &'static str> {
        let _spreadsheet = {
            let structured_rows = read_csv(&configurations.csv_path, &configurations.filter)?;
            spreadsheet::mount(structured_rows)
        };

        Ok(())
    }
}
