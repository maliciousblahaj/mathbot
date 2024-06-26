use serenity::{all::{Context, EventHandler, Message}, async_trait};

use crate::{command::Command, Error, Result};

pub struct Bot {
    prefix: String,
    commands: Option<Vec<Command>>,
    //TODO: Add modelcontroller to this
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content == "dev test" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello world!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
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