use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

pub mod solve;

pub fn commands() -> Vec<Command> {
    let rootinfo = CommandType::RootCommand { category: CommandCategory::Math };
    Vec::from([
        Command::new(
            solve::solve,
            vec_of_strings!("solve", "calculate", "calc", "cal", "sol", "solv"),
            rootinfo.clone(),
            CommandHelp::new("Make the bot calculate an expression", " {expression}"),
        ),
    ])
}