use std::{process::Command, thread, time::Duration};

use tg_api::{Client, Configurations as TgConfigs, Value};

use crate::Configurations;

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let telegram = Client::new(TgConfigs::new(
        String::from("5600687475:AAFtM9o3pcnECnDCMeM42OC0Rb1etknjk_Q"),
        None,
    ));

    let response = telegram.get_update(None)?;
    let mut last_update: u64 = 0;
    let mut i = 0;
    loop {
        if response["result"][i] == Value::Null {
            break;
        }
        last_update = response["result"][i]["update_id"].clone().as_u64().unwrap();
        i += 1;
    }
    loop {
        thread::sleep(Duration::from_secs(1));
        let response_chat = telegram.get_update(Some(last_update))?;
        let mut i = 0;
        loop {
            if response_chat["result"][i] == Value::Null {
                break;
            }
            let item = response_chat["result"][i].clone();

            if last_update != item["update_id"].as_u64().unwrap() {
                let chat_id = item["message"]["chat"]["id"].clone().as_i64().unwrap();
                let chat_message = String::from(item["message"]["text"].clone().as_str().unwrap());

                for command in configurations.commands.iter() {
                    if chat_message.contains(&command.name) {
                        if command.allowed_chats.contains(&chat_id) {
                            let arguments: Vec<&str> = command.script.split(" ").collect();
                            let mut command = Command::new(arguments[0]);
                            for argument in arguments[1..].iter() {
                                command.arg(argument);
                            }
                            command.spawn().unwrap();
                            break;
                        }
                    }
                }

                last_update = item["update_id"].clone().as_u64().unwrap();
            }
            i += 1;
        }
    }
}
