use std::process;

use invoices_status::application;

fn main() {
    if let Err(message) = application::run() {
        eprintln!("{message}");
        process::exit(1)
    }
}
