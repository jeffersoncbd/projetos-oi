use string_to_static;
use tg_api::ImageMessage;
use uuid::Uuid;

use crate::{
    tools::{csv, report, telegram},
    Configurations,
};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let tg = telegram::new_client(configurations.telegram_token);

    let spreadsheets = {
        let csv_content = csv::read(&configurations.csv_path)?;
        let structured_csv = csv::estruture(&csv_content);
        report::mount(structured_csv)
    };

    for spreadsheet in spreadsheets {
        let id = Uuid::new_v4();
        let report_path = format!("/tmp/{}.png", id.to_string());
        spreadsheet.save_png(&report_path)?;
        if let Err(error) = tg.send_image(ImageMessage {
            image_path: &report_path,
            to: &configurations.destiny_id.as_str(),
        }) {
            let feedback = format!("Telegram: {}", error);
            return Err(string_to_static::parse(feedback));
        };
    }

    Ok(())
}
