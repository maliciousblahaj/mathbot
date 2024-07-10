use core::fmt;
use std::{collections::HashMap, fmt::Display, future::Future, sync::Arc};
use crate::{bot::GlobalState, error::ClientError, model::account::{Account, AccountController, AccountQueryKey}, ui::{embed::{ButtonEmoji, EmbedCtx}, ButtonInfo, ButtonMessage}, Error, Result};
use indexmap::IndexMap;
use serenity::{all::{ButtonStyle, Context, CreateButton, CreateMessage, Message}, futures::future::BoxFuture};
use strum::IntoEnumIterator;
use std::sync::RwLock;
/// The category a root command can have.
/// 
/// Having the category of Admin means the command only generates in the help menu for admins
/// and having the category of Test means the command does not show up in the help menu at all
#[derive(Debug, Clone, strum_macros::AsRefStr, strum_macros::EnumIter, Hash, PartialEq, Eq)]
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
    RootCommand,
    SubCommand,
}

impl Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::SubCommand => format!("Subcommand"),
            Self::RootCommand => format!("Root command"),
        })
    }
}

///If a command index is root, it groups command names by their categories
/// else just by name
#[derive(Debug, Clone)]
pub enum CommandIndex {
    Root(IndexMap<CommandCategory, Vec<String>>),
    Sub(Vec<String>),
}

impl CommandIndex {
    pub fn new(cmd_type: &CommandType) -> Self {
        match cmd_type {
            CommandType::RootCommand => {    
                let mut map = IndexMap::new();
                for category in CommandCategory::iter() {
                    map.insert(category, Vec::new());
                }
                
                Self::Root(map)
            },
            CommandType::SubCommand => Self::Sub(Vec::new()),
        }
    }
    
    pub fn insert(&mut self, command: &Command) -> Result<()>{
        match self {
            Self::Root(map) => {
                let key = command.get_cmd_category();
                let cmd_vec = map.get_mut(key)
                    .ok_or(Error::CommandCategoryKeyDoesntExist)?;
                cmd_vec.push(command.get_aliases()[0].to_string());
            },
            Self::Sub(cmd_vec) => {cmd_vec.push(command.get_aliases()[0].to_string())}
        };
        Ok(())
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
    /// Will return None if no command index exists
    pub fn get_command_index(&self) -> Option<&CommandIndex> {
        self.command_index.as_ref()
    }

    /// Get a command by its name or alias
    pub fn get_command_by_alias <S: AsRef<str> + Display>(&self, name: S) -> Option<&Box<Command>> {
        self.get_command(self.command_map.get(&name.to_string().to_lowercase())?)
    }

    /// Registers a command by adding it to the commands field, and adding all its aliases to the command_map field
    pub fn register_command(&mut self, command: Command) -> Result<()>{
        let name = command.get_name().clone();

        if self.command_index.is_none() {
            self.command_index = Some(CommandIndex::new(command.get_cmd_type()?));
        }

        if self.commands.contains_key(&name) {
            return Err(Error::RegisterCommandAlreadyExists);
        }

        for alias in command.get_aliases(){
            if self.command_map.contains_key::<String>(alias) {
                return Err(Error::RegisterAliasAlreadyExists)
            }

            self.command_map.insert(alias.clone(), name.clone());
        }

        self.command_index.as_mut().ok_or(Error::CommandIndexDoesntExist)?.insert(&command)?;
        self.commands.insert(name, Box::new(command));
        Ok(())
    }
}

type CommandHandler = Box<dyn Fn(CommandParams) -> BoxFuture<'static, Result<()>> + 'static + Send + Sync>;

/// A Command's name is the 0th element of the aliases vector
#[derive(Clone)] 
pub struct Command
{
    handle: Arc<CommandHandler>,
    aliases: Vec<String>,
    cmd_type: Option<CommandType>,
    cmd_category: CommandCategory,
    help: CommandHelp,
    subcommands: Option<Arc<RwLock<CommandMap>>>,
}

impl Command
{
    pub fn new<F, T> (
        handle: F,
        aliases: Vec<String>, 
        cmd_category: CommandCategory,
        help: CommandHelp,
    ) -> Self
    where 
        T: Future<Output = Result<()>> + 'static + Send,
        F: Fn(CommandParams) -> T + 'static + Send + Sync,
    {
        let handle: CommandHandler = Box::new(move |params| {Box::pin(handle(params))});
        Self {
            handle: Arc::new(handle),
            aliases,
            cmd_type: None,
            cmd_category,
            help,
            subcommands: None,
        }
    }

    pub fn set_cmd_type(&mut self, cmd_type: CommandType) {
        self.cmd_type = Some(cmd_type);
    }

    pub async fn run(&self, params: CommandParams) -> Result<()> {
        (&self.handle)(params).await
    }

    pub fn get_aliases(&self) -> &Vec<String> {
        &self.aliases
    }
    
    pub fn get_name(&self) -> &String {
        &self.aliases[0]
    }

    pub fn get_cmd_type(&self) -> Result<&CommandType> {
        (self.cmd_type.as_ref()).ok_or(Error::CommandTypeNotRegistered)
    }

    pub fn get_cmd_category(&self) -> &CommandCategory {
        &self.cmd_category
    }

    pub fn get_subcommands(&self) -> Option<&Arc<RwLock<CommandMap>>> {
        self.subcommands.as_ref()
    }

    pub fn has_subcommands(&self) -> bool {
        !self.subcommands.is_none()
    }

    pub fn get_help(&self) -> &CommandHelp {
        &self.help
    }

    
    /// Register multiple subcommands
    /// 
    /// For single commands, use register_single
    pub fn register(
        mut self,
        commands: Vec<Command>,
    ) -> Result<Self> {
        for command in commands {
            self = self.register_single(command)?;
        }

        Ok(self)
    }

    /// Register a single subcommand
    pub fn register_single(
        mut self,
        mut command: Command,
    ) -> Result<Self> {
        command.set_cmd_type(CommandType::SubCommand);
        if self.subcommands.is_none() {
            self.subcommands = Some(Arc::new(RwLock::new(CommandMap::new())));
        }
        self.subcommands.as_mut().unwrap().write()
            .map_err(|_| Error::SubCommandsRwLockPoisoned)?
            .register_command(command).unwrap();

        Ok(self)
    }


}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
        .field("aliases", &self.aliases)
        .field("type", &self.cmd_type)
        .field("subcommands", &self.subcommands.as_ref())
        .finish()
    }
}


/// Struct for parameters to a command
/// 
/// Includes args, context, and message
#[derive(Clone)]
pub struct CommandParams{
    pub args: Vec<String>,
    pub args_str: String,
    pub aliassequence: Vec<String>,
    pub account: Option<Account>,
    pub ctx: Context, 
    pub msg: Message,
    pub state: GlobalState,
    pub bot_prefix: String,
    pub bot_commands: Arc<RwLock<CommandMap>>,
}

impl CommandParams {
    pub fn new(args: Vec<String>, args_str: String, aliassequence: Vec<String>, account: Option<Account>, ctx: Context, msg: Message, state: GlobalState, bot_prefix: String, bot_commands: Arc<RwLock<CommandMap>>) -> Self {
        Self {
            args,
            args_str,
            aliassequence,
            account,
            ctx,
            msg,
            state,
            bot_prefix,
            bot_commands,
        }
    }

    pub fn get_embed_ctx(&self) -> EmbedCtx {
        match &self.account {
            Some(acc) => EmbedCtx::new(acc.username.clone(), acc.avatar_url.clone()),
            None => EmbedCtx::new(self.msg.author.name.to_string(), self.msg.author.avatar_url().unwrap_or(self.msg.author.default_avatar_url()))
        }
    }

    ///Parses account referencing input from the user and returns an account if applicable
    pub async fn get_account_by_user_input<S: AsRef<str> + Display>(&self, input: S) -> Option<Account> {
        let mut input = input.to_string();
        if let Some(s) = input.strip_prefix("@/") { input = s.to_string(); }
        if let Some(s) = input.strip_prefix("<@") { input = s.to_string(); }
        if let Some(s) = input.strip_suffix(">") { input = s.to_string(); }
        let key = match input.parse::<i64>() {
            Ok(id) => AccountQueryKey::user_id(id),
            Err(_) => AccountQueryKey::username(input.to_string()),
        };
        let mut ac = AccountController::new(self.state.get_model_controller(), key);
        ac.fetch_account().await.ok()
    }

    ///Returns a client error if the user does not have an account, else a reference to the account in question
    pub fn require_account(&self) -> Result<&Account> {
        let Some(account) = &self.account else {return Err(Error::Client(ClientError::AccountRequired(self.bot_prefix.clone())));};
        if account.is_banned()? {return Err(Error::Client(ClientError::AccountIsBanned(account.banned)));}
        Ok(account)
    }

    ///Returns none if the user is not an admin, else a reference to the account in question
    pub fn require_admin(&self) -> Option<&Account> {
        match &self.account {
            Some(account) if account.is_admin() => Some(account),
            _ => {return None;}
        }
    }

    ///Will return true if the user confirms, false if the user declines, 
    /// and error if something went wrong during the confirmation process.
    pub async fn await_confirmation(&self, message: CreateMessage) -> Result<bool> {
        let mut confirm = ButtonMessage::new(
            message,
            &self, 
            vec![
                ButtonInfo::new(
                    "confirm",
                    CreateButton::new("confirm")
                        .emoji(ButtonEmoji::Confirm.emoji())
                        .style(ButtonStyle::Success),
                ),
                ButtonInfo::new(
                    "decline",
                    CreateButton::new("decline")
                        .emoji(ButtonEmoji::Decline.emoji())
                        .style(ButtonStyle::Danger),
                ),
            ]
        );
        match confirm.send().await?.run_interaction(20).await? {
            Some(id) => {
                confirm.disable_buttons().await?;
                match id.as_str() {
                    "confirm" => Ok(true),
                    "decline" => Ok(false),
                    _ => Err(Error::InvalidInteractionId),
                }
            },
            None => Ok(false)
        }
    }
}

#[derive(Clone)]
pub struct CommandHelp {
    description: String,
    usage: String,
}

impl CommandHelp {
    pub fn new<S: AsRef<str> + Display, T: AsRef<str> + Display> (description: S, usage: T) -> Self {
        Self {
            description: description.to_string(),
            usage: usage.to_string(),
        }
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_usage(&self) -> &String {
        &self.usage
    }
}