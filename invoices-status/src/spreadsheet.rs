use crate::csv::StructuredRow;
use std::collections::HashMap;

pub fn mount(rows: Vec<StructuredRow>) -> HashMap<String, HashMap<String, String>> {
    let mut spreadsheet: HashMap<String, HashMap<String, String>> = HashMap::new();

    for row in rows {
        match spreadsheet.get_mut(&row.status) {
            Some(status) => match status.get_mut(&row.due_date.string) {
                Some(value) => {
                    let part: u64 = row.amount.parse().unwrap();
                    let old_value: u64 = value.parse().unwrap();
                    *value = (old_value + part).to_string();
                }
                None => {
                    status.insert(row.due_date.string, row.amount);
                }
            },
            None => {
                let date = HashMap::from([(row.due_date.string, row.amount)]);
                spreadsheet.insert(row.status, date);
            }
        }
    }

    spreadsheet
}
