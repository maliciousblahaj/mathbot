use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

pub mod solve;

pub fn commands() -> Vec<Command> {
    let sharedtype = CommandType::RootCommand { category: CommandCategory::Math };
    vec![
        Command::new(
            solve::solve,
            vec_of_strings!("solve", "calculate", "calc", "cal", "sol", "solv"),
            sharedtype.clone(),
            CommandHelp::new("Make the bot calculate an expression using floating point math. Currently supported constants are `PI` and `TAU`, and basic trigonometry functions.", " {expression}"),
        ),
    ]
}