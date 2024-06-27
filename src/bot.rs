use std::sync::{Arc, Mutex};
use serenity::{all::{Context, EventHandler, Message}, async_trait};

use crate::{Result,command::Command};


pub struct Global {
    prefix: String,
    commands: Option<Vec<Command>>,
    //TODO: Add modelcontroller to this
}

impl Global {
    pub fn register_command(
        &mut self,
        command: Command,
    ) -> Result<()> {
        if self.commands.is_none() {
            self.commands = Some(Vec::new());
        }

        self.commands.as_mut().expect("impossible error").push(command);
        Ok(())
    }
}

pub struct Bot {
    global: Arc<Mutex<Global>>,
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
            global: Arc::new(Mutex::new(
                Global {
                    prefix: prefix.to_string(),
                    commands: None,
                }
            ))
        }
    }

    /// Register a command to Bot.commands (builder pattern)
    pub fn register(
        self,
        command: Command,
    ) -> Self {
        self.global.lock().expect("Global Mutex is poisoned (register)")
            .register_command(command).expect("Failed to register command");

        self
    }
}