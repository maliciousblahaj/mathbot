use std::{fmt::Display, sync::{Arc, RwLock}};
use crate::{bot::Bot, command::{Command, CommandMap}};


impl Bot {
    pub async fn parse_message<S: AsRef<str> + Display>(&self, message: S) -> Option<ParsedCommand> {
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
pub fn parse_command<S: AsRef<str> + Display>(cmd_map: &Arc<RwLock<CommandMap>>, parts: Vec<S>) -> Option<(Box<Command>, Vec<String>, String, Vec<String>, Vec<String>)> {
    let mut parts = parts.iter().map(|arg| arg.to_string()).peekable();
    let alias = parts.next()?;
    let mut command = cmd_map.read().expect("RwLock Poisoned while parsing command").get_command_by_alias(&alias)?.clone();
    let mut commandsequence = Vec::from([command.get_name().to_string()]);
    let mut aliassequence = Vec::from([alias]);
    loop {
        let Some(next) = parts.peek() else {break;};
        let Some(subcommands) = command.get_subcommands() else {break;};
        let rwguard = subcommands.read().expect("RwLock Poisoned while parsing command (2)");
        let newcommand = match rwguard.get_command_by_alias(next){
            Some(cmd) => cmd.clone(),
            None => {break;}
        };

        drop(rwguard);
        command = newcommand;
        
        commandsequence.push(command.get_name().to_string());
        aliassequence.push(parts.next()?);
    }

    let args: Vec<String> = parts.collect();
    let args_str = args.join(" ");

    Some((command, args, args_str, commandsequence, aliassequence))
}


#[derive(Debug)]
pub struct ParsedCommand {
    pub command: Box<Command>,
    pub args: Vec<String>,
    pub args_str: String,
    pub commandsequence: Vec<String>,
    pub aliassequence: Vec<String>,
}

