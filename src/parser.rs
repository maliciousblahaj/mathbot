use std::fmt::Display;

use crate::{bot::Bot, command::Command, Result};


impl Bot {
    pub fn parse_message<S: AsRef<str> + Display>(&self, message: S) -> Option<ParsedCommand> {
        let message = message.to_string();
        if !message.starts_with(self.get_prefix()) {
            return None;
        }

        let mut parts = message[1..].split_whitespace().map(|arg| arg.to_string()).peekable();

        let mut command = self.get_command(parts.next()?)?;
        loop {

            command = match (|| -> Option<&Box<Command>> {Some(command.get_subcommand(parts.peek()?)?)}()) {
                None => {break;},
                Some(cmd) => cmd.as_ref(),
            };
            parts.next();
        }

        let args: Vec<String> = parts.collect();

        Some(ParsedCommand{
            originalmessage: message,
            args,
            command,
        })
    }
}



#[derive(Debug)]
pub struct ParsedCommand <'a> {
    originalmessage: String,
    args: Vec<String>,
    command: &'a Command,
}

