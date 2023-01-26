use crate::structs::{DataUnity, DueDate, StructuredCSVRow};

pub struct Headers<'a> {
    pub rows: Vec<&'a str>,
    pub columns: Vec<&'a str>,
}

pub struct Size {
    pub rows: usize,
    pub columns: usize,
}

pub struct GeneralSpreadsheet<'a> {
    pub headers: Headers<'a>,
    pub content: Vec<Vec<String>>,
    pub size: Size,
    pub backlog: String,
    pub notification: String,
}

pub const BACKLOG: [&str; 2] = ["Aguardando Fatura", "Aguardando Fatura Resumida"];
pub const NOTIFICATION: [&str; 4] = [
    "Aguardando Processamento",
    "Download da Fatura Realizado",
    "Em Processamento",
    "Carregada",
];

fn push_data<'a, T: PartialEq>(header: &'a mut Vec<T>, value: T) {
    if !header.contains(&value) {
        header.push(value);
    }
}

fn calculate_values<'a>(csv: &Vec<&DataUnity>, spreadsheet: &mut GeneralSpreadsheet<'a>) {
    let last_row = spreadsheet.size.rows - 1;
    let last_column = spreadsheet.size.columns - 1;

    for item in csv.iter() {
        let r = spreadsheet
            .headers
            .rows
            .iter()
            .position(|&r| r == item.status)
            .unwrap();
        let c = spreadsheet
            .headers
            .columns
            .iter()
            .position(|&c| c == item.due_date.string)
            .unwrap();
        let old_row_total: u64 = spreadsheet.content[r][last_column].parse().unwrap();
        let old_column_total: u64 = spreadsheet.content[last_row][c].parse().unwrap();
        let old_super_total: u64 = spreadsheet.content[last_row][last_column].parse().unwrap();
        let old_value: u64 = spreadsheet.content[r][c].parse().unwrap();
        let current_value: u64 = item.amount.parse().unwrap();
        spreadsheet.content[r][c] = (old_value + current_value).to_string();
        spreadsheet.content[r][last_column] = (old_row_total + current_value).to_string();
        spreadsheet.content[last_row][c] = (old_column_total + current_value).to_string();
        spreadsheet.content[last_row][last_column] = (old_super_total + current_value).to_string();
        if BACKLOG.contains(&item.status) {
            let old_backlog: u64 = spreadsheet.backlog.parse().unwrap();
            spreadsheet.backlog = (old_backlog + current_value).to_string();
        }
        if NOTIFICATION.contains(&item.status) {
            let old_notification: u64 = spreadsheet.notification.parse().unwrap();
            spreadsheet.notification = (old_notification + current_value).to_string();
        }
    }
}

pub fn mount<'a>(
    unfiltered_csv: &'a Vec<StructuredCSVRow<'a>>,
    data_type: &str,
) -> GeneralSpreadsheet<'a> {
    let mut csv: Vec<&DataUnity> = Vec::new();

    for row in unfiltered_csv {
        match data_type {
            "email" => {
                if let StructuredCSVRow::Email(item) = row {
                    csv.push(item);
                }
            }
            "whatsapp" => {
                if let StructuredCSVRow::Whatsapp(item) = row {
                    csv.push(item);
                }
            }
            _ => panic!("The filters accepted are \"email\" and \"whatsapp\""),
        };
    }

    let mut rows_headers = Vec::new();
    let mut columns_headers: Vec<&DueDate> = Vec::new();

    for row in &csv {
        push_data(&mut rows_headers, row.status);
        push_data(&mut columns_headers, &row.due_date);
    }

    rows_headers.sort();
    columns_headers.sort_by(|a, b| a.utc.cmp(&b.utc));
    let mut columns_headers: Vec<&str> = columns_headers
        .iter()
        .map(|item| item.string.as_str())
        .collect();

    rows_headers.push("Total Geral");
    columns_headers.push("Total Geral");

    let headers = Headers {
        rows: rows_headers.clone(),
        columns: columns_headers.clone(),
    };
    let content = vec![vec![String::from("0"); columns_headers.len()]; rows_headers.len()];
    let size = Size {
        rows: rows_headers.len(),
        columns: columns_headers.len(),
    };

    let mut spreadsheet = GeneralSpreadsheet {
        headers,
        content,
        size,
        backlog: String::from("0"),
        notification: String::from("0"),
    };

    calculate_values(&csv, &mut spreadsheet);

    spreadsheet
}
