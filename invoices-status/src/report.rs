use crate::{csv::StructuredRow, tools::date::Date};
use spreadsheet_maker::{Cell, Spreadsheet};
use std::collections::HashMap;

pub const BACKLOG: [&str; 3] = [
    "Aguardando Fatura",
    "Aguardando Fatura Resumida",
    "Carregada",
];
pub const NOTIFICATION: [&str; 3] = [
    "Aguardando Processamento",
    "Download da Fatura Realizado",
    "Em Processamento",
];

pub fn insert(rows: Vec<StructuredRow>, spreadsheet: &mut Spreadsheet) {
    spreadsheet.set_margin(10);

    let mut report: HashMap<String, HashMap<String, String>> = HashMap::new();

    let mut column_headers: Vec<Date> = Vec::new();
    let mut row_headers: Vec<String> = Vec::new();

    for row in rows {
        if !column_headers.contains(&row.due_date) {
            column_headers.push(row.due_date.clone());
        }
        if !row_headers.contains(&row.status) {
            row_headers.push(row.status.clone());
        }
        match report.get_mut(&row.status) {
            Some(status) => match status.get_mut(&row.due_date.string) {
                Some(value) => {
                    let part: u64 = row.amount.parse().unwrap();
                    let old_value: u64 = value.parse().unwrap();
                    *value = (old_value + part).to_string();
                }
                None => {
                    status.insert(row.due_date.string, row.amount);
                }
            },
            None => {
                let date = HashMap::from([(row.due_date.string, row.amount)]);
                report.insert(row.status, date);
            }
        }
    }

    row_headers.sort();
    column_headers.sort_by(|a, b| a.utc.cmp(&b.utc));
    let column_headers: Vec<String> = column_headers
        .iter()
        .map(|date| date.string.clone())
        .collect();

    for (i, header) in column_headers.iter().enumerate() {
        let column = i as u32 + 2;
        spreadsheet.set_cell(Cell {
            content: header.to_string(),
            column,
            row: 1,
            color: None,
        })
    }

    spreadsheet.set_column_width(1, 175);

    spreadsheet.set_row_custom_font(1, "bold");
    for (i, header) in row_headers.iter().enumerate() {
        let row = i as u32 + 2;
        spreadsheet.set_cell(Cell {
            content: header.to_string(),
            column: 1,
            row,
            color: None,
        })
    }

    let mut backlog: u32 = 0;
    let mut notification: u32 = 0;

    for (row_header, row) in report {
        let r = row_headers
            .iter()
            .position(|header| header == &row_header)
            .unwrap() as u32;
        for (column_header, content) in row {
            let c = column_headers
                .iter()
                .position(|header| header == &column_header)
                .unwrap() as u32;
            if BACKLOG.contains(&&row_header.as_str()) {
                spreadsheet.set_row_color(r as usize + 2, Some([0, 176, 80, 255]));
                let new_part: u32 = content.parse().unwrap();
                backlog += new_part;
            }
            if NOTIFICATION.contains(&&row_header.as_str()) {
                spreadsheet.set_row_color(r as usize + 2, Some([255, 255, 0, 255]));
                let new_part: u32 = content.parse().unwrap();
                notification += new_part;
            }
            spreadsheet.set_cell(Cell {
                content: content.clone(),
                column: c + 2,
                row: r + 2,
                color: None,
            });
            spreadsheet.add_in_cell(Cell {
                content: content.clone(),
                column: c + 2,
                row: row_headers.len() as u32 + 2,
                color: None,
            });
            spreadsheet.add_in_cell(Cell {
                content: content.clone(),
                column: column_headers.len() as u32 + 2,
                row: r + 2,
                color: None,
            });
            spreadsheet.add_in_cell(Cell {
                content: content.clone(),
                column: column_headers.len() as u32 + 2,
                row: row_headers.len() as u32 + 2,
                color: None,
            })
        }
    }

    let last_header = row_headers.len() + 1;
    let total_row: usize = last_header + 1;
    {
        spreadsheet.set_row_height(last_header, 35);

        spreadsheet.set_row_color(total_row, Some([191, 191, 191, 255]));
        spreadsheet.set_row_font_size(total_row, 16);
        spreadsheet.set_row_height(total_row, 22);
        spreadsheet.set_row_custom_font(total_row, "bold");
        spreadsheet.set_cell(Cell {
            content: String::from("Total geral"),
            row: total_row as u32,
            column: 1,
            color: None,
        });

        let total_column: usize = column_headers.len() + 2;
        spreadsheet.set_column_width(column_headers.len() + 1, 80);
        spreadsheet.set_column_font_size(total_column, 16);
        spreadsheet.set_column_custom_font(total_column, "bold");
        spreadsheet.set_cell(Cell {
            content: String::from("Total geral"),
            row: 1,
            column: total_column as u32,
            color: None,
        });
        spreadsheet.set_column_width(total_column, 80);

        spreadsheet.set_cell_font_size((total_column as u32, total_row as u32), 18);
    }

    let backlog_row: u32 = (total_row as u32) + 2;
    let notification_row: u32 = backlog_row + 1;
    {
        spreadsheet.set_cell(Cell {
            content: String::from("Backlog"),
            row: backlog_row,
            column: 1,
            color: None,
        });
        spreadsheet.set_row_font_size(backlog_row as usize, 20);
        spreadsheet.set_row_height(backlog_row as usize, 25);
        spreadsheet.set_row_custom_font(backlog_row as usize, "bold");
        spreadsheet.set_cell_color((1, backlog_row), Some([0, 176, 80, 255]));
        spreadsheet.set_cell(Cell {
            content: backlog.to_string(),
            row: backlog_row,
            column: 2,
            color: None,
        });
        spreadsheet.set_cell_color((2, backlog_row), Some([0, 176, 80, 255]));
    }
    {
        spreadsheet.set_cell(Cell {
            content: String::from("Notification"),
            row: notification_row,
            column: 1,
            color: None,
        });
        spreadsheet.set_cell_color((1, notification_row), Some([255, 255, 0, 255]));
        spreadsheet.set_row_font_size(notification_row as usize, 20);
        spreadsheet.set_row_height(notification_row as usize, 25);
        spreadsheet.set_row_custom_font(notification_row as usize, "bold");
        spreadsheet.set_cell(Cell {
            content: notification.to_string(),
            row: notification_row,
            column: 2,
            color: None,
        });
        spreadsheet.set_cell_color((2, notification_row), Some([255, 255, 0, 255]));
    }
}
