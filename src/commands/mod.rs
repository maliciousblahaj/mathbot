use crate::command::{Command, CommandCategory, CommandParams, CommandType};
use crate::{appearance, Result};

pub mod math;
pub mod misc;
pub mod info;

use crate::vec_of_strings;


//TODO: finish this command
async fn help(params: CommandParams) -> Result<()> {
    //TODO: implement specific command help
    let prefix = &params.bot_prefix;

    let embed = appearance::Embed::BaseEmbed(&params)
        .title("Help menu")
        .description(format!("Here are all of the base commands. Write `{prefix}help {{command}}` to learn more about the commands"))
    ;

    Ok(())
}

pub fn help_command() -> Command {
    Command::new(
        help,
        vec_of_strings!("help", "h"),
        CommandType::RootCommand { category: CommandCategory::Info },
    )
}