use tg_api::ImageMessage;
use uuid::Uuid;

use crate::{
    cases::{history, report},
    telegram, Configurations,
};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let tg = telegram::new_client(configurations.telegram_token);
    let executions = history::read(&configurations.history_folder_path)?;
    let spreadsheet = report::structure(executions)?;
    let id = Uuid::new_v4();
    let report_path = format!("/tmp/{}.png", id.to_string());
    spreadsheet.save_png(&report_path)?;
    tg.send_image(ImageMessage {
        image_path: &report_path,
        to: &configurations.destiny_id.as_str(),
    })?;
    Ok(())
}
