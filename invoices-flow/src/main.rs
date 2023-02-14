use std::{env, process};

use invoices_flow::{application, Configurations};

fn main() {
    let configurations = match Configurations::from(env::args()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Erro de configuração: {error}");
            process::exit(1);
        }
    };
    if let Err(error) = application::run(configurations) {
        eprintln!("Erro na aplicação: {error}");
        process::exit(1);
    };
}
