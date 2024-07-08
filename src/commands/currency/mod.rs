use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod iteminfo;
mod balance;
mod transfer;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Currency;
    vec![
        Command::new(
            iteminfo::iteminfo,
            vec_of_strings!("iteminfo", "itemi", "item", "iteminf"),
            category.clone(),
            CommandHelp::new("Look up info about a specific item", " {item}"),
        ),
        Command::new(
            transfer::transfer,
            vec_of_strings!("transfer", "trans", "transf", "gift", "give"),
            category.clone(),
            CommandHelp::new("Transfer MathCoins to someone else (preferably the admins)", " {amount} {user}"),
        ),
        Command::new(
            balance::balance,
            vec_of_strings!("balance", "bal"),
            category.clone(),
            CommandHelp::new("View your or someone else's balance", " /{username}"),
        ),
    ]
}