use std::collections::HashMap;

use crate::structs::{DataUnity, StructuredCSVRow};

pub fn mount<'a>(
    unfiltered_csv: &'a Vec<StructuredCSVRow<'a>>,
    data_type: &str,
) -> HashMap<String, HashMap<String, String>> {
    let mut csv: Vec<&DataUnity> = Vec::new();

    for row in unfiltered_csv {
        match data_type {
            "email" => {
                if let StructuredCSVRow::Email(item) = row {
                    csv.push(item);
                }
            }
            "whatsapp" => {
                if let StructuredCSVRow::Whatsapp(item) = row {
                    csv.push(item);
                }
            }
            _ => panic!("The filters accepted are \"email\" and \"whatsapp\""),
        };
    }

    let mut structured_csv: HashMap<String, HashMap<String, String>> = HashMap::new();

    for row in &csv {
        match structured_csv.get_mut(row.status) {
            Some(status) => {
                match status.get_mut(&row.due_date.string) {
                    Some(value) => {
                        let last_amount: u64 = value.parse().unwrap();
                        let part: u64 = row.amount.parse().unwrap();
                        *value = (last_amount + part).to_string();
                    }
                    None => {
                        status.insert(row.due_date.string.clone(), row.amount.to_string());
                    }
                };
            }
            None => {
                let mut hash = HashMap::new();
                hash.insert(row.due_date.string.clone(), row.amount.to_string());
                structured_csv.insert(row.status.to_string(), hash);
            }
        };
    }

    structured_csv.iter().max_by(|a, b| a.0.cmp(&b.0));

    structured_csv
}
