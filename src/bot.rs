use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serenity::{all::{Context, EventHandler, Message}, async_trait};

use crate::logging::log;
use crate::{Result,Error, command::Command};

pub struct Bot {
    prefix: String,
    commands: HashMap<String, Command>,
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
            commands: HashMap::new(),
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
        self.register_command(command).unwrap();

        self
    }
    
    fn register_command(&mut self, command: Command) -> Result<()>{
        let name = command.aliases[0].clone();
        if self.commands.contains_key(&name) {
            return Err(Error::RegisterCommandAlreadyExists);
        }

        self.commands.insert(name, command);
        Ok(())
    }

    pub fn prefix(&self) -> &str{
        &self.prefix
    }

    pub fn commands(&self) -> &HashMap<String, Command> {
        &self.commands
    }
}