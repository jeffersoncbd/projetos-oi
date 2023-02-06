use std::{env, process};

use chain_monitor::{application, configurations::Configurations};

fn main() {
    let args = env::args();
    let configurations = match Configurations::new(args) {
        Ok(configurations) => configurations,
        Err(error) => {
            eprintln!("Configuration error: {}", error);
            process::exit(1);
        }
    };
    if let Err(error) = application::run(configurations) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    };
}
