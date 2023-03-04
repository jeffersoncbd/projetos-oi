use std::{env, process};

use telegram_observer::{application, Configurations};

fn main() {
    let configurations = match Configurations::from(env::args()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("[Erro] Configurações -> {error}");
            process::exit(1);
        }
    };

    if let Err(error) = application::run(configurations) {
        eprintln!("[Erro] Aplicação -> {error}");
        process::exit(1);
    }
}
