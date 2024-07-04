use mathbot::error::ClientError;
use mathbot::ui::embed::{base_embed, base_embed_no_author, EmbedCtx};
use mathbot::ui::{ButtonInfo, ButtonMessage};
use mathbot::command::CommandParams;
use mathbot::{Result, Error};
use mathbot::ui::embed::ColorType;
use serenity::all::{ButtonStyle, Color, CreateButton, CreateEmbed, CreateMessage};

pub async fn account(params: CommandParams) -> Result<()> {
    let Some(account) = params.account else {return Err(Error::Client(ClientError::AccountRequired(params.bot_prefix)));};

    Ok(())
}

pub async fn account_create(params: CommandParams) -> Result<()> {
    if params.account.is_some() {return Err(Error::Client(ClientError::AccountCreateAccountAlreadyExists));}

    let initmsg = CreateMessage::new()
        .embed(
            base_embed_no_author(ColorType::Info)
                .title("MathBot©™ account creation")
                .description("To create your MathBot©™ account, we kindly ask you to confirm your acceptance of our Terms of Service. By accepting our Terms of Service, you acknowledge that you have read, understood, and agreed to comply with all the terms and conditions outlined in our agreement. This step is crucial in maintaining the integrity, security, and quality of our service.\n\nhttps://example.com/terms-of-service")
        );

    let mut message = ButtonMessage::new(
        initmsg,
        &params, 
        vec![
            ButtonInfo::new(
                "accept",
                CreateButton::new("accept")
                    .label("Accept")
                    .style(ButtonStyle::Primary)
            ),
            ButtonInfo::new(
                "fakeaccept",
                CreateButton::new("fakeaccept")
                    .label("Accept")
                    .style(ButtonStyle::Secondary)
            ),
        ]
    );
    if let Some(id) = message.send().await?.run_interaction(20).await? {
        let embed = match id.as_str() {
            "accept" | "fakeaccept" => 
                base_embed(&EmbedCtx::from_params(&params), ColorType::Success).title("Success").description("Successfully created your account."),
            _ => {return Err(Error::InvalidInteractionId)},
        };
        message.edit_message_disabled(embed).await?;
    }

    Ok(())
}