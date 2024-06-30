use std::borrow::Borrow;

use serenity::all::{CreateMessage, EmbedField};

use crate::appearance::embed::ColorType;
use crate::command::{Command, CommandCategory, CommandHelp, CommandIndex, CommandParams, CommandType};
use crate::parser::parse_command;
use crate::{appearance, Error, Result};

pub mod math;
pub mod misc;
pub mod info;

use crate::vec_of_strings;

async fn help(params: CommandParams) -> Result<()> {
    if let Some((command, _, commandsequence)) = parse_command(&params.bot_commands, params.args.clone()){

        let embed = appearance::embed::HelpEmbed(&params, command, &commandsequence)?;

        let message = CreateMessage::new()
            .embed(embed);

        params.msg.channel_id.send_message(&params.ctx.http, message).await?;

        return Ok(());
    }

    let prefix = &params.bot_prefix;

    let mut embed = appearance::embed::BaseEmbed(&params, ColorType::Info)
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
        s = match s.strip_suffix(", ") {
            Some(cleaned) => cleaned.to_string(),
            None => s,
        };
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
        vec_of_strings!("help", "halp", "h"),
        CommandType::RootCommand { category: CommandCategory::Info },
        CommandHelp::new("Look up how a specific command is used. `/{}` indicates it's an optional input and `{}` indicates it's a required input. If a command has subcommands you can use `{{command}} {command} {subcommand}` to view its help page", " /{command}"),
    )
}