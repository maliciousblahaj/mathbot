use std::io::Write;

use command::CommandType;

use crate::bot::Bot;
use crate::{command::{self, CommandParams}, Result};

mod testcommands {
    use command::{Command, CommandCategory, CommandHelp};

    use crate::vec_of_strings;

    use super::*;

    async fn testcommand(_params: CommandParams) -> Result<()> {Ok(())} 
    async fn hicommand(_params: CommandParams) -> Result<()> {Ok(())} 
    async fn byecommand(_params: CommandParams) -> Result<()> {Ok(())} 
    async fn uwucommand(_params: CommandParams) -> Result<()> {Ok(())} 

    pub fn setupcommands() -> Vec<Command> {
        let test = Command::new(
            testcommand, 
            vec_of_strings!("test", "test2", "t"),
            CommandType::RootCommand { category: CommandCategory::Test },
            CommandHelp::new("",""),
        );
        let uwu = Command::new(
            uwucommand,
            vec_of_strings!("owo", "uwu", ":3", ">w<"),
            CommandType::SubCommand,
            CommandHelp::new("",""),
        );

        let hi = Command::new(
            hicommand, 
            vec_of_strings!("hi", "hello", "haiii", "haii", "hai", "haiiii", "h"),
            CommandType::RootCommand { category: CommandCategory::Test },
            CommandHelp::new("",""),
        ).register(uwu).unwrap();

        let bye = Command::new(
            byecommand,
            vec_of_strings!("bye", "byy"),
            CommandType::RootCommand { category: CommandCategory::Info },
            CommandHelp::new("",""),
        );

        vec![test, hi, bye]
    }
}



fn makebot() -> Bot {
    let commands = testcommands::setupcommands();

    let bot = {
        let mut bot = Some(Bot::new("dev "));
        for command in commands {
            bot = Some(bot.take().unwrap().register(command).unwrap())
        }
        bot.take().unwrap()
    };

    bot
}

#[test]
fn setup_bot() {
    let bot = makebot();
    println!("{bot:?}");
}

#[ignore]
#[test]
fn parse_commands() {
    let bot = makebot();

    //TODO: replace with actual unit tests
    use std::io::{stdin, stdout};
    loop {
        let mut s = String::new();
        println!("enter your command (q to quit): ");
        let _ = stdout().flush();
        let stdin = stdin(); // We get `Stdin` here.
        stdin.read_line(&mut s).unwrap();
        let s = s.trim();
        if s == "q" {
            break;
        }

        
        let parsed = bot.parse_message(s);
        
        println!("{s}: {parsed:?}");
    }
}
