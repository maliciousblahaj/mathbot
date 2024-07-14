use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod admin;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Admin;
    vec![
        admin::command().unwrap()
    ]
}