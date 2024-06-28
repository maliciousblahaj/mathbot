use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serenity::{all::{Context, EventHandler, Message}, async_trait};

use crate::logging::log;
use crate::{Result,Error, command::Command};

pub struct Bot {
    global: Arc<Mutex<Global>>,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(E) = self.handle_message(ctx, msg).await {
            
        }
    }
}

impl Bot {
    async fn handle_message(&self, ctx: Context, msg: Message) -> Result<()>{
        log(&msg.content);
        if msg.content == "dev test" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello world!").await {
                println!("Error sending message: {why:?}");
            }
        }
        Ok(())
    }
}


pub struct Global {
    prefix: String,
    commands: HashMap<String, Command>
    //TODO: Add modelcontroller to this
}

impl Global {
    pub fn new(prefix: String) -> Self {
        Global {
            prefix,
            commands: HashMap::new(),
        }
    }

    pub fn register_command(
        &mut self,
        command: Command,
    ) -> Result<()> {
        let name = command.aliases[0].clone();
        if self.commands.contains_key(&name) {
            return Err(Error::RegisterCommandAlreadyExists);
        }

        self.commands.insert(name, command);
        Ok(())
    }
}




impl Bot {
    pub fn new(prefix: &str) -> Self {
        Self {
            global: Arc::new(Mutex::new(
                Global::new(prefix.to_string())
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