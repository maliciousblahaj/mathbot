use mathbot::{command::{Command, CommandCategory, CommandHelp}, vec_of_strings};

mod solve;
mod simplemathproblem;
mod answer;
mod solution;
mod rng;
mod fractionify;

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
        Command::new(
            solution::solution,
            vec_of_strings!("solution"),
            category.clone(),
            CommandHelp::new("Reveal the solution of the ongoing math problem in your current channel (you won't gain any rewards)", ""),
        ),
        Command::new(
            rng::rng,
            vec_of_strings!("rng", "random", "randomnumber", "randomnumbergenerator"),
            category.clone(),
            CommandHelp::new("Generate a random integer in an inclusive range", " {start number} {end number}"),
        ),
        Command::new(
            fractionify::fractionify,
            vec_of_strings!("fractionify", "frac", "fractionize", "fract", "fraction"),
            category.clone(),
            CommandHelp::new("Turn any non-irrational decimal number into a fraction! To use it with repeating patterns, surround those with parenthesis like this: 1.33333 -> 1.(3)", " {decimal number}{optional repeating pattern surrounded by parenthesis}"),
        ),
    ]
}