use chrono::{DateTime, Days, Utc};
use spreadsheet_maker::{Cell, Spreadsheet};
use std::collections::HashMap;

use super::history::ExecutionData;

struct Headers {
    pub column: Vec<DateTime<Utc>>,
    pub row: Vec<DateTime<Utc>>,
}
struct Report<'a> {
    reports: HashMap<&'a str, Vec<&'a str>>,
    headers: Headers,
}
impl<'a> Report<'a> {
    fn from(executions: &Vec<ExecutionData>) -> Report {
        let mut column = Vec::new();
        let mut row = Vec::new();
        let mut reports: HashMap<&str, Vec<&str>> = HashMap::new();

        for execution in executions {
            column.push(execution.date.clone());
            let rows: Vec<&str> = execution.content.split("\n").collect();
            for row in rows {
                let cells: Vec<&str> = row.split(";").map(|c| c.trim()).collect();
                match reports.get_mut(cells[2]) {
                    Some(status) => {
                        if !status.contains(&cells[1]) {
                            status.push(cells[1])
                        }
                    }
                    None => {
                        reports.insert(cells[2], vec![cells[1]]);
                    }
                };
            }
        }

        let now = Utc::now();
        for i in 1..11 {
            let row_date = now.checked_add_days(Days::new(i)).unwrap();
            row.push(row_date);
        }

        Report {
            reports,
            headers: Headers { column, row },
        }
    }
}

pub fn structure(executions: Vec<ExecutionData>) -> Result<Spreadsheet, &'static str> {
    let report = Report::from(&executions);
    let mut sub_report: Vec<String> = Vec::new();

    let mut spreadsheet = Spreadsheet::new(String::from("Relatório de escoamento do backlog"));
    spreadsheet.set_margin(10);

    let mut factor: usize = 0;
    let rows_len = report.headers.row.len() + 2;
    for (head, status) in &report.reports {
        for status_name in status {
            let row = ((factor * rows_len) + factor + 1) as u32;

            let title = format!("{head} - {status_name}");
            sub_report.push(title.clone());
            spreadsheet.set_cell(Cell {
                content: title,
                color: None,
                column: 1,
                row,
            });
            spreadsheet.set_cell_font_size((1, row), 20);
            spreadsheet.set_row_custom_font(row as usize, "bold");
            spreadsheet.set_row_height(row as usize, 25);

            for (k, column_header) in report.headers.column.iter().enumerate() {
                let k = k as u32 + 2;
                spreadsheet.set_cell(Cell {
                    content: column_header.format("%d/%m %H:%M").to_string(),
                    column: k,
                    row: row + 1,
                    color: None,
                });
                spreadsheet.set_row_custom_font(row as usize + 1, "bold");
                spreadsheet.set_column_width(k as usize, 100);
            }

            for (k, row_header) in report.headers.row.iter().enumerate() {
                let k = k as u32 + row + 2;
                spreadsheet.set_cell(Cell {
                    content: row_header.format("%d/%m/%y").to_string(),
                    column: 1,
                    row: k,
                    color: None,
                });
                spreadsheet.set_column_width(1, 90);
            }

            factor += 1;
        }
    }

    for execution in &executions {
        let rows = execution.content.split("\n");
        for row in rows {
            let cells: Vec<&str> = row.split(";").map(|c| c.trim()).collect();
            let column_number = report
                .headers
                .column
                .iter()
                .position(|c| c.to_string() == execution.date.to_string())
                .unwrap();

            let row_number = report.headers.row.iter().position(|r| {
                r.format("%d-%b-%y").to_string().to_uppercase() == String::from(cells[0])
            });
            let row_number = match row_number {
                Some(number) => number,
                None => continue,
            };
            let factor = sub_report
                .iter()
                .position(|t| t == &format!("{} - {}", cells[2], cells[1]))
                .unwrap();
            let row_number = ((factor * rows_len) + factor + row_number) as u32;

            spreadsheet.add_in_cell(Cell {
                color: None,
                column: column_number as u32 + 2,
                content: String::from(cells[3]),
                row: row_number + 3,
            });
        }
    }

    Ok(spreadsheet)
}
