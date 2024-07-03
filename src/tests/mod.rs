use std::io::Write;

use crate::bot::{Bot, BotBuilder};
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
            CommandCategory::Test,
            CommandHelp::new("",""),
        );
        let uwu = Command::new(
            uwucommand,
            vec_of_strings!("owo", "uwu", ":3", ">w<"),
            CommandCategory::Test,
            CommandHelp::new("",""),
        );

        let hi = Command::new(
            hicommand, 
            vec_of_strings!("hi", "hello", "haiii", "haii", "hai", "haiiii", "h"),
            CommandCategory::Test,
            CommandHelp::new("",""),
        ).register_single(uwu).unwrap();

        let bye = Command::new(
            byecommand,
            vec_of_strings!("bye", "byy"),
            CommandCategory::Test,
            CommandHelp::new("",""),
        );

        vec![test, hi, bye]
    }
}



/*
fn makebot() -> Bot {
    let commands = testcommands::setupcommands();

    let bot = {
        let mut bot = Some(BotBuilder::new("dev ").unwrap());
        for command in commands {
            bot = Some(bot.take().unwrap().register_single(command).unwrap())
        }
        bot.take().unwrap()
    };

    bot.build()
}

#[test]
fn setup_bot() {
    let bot = makebot();
    //println!("{bot}"); i need to impl display for bot first
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
*/
