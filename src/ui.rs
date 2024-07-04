use std::fmt::Display;
use std::time::Duration;

use indexmap::IndexMap;
use serenity::all::{ButtonStyle, ComponentInteractionDataKind, CreateActionRow, CreateButton, CreateEmbed, CreateMessage, EditMessage, Message};

use crate::command::CommandParams;
use crate::{send_message, Error, Result, SendCtx};

pub mod embed {
    use chrono::{Datelike, Local};
    use rand::seq::SliceRandom;
    use serenity::all::{Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, EmojiId, Timestamp};
    use crate::{command::CommandParams, error::ClientErrInfo};

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

    pub enum ButtonEmoji {
        First,
        Previous,
        Next,
        Last,
        Confirm,
        Decline,
    }

    impl ButtonEmoji {
        pub fn emoji_str(&self) -> &str {
            match self {
                Self::First => "<:first:1128306473160691794>",
                Self::Previous => "<:previous:1128306474372833343>",
                Self::Next => "<:next:1128306476830703758>",
                Self::Last => "<:last:1128306478370009121>",
                Self::Confirm => "<:checkmark:1130571724979712072>",
                Self::Decline => "<:crossmark:1130572943836053534>",
            }
        }
        pub fn emoji_id(&self) -> u64 {
            match self {
                Self::First => 1128306473160691794,
                Self::Previous => 1128306474372833343,
                Self::Next => 1128306476830703758,
                Self::Last => 1128306478370009121,
                Self::Confirm => 1130571724979712072,
                Self::Decline => 1130572943836053534,
            } 
        } 
        pub fn emoji(&self) -> EmojiId {
            EmojiId::new(self.emoji_id())
        } 
    }

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

    //params.msg.author.avatar_url().unwrap_or(params.msg.author.default_avatar_url())
    pub struct EmbedCtx {
        author_name: String,
        author_avatar_url: String,
    }

    impl EmbedCtx {
        pub fn new(author_name: String, author_avatar_url: String) -> Self {
            Self {author_name, author_avatar_url}
        }

        pub fn from_params(params: &CommandParams) -> Self {
            Self {
                author_name: params.msg.author.name.to_string(),
                author_avatar_url: params.msg.author.avatar_url().unwrap_or(params.msg.author.default_avatar_url())
            }
        }
    }

    pub fn base_embed(ctx: &EmbedCtx, colortype: ColorType) -> CreateEmbed{
        let randomfootermsg = FOOTER_MESSAGES.choose(&mut rand::thread_rng()).unwrap().to_string();
        let footer = CreateEmbedFooter::new(randomfootermsg)
            .icon_url(MATHBOT_AVATAR_URL.to_string());
        let author = CreateEmbedAuthor::new(format!("@{}", &ctx.author_name))
            .icon_url(&ctx.author_avatar_url);
        let timestamp = Local::now().with_year(1987).unwrap_or(Local::now()).timestamp();

        CreateEmbed::new()
            .footer(footer)
            .author(author)
            .timestamp(Timestamp::from_unix_timestamp(timestamp).unwrap_or(Timestamp::now()))
            .color(Colour::new(colortype.color()))
    }

    pub fn base_embed_no_author(colortype: ColorType) -> CreateEmbed{
        let randomfootermsg = FOOTER_MESSAGES.choose(&mut rand::thread_rng()).unwrap().to_string();
        let footer = CreateEmbedFooter::new(randomfootermsg)
            .icon_url(MATHBOT_AVATAR_URL.to_string());
        let timestamp = Local::now().with_year(1987).unwrap_or(Local::now()).timestamp();

        CreateEmbed::new()
            .footer(footer)
            .timestamp(Timestamp::from_unix_timestamp(timestamp).unwrap_or(Timestamp::now()))
            .color(Colour::new(colortype.color()))
    }

    pub fn error_embed(ctx: &EmbedCtx, clienterrinfo: ClientErrInfo) -> CreateEmbed {
        base_embed(ctx, ColorType::Failure)
            .title(clienterrinfo.get_title())
            .description(clienterrinfo.get_description())
    }

    pub fn error_embed_no_author(clienterrinfo: ClientErrInfo) -> CreateEmbed {
        base_embed_no_author(ColorType::Failure)
            .title(clienterrinfo.get_title())
            .description(clienterrinfo.get_description())
    }

    
}


pub struct ButtonInfo {
    custom_id: String,
    button: CreateButton,

}
//  embed callback:      where T: Fn(&CommandParams) -> CreateEmbed + 'static + Send

impl ButtonInfo {
    pub fn new<S: AsRef<str> + Display>(custom_id: S, button: CreateButton) -> Self
    {
        Self {
            custom_id: custom_id.to_string(),
            button,
        }
    }
}

pub struct ButtonMessage {
    message: CreateMessage,
    sent_message: Option<Message>,
    params: CommandParams,
    button_index: IndexMap<String, ButtonInfo>,
}

impl ButtonMessage {
    pub fn new(message: CreateMessage, params: &CommandParams, buttons: Vec<ButtonInfo> ) -> Self {
        let mut button_index = IndexMap::with_capacity(buttons.len());
        for buttoninfo in buttons {
            button_index.insert(buttoninfo.custom_id.clone(), buttoninfo);
        }
        Self {
            message,
            sent_message: None,
            params: params.clone(),
            button_index,
        }
    }

    pub async fn send(&mut self) -> Result<&mut Self> {
        self.sent_message = Some(send_message(
            self.message.clone()
                .components(vec![self.get_buttons()]),
            &SendCtx::from_params(&self.params)
        ).await?);
        Ok(self)
    }
    pub fn get_buttons(&self) -> CreateActionRow {
        let mut newbuttons = Vec::new();
        for (_, buttoninfo) in self.button_index.iter() {
            newbuttons.push(
                buttoninfo.button.clone()
            )
        }
        CreateActionRow::Buttons(newbuttons)
    } 

    pub fn get_disabled_buttons(&self) -> CreateActionRow {
        let mut newbuttons = Vec::new();
        for (_, buttoninfo) in self.button_index.iter() {
            newbuttons.push(
                buttoninfo.button.clone()
                    .disabled(true)
                    .style(ButtonStyle::Secondary)
            )
        }
        CreateActionRow::Buttons(newbuttons)
    } 

    pub async fn disable_buttons(&mut self) -> Result<()> {
        if self.sent_message.is_none() {return Err(Error::ButtonMessageNotSentYet);};

        let updated = EditMessage::new().components(vec![self.get_disabled_buttons()]);

        self.sent_message.as_mut().unwrap().edit(&self.params.ctx.http, updated)
            .await.map_err(|e| Error::FailedToEditMessage(e))?;
        return Ok(());
    }

    
    pub async fn run_interaction(&mut self, timeout: u64) -> Result<Option<String>> {
        let Some(sent_message) = &self.sent_message 
            else {return Err(Error::ButtonMessageNotSentYet);};

        let interaction = match sent_message
            .await_component_interaction(&self.params.ctx.shard)
            .timeout(Duration::from_secs(timeout))
            .author_id(self.params.msg.author.id.clone())
            .await
            {
                Some(x) => x,
                None => {self.disable_buttons().await?; return Ok(None);},
            };

        let ComponentInteractionDataKind::Button = interaction.data.kind else {return Ok(None);};
        
        let custom_id = interaction.data.custom_id.clone();

        let cache_http = self.params.ctx.http.clone();
        tokio::spawn(async move {
            interaction.defer(cache_http).await
        });

        Ok(Some(custom_id))
    }

    pub async fn edit_message(&mut self, embed: CreateEmbed) -> Result<()> {
        let updated = EditMessage::new()
            .embed(embed)
            .components(vec![self.get_buttons()]);

        self.sent_message.as_mut().unwrap().edit(&self.params.ctx.http, updated)
            .await.map_err(|e| Error::FailedToEditMessage(e))?;
        Ok(())
    }

    pub async fn edit_message_disabled(&mut self, embed: CreateEmbed) -> Result<()> {
        let updated = EditMessage::new()
            .embed(embed)
            .components(vec![self.get_disabled_buttons()]);

        self.sent_message.as_mut().unwrap().edit(&self.params.ctx.http, updated)
            .await.map_err(|e| Error::FailedToEditMessage(e))?;
        Ok(())
    }
}