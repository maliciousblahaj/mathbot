use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

pub fn commands() -> Vec<Command> {
    let sharedtype = CommandType::RootCommand { category: CommandCategory::Admin };
    vec![
        Command::new(
            super::test::test,
            vec_of_strings!("test"),
            sharedtype.clone(),
            CommandHelp::new("desc", "usage"),
        ),
    ]
}