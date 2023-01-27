use crate::tools::{self, date::Date};
use std::fs;

pub struct StructuredRow {
    pub due_date: Date,
    pub status: String,
    pub amount: String,
}
impl StructuredRow {
    pub fn new(
        due_date: String,
        status: String,
        amount: String,
    ) -> Result<StructuredRow, &'static str> {
        let due_date = tools::date::parse(due_date)?;
        Ok(StructuredRow {
            due_date,
            status,
            amount,
        })
    }
}

pub fn read_csv(path: &str, filter: &String) -> Result<Vec<StructuredRow>, &'static str> {
    let csv = match fs::read_to_string(path) {
        Ok(csv) => csv,
        Err(_) => return Err("Falha ao tentar ler arquivo .csv"),
    };
    let csv = csv.replace("N??o", "Não");
    let rows: Vec<&str> = csv.split("\n").collect();

    let mut structured: Vec<StructuredRow> = Vec::new();

    for (i, row) in rows[1..].iter().enumerate() {
        if row == &"" || row.contains("Fatura Não Encontrada") {
            continue;
        }
        let cells: Vec<&str> = row.split(";").collect();
        if cells.len() != 4 {
            let error_message =
                format_args!("Coluna {} com células não esperadas:\n{}", i + 1, row)
                    .as_str()
                    .unwrap();
            return Err(&error_message);
        }
        let row_type = cells[2].trim();
        if &row_type.trim().to_lowercase() == filter {
            let structured_row = StructuredRow::new(
                cells[0].trim().to_string(),
                cells[1].trim().to_string(),
                cells[3].trim().to_string(),
            )?;
            structured.push(structured_row);
        }
    }

    Ok(structured)
}
