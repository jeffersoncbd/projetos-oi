use tg_api::ImageMessage;
use uuid::Uuid;

use crate::{csv, data, reports, telegram, Configurations};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let tg = telegram::new_client(configurations.telegram_token);

    let spreadsheets = {
        let data = {
            let csv_content = csv::read(&configurations.csv_path)?;
            data::Structure::from(csv_content, &configurations.filtering_period)
        };

        [
            reports::mount(&data, "WHATSAPP")?,
            reports::mount(&data, "E-MAIL")?,
        ]
    };

    for spreadsheet in spreadsheets {
        let id = Uuid::new_v4();
        let report_path = format!("/tmp/{}.png", id.to_string());
        if let Some(spreadsheet) = spreadsheet {
            spreadsheet.save_png(&report_path)?;
            if let Err(error) = tg.send_image(ImageMessage {
                image_path: &report_path,
                to: &configurations.destiny_id.as_str(),
            }) {
                let feedback = format!("Telegram: {}", error);
                return Err(string_to_static::parse(feedback));
            };
        }
    }
    Ok(())
}
