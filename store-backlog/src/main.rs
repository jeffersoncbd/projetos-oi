use std::{env, process};

use store_backlog::{application, Configurations};

fn main() {
    let configurations = match Configurations::from(env::args()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Erro de configuração: {error}");
            process::exit(1);
        }
    };
    if let Err(error) = application::run(configurations) {
        println!("Erro na aplicação: {error}");
        process::exit(1);
    };
}
