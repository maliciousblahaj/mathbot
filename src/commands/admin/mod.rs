use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Admin;
    vec![
        Command::new(
            super::test::test::test,
            vec_of_strings!("test"),
            category.clone(),
            CommandHelp::new("desc", "usage"),
        ),
    ]
}