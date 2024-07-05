use std::fmt::Display;

use serenity::all::CreateEmbed;

use mathbot::ui::embed::{base_embed, ColorType};
use mathbot::command::{Command, CommandCategory, CommandIndex, CommandParams};
use mathbot::parser::parse_command;
use mathbot::{send_embed, Error, Result, SendCtx};
use itertools::Itertools;

pub async fn help(params: CommandParams) -> Result<()> {
    if let Some((command, _, _, commandsequence, _)) = parse_command(&params.bot_commands, params.args.clone()){

        let embed = help_embed(&params, &command, &commandsequence)?;

        send_embed(embed, &SendCtx::from_params(&params)).await?;

        return Ok(());
    }

    let prefix = &params.bot_prefix;

    let mut embed = base_embed(&params.get_embed_ctx(), ColorType::Info)
        .title("Help menu")
        .description(format!("Here are all of the base commands. To run the commands, specify `{prefix}` before them. Write `{prefix}help {{command}}` to learn more about the commands"));

    for (category, cmdvec) in 
        match params.bot_commands.read().expect("Help menu Command RwLock posioned").get_command_index().ok_or(Error::CommandIndexDoesntExist)? {
            CommandIndex::Root(indexmap) => indexmap,
            _ => {return Err(Error::SubcommandIndexAtRootLevel);}
        }
    {
        match category {
            CommandCategory::Test => {continue;},
            CommandCategory::Admin => {
                match &params.account {
                    Some(acc) if acc.is_admin() => (),
                    _ => {continue;}
                }
            },
            _ => (),
        }
        let mut s = String::new();
        for cmdname in cmdvec {
            s.push_str(&format!("`{}`, ", cmdname))
        }
        s = match s.strip_suffix(", ") {
            Some(cleaned) => cleaned.to_string(),
            None => s,
        };
        embed = embed.field(category.as_ref(), s, false);
    }
    
    send_embed(embed, &SendCtx::from_params(&params)).await?;

    Ok(())
}

fn help_embed<S: AsRef<str> + Display>(params: &CommandParams, command: &Command, commandsequence: &Vec<S>) -> Result<CreateEmbed> {
    let commandstring = commandsequence.into_iter().map(|str| str.to_string()).collect::<Vec<String>>().join(" ");
    let commandhelp = command.get_help();
    let prefix = &params.bot_prefix;

    let mut embed = base_embed(&params.get_embed_ctx(), ColorType::Info)
        .title(format!("Command help  —  `{prefix}{commandstring}`"))
        .description(
            commandhelp.get_description()
                .replace("{{command}}", format!("{prefix}{commandstring}").as_str())
            )
        .field("Usage", format!("`{}{}{}`",prefix, commandstring, commandhelp.get_usage()),true)
        .field("Category", format!("`{}`", command.get_cmd_category().get_string()), true);
    
    if command.has_subcommands() {
        embed = embed.field(
            "Subcommands",
            match command.get_subcommands()
                        .ok_or(Error::ImpossibleError)?
                        .read().expect("Help embed Command RwLock posioned")
                        .get_command_index()
                        .ok_or(Error::CommandIndexDoesntExist)?
                    {
                        CommandIndex::Root(_) => {return Err(Error::CommandIndexWrongType)},
                        CommandIndex::Sub(subcommands) => subcommands.into_iter().map(|s| format!("`{s}`")).join(", "),
                    },
            true,
        );
    }

    Ok(embed)
}