use crate::{configurations::Configurations, csv::read_csv, report};
use spreadsheet_maker::Spreadsheet;

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let mut spreadsheet = Spreadsheet::new(format!(
        "Verificação E-Mail Seguro - {}",
        configurations.filter.to_uppercase()
    ));
    {
        let structured_rows = read_csv(&configurations.csv_path, &configurations.filter)?;
        report::insert(structured_rows, &mut spreadsheet);
    };
    let output = format!(
        "{}/{}.png",
        configurations.output_path, configurations.filter
    );

    spreadsheet.save_png(&output.as_str())?;

    Ok(())
}
