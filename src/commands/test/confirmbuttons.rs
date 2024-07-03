use mathbot::appearance::embed::{base_embed, ButtonEmoji, ColorType, EmbedCtx};
use mathbot::appearance::{ButtonInfo, ButtonMessage};
use mathbot::command::CommandParams;
use mathbot::Result;
use serenity::all::{ButtonStyle, CreateEmbed, CreateMessage};

pub async fn test(params: CommandParams) -> Result<()> {
    let initmsg = CreateMessage::new()
        .embed(confirm_embed(&EmbedCtx::from_params(&params))
            .title("Confirm Test")
            .description("Are you sure you want to confirm?")
    );
    let message = ButtonMessage::new(
        initmsg,
        params, 
        20,
        vec![
            ButtonInfo::new(
                "confirm",
                ButtonEmoji::Confirm.emoji(),
                ButtonStyle::Success,
                |params| base_embed(&EmbedCtx::from_params(params), ColorType::Success).title("Confirmed!"),
            ),
            ButtonInfo::new(
                "decline",
                ButtonEmoji::Decline.emoji(),
                ButtonStyle::Danger,
                |params| base_embed(&EmbedCtx::from_params(params), ColorType::Failure).title("Declined!"),
            ),
        ]
    );

    message.send().await?.run_interaction().await?;
    
    Ok(())
}


pub fn confirm_embed(ctx: &EmbedCtx) -> CreateEmbed {
    base_embed(ctx, ColorType::Info)
}