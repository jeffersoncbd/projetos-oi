use chrono::{DateTime, Local};

use crate::tools::telegram;

pub fn send_title_of_general_spreadsheets() {
    let local_now: DateTime<Local> = Local::now();
    let time = local_now.format("%H:%M");
    let text = format!("Verificação E-Mail Seguro executado às {} H", time);
    telegram::send_text(String::from(text), "GENERAL_CHAT_ID");
}
