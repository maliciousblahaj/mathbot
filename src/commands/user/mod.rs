use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod account;

pub fn commands() -> Vec<Command> {
    let category = CommandCategory::User;
    vec![
        Command::new(
            account::account,
            vec_of_strings!("account", "a", "p", "profile"),
            category.clone(),
            CommandHelp::new("Look up info about your or someone else's account", " /{account}"),
        )
            .register(
                vec![
                    Command::new(
                        account::account_create,
                        vec_of_strings!("create"),
                        category.clone(),
                        CommandHelp::new("Create your own account if you don't already have one", ""),
                    ),
                ]
            ).unwrap()
    ]
}