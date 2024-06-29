use core::fmt;
use std::{cell::RefCell, collections::HashMap, fmt::Display, future::Future, hash::Hash, string, sync::{Arc, Mutex}};
use crate::{bot::{Bot, GlobalState}, Error, Result};
use indexmap::IndexMap;
use serenity::{all::{Context, Message}, futures::future::BoxFuture};
use strum::IntoEnumIterator;

/// The category a root command can have.
/// 
/// Having the category of Admin means the command only generates in the help menu for admins
/// and having the category of Test means the command does not show up in the help menu at all
#[derive(Debug, Clone, strum_macros::AsRefStr, strum_macros::EnumIter)]
pub enum CommandCategory {
    Info,
    User,
    Currency,
    Fun,
    Math,
    Admin,
    Test,
}

impl CommandCategory {
    pub fn get_string(&self) -> String {
        self.as_ref().to_string()
    }
}

/// A command can either be a root command or a subcommand.
/// 
/// Root commands have a category assigned to them, but subcommands don't.
/// Commands form a tree structure, where every command, root or sub, can have a subcommand
#[derive(Debug, Clone)]
pub enum CommandType {
    RootCommand {category: CommandCategory},
    SubCommand,
}

///If a command index is root, it groups command names by their categories
/// else just by name
#[derive(Debug, Clone)]
pub enum CommandIndex {
    Root(IndexMap<String, Option<Vec<String>>>),
    Sub(Vec<String>),
}

impl CommandIndex {
    pub fn new(cmd_type: &CommandType) -> Self {
        match cmd_type {
            CommandType::RootCommand { category: _ } => {    
                let mut map = IndexMap::new();
                for category in CommandCategory::iter() {
                    map.insert(category.get_string(), None);
                }
                
                Self::Root(map)
            },
            CommandType::SubCommand => Self::Sub(Vec::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandMap {
    commands: HashMap<String, Box<Command>>,
    command_map: HashMap<String, String>,
    command_index: Option<CommandIndex>,
}

impl CommandMap {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            command_map: HashMap::new(),
            command_index: None,
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

    /// Get the index of a CommandMap
    /// If the CommandMap contains root commands, it will list
    pub fn get_command_index(&self) -> Option<&CommandIndex> {
        self.command_index.as_ref()
    }

    /// Get a command by its name or alias
    pub fn get_command_by_alias <S: AsRef<str> + Display>(&self, name: S) -> Option<&Box<Command>> {
        self.get_command(self.command_map.get(&name.to_string())?)
    }

    /// Registers a command by adding it to the commands field, and adding all its aliases to the command_map field
    pub fn register_command(&mut self, command: Command) -> Result<()>{
        let aliases = command.get_aliases();
        let name = aliases[0].clone();

        if self.command_index.is_none() {
            self.command_index = Some(CommandIndex::new(command.get_cmd_type()));
        }

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

type CommandHandler = Box<dyn Fn(CommandParams) -> BoxFuture<'static, Result<()>> + Send + Sync>;

//TODO: add documentation for commands (for help menu)
/// A Command's name is the 0th element of the aliases vector
/// Cloning a command makes its handle value None, making its run method return error
pub struct Command
{
    handle: Option<CommandHandler>,
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
        T: Future<Output = Result<()>> + 'static + Send,
    {
        let handle: CommandHandler = Box::new(move |params| {Box::pin(handle(params))});
        Self {
            handle: Some(handle),
            aliases,
            cmd_type,
            subcommands: None,
        }
    }

    pub async fn run(&self, params: CommandParams) -> Result<()> {
        (self.handle.as_ref().ok_or(Error::NoCommandHandle)?)(params).await
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

impl Clone for Command {
    fn clone(&self) -> Self {
        Self {
            handle: None,
            aliases: self.aliases.clone(),
            cmd_type: self.cmd_type.clone(),
            subcommands: self.subcommands.clone(),
        }
    }
}

/// Struct for parameters to a command
/// 
/// Includes args, context, and message
pub struct CommandParams{
    pub args: Vec<String>,
    pub ctx: Context, 
    pub msg: Message,
    pub state: Arc<Mutex<GlobalState>>,
    pub bot_prefix: String,
    pub bot_commands: CommandMap,
}

impl CommandParams {
    pub fn new(args: Vec<String>, ctx: Context, msg: Message, state: Arc<Mutex<GlobalState>>, bot_prefix: String, bot_commands: CommandMap) -> Self {
        Self {
            args,
            ctx,
            msg,
            state,
            bot_prefix,
            bot_commands,
        }
    }
}