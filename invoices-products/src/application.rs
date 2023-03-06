use crate::{csv, data, reports, Configurations};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let data = {
        let csv_content = csv::read(&configurations.csv_path)?;
        data::Structure::from(csv_content, &configurations.filtering_period)
    };

    let whatsapp_spreadsheet = reports::mount(&data, "WHATSAPP")?;
    let email_spreadsheet = reports::mount(&data, "E-MAIL")?;

    whatsapp_spreadsheet.save_png("whatsapp.png")?;
    email_spreadsheet.save_png("email.png")?;
    Ok(())
}
