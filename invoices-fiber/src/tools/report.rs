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

struct Report<'a> {
    pub rows: Vec<&'a str>,
    pub spreadsheet: Spreadsheet,
    pub totals: (u32, u32),
}
struct Reports<'a> {
    pub column_headers: Vec<&'a str>,
    pub reports: HashMap<&'a str, Report<'a>>,
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

    let mut reports = HashMap::new();

    for report_name in report_names {
        let spreadsheet = Spreadsheet::new(format!(
            "[Nova Fibra] Relatório de faturas - {}",
            report_name
        ));
        let mut spreadsheet_rows = Vec::new();
        let mut totals = (0, 0);

        for row in rows {
            if row.tipo_pro == report_name {
                if !spreadsheet_rows.contains(&row.status_notificacao) {
                    spreadsheet_rows.push(&row.status_notificacao)
                }

                // Caso valor atual seja BACKLOG inclui no total
                if BACKLOG.contains(&row.status_notificacao.to_uppercase().as_str()) {
                    let current_value: u32 = row.count.parse().unwrap();
                    totals.0 = totals.0 + current_value;
                }
                // Caso valor atual seja NOTIFICATION inclui no total
                if NOTIFICATION.contains(&row.status_notificacao.to_uppercase().as_str()) {
                    let current_value: u32 = row.count.parse().unwrap();
                    totals.1 = totals.1 + current_value;
                }
            }
        }
        spreadsheet_rows.sort();

        reports.insert(
            report_name,
            Report {
                rows: spreadsheet_rows,
                spreadsheet,
                totals,
            },
        );
    }

    Reports {
        column_headers,
        reports,
    }
}

pub fn print(rows: Vec<Row>) {
    let mut reports = extract_reports(&rows);

    for (report_type, report) in &mut reports.reports {
        let backlog_row_number = report.rows.len() + 4;
        let notification_row_number = backlog_row_number + 1;
        let row_headers_column_number = 1;
        let column_headers_row_number = 1;

        // Imprime todos os títulos das colunas
        for (i, header) in reports.column_headers.iter().enumerate() {
            let column_number = i as u32 + 2;
            report.spreadsheet.set_cell(Cell {
                column: column_number,
                row: column_headers_row_number,
                content: String::from(header.clone()),
                color: None,
            });
            report.spreadsheet.set_column_width(i + 2, 80);
        }
        // Ajusta largura da coluna de títulos
        report
            .spreadsheet
            .set_column_width(row_headers_column_number, 230);

        // Imprime todos os títulos das linhas
        for (i, header) in report.rows.iter().enumerate() {
            let row_number = i as u32 + 2;

            report.spreadsheet.set_cell(Cell {
                column: row_headers_column_number as u32,
                row: row_number,
                content: String::from(header.clone()),
                color: None,
            });

            if report_type != &"Sem tipo" {
                // Pinta de verde ou amarelo se título for backlog ou notification
                if BACKLOG.contains(&header.to_uppercase().as_str()) {
                    report
                        .spreadsheet
                        .set_row_color(row_number as usize, Some([0, 176, 80, 255]));
                }
                if NOTIFICATION.contains(&header.to_uppercase().as_str()) {
                    report
                        .spreadsheet
                        .set_row_color(row_number as usize, Some([255, 255, 0, 255]));
                };
            };
        }

        if report_type == &"Sem tipo" {
            continue;
        }

        // Zera célula do total de backlog
        report.spreadsheet.add_in_cell(Cell {
            column: 2,
            row: backlog_row_number as u32,
            content: String::from("0"),
            color: None,
        });
        // Zera célula do total de notification
        report.spreadsheet.add_in_cell(Cell {
            column: 2,
            row: notification_row_number as u32,
            content: String::from("0"),
            color: None,
        });

        // Imprime título backlog
        report.spreadsheet.set_cell(Cell {
            column: row_headers_column_number as u32,
            row: backlog_row_number as u32,
            content: String::from("BACKLOG"),
            color: None,
        });
        // Imprime total backlog
        report.spreadsheet.add_in_cell(Cell {
            column: row_headers_column_number as u32 + 1,
            row: backlog_row_number as u32,
            content: report.totals.0.to_string(),
            color: None,
        });

        // Imprime título notification
        report.spreadsheet.set_cell(Cell {
            column: row_headers_column_number as u32,
            row: notification_row_number as u32,
            content: String::from("NOTIFICATION"),
            color: None,
        });
        // Imprime total notification
        report.spreadsheet.add_in_cell(Cell {
            column: row_headers_column_number as u32 + 1,
            row: notification_row_number as u32,
            content: report.totals.1.to_string(),
            color: None,
        });

        // Formata total BACKLOG
        {
            report.spreadsheet.set_cell_color(
                (row_headers_column_number as u32, backlog_row_number as u32),
                Some([0, 176, 80, 255]),
            );
            report.spreadsheet.set_cell_color(
                (
                    row_headers_column_number as u32 + 1,
                    backlog_row_number as u32,
                ),
                Some([0, 176, 80, 255]),
            );
        }
        // Formata total NOTIFICATION
        {
            report.spreadsheet.set_cell_color(
                (
                    row_headers_column_number as u32,
                    notification_row_number as u32,
                ),
                Some([255, 255, 0, 255]),
            );
            report.spreadsheet.set_cell_color(
                (
                    row_headers_column_number as u32 + 1,
                    notification_row_number as u32,
                ),
                Some([255, 255, 0, 255]),
            );
        }
    }

    for row in &rows {
        let report = reports.reports.get_mut(row.tipo_pro).unwrap();
        report.spreadsheet.set_margin(10);

        // ###################### POSIÇÕES DA TABELA ###################### //
        let row_headers_column_number: usize = 1;
        let column_headers_row_number: usize = 1;

        let total_column_number = reports.column_headers.len() + 2;
        let total_row_number = report.rows.len() + 2;

        let this_row_number = report
            .rows
            .iter()
            .position(|r| r == &row.status_notificacao)
            .unwrap() as u32
            + 2;

        let this_column_number = reports
            .column_headers
            .iter()
            .position(|h| h == &row.data_venc)
            .unwrap() as u32
            + 2;
        // ################################################################ //

        // Adiciona na valor atual na célula correspondente
        report.spreadsheet.add_in_cell(Cell {
            column: this_column_number,
            row: this_row_number,
            content: String::from(row.count),
            color: None,
        });

        // Imprime título da coluna Total Geral
        report.spreadsheet.set_cell(Cell {
            column: total_column_number as u32,
            row: column_headers_row_number as u32,
            content: String::from("TOTAL GERAL"),
            color: None,
        });
        // Adiciona total da linha atual na coluna Total Geral
        report.spreadsheet.add_in_cell(Cell {
            column: total_column_number as u32,
            row: this_row_number as u32,
            content: String::from(row.count),
            color: None,
        });
        // Formata coluna Total Geral
        report
            .spreadsheet
            .set_column_width(total_column_number, 110);

        // Imprime título da linha TOTAL GERAL
        report.spreadsheet.set_cell(Cell {
            column: row_headers_column_number as u32,
            row: total_row_number as u32,
            content: String::from("TOTAL GERAL"),
            color: None,
        });
        // Adiciona total da coluna atual na linha Total Geral
        report.spreadsheet.add_in_cell(Cell {
            column: this_column_number,
            row: total_row_number as u32,
            content: String::from(row.count),
            color: None,
        });

        // Adiciona total geral de tudo
        report.spreadsheet.add_in_cell(Cell {
            column: total_column_number as u32,
            row: total_row_number as u32,
            content: String::from(row.count),
            color: None,
        });
    }

    for (report_name, report) in reports.reports {
        let file_name = format!("{}.png", report_name);
        report.spreadsheet.save_png(&file_name).unwrap();
    }
}
