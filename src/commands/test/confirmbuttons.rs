use std::fmt::Display;

use mathbot::appearance::embed::{base_embed, ButtonEmoji, ColorType, EmbedCtx};
use mathbot::command::CommandParams;
use mathbot::{send_message, Result, SendCtx};
use serenity::all::{ButtonStyle, CreateButton, CreateEmbed, CreateMessage, EmojiId};

pub async fn test(params: CommandParams) -> Result<()> {
    let message = CreateMessage::new()
        .embed(confirm_embed(
                &EmbedCtx::from_params(&params),
                "Confirm Test", 
                "Are you sure you want to confirm?"
        ))
        .button(
            CreateButton::new("confirm")
                .emoji(EmojiId::new(ButtonEmoji::Confirm.emoji_id()))
                .style(ButtonStyle::Success)
        )
        .button(
            CreateButton::new("decline")
                .emoji(EmojiId::new(ButtonEmoji::Decline.emoji_id()))
                .style(ButtonStyle::Danger)
        )
        ;

    send_message(message, &SendCtx::from_params(&params)).await?;
    Ok(())
}


pub fn confirm_embed<S: AsRef<str> + Display>(ctx: &EmbedCtx, title: S, description: S) -> CreateEmbed {
    base_embed(ctx, ColorType::Info)
        .title(title.to_string())
        .description(description.to_string())
}