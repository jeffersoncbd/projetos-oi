use std::{env, process};

use tg_api::Configurations;

fn main() {
    let telegram_token = String::from("5600687475:AAFtM9o3pcnECnDCMeM42OC0Rb1etknjk_Q");
    let configurations = Configurations::new(telegram_token, None);
    if let Err(error) = bot_oi::run(env::args(), configurations) {
        eprintln!("{error}");
        process::exit(1);
    };
}
