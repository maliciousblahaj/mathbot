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

#[derive(Debug)]
pub struct CommandMap {
    commands: HashMap<String, Box<Command>>,
    command_map: HashMap<String, String>,
}

impl CommandMap {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            command_map: HashMap::new(),
        }
    }

    pub fn get_commands(&self) -> &HashMap<String, Box<Command>> {
        &self.commands
    }

    /// Get a command by its name
    pub fn get_command <S: AsRef<str> + Display>(&self, name: S) -> Option<&Box<Command>> {
        self.commands.get(&name.to_string())
    }

    pub fn get_command_map(&self) -> &HashMap<String, String> {
        &self.command_map
    }

    /// Get a command by its name or alias
    pub fn get_command_by_alias <S: AsRef<str> + Display>(&self, name: S) -> Option<&Box<Command>> {
        self.get_command(self.command_map.get(&name.to_string())?)
    }

    /// Registers a command by adding it to the commands field, and adding all its aliases to the command_map field
    pub fn register_command(&mut self, command: Command) -> Result<()>{
        let aliases = command.get_aliases();
        let name = aliases[0].clone();
        if self.commands.contains_key(&name) {
            return Err(Error::RegisterCommandAlreadyExists);
        }

        for alias in aliases{
            if self.command_map.contains_key::<String>(alias) {
                return Err(Error::RegisterAliasAlreadyExists)
            }

            self.command_map.insert(alias.clone(), name.clone());
        }

        self.commands.insert(name, Box::new(command));
        Ok(())
    }
}

//TODO: add documentation for commands (for help menu)
/// A Command's name is the 0th element of the aliases vector
pub struct Command
{
    handle: CommandHandler,
    aliases: Vec<String>,
    cmd_type: CommandType,
    subcommands: Option<CommandMap>,
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

    pub fn get_subcommands(&self) -> Option<&CommandMap> {
        self.subcommands.as_ref()
    }

    ///Register a subcommand to a command
    pub fn register(
        mut self,
        command: Command,
    ) -> Self {
        if self.subcommands.is_none() {
            self.subcommands = Some(CommandMap::new())
        }
        self.subcommands.as_mut().unwrap().register_command(command).unwrap();

        self
    }


}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
        .field("aliases", &self.aliases)
        .field("type", &self.cmd_type)
        .field("subcommands", &self.subcommands.as_ref().map(|s_com_map| {s_com_map.get_commands().len()}))
        .finish()
    }
}

/// Struct for parameters to a command
/// 
/// Includes args, context, and message
pub struct CommandParams {
    args: Vec<String>,
    ctx: Context, 
    msg: Message,
}

impl CommandParams {
    pub fn new(args: Vec<String>, ctx: Context, msg: Message) -> Self {
        Self {
            args,
            ctx,
            msg,
        }
    }
}