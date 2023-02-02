use dotenv::dotenv;
use ged_parser::{
    csv,
    logs::send_title_of_general_spreadsheets,
    spreadsheets,
    tools::{comparator_filter, printer, telegram},
};
use std::env;

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if &args.len() == &(1 as usize) {
        send_title_of_general_spreadsheets();
        return;
    }
    let last_csv = csv::read("last_execution.csv");
    let structured_last_csv = csv::estruture(&last_csv);
    let comparative_spreadsheet = spreadsheets::comparative::mount(&structured_last_csv, &args[2]);

    let csv = csv::read(&args[1]);
    let structured_csv = csv::estruture(&csv);
    comparator_filter::filter_and_save(&structured_csv);

    let general_spreadsheet = spreadsheets::general::mount(&structured_csv, &args[2]);
    printer::general(&general_spreadsheet, &args[2]);

    printer::comparative(&general_spreadsheet, &comparative_spreadsheet, &args[2]);

    // telegram::send_image(format!("{}.png", &args[2]), "GENERAL_CHAT_ID");
    telegram::send_image(String::from("comparison.png"), "COMPARATIVE_CHAT_ID");
}
