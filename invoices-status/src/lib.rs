mod csv;
mod tools;

pub mod application {
    use std::process;

    use crate::csv::read_csv;

    pub fn run() {
        let filter = String::from("e-mail");
        let _structured_rows = match read_csv("model.csv", &filter) {
            Ok(rows) => rows,
            Err(error) => {
                eprintln!("read_csv error: {error}");
                process::exit(1);
            }
        };
    }
}
