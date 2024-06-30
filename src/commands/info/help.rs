use std::fmt::Display;

use serenity::all::CreateEmbed;

use mathbot::appearance::embed::{base_embed, ColorType, EmbedCtx};
use mathbot::command::{Command, CommandIndex, CommandParams};
use mathbot::parser::parse_command;
use mathbot::{send_embed, Error, Result, SendCtx};

pub async fn help(params: CommandParams) -> Result<()> {
    if let Some((command, _, _, commandsequence, _)) = parse_command(&params.bot_commands, params.args.clone()){

        let embed = help_embed(&params, command, &commandsequence)?;

        send_embed(embed, &SendCtx::from_params(&params)).await?;

        return Ok(());
    }

    let prefix = &params.bot_prefix;

    let mut embed = base_embed(&EmbedCtx::from_params(&params), ColorType::Info)
        .title("Help menu")
        .description(format!("Here are all of the base commands. To run the commands specify `{prefix}` before them. Write `{prefix}help {{command}}` to learn more about the commands"));

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
            s.push_str(&format!("`{}`, ", cmdname))
        }
        s = match s.strip_suffix(", ") {
            Some(cleaned) => cleaned.to_string(),
            None => s,
        };
        embed = embed.field(name, s, false);
    }
    
    send_embed(embed, &SendCtx::from_params(&params)).await?;

    Ok(())
}

fn help_embed<S: AsRef<str> + Display>(params: &CommandParams, command: &Command, commandsequence: &Vec<S>) -> Result<CreateEmbed> {
    let commandstring = commandsequence.into_iter().map(|str| str.to_string()).collect::<Vec<String>>().join(" ");
    let commandhelp = command.get_help();
    let prefix = &params.bot_prefix;

    let mut embed = base_embed(&EmbedCtx::from_params(&params), ColorType::Info)
        .title(format!("{prefix}{commandstring} help"))
        .description(
            commandhelp.get_description()
                .replace("{{command}}", format!("{prefix}{commandstring}").as_str())
            )
        .field("Usage", format!("`{}{}{}`",prefix, commandstring, commandhelp.get_usage()),true)
        .field("Type", format!("`{}`", command.get_cmd_type().to_string()), true);
    
    if command.has_subcommands() {
        embed = embed.field(
            "Subcommands",
            match command.get_subcommands()
                        .ok_or(Error::ImpossibleError)?
                        .get_command_index()
                        .ok_or(Error::CommandIndexDoesntExist)?
                    {
                        CommandIndex::Root(_) => {return Err(Error::CommandIndexWrongType)},
                        CommandIndex::Sub(subcommands) => subcommands.join(", "),
                    },
            true,
        );
    }

    Ok(embed)
}