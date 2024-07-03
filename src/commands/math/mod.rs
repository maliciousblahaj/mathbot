use mathbot::{command::{Command, CommandCategory, CommandHelp}, vec_of_strings};

mod solve;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Math;
    vec![
        Command::new(
            solve::solve,
            vec_of_strings!("solve", "calculate", "calc", "cal", "sol", "solv"),
            category.clone(),
            CommandHelp::new("Make the bot calculate an expression using floating point math. Currently supported constants are `PI` and `TAU`, and basic trigonometry functions.", " {expression}"),
        ),
    ]
}