use crate::tools::{self, date::Date, format_adapter};
use std::fs;

pub struct StructuredRow {
    pub due_date: Date,
    pub status: String,
    pub amount: String,
}
impl StructuredRow {
    pub fn new(
        due_date: Date,
        status: String,
        amount: String,
    ) -> Result<StructuredRow, &'static str> {
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
        Err(_) => {
            let feedback = format!(
                "Falha ao tentar ler arquivo .csv no caminho:\n  \"{}\"",
                path
            );
            return Err(format_adapter(feedback));
        }
    };
    let csv = csv.replace("N??o", "Não");
    let rows: Vec<&str> = csv.split("\n").collect();

    let mut structured: Vec<StructuredRow> = Vec::new();

    for (i, row) in rows[1..].iter().enumerate() {
        let row_number = i + 2;
        if row == &"" || row.contains("Fatura Não Encontrada") {
            continue;
        }
        let cells: Vec<&str> = row.split(";").collect();
        if cells.len() != 4 {
            let feedback = format!("Linha {} número incorreto de colunas:\n{}", row_number, row);
            return Err(format_adapter(feedback));
        }
        let row_type = cells[2].trim();
        if &row_type.trim().to_lowercase() == filter {
            let due_date = match tools::date::parse(cells[0].trim().to_string()) {
                Ok(value) => value,
                Err(error) => {
                    let feedback = format!(
                        "Falha ao carregar célula da linha {} e coluna VENCIMENT: {}:\n  \"{}\"",
                        row_number, error, row
                    );
                    return Err(format_adapter(feedback));
                }
            };

            let _parsed: u64 = match cells[3].trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    let feedback = format!(
                        "Falha ao carregar célula da linha {} e coluna QUANTIDADE:\n{}",
                        row_number, row
                    );
                    return Err(format_adapter(feedback));
                }
            };

            let structured_row = StructuredRow::new(
                due_date,
                cells[1].trim().to_string(),
                cells[3].trim().to_string(),
            )?;
            structured.push(structured_row);
        }
    }

    Ok(structured)
}
