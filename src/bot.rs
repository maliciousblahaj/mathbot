use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serenity::{all::{Context, EventHandler, Message}, async_trait};

use crate::logging::log;
use crate::{Result,Error, command::Command};

#[derive(Debug)]
pub struct Bot {
    prefix: String,
    commands: HashMap<String, Command>,
    command_map: HashMap<String, String>,
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
            commands: HashMap::new(),
            command_map: HashMap::new(),
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
        let name = command.get_aliases()[0].clone();
        if self.commands.contains_key(&name) {
            return Err(Error::RegisterCommandAlreadyExists);
        }
        for alias in command.get_aliases(){
            if self.command_map.contains_key(alias) {
                return Err(Error::RegisterAliasAlreadyExists)
            }

            self.command_map.insert(alias.clone(), name.clone());
        }
        self.commands.insert(name, command);
        Ok(())
    }

    pub fn get_prefix(&self) -> &str{
        &self.prefix
    }

    pub fn get_commands(&self) -> &HashMap<String, Command> {
        &self.commands
    }

    pub fn get_command <S: AsRef<str> + Display>(&self, name: S) -> Option<&Command> {
        self.commands.get(&name.to_string())
    }

    pub fn get_command_map(&self) -> &HashMap<String, String> {
        &self.command_map
    }

    pub fn get_command_by_alias <S: AsRef<str> + Display>(&self, name: S) -> Option<&Command> {
        self.get_command(self.command_map.get(&name.to_string())?)
    }


}