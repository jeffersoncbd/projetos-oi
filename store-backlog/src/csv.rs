use std::fs;

use chrono::Utc;

use crate::tools::string_to_static;

const BACKLOG: [&str; 3] = [
    "Aguardando Fatura",
    "Aguardando Fatura Resumida",
    "Carregada",
];

pub fn read(path: &String) -> Result<String, &'static str> {
    let csv_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            let feedback = format!("Falha ao ler arquivo do destino \"{}\" - {}", path, error);
            return Err(string_to_static(feedback));
        }
    };
    Ok(csv_content)
}

pub fn structure_backlog(csv: &String) -> Result<String, &'static str> {
    let mut backlog_csv: Vec<&str> = Vec::new();

    let rows: Vec<&str> = csv.split("\n").collect();
    for row in rows {
        let columns: Vec<&str> = row.split(";").map(|c| c.trim()).collect();
        if columns.len() != 4 {
            continue;
        }
        if BACKLOG.contains(&columns[1]) {
            backlog_csv.push(row);
        }
    }

    Ok(String::from(backlog_csv.join("\n")))
}

pub fn save(content: String, destiny_path: &String) -> Result<(), &'static str> {
    let now = Utc::now();
    let file_name = format!("{}.csv", now.to_string());
    if let Err(error) = fs::write(format!("{}/{}", destiny_path, file_name), content) {
        let feedback = format!(
            "Falha ao tentar ao tentar salvar o arquivo na pasta \"{}\" - {}",
            destiny_path, error
        );
        return Err(string_to_static(feedback));
    };
    Ok(())
}
