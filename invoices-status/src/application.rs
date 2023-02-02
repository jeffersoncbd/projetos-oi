use crate::{configurations::Configurations, csv::read_csv, report};
use chrono::Utc;
use spreadsheet_maker::Spreadsheet;
use tg_api::{Client, Configurations as TgConfigurations, ImageMessage};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let telegram = Client::new(TgConfigurations::new(configurations.telegram_token, None));

    let mut spreadsheet = Spreadsheet::new(format!(
        "Verificação E-Mail Seguro - {}",
        configurations.filter.to_uppercase()
    ));
    {
        let structured_rows = read_csv(&configurations.csv_path, &configurations.filter)?;
        report::insert(structured_rows, &mut spreadsheet);
    };

    let image_path = format!(
        "{}/{}-{}.png",
        configurations.output_path,
        configurations.filter,
        Utc::now().timestamp()
    );

    spreadsheet.save_png(&image_path.as_str())?;

    telegram.send_image(ImageMessage {
        image_path: &image_path,
        to: &configurations.destiny_id.as_str(),
    })?;

    Ok(())
}
