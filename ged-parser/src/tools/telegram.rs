use reqwest::blocking::{self, multipart};
use std::{collections::HashMap, env};

pub fn send_text(text: String, to: &'static str) {
    let telegram_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("Failed to load \"TELEGRAM_BOT_TOKEN\" in the \".env\" file");
    let chat_id = env::var(to).expect("Failed to load \"{to}\" in the \".env\" file");

    let client = blocking::Client::new();

    let map = HashMap::from([("chat_id", chat_id), ("text", text)]);

    let request = client
        .post(format!(
            "https://api.telegram.org/bot{}/sendMessage",
            telegram_token
        ))
        .json(&map);
    request
        .send()
        .expect("Failed to send the request to telegram.");
}

pub fn send_image(file_name: String, to: &'static str) {
    let telegram_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("Failed to load \"TELEGRAM_BOT_TOKEN\" in the \".env\" file");
    let chat_id = env::var(to).expect("Failed to load \"{to}\" in the \".env\" file");

    let client = blocking::Client::new();

    let form = multipart::Form::new()
        .text("chat_id", chat_id)
        .file("photo", &file_name)
        .expect(&format!("Failed to load the file \"{}\"", &file_name));

    let request = client
        .post(format!(
            "https://api.telegram.org/bot{}/sendPhoto",
            telegram_token
        ))
        .multipart(form);
    request
        .send()
        .expect("Failed to send the request to telegram.");
}
