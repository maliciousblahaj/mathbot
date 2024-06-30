pub mod embed {
    use std::fmt::Display;

    use chrono::{Datelike, Local};
    use phf::phf_map;
    use rand::seq::SliceRandom;
    use serenity::all::{Color, Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Timestamp};
    use crate::{command::{Command, CommandIndex, CommandParams}, Error, Result};

    pub const COLOR_TYPES: phf::Map<&'static str, i32>= phf_map! {
        "success" => 0x64FF64,
        "failure" => 0xFF6464, 
        "info" => 0xFFFFFF,
        "admin" => 0x000000,
        "settings" => 0xCCCCCC,
        "tool" => 0xC291FF,
        "fun" => 0xF482FF,
        "userinfo" => 0x02BFFF,
        "currency" => 0x61FFFF,
    };

    pub const FOOTER_MESSAGES: &'static [&'static str] = &[
            "Did you know that 99% of gambling addicts quit right before they win a zillion MathCoins?",
            "dQw4w9WgXcQ",
            "Have you paid your obligatory account tax to the superior admins of MathBot yet?",
            "Nothing happened in 1989",
            "Why don't you try winning against our super advanced omniscient rock paper scissors AI?",
            "Did you know, there is a secret command to earn 999999999 MathCoins: `!transfer @/admin all`",
            "Did you know Tobias has cloned himself twice?",
            "Just a reminder in favor of your landlord: Your rent is due",
            "We have been trying to reach you about your car's extended warranty",
            "I am the bot that arranges the blocks",
            "Don't forget to praise the holy Skittlern",
            "Rule 1: We don't talk about the rules.",
            "If you just saw an admin abusing their commands, you didn't",
            "Just a reminder: MathBot is completely closed source and proprietary software, and any unauthorized use of it will lead to a lawsuit",
            "All hail supreme leader Kim Jong-Ugn",
            "De e najs!",
            "This discord bot was originally coded in light mode",
        ];

    pub const MATHBOT_AVATAR_URL: &'static str = "https://cdn.discordapp.com/avatars/992315441735270470/11acad15a810ef9d68cf14d7b07db43b.webp";

    pub enum ColorType {
        Success,
        Failure,
        Info,
        Admin,
        Settings,
        Tool,
        Fun,
        UserInfo,
        Currency,
    }
    impl ColorType {
        pub fn color(&self) -> u32 {
            match self {
                Self::Success => 0x64FF64,
                Self::Failure => 0xFF6464,
                Self::Info => 0xFFFFFF,
                Self::Settings => 0xCCCCCC,
                Self::Tool => 0xC291FF,
                Self::Fun => 0xF482FF,
                Self::UserInfo => 0x02BFFF,
                Self::Currency => 0x61FFFF,
                Self::Admin => 0x000000,
            }
        }
    }


    pub fn BaseEmbed(params: &CommandParams, colortype: ColorType) -> CreateEmbed{
        let randomfootermsg = FOOTER_MESSAGES.choose(&mut rand::thread_rng()).unwrap().to_string();
        let footer = CreateEmbedFooter::new(randomfootermsg)
            .icon_url(MATHBOT_AVATAR_URL.to_string());
        let author = CreateEmbedAuthor::new(format!("@{}", params.msg.author.name))
            .icon_url(params.msg.author.avatar_url().unwrap_or(params.msg.author.default_avatar_url()));
        let timestamp = Local::now().with_year(1987).unwrap_or(Local::now()).timestamp();

        CreateEmbed::new()
            .footer(footer)
            .author(author)
            .timestamp(Timestamp::from_unix_timestamp(timestamp).unwrap_or(Timestamp::now()))
            .color(Colour::new(colortype.color()))
    }

    fn get_command_string<S: AsRef<str> + Display>(commandsequence: &Vec<S>) -> String {
        let mut string = String::new();
        for commandname in commandsequence {
            string.push_str(format!("{commandname} ").as_str());
        }
        string.trim().to_owned()
    }

    pub fn HelpEmbed<S: AsRef<str> + Display>(params: &CommandParams, command: &Command, commandsequence: &Vec<S>) -> Result<CreateEmbed> {
        let commandstring = get_command_string(commandsequence);
        let commandhelp = command.get_help();
        let prefix = &params.bot_prefix;

        let mut embed = BaseEmbed(params, ColorType::Info)
            .title(format!("{prefix}{commandstring} help"))
            .description(
                commandhelp.get_description()
                    .replace("{{command}}", format!("{prefix}{commandstring}").as_str())
                )
            .field("Usage", format!("`{}{}{}`",prefix, commandstring, commandhelp.get_usage()),true)
            .field("Type", format!("`{}`", command.get_cmd_type().to_string()), true);
        
        if command.has_subcommands() {
            embed = embed.field(
                "Subcommands",
                match command.get_subcommands()
                            .ok_or(Error::ImpossibleError)?
                            .get_command_index()
                            .ok_or(Error::CommandIndexDoesntExist)?
                        {
                            CommandIndex::Root(_) => {return Err(Error::CommandIndexWrongType)},
                            CommandIndex::Sub(subcommands) => subcommands.join(", "),
                        },
                true,
            );
        }

        Ok(embed)
    }
}