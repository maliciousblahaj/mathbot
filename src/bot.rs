use std::{fmt::Display, sync::Arc};
use crate::{error::ClientErrInfo, get_current_timestamp_secs, model::{account::AccountController, ModelController}, send_embed, ui::embed::error_embed, SendCtx};
use color_eyre::owo_colors::OwoColorize;
use serenity::{all::{Context, EventHandler, Message, Ready}, async_trait};
use sqlx::SqlitePool;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::command::{CommandMap, CommandParams, CommandType};
use crate::logging::log;
use crate::{Result, Error, command::Command, BOT_VERSION};

pub struct BotBuilder {
    prefix: String,
    commands: Arc<RwLock<CommandMap>>,
    state: GlobalState,
}

impl BotBuilder {
    pub fn new<S: AsRef<str> + Display>(prefix: S, database: SqlitePool) -> Result<Self> {
        Ok(
        Self {
            prefix: prefix.to_string(),
            commands: Arc::new(RwLock::new(CommandMap::new())),
            state: GlobalState::new(database)?,
        })
    }

    /// Register a command
    pub fn register_single(
        self,
        mut command: Command,
    ) -> Result<Self> {
        command.set_cmd_type(CommandType::RootCommand);
        self.commands.write().map_err(|_| Error::CommandsRwLockPoisoned)?.register_command(command)?;

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



pub struct Bot {
    prefix: String,
    commands: Arc<RwLock<CommandMap>>,
    state: GlobalState,
}

impl Bot {
    fn new(
        prefix: String,
        commands: Arc<RwLock<CommandMap>>,
        state: GlobalState,
    ) -> Self {
        Self {
            prefix,
            commands,
            state,
        }
    }

    async fn handle_message(&self, ctx: Context, msg: Message) -> Result<()>{
        if msg.author.bot{
            return Ok(());
        }

        let message_id_full = Uuid::new_v4();
        let message_id = &message_id_full.to_string()[..8];

        

        let Some(parsed) = self.parse_message(&msg.content).await
            //if the message is not a command, return
            else {
                log(format!("{:5} - {} - {}", "[MSG]".bright_green() , &msg.author.name.bright_green(), &msg.content));
                return Ok(());
            };
        
        log(format!("{:5} - {} - {} - {}", "[CMD]".cyan(), message_id.purple(), &msg.author.name.bright_green(), &msg.content));



        let authoraccount = {
            let mut accountcontroller = AccountController::new(
                self.get_state().get_model_controller(), crate::model::account::AccountQueryKey::user_id(msg.author.id.try_into()
                    .map_err(|_| Error::ProcessMessageAccountIdConversionFailed)?)
            );
            accountcontroller.fetch_account().await.ok()
        };
        //TEMP!!
        let (content, authorname) = (&msg.content.clone(), &msg.author.name.clone()); 

        let params = CommandParams::new(parsed.args, parsed.args_str, parsed.aliassequence, authoraccount.clone(), ctx, msg, self.get_state().clone(), self.get_prefix().to_string(), self.get_commands().clone(), message_id_full.clone());
        let command = parsed.command;
        let embedctx = params.get_embed_ctx();
        let sendctx = SendCtx::from_params(&params);

        if let Err(e) = command.run(params).await {
            let error_info = match e {
                Error::Client(ce) => {
                    log(format!("{} - {ce:?}", "[ERR]".red()));
                    ce.get_description()
                }
                e => {
                    log(format!("{} - {e:?}", "[ERR]".red()));
                    ClientErrInfo::new("Internal error", "Something went wrong")},
            };
            let embed = error_embed(&embedctx, error_info);
                
            send_embed(embed, &sendctx).await?;
        }
        //temp for debugging latency issues. Sent when finished handling command
        log(format!("{:5} - {} - {} - {}", "[RES]".blue(), message_id.purple(), authorname.bright_green(), content));

        Ok(())
    }
}


#[derive(Clone)]
pub struct GlobalState {
    start_time: u64,
    smp_answers: Arc<Mutex<HashMap<u64, i64>>>,
    mc: Arc<ModelController>,
}

impl GlobalState {
    pub fn new(database: SqlitePool) -> Result<Self> {
        Ok(
            Self {
                start_time: get_current_timestamp_secs()?,
                smp_answers: Arc::new(Mutex::new(HashMap::new())),
                mc: Arc::new(ModelController::new(database)),
            }
        )
    }

    pub fn get_start_time(&self) -> &u64 {
        &self.start_time
    }

    pub fn get_smp_answers(&self) -> &Arc<Mutex<HashMap<u64, i64>>> {
        &self.smp_answers
    }

    pub fn get_model_controller(&self) -> &Arc<ModelController> {
        &self.mc
    }
}



#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(e) = self.handle_message(ctx, msg).await{
            log(
                format!("{:5} - {}", "[ERR]".red(), e.red())
            );
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        log(
            format!("{:5} - {} successfully connected", "[SYS]".blink(), BOT_VERSION.bold())
        );
    }
}

impl Bot {

    pub fn get_prefix(&self) -> &str{
        &self.prefix
    }

    pub fn get_commands(&self) -> &Arc<RwLock<CommandMap>> {
        &self.commands
    }

    pub fn get_state(&self) -> &GlobalState {
        &self.state
    }

}