use mathbot::command::CommandParams;
use mathbot::error::ClientError;
use mathbot::ui::embed::{base_embed, base_embed_no_author, ColorType, EmbedCtx};
use mathbot::ui::{ButtonInfo, ButtonMessage};
use mathbot::{Error, Result};
use serenity::all::{ButtonStyle, CreateButton, CreateMessage};

pub async fn create(params: CommandParams) -> Result<()> {
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
    let message = message.send().await?;
    message.set_buttons(vec![
        ButtonInfo::new(
            "accept",
            CreateButton::new("accept")
                .label("Accept")
                .style(ButtonStyle::Primary)
        ),
        ButtonInfo::new(
            "decline",
            CreateButton::new("decline")
                .label("Decline")
                .style(ButtonStyle::Secondary)
        ),
    ]);
    if let Some(id) = message.run_interaction(20).await? {
        (&params).state.get_model_controller()
            .create_account(
                (&params).msg.author.id.into(), 
                (&params).msg.author.name.clone(), 
                (&params).msg.author.avatar_url().unwrap_or((&params).msg.author.default_avatar_url())
            ).await.map_err(|e| Error::Client(ClientError::FailedToCreateAccount(Box::new(e))))?;

        let embed = match id.as_str() {
            "accept" | "fakeaccept" => 
                base_embed(&params.get_embed_ctx(), ColorType::Success).title("Success").description("Successfully created your account."),
            _ => {return Err(Error::InvalidInteractionId)},
        };
        message.edit_message_disabled(embed).await?;
    }

    Ok(())
}

