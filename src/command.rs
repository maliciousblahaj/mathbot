use core::fmt;
use std::{collections::HashMap, fmt::Display, future::Future};

use crate::{Error, Result};
use serenity::{all::{Context, Message}, futures::future::BoxFuture};

/// The category a root command can have.
/// 
/// Having the category of Admin means the command only generates in the help menu for admins
/// and having the category of Test means the command does not show up in the help menu at all
#[derive(Debug)]
pub enum CommandCategory {
    Info,
    User,
    Currency,
    Fun,
    Math,
    Admin,
    Test,
}

/// A command can either be a root command or a subcommand.
/// 
/// Root commands have a category assigned to them, but subcommands don't.
/// Commands form a tree structure, where every command, root or sub, can have a subcommand
#[derive(Debug)]
pub enum CommandType {
    RootCommand {category: CommandCategory},
    SubCommand,
}


type CommandHandler = Box<dyn FnMut(CommandParams) -> BoxFuture<'static, Result<()>> + Send + Sync>;

//TODO: add documentation for commands (for help menu)
/// A Command's name is the 0th element of the aliases vector
pub struct Command
{
    handle: CommandHandler,
    aliases: Vec<String>,
    cmd_type: CommandType,
    subcommands: Option<HashMap<String, Box<Command>>>,
}

impl Command
{
    pub fn new<T> (
        handle: fn(CommandParams) -> T, 
        aliases: Vec<String>, 
        cmd_type: CommandType,
    ) -> Self
    where 
        T: Future<Output = Result<()>> + 'static + Send + Sync,
    {
        let handle: CommandHandler = Box::new(move |params| {Box::pin(handle(params))});
        Self {
            handle: Box::new( handle ),
            aliases,
            cmd_type,
            subcommands: None,
        }
    }

    pub async fn run(&mut self, params: CommandParams) -> Result<()> {
        (self.handle)(params).await
    }

    pub fn get_aliases(&self) -> &Vec<String> {
        &self.aliases
    }

    pub fn get_cmd_type(&self) -> &CommandType {
        &self.cmd_type
    }

    pub fn get_subcommands(&self) -> Option<&HashMap<String, Box<Command>>> {
        self.subcommands.as_ref()
    }

    pub fn get_subcommand <S: AsRef<str> + Display>(&self, name: S) -> Option<&Box<Command>> {
        self.get_subcommands()?.get(&name.to_string())
    }

    pub fn register(
        mut self,
        command: Command,
    ) -> Self {
        self.register_subcommand(command).unwrap();

        self
    }
    
    fn register_subcommand(&mut self, command: Command) -> Result<()>{
        if self.subcommands.is_none() {
            self.subcommands = Some(HashMap::new());
        }

        let subcommands = self.subcommands.as_mut().unwrap();

        let name = command.get_aliases()[0].clone();
        if subcommands.contains_key(&name) {
            return Err(Error::RegisterSubcommandAlreadyExists);
        }

        subcommands.insert(name, Box::new(command));
        Ok(())
    }

}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
        .field("aliases", &self.aliases)
        .field("type", &self.cmd_type)
        .field("subcommands", &self.subcommands.as_ref().map(|s_com_hashmap| {s_com_hashmap.len()}))
        .finish()
    }
}

/// Struct for parameters and context to a command
/// 
/// will include things like args in the future
pub struct CommandParams {
    ctx: Context, 
    msg: Message,

    // TODO: add more things like args
}