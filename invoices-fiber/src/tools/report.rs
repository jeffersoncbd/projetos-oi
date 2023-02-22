use std::collections::HashMap;

use chrono::{DateTime, TimeZone, Utc};
use spreadsheet_maker::{Cell, Spreadsheet};

use super::csv::Row;

const BACKLOG: [&str; 8] = [
    "AGUARDANDO DEFINIR DESTINATARIO",
    "AGUARDANDO QRCODE",
    "AGUARDANDO FATURA",
    "AGUARDANDO FATURA FIBRA",
    "AGUARDANDO FATURA RESUMIDA",
    "ARGUMENTOS INCOMPLETOS",
    "CARREGADA",
    "PRAZO EXCEDIDO",
];
const NOTIFICATION: [&str; 4] = [
    "AGUARDANDO PROCESSAMENTO",
    "DOWNLOAD DA FATURA REALIZADO",
    "EM PROCESSAMENTO",
    "PROCESSADO COM FALHA",
];

struct Headers<'a> {
    pub columns: Vec<&'a str>,
    pub rows: Vec<&'a str>,
}
struct Reports<'a> {
    pub headers: Headers<'a>,
    pub spreadsheets: HashMap<&'a str, Spreadsheet>,
}

fn str_to_date_time(value: &str) -> DateTime<Utc> {
    let day: u32 = value[0..2].parse().unwrap();
    let month: u32 = value[3..5].parse().unwrap();
    let year: i32 = value[6..8].parse().unwrap();
    Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}

fn extract_reports<'a>(rows: &'a Vec<Row>) -> Reports<'a> {
    let mut column_headers: Vec<(&str, DateTime<Utc>)> = Vec::new();
    let mut row_headers = Vec::new();
    let mut report_names: Vec<&str> = Vec::new();

    for row in rows {
        let column_header = (row.data_venc, str_to_date_time(row.data_venc));
        if !column_headers.contains(&column_header) {
            column_headers.push(column_header);
        }
        if !row_headers.contains(&row.status_notificacao) {
            row_headers.push(row.status_notificacao)
        }
        if !report_names.contains(&row.tipo_pro) {
            report_names.push(row.tipo_pro)
        }
    }

    report_names.sort();
    row_headers.sort();
    column_headers.sort_by(|a, b| a.1.cmp(&b.1));
    let column_headers = column_headers.iter().map(|h| h.0).collect();

    let mut spreadsheets = HashMap::new();

    for report_name in report_names {
        spreadsheets.insert(
            report_name,
            Spreadsheet::new(format!(
                "[Nova Fibra] Relatório de faturas - {}",
                report_name
            )),
        );
    }

    Reports {
        headers: Headers {
            columns: column_headers,
            rows: row_headers,
        },
        spreadsheets,
    }
}

pub fn print(rows: Vec<Row>) {
    let mut reports = extract_reports(&rows);

    for (_, spreadsheet) in &mut reports.spreadsheets {
        spreadsheet.add_in_cell(Cell {
            column: 2,
            row: reports.headers.rows.len() as u32 + 4,
            content: String::from("0"),
            color: None,
        });

        spreadsheet.add_in_cell(Cell {
            column: 2,
            row: reports.headers.rows.len() as u32 + 5,
            content: String::from("0"),
            color: None,
        });
    }

    for row in &rows {
        let spreadsheet = reports.spreadsheets.get_mut(row.tipo_pro).unwrap();
        spreadsheet.set_margin(10);

        for (i, row_headers) in reports.headers.rows.iter().enumerate() {
            spreadsheet.set_cell(Cell {
                column: 1,
                row: i as u32 + 2,
                content: String::from(row_headers.clone()),
                color: None,
            });
            if BACKLOG.contains(&row_headers.to_uppercase().as_str()) {
                spreadsheet.set_row_color(i + 2, Some([0, 176, 80, 255]));
            }
            if NOTIFICATION.contains(&row_headers.to_uppercase().as_str()) {
                spreadsheet.set_row_color(i + 2, Some([255, 255, 0, 255]));
            }
        }
        spreadsheet.set_column_width(1, 230);

        for (i, column_headers) in reports.headers.columns.iter().enumerate() {
            spreadsheet.set_cell(Cell {
                column: i as u32 + 2,
                row: 1,
                content: String::from(column_headers.clone()),
                color: None,
            });
            spreadsheet.set_column_width(i + 2, 80);
            spreadsheet.set_row_custom_font(1, "bold");
        }

        let column_number = reports
            .headers
            .columns
            .iter()
            .position(|h| h == &row.data_venc)
            .unwrap() as u32
            + 2;
        let row_number = reports
            .headers
            .rows
            .iter()
            .position(|h| h == &row.status_notificacao)
            .unwrap() as u32
            + 2;
        spreadsheet.add_in_cell(Cell {
            column: column_number,
            row: row_number,
            content: String::from(row.count),
            color: None,
        });

        spreadsheet.set_cell(Cell {
            column: reports.headers.columns.len() as u32 + 2,
            row: 1,
            content: String::from("TOTAL GERAL"),
            color: None,
        });
        spreadsheet.add_in_cell(Cell {
            column: reports.headers.columns.len() as u32 + 2,
            row: row_number,
            content: String::from(row.count),
            color: None,
        });
        spreadsheet.set_column_custom_font(reports.headers.columns.len() + 2, "bold");
        spreadsheet.set_column_font_size(reports.headers.columns.len() + 2, 16);
        spreadsheet.set_column_width(reports.headers.columns.len() + 2, 110);

        spreadsheet.set_cell(Cell {
            column: 1,
            row: reports.headers.rows.len() as u32 + 2,
            content: String::from("TOTAL GERAL"),
            color: None,
        });
        spreadsheet.add_in_cell(Cell {
            column: column_number,
            row: reports.headers.rows.len() as u32 + 2,
            content: String::from(row.count),
            color: None,
        });
        spreadsheet.set_row_custom_font(reports.headers.rows.len() + 2, "bold");
        spreadsheet.set_row_font_size(reports.headers.rows.len() + 2, 16);

        spreadsheet.add_in_cell(Cell {
            column: reports.headers.columns.len() as u32 + 2,
            row: reports.headers.rows.len() as u32 + 2,
            content: String::from(row.count),
            color: None,
        });
        spreadsheet.set_cell_font_size(
            (
                reports.headers.columns.len() as u32 + 2,
                reports.headers.rows.len() as u32 + 2,
            ),
            18,
        );

        spreadsheet.set_cell(Cell {
            column: 1,
            row: reports.headers.rows.len() as u32 + 4,
            content: String::from("BACKLOG"),
            color: None,
        });
        spreadsheet.set_cell_color(
            (1, reports.headers.rows.len() as u32 + 4),
            Some([0, 176, 80, 255]),
        );
        if BACKLOG.contains(&row.status_notificacao.to_uppercase().as_str()) {
            spreadsheet.add_in_cell(Cell {
                column: 2,
                row: reports.headers.rows.len() as u32 + 4,
                content: String::from(row.count),
                color: None,
            });
        }
        spreadsheet.set_cell_color(
            (2, reports.headers.rows.len() as u32 + 4),
            Some([0, 176, 80, 255]),
        );

        spreadsheet.set_cell(Cell {
            column: 1,
            row: reports.headers.rows.len() as u32 + 5,
            content: String::from("NOTIFICATION"),
            color: None,
        });
        spreadsheet.set_cell_color(
            (1, reports.headers.rows.len() as u32 + 5),
            Some([255, 255, 0, 255]),
        );
        if NOTIFICATION.contains(&row.status_notificacao.to_uppercase().as_str()) {
            spreadsheet.add_in_cell(Cell {
                column: 2,
                row: reports.headers.rows.len() as u32 + 5,
                content: String::from(row.count),
                color: None,
            });
        }
        spreadsheet.set_cell_color(
            (2, reports.headers.rows.len() as u32 + 5),
            Some([255, 255, 0, 255]),
        );

        spreadsheet.set_row_custom_font(reports.headers.rows.len() + 4, "bold");
        spreadsheet.set_row_font_size(reports.headers.rows.len() + 4, 20);
        spreadsheet.set_row_height(reports.headers.rows.len() + 4, 25);

        spreadsheet.set_row_custom_font(reports.headers.rows.len() + 5, "bold");
        spreadsheet.set_row_font_size(reports.headers.rows.len() + 5, 20);
        spreadsheet.set_row_height(reports.headers.rows.len() + 5, 25);
    }

    for (report_name, spreadsheet) in reports.spreadsheets {
        let file_name = format!("{}.png", report_name);
        spreadsheet.save_png(&file_name).unwrap();
    }
}
