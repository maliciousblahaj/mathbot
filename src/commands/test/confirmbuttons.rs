use std::time::Duration;


use mathbot::appearance::embed::{base_embed, ButtonEmoji, ColorType, EmbedCtx};
use mathbot::command::CommandParams;
use mathbot::{send_message, Error, Result, SendCtx};
use serenity::all::{ActionRowComponent, ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateMessage, EditMessage, EmojiId};

pub async fn test(params: CommandParams) -> Result<()> {

    let components = vec![CreateActionRow::Buttons(vec![
        CreateButton::new("confirm")
            .emoji(EmojiId::new(ButtonEmoji::Confirm.emoji_id()))
            .style(ButtonStyle::Success),
        CreateButton::new("decline")
            .emoji(EmojiId::new(ButtonEmoji::Decline.emoji_id()))
            .style(ButtonStyle::Danger),
    ])];
    let message = CreateMessage::new()
        .embed(confirm_embed(&EmbedCtx::from_params(&params))
                    .title("Confirm Test")
                    .description("Are you sure you want to confirm?")
        )
        .components(components);

    let mut message = send_message(message, &SendCtx::from_params(&params)).await?;


    let _interaction = match message
        .await_component_interaction(&params.ctx.shard)
        .timeout(Duration::from_secs(20))
        .await
    {
        Some(x) => x,
        None => {
            let mut buttons = Vec::new();
            let components = &message.components.get(0).ok_or(Error::ButtonComponentNotFound)?.components;
            match &components[0] {
                ActionRowComponent::Button(b) => {
                    buttons.push(CreateButton::new("confirm")
                        .emoji(b.emoji.clone().ok_or(Error::NoEmojiOnButton)?)
                        .disabled(true)
                        .style(ButtonStyle::Secondary));
                },
                _ => (),              
            } 
            match &components[1] {
                ActionRowComponent::Button(b) => {
                    buttons.push(CreateButton::new("decline")
                        .emoji(b.emoji.clone().ok_or(Error::NoEmojiOnButton)?)
                        .disabled(true)
                        .style(ButtonStyle::Secondary));
                },
                _ => (),              
            } 
            let updated = EditMessage::new().components(vec![CreateActionRow::Buttons(buttons)]);
            message.edit(&params.ctx.http, updated).await.map_err(|e| Error::FailedToEditMessage(e))?;
            return Ok(());
        },
    };



    Ok(())
}


pub fn confirm_embed(ctx: &EmbedCtx) -> CreateEmbed {
    base_embed(ctx, ColorType::Info)
}