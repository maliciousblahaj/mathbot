use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod account;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::User;
    vec![
        account::command().unwrap(),
    ]
}