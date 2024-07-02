use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

mod say;

pub fn commands() -> Vec<Command> {
    let sharedtype = CommandType::RootCommand { category: CommandCategory::Fun };
    vec![
        Command::new(
            say::say,
            vec_of_strings!("say", "säg"),
            sharedtype.clone(),
            CommandHelp::new("Make the bot repeat something you've said (it better not be anything negative towards the admins)", " {something}"),
        ),
    ]
}