use mathbot::{command::{Command, CommandCategory, CommandHelp}, vec_of_strings};

pub mod test;
mod confirmbuttons;

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
    ]
}