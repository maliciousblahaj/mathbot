use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod admin;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Admin;
    vec![
        Command::new(
            admin::admin,
            vec_of_strings!("admin"),
            category.clone(),
            CommandHelp::new("Bully MathBot users by modifying their data", ""),
        ),
    ]
}