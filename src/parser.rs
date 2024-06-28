use std::fmt::Display;

use crate::{bot::Bot, command::Command};


impl Bot {
    fn parse_message<S: AsRef<str> + Display>(&self, message: S) -> Option<ParsedCommand> {
        let message = message.to_string();
        if !message.starts_with(self.get_prefix()) {
            return None;
        }

        let mut parts = message[1..].split_whitespace().peekable();
        
        let command = self.get_command(parts.next()?)?;
        loop {
            //i hope the question marks here will just exit the loop, not the function
            // oh no, it did in fact exit the function :(
            let next = command.get_subcommand(parts.peek()?)?; 
            
        }

        None
    }
}




struct ParsedCommand <'a> {
    originalmessage: String,
    args: Vec<String>,
    command: &'a Command,
}