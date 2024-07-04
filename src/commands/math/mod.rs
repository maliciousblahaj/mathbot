use mathbot::{command::{Command, CommandCategory, CommandHelp}, vec_of_strings};

mod solve;
mod simplemathproblem;
mod answer;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Math;
    vec![
        Command::new(
            solve::solve,
            vec_of_strings!("solve", "calculate", "calc", "cal", "sol", "solv"),
            category.clone(),
            CommandHelp::new("Make the bot calculate an expression using floating point math. Currently supported constants are `PI` and `TAU`, and basic trigonometry functions.", " {expression}"),
        ),
        Command::new(
            simplemathproblem::simplemathproblem,
            vec_of_strings!("simplemathproblem", "smp"),
            category.clone(),
            CommandHelp::new("Make the bot generate a math problem for the channel to solve. Whoever solves it first gets a reward", ""),
        ),
        Command::new(
            answer::answer,
            vec_of_strings!("answer", "ans", "an"),
            category.clone(),
            CommandHelp::new("Answer a simple math problem. If you get it right, you earn 10 MathCoins!", " {answer}"),
        ),
    ]
}