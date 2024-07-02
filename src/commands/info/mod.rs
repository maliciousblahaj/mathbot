use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod ping;
mod help;
mod botinfo;

pub fn commands() -> Vec<Command> {
    let sharedtype = CommandType::RootCommand { category: CommandCategory::Info };
    vec![
        Command::new(
            help::help,
            vec_of_strings!("help", "halp", "h"),
            sharedtype.clone(),
            CommandHelp::new("Look up how a specific command is used. `/{}` indicates it's an optional input and `{}` indicates it's a required input. If a command has subcommands you can use `{{command}} {command} {subcommand}` to view its help page", " /{command}"),
        ),
        Command::new(
            ping::ping,
            vec_of_strings!("ping", "latency"),
            sharedtype.clone(),
            CommandHelp::new("Check the latency of the bot", ""),
        ),
        Command::new(
            botinfo::botinfo,
            vec_of_strings!("botinfo"),
            sharedtype.clone(),
            CommandHelp::new("Look up info about the current instance running the bot", "")
        )
    ]
}