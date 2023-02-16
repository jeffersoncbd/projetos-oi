use std::{env, process};

use invoices_fiber::{application, Configurations};

fn main() {
    let configurations = match Configurations::from(env::args()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("[Error] Configurations -> {error}");
            process::exit(1);
        }
    };

    if let Err(error) = application::run(configurations) {
        eprintln!("[Error] Application -> {error}");
        process::exit(1);
    }
}
