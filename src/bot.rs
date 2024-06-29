use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serenity::{all::{Context, EventHandler, Message}, async_trait};

use crate::command::CommandMap;
use crate::logging::log;
use crate::{Result,Error, command::Command};

#[derive(Debug)]
pub struct Bot {
    prefix: String,
    commands: CommandMap,
    pub global: Arc<Mutex<Global>>,
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


#[derive(Debug)]
pub struct Global {
    //TODO: Add modelcontroller to this
}

impl Global {
    pub fn new() -> Self {
        Global {
        }
    }


}



#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(E) = self.handle_message(ctx, msg).await {
            log(E);
        }
    }
}

impl Bot {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            commands: CommandMap::new(),
            global: Arc::new(Mutex::new(
                Global::new()
            )),
        }
    }

    /// Register a command to Bot.commands (builder pattern)
    pub fn register(
        mut self,
        command: Command,
    ) -> Self {
        self.commands.register_command(command).unwrap();

        self
    }


    pub fn get_prefix(&self) -> &str{
        &self.prefix
    }

    pub fn get_commands(&self) -> &CommandMap {
        &self.commands
    }

}