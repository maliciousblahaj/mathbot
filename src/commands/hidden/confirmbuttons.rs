use mathbot::ui::embed::{base_embed, ButtonEmoji, ColorType, EmbedCtx};
use mathbot::ui::{ButtonInfo, ButtonMessage};
use mathbot::command::CommandParams;
use mathbot::{Error, Result};
use serenity::all::{ButtonStyle, CreateButton, CreateEmbed, CreateMessage};

pub async fn test(params: CommandParams) -> Result<()> {
    let initmsg = CreateMessage::new()
        .embed(confirm_embed(&params.get_embed_ctx())
            .title("Confirm Test")
            .description("Are you sure you want to confirm?")
    );
    let mut message = ButtonMessage::new(
        initmsg,
        &params, 
        vec![
            ButtonInfo::new(
                "confirm",
                CreateButton::new("confirm")
                    .emoji(ButtonEmoji::Confirm.emoji())
                    .style(ButtonStyle::Success),
            ),
            ButtonInfo::new(
                "decline",
                CreateButton::new("decline")
                    .emoji(ButtonEmoji::Decline.emoji())
                    .style(ButtonStyle::Danger),
            ),
        ]
    );
    if let Some(id) = message.send().await?.run_interaction(20).await? {
        let embed = match id.as_str() {
            "confirm" => 
                base_embed(&params.get_embed_ctx(), ColorType::Success).title("Confirmed!"),
            "decline" => 
                base_embed(&params.get_embed_ctx(), ColorType::Failure).title("Declined!"),
            _ => {return Err(Error::InvalidInteractionId)},
        };
        message.edit_message_disabled(embed).await?;
    }
    
    Ok(())
}


pub fn confirm_embed(ctx: &EmbedCtx) -> CreateEmbed {
    base_embed(ctx, ColorType::Info)
}