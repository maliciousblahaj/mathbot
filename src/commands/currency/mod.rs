use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod iteminfo;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::Currency;
    vec![
        Command::new(
            iteminfo::iteminfo,
            vec_of_strings!("iteminfo", "itemi", "item", "iteminf"),
            category.clone(),
            CommandHelp::new("Look up info about a specific item", " {item}"),
        ),
    ]
}