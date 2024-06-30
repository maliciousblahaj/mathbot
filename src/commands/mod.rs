use serenity::all::{CreateMessage, EmbedField};

use crate::command::{Command, CommandCategory, CommandIndex, CommandParams, CommandType};
use crate::{appearance, Error, Result};

pub mod math;
pub mod misc;
pub mod info;

use crate::vec_of_strings;

async fn help(params: CommandParams) -> Result<()> {
    //TODO: implement specific command help
    let prefix = &params.bot_prefix;

    let mut embed = appearance::Embed::BaseEmbed(&params)
        .title("Help menu")
        .description(format!("Here are all of the base commands. Write `{prefix}help {{command}}` to learn more about the commands"));

    for (name, cmdvec) in 
        match params.bot_commands.get_command_index().ok_or(Error::CommandIndexDoesntExist)? {
            CommandIndex::Root(indexmap) => indexmap,
            _ => {return Err(Error::SubcommandIndexAtRootLevel);}
        }
    {
        match name.to_lowercase().as_str() {
            "test" => {continue;},
            "admin" => {}, //TODO: do an user check once the database is configured
            _ => (),
        }
        let mut s = String::new();
        for cmdname in cmdvec {
            s.push_str(&format!("`{}{}`, ", prefix, cmdname))
        }
        //remove the last ", "
        if s.len() >= 2 {
            s = s[..s.len() - 2].to_string();
        }
        embed = embed.field(name, s, false);
    }
    
    let message = CreateMessage::new()
        .embed(embed);

    params.msg.channel_id.send_message(&params.ctx.http, message).await?;

    Ok(())
}

pub fn help_command() -> Command {
    Command::new(
        help,
        vec_of_strings!("help", "h"),
        CommandType::RootCommand { category: CommandCategory::Info },
    )
}