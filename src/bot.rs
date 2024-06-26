use crate::{command::Command, Error, Result};

pub struct Bot {
    prefix: String,
    commands: Option<Vec<Command>>,
}

impl Bot {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            commands: None,
        }
    }

    /// Register a command to Bot.commands (builder pattern)
    pub fn register(
        &mut self,
        command: Command,
    ) -> &mut Self {
        if self.commands.is_none() {
            self.commands = Some(Vec::new());
        }

        self.commands.as_mut().expect("Failed to append command to Bot.commands")
            .push(command);

        self
    }
}