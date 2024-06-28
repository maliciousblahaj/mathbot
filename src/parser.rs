use std::fmt::Display;

use crate::{bot::Bot, command::Command};


impl Bot {
    fn parse_message<S: AsRef<str> + Display>(&self, message: S) -> Option<ParsedCommand> {
        let message = message.to_string();
        if !message.starts_with(self.prefix()) {
            return None;
        }

        let mut parts = message[1..].split_whitespace();
        
        None
    }
}




struct ParsedCommand <'a> {
    originalmessage: String,
    args: Vec<String>,
    command: &'a Command,
}