use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod shop;
mod iteminfo;
mod balance;
mod transfer;
mod gamble;
mod slots;
mod mine;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Currency;
    vec![
        Command::new(
            gamble::gamble,
            vec_of_strings!("gamble", "bet", "gamb", "gam"),
            category.clone(),
            CommandHelp::new("Invest your money in the MathBot gambling industry.", " {amount}"),
        ),
        Command::new(
            slots::slots,
            vec_of_strings!("slots", "casino"),
            category.clone(),
            CommandHelp::new("Play the MathBot Casino slot machine, and perhaps you'll win the jackpot!", " {amount}"),
        ),
        Command::new(
            balance::balance,
            vec_of_strings!("balance", "bal"),
            category.clone(),
            CommandHelp::new("View your or someone else's balance in MathCoins", " {username?}"),
        ),
        Command::new(
            transfer::transfer,
            vec_of_strings!("transfer", "trans", "transf", "gift", "give"),
            category.clone(),
            CommandHelp::new("Transfer MathCoins to someone else (preferably the admins)", " {amount} {user}"),
        ),
        Command::new(
            iteminfo::iteminfo,
            vec_of_strings!("iteminfo", "itemi", "item", "iteminf"),
            category.clone(),
            CommandHelp::new("Look up info about a specific item", " {item}"),
        ),
        shop::command().unwrap(),
        mine::command().unwrap(),
    ]
}