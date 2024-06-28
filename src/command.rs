use std::{collections::HashMap, future::Future};

use crate::Result;
use serenity::{all::{Context, Message}, futures::future::BoxFuture};

/// The category a root command can have.
/// 
/// Having the category of Admin means the command only generates in the help menu for admins
/// and having the category of Test means the command does not show up in the help menu at all
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
        subcommands: Option<HashMap<String, Box<Command>>>,
    ) -> Self
    where 
        T: Future<Output = Result<()>> + 'static + Send + Sync,
    {
        let handle: CommandHandler = Box::new(move |params| {Box::pin(handle(params))});
        Self {
            handle: Box::new( handle ),
            aliases,
            cmd_type,
            subcommands,
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

}


/// Struct for parameters and context to a command
/// 
/// will include things like args in the future
pub struct CommandParams {
    ctx: Context, 
    msg: Message,

    // TODO: add more things like args
}