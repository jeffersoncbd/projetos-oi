use std::{thread, time::Duration};

use tg_api::{Client, Configurations, FileMessage, ImageMessage, TextMessage, Value};

pub fn run(
    mut args: impl Iterator<Item = String>,
    configurations: Configurations,
) -> Result<(), &'static str> {
    args.next();

    let telegram = Client::new(configurations);

    let command_or_id = match args.next() {
        Some(arg) => arg,
        None => return Err("Use \"watch\" ou informe o ID destinatário (em $1)."),
    };

    match command_or_id.as_str() {
        "watch" => {
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

            println!("Aguardando mensagens...\n");
            for _ in 1..60 {
                thread::sleep(Duration::from_secs(1));
                let response_chat = telegram.get_update(Some(last_update))?;
                let mut i = 0;
                loop {
                    if response_chat["result"][i] == Value::Null {
                        break;
                    }
                    let item = response_chat["result"][i].clone();

                    if last_update != item["update_id"].as_u64().unwrap() {
                        let chat_type = item["message"]["chat"]["type"].clone();
                        let chat_id = item["message"]["chat"]["id"].clone();
                        let chat_name: Value = match &chat_type {
                            Value::String(chat_type) => match chat_type.as_str() {
                                "private" => item["message"]["chat"]["first_name"].clone(),
                                "supergroup" => item["message"]["chat"]["title"].clone(),
                                "group" => item["message"]["chat"]["title"].clone(),
                                _ => panic!("type of chat is invalid!"),
                            },
                            _ => panic!("type of chat is invalid!"),
                        };
                        let chat_message = item["message"]["text"].clone();
                        println!("[{} | {}] {}:", chat_type, chat_id, chat_name);
                        println!("{}\n", chat_message);

                        last_update = item["update_id"].clone().as_u64().unwrap();
                    }
                    i += 1;
                }
            }
            println!("encerrado!");
        }
        _ => match args.next() {
            Some(message_or_flag) => match message_or_flag.as_str() {
                "-i" => {
                    let image_path = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Informe o caminho da imagem (em $3)."),
                    };
                    telegram.send_image(ImageMessage {
                        image_path: &image_path,
                        to: &command_or_id,
                    })?;
                }
                "-f" => {
                    let file_path = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Informe o caminho do arquivo (em $3)."),
                    };
                    telegram.send_file(FileMessage {
                        file_path: &file_path,
                        to: &command_or_id,
                    })?;
                }
                _ => {
                    telegram.send_text(TextMessage {
                        content: &message_or_flag,
                        to: &command_or_id,
                    })?;
                }
            },
            None => return Err(
                "Entre com uma mensagem ou flag (em $2)\n  -i  para imagens\n  -f  para arquivos).",
            ),
        }, // _ => return Err("O comando informado não é aceito, utilize:\n- message\n- image\n- watch"),
    };
    Ok(())
}
