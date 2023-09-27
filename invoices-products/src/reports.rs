use chrono::{DateTime, TimeZone, Utc};
use spreadsheet_maker::{Cell, Spreadsheet};

use crate::data::Structure;

pub fn string_date_to_utc(date: &String) -> DateTime<Utc> {
    let year: i32 = date[6..8].parse().unwrap();
    let month: u32 = date[3..5].parse().unwrap();
    let day: u32 = date[0..2].parse().unwrap();
    Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}

pub fn mount(
    data: &Vec<Structure>,
    product_type: &str,
) -> Result<Option<Spreadsheet>, &'static str> {
    let filtered_data: Vec<&Structure> =
        data.iter().filter(|row| row.tipo == product_type).collect();

    let mut status: Vec<&String> = Vec::new();
    for row in &filtered_data {
        if !status.contains(&&row.status) {
            status.push(&row.status);
        }
    }
    if status.len() == 0 {
        return Ok(None);
    }
    if status.len() > 1 {
        return Err("a quantidade de status no relatório de produtos foi maior que 1, isso requer uma atualização no bot!");
    }

    let spreadsheet_title = format!("[Produtos] Verificação E-Mail Seguro - {}", product_type);
    let mut spreadsheet = Spreadsheet::new(spreadsheet_title);
    spreadsheet.set_margin(10);
    spreadsheet.set_cell(Cell {
        column: 1,
        row: 1,
        content: String::from(status[0]),
        color: None,
    });
    spreadsheet.set_cell_custom_font((1, 1), "bold");
    spreadsheet.set_cell_font_size((1, 1), 18);
    spreadsheet.set_row_height(1, 25);

    let mut columns_headers: Vec<&String> = Vec::new();
    for row in &filtered_data {
        if !columns_headers.contains(&&row.venciment) {
            columns_headers.push(&row.venciment);
        }
    }
    columns_headers.sort_by(|a, b| string_date_to_utc(a).cmp(&string_date_to_utc(b)));
    let mut rows_headers: Vec<&String> = Vec::new();
    for row in &filtered_data {
        if !rows_headers.contains(&&row.produto) {
            rows_headers.push(&row.produto)
        }
    }
    rows_headers.sort();

    let column_to_columns_headers: u32 = 2;
    let row_to_rows_headers: u32 = 3;

    for (i, header) in columns_headers.iter().enumerate() {
        spreadsheet.set_cell(Cell {
            column: i as u32 + column_to_columns_headers,
            row: 2,
            content: String::from(header.clone()),
            color: None,
        });
    }
    for (i, header) in rows_headers.iter().enumerate() {
        spreadsheet.set_cell(Cell {
            column: 1,
            row: i as u32 + row_to_rows_headers,
            content: String::from(header.clone()),
            color: None,
        })
    }
    spreadsheet.set_cell(Cell {
        column: columns_headers.len() as u32 + 2,
        row: 2,
        content: String::from("Total Geral"),
        color: None,
    });
    spreadsheet.set_cell(Cell {
        column: 1,
        row: rows_headers.len() as u32 + 3,
        content: String::from("Total Geral"),
        color: None,
    });

    for data in filtered_data {
        let column_number = columns_headers
            .iter()
            .position(|h| h == &&data.venciment)
            .unwrap();
        let row_number = rows_headers
            .iter()
            .position(|h| h == &&data.produto)
            .unwrap();

        let amount: u32 = data.quantidade.parse().unwrap();
        let color = if amount > 1000 {
            Some([255, 0, 0, 255])
        } else if amount > 700 {
            Some([255, 153, 0, 255])
        } else if amount > 400 {
            Some([255, 255, 0, 255])
        } else {
            None
        };
        spreadsheet.add_in_cell(Cell {
            column: column_number as u32 + 2,
            row: row_number as u32 + 3,
            content: String::from(data.quantidade.clone()),
            color,
        });
        spreadsheet.add_in_cell(Cell {
            column: columns_headers.len() as u32 + 2,
            row: row_number as u32 + 3,
            content: String::from(data.quantidade.clone()),
            color: None,
        });
        spreadsheet.add_in_cell(Cell {
            column: column_number as u32 + 2,
            row: rows_headers.len() as u32 + 3,
            content: String::from(data.quantidade.clone()),
            color: None,
        });
        spreadsheet.add_in_cell(Cell {
            column: columns_headers.len() as u32 + 2,
            row: rows_headers.len() as u32 + 3,
            content: String::from(data.quantidade.clone()),
            color: None,
        });
    }

    let min_width: usize = if columns_headers.len() < 4 {
        spreadsheet.set_cell(Cell {
            column: 6,
            row: 1,
            content: String::from(""),
            color: None,
        });
        6
    } else {
        columns_headers.len() + 2
    };
    spreadsheet.set_column_width(1, 110);
    for i in 2..min_width {
        spreadsheet.set_column_width(i, 90)
    }
    spreadsheet.set_column_width(columns_headers.len() + 2, 90);

    spreadsheet.set_row_custom_font(2, "bold");
    spreadsheet.set_row_custom_font(rows_headers.len() + 3, "bold");
    spreadsheet.set_column_custom_font(columns_headers.len() + 2, "bold");

    Ok(Some(spreadsheet))
}
