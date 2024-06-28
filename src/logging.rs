use std::fmt::Display;

use chrono::Utc;

pub fn log<S: AsRef<str> + Display>(content: S) {
    let time = Utc::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    let log = format!("{time:<21} - {content}");

    println!("{}", log)
}