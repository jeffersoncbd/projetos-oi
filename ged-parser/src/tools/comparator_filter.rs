use std::fs;

use crate::structs::StructuredCSVRow;

const HEADERS: [&str; 3] = [
    "Aguardando Fatura",
    "Aguardando Fatura Resumida",
    "Carregada",
];

pub fn filter_and_save(csv: &Vec<StructuredCSVRow>) {
    let mut filtered = Vec::new();

    for row in csv {
        match row {
            StructuredCSVRow::Email(email_row) => {
                if HEADERS.contains(&email_row.status) {
                    filtered.push(row.clone());
                }
            }
            StructuredCSVRow::Whatsapp(whatsapp_row) => {
                if HEADERS.contains(&whatsapp_row.status) {
                    filtered.push(row.clone());
                }
            }
        };
    }

    let mut new_csv = String::new();

    for row in filtered {
        match row {
            StructuredCSVRow::Email(row) => {
                new_csv = format!(
                    "{}\n{};{};E-MAIL;{}",
                    new_csv, row.due_date.string, row.status, row.amount
                );
            }
            StructuredCSVRow::Whatsapp(row) => {
                new_csv = format!(
                    "{}\n{};{};WHATSAPP;{}",
                    new_csv, row.due_date.string, row.status, row.amount
                );
            }
        }
    }

    fs::write(
        "last_execution.csv",
        format!("VENCIMENT;STATUS;TIPO;QUANTIDADE\n{}", new_csv),
    )
    .expect("msg");
}
