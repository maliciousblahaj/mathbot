use mathbot::{command::{Command, CommandCategory, CommandHelp}, vec_of_strings};

mod test;
mod confirmbuttons;
mod explainusererror;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Test;
    vec![
        Command::new(
            test::test,
            vec_of_strings!("hello", "helloworld", "hello_world", "test"),
            category.clone(),
            CommandHelp::new("Make the bot say hello world.", ""),
        ),
        Command::new(
            confirmbuttons::test,
            vec_of_strings!("confirmtest", "buttontest"),
            category.clone(),
            CommandHelp::new("Test a confirm and deny view", ""),
        ),
        Command::new(
            explainusererror::explainusererror,
            vec_of_strings!("--explain"),
            category.clone(),
            CommandHelp::new("Explain an error", " {error code}"),
        ),
    ]
}