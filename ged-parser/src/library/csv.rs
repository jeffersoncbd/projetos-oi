use std::fs;

use crate::structs::{DataUnity, StructuredCSVRow};

pub fn read(path: &str) -> String {
    let bytes = fs::read(path).expect("It failed to read the file.");
    let content: String = String::from_utf8_lossy(&bytes)
        .parse()
        .expect("It failed to convert bytes in text.");

    content.replace("N??o", "Não")
}

pub fn estruture(csv: &String) -> Vec<StructuredCSVRow> {
    let rows: Vec<&str> = csv.split("\n").collect();
    let mut structured: Vec<StructuredCSVRow> = Vec::new();
    for (i, row) in rows[1..].iter().enumerate() {
        if row == &"" || row.contains("Fatura Não Encontrada") {
            continue;
        }

        let cells: Vec<&str> = row.split(";").collect();
        if cells.len() == 4 {
            let row_type = cells[2].trim();
            let data_unity = DataUnity::new(cells[0].trim(), cells[1].trim(), cells[3].trim());

            match row_type {
                "WHATSAPP" => structured.push(StructuredCSVRow::Whatsapp(data_unity)),
                "E-MAIL" => structured.push(StructuredCSVRow::Email(data_unity)),
                _ => panic!(
          "It failed to structure the CSV because the type \"{}\" (column 3) is not expected.",
          row_type
        ),
            }
        } else {
            panic!("Row {} with unexpected content:\n  {}", i + 2, row);
        }
    }

    structured
}
