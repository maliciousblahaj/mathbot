use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod account;
mod inventory;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::User;
    vec![
        account::command().unwrap(),
        Command::new(
            inventory::inventory,
            vec_of_strings!("inventory", "inv", "invent", "in", "inve", "items"),
            category.clone(),
            CommandHelp::new("Look up what items you have in your inventory", " {page?}"),
        ),
    ]
}