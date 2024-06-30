use crate::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

pub mod ping;
pub mod help;
pub mod botinfo;

//         "botinfo": Command("botinfo", "Look up info about the current instance running the bot", ""),
pub fn commands() -> Vec<Command> {
    let rootinfo = CommandType::RootCommand { category: CommandCategory::Info };
    Vec::from([
        Command::new(
            ping::ping,
            vec_of_strings!("ping", "latency"),
            rootinfo.clone(),
            CommandHelp::new("Check the latency of the bot", ""),
        ),
        Command::new(
            help::help,
            vec_of_strings!("help", "halp", "h"),
            rootinfo.clone(),
            CommandHelp::new("Look up how a specific command is used. `/{}` indicates it's an optional input and `{}` indicates it's a required input. If a command has subcommands you can use `{{command}} {command} {subcommand}` to view its help page", " /{command}"),
        ),
        Command::new(
            botinfo::botinfo,
            vec_of_strings!("botinfo"),
            rootinfo.clone(),
            CommandHelp::new("Look up info about the current instance running the bot", "")
        )
    ])
}