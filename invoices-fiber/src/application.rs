use crate::{
    tools::{csv, report, telegram},
    Configurations,
};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let _tg = telegram::new_client(configurations.telegram_token);

    let _spreadsheets = {
        let csv_content = csv::read(&configurations.csv_path)?;
        let structured_csv = csv::estruture(&csv_content);
        report::mount(structured_csv)
    };

    Ok(())
}
