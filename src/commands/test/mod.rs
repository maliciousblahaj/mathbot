use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

pub mod test;
mod confirmbuttons;

pub fn commands() -> Vec<Command> {
    let sharedtype = CommandType::RootCommand { category: CommandCategory::Test };
    vec![
        Command::new(
            test::test,
            vec_of_strings!("hello", "helloworld", "hello_world", "test"),
            sharedtype.clone(),
            CommandHelp::new("Make the bot say hello world.", ""),
        ),
        Command::new(
            confirmbuttons::test,
            vec_of_strings!("confirmtest", "buttontest"),
            sharedtype.clone(),
            CommandHelp::new("Test a confirm and deny view", ""),
        ),
    ]
}