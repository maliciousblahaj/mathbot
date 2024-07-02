use std::fmt::Display;

use chrono::Local;


//TODO: Implement things like color logging for system messages and errors
/*
macro_rules! log {
    () => {
        
    };
}
*/

pub fn log<S: AsRef<str> + Display>(content: S) {
    let time = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    let log = format!("{time:<21} - {content}");

    println!("{}", log)
}