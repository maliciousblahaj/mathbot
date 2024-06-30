use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandType}, vec_of_strings};

pub mod say;

pub fn commands() -> Vec<Command> {
    let rootinfo = CommandType::RootCommand { category: CommandCategory::Fun };
    Vec::from([
        Command::new(
            say::say,
            vec_of_strings!("say", "säg"),
            rootinfo.clone(),
            CommandHelp::new("Make the bot repeat something you've said (it better not be anything negative towards the admins)", " {something}"),
        ),
    ])
}