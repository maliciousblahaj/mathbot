use std::sync::Arc;
use crate::{appearance::embed::{error_embed, EmbedCtx}, error::ClientErrInfo, get_current_timestamp_secs, send_embed, SendCtx};
use serenity::{all::{Context, EventHandler, Message}, async_trait};
use tokio::sync::Mutex;

use crate::command::{CommandMap, CommandParams, CommandType};
use crate::logging::log;
use crate::{Result, Error, command::Command};

pub struct BotBuilder {
    prefix: String,
    commands: CommandMap,
    state: Arc<Mutex<GlobalState>>,
}

impl BotBuilder {
    pub fn new(prefix: &str) -> Result<Self> {
        Ok(
        Self {
            prefix: prefix.to_string(),
            commands: CommandMap::new(),
            state: Arc::new(Mutex::new(
                GlobalState::new()?
            )),
        })
    }

    /// Register a command
    pub fn register_single(
        mut self,
        command: Command,
    ) -> Result<Self> {
        if let CommandType::SubCommand = command.get_cmd_type() {
            return Err(Error::SubcommandAtRootLevel);
        }
        self.commands.register_command(command)?;

        Ok(self)
    }

    /// Register multiple commands
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

    pub fn build(self) -> Bot {
        Bot::new(
            self.prefix,
            self.commands,
            self.state,
        )

    }
}



#[derive(Debug)]
pub struct Bot {
    prefix: String,
    commands: CommandMap,
    state: Arc<Mutex<GlobalState>>,
}

impl Bot {
    fn new(
        prefix: String,
        commands: CommandMap,
        state: Arc<Mutex<GlobalState>>,
    ) -> Self {
        Self {
            prefix,
            commands: commands,
            state,
        }
    }

    async fn handle_message(&self, ctx: Context, msg: Message) -> Result<()>{
        if !msg.author.bot{
            log(&msg.content);
        }

        let parsed = match self.parse_message(&msg.content) {
            //if the message is not a command, return
            None => {return Ok(());},
            Some(command) => command,
        };

        let params = CommandParams::new(parsed.args, parsed.args_str, ctx, msg, self.get_state(), self.get_prefix().to_string(), self.get_commands().clone());
        let command = parsed.command;
        let embedctx = EmbedCtx::from_params(&params);
        let sendctx = SendCtx::from_params(&params);

        if let Err(e) = command.run(params).await {
            let error_info = match e {
                Error::Client(ce) => ce.get_description(),
                _ => ClientErrInfo::new("Internal error", "Something went wrong"),
            };
            send_embed(error_embed(&embedctx, error_info), &sendctx).await?;
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct GlobalState {
    start_time: u64,
    //TODO: Add modelcontroller to this
}

impl GlobalState {
    pub fn new() -> Result<Self> {
        Ok(
        Self {
            start_time: get_current_timestamp_secs()?
        })
    }

    pub fn get_start_time(&self) -> &u64 {
        &self.start_time
    }
}



#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(e) = self.handle_message(ctx, msg).await{
            log(e);
        }
    }
}

impl Bot {

    pub fn get_prefix(&self) -> &str{
        &self.prefix
    }

    pub fn get_commands(&self) -> &CommandMap {
        &self.commands
    }

    pub fn get_state(&self) -> Arc<Mutex<GlobalState>> {
        (&self.state).clone()
    }

}