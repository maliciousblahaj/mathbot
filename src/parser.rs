use std::fmt::Display;

use crate::{bot::Bot, command::{Command, CommandMap}};


impl Bot {
    pub fn parse_message<S: AsRef<str> + Display>(&self, message: S) -> Option<ParsedCommand> {
        let message = message.to_string();
        let prefix = self.get_prefix();
        if !message.starts_with(prefix) {
            return None;
        }

        let parts: Vec<&str> = message.get(prefix.len()..)?.split_whitespace().collect();

        let (command, args, args_str, commandsequence, aliassequence ) = parse_command( self.get_commands(), parts)?;

        Some(ParsedCommand{
            command,
            args,
            args_str,
            commandsequence,
            aliassequence,
        })
    }
}

// Takes a CommandMap of the root and the parts of the command, and splits the command and its args, and also returns some other nice-to-haves
pub fn parse_command<'a, S: AsRef<str> + Display>(cmd_map: &'a CommandMap, parts: Vec<S>) -> Option<(&'a Box<Command>, Vec<String>, String, Vec<String>, Vec<String>)> {
    let mut parts = parts.iter().map(|arg| arg.to_string()).peekable();
    let alias = parts.next()?;
    let mut command = cmd_map.get_command_by_alias(&alias)?;
    let mut commandsequence = Vec::from([command.get_name().to_string()]);
    let mut aliassequence = Vec::from([alias]);
    loop {
        command = match || -> Option<&Box<Command>> {Some(command.get_subcommands()?.get_command_by_alias(parts.peek()?)?)}() {
            //The closure will return None and exit the loop if the next part is not a command 
            None => {break;},
            Some(cmd) => cmd,
        };
        commandsequence.push(command.get_name().to_string());
        aliassequence.push(parts.next()?);
    }

    let args: Vec<String> = parts.collect();
    let args_str = args.join(" ");

    Some((command, args, args_str, commandsequence, aliassequence))
}


#[derive(Debug)]
pub struct ParsedCommand <'a> {
    pub command: &'a Box<Command>,
    pub args: Vec<String>,
    pub args_str: String,
    pub commandsequence: Vec<String>,
    pub aliassequence: Vec<String>,
}

