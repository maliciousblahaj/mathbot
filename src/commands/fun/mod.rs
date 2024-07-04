use mathbot::{command::{Command, CommandCategory, CommandHelp}, vec_of_strings};

mod say;
mod mrbean;
mod choose;
mod rockpaperscissors;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Fun;
    vec![
        Command::new(
            say::say,
            vec_of_strings!("say", "säg"),
            category.clone(),
            CommandHelp::new("Make the bot repeat something you've said (it better not be anything negative towards the admins)", " {something}"),
        ),
        Command::new(
            mrbean::mrbean,
            vec_of_strings!("mrbean"),
            category.clone(),
            CommandHelp::new("Make the bot speak bri'ish", ""),
        ),
        Command::new(
            choose::choose,
            vec_of_strings!("choose", "decide"),
            category.clone(),
            CommandHelp::new("Helps you choose between different things", " {*args}"),
        ),
        Command::new(
            rockpaperscissors::rockpaperscissors,
            vec_of_strings!("rockpaperscissors", "rps"),
            category.clone(),
            CommandHelp::new("Play a round of rock paper scissors against a super advanced AI. If you manage to win against it, you win 1 zillion MathCoins", " {your choice}"),
        ),
    ]
}