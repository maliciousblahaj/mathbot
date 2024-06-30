use crate::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

pub mod ping;
pub mod help;


pub fn commands() -> Vec<Command> {
    Vec::from([
        Command::new(
            ping::ping,
            vec_of_strings!("ping", "latency"),
            CommandType::RootCommand { category: CommandCategory::Info },
            CommandHelp::new("Check the latency of the bot", ""),
        ),
        Command::new(
            help::help,
            vec_of_strings!("help", "halp", "h"),
            CommandType::RootCommand { category: CommandCategory::Info },
            CommandHelp::new("Look up how a specific command is used. `/{}` indicates it's an optional input and `{}` indicates it's a required input. If a command has subcommands you can use `{{command}} {command} {subcommand}` to view its help page", " /{command}"),
        )
    ])
}