use std::fmt::Display;

use chrono::Local;
use color_eyre::owo_colors::OwoColorize;

pub enum LogType {
    System,
    Message,
    CommandRecieved(String),
    CommandResponded(String),
    ClientError,
    Error,
}

impl LogType {
    fn get_string_color(&self) -> String {
        match self {
            Self::System => "[SYS]".bold().to_string(),
            Self::Message => "[MSG]".bright_green().to_string(),
            Self::CommandRecieved(id) => format!("{} - {}", "[CMD]".cyan(), id.purple()),
            Self::CommandResponded(id) => format!("{} - {}", "[RES]".blue(), id.purple()),
            Self::ClientError => "[ERR]".bright_red().to_string(),
            Self::Error => "[ERR]".red().to_string(),
        }
    }
    #[allow(unused)]
    fn get_string(&self) -> String {
        match self {
            Self::System => "[SYS]".to_string(),
            Self::Message => "[MSG]".to_string(),
            Self::CommandRecieved(id) => format!("{} - {}", "[CMD]", id),
            Self::CommandResponded(id) => format!("{} - {}", "[RES]", id),
            Self::ClientError => "[ERR]".to_string(),
            Self::Error => "[ERR]".to_string(),
        }
    }
}

pub fn log<S: AsRef<str> + Display>(content: S, logtype: LogType) {
    let time = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    let logprint = format!("{time:<21} - {} - {content}", logtype.get_string_color());

    println!("{}", logprint)
}