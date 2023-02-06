use std::{collections::HashMap, fs};

use super::string_to_static;

pub fn load(path: &String) -> Result<String, &'static str> {
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            return Err(string_to_static(format!(
                "Falha ao ler arquivo \"{}\":\n{}",
                path, error
            )))
        }
    };

    Ok(content.trim().to_string())
}

pub type Logs<'a> = Vec<HashMap<&'a str, &'a str>>;

pub fn struct_csv(content: &String) -> Result<Logs, &'static str> {
    let rows: Vec<&str> = content.split("\n").collect();
    let headers: Vec<&str> = rows[0].split(";").into_iter().map(|h| h.trim()).collect();
    let mut structured: Logs = Vec::new();
    for (i, row) in rows[2..].iter().enumerate() {
        if row.trim() == "" {
            continue;
        }
        let cells: Vec<&str> = row.split(";").collect();
        if cells.len() != headers.len() {
            let feedback = format!("O .csv é inválido, verifique a linha {}:\n{}", i + 3, row);
            return Err(string_to_static(feedback));
        }
        let mut structured_row: HashMap<&str, &str> = HashMap::new();
        for (j, cell) in cells.iter().enumerate() {
            structured_row.insert(headers[j], cell.trim());
        }
        structured.push(structured_row);
    }
    Ok(structured)
}
