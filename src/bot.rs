use crate::{command::{Command, CommandParams}, Error, Result};
use serenity::all::{Context, Message};


pub struct Bot {
    prefix: String,
    commands: Option<Vec<Command>>,
}

impl Bot {
    fn new(prefix: String) -> Self {
        Self {
            prefix,
            commands: None,
        }
    }

    /// Register a command on the Bot struct
    fn register(
        &mut self,
        handle: fn(CommandParams) -> Result<()>, 
        aliases: Vec<String>,
    ) -> Result<()> {
        let command = Command::new(handle, aliases);

        if self.commands.is_none() {
            self.commands = Some(Vec::new());
        }

        self.commands.as_mut().expect("Failed to append command to Bot.commands while registring")
            .push(command);

        Ok(())
    }
}