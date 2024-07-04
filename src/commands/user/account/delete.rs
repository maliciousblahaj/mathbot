use mathbot::{command::CommandParams, error::ClientError, model::account::{AccountController, AccountQueryKey}, send_embed, ui::{embed::{base_embed, ColorType, EmbedCtx}, ButtonInfo, ButtonMessage}, Error, Result, SendCtx};
use serenity::all::{ButtonStyle, CreateButton, CreateMessage};

pub async fn delete(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let initmsg = CreateMessage::new()
        .embed(base_embed(&EmbedCtx::from_params(&params), ColorType::Failure)
            .title("Account deletion")
            .description(format!("<@{}>, Are you sure you want to delete your account?", account.user_id))
        );

    let mut message = ButtonMessage::new(
        initmsg,
        &params, 
        vec![
            ButtonInfo::new(
                "no",
                CreateButton::new("no")
                    .label("No")
                    .style(ButtonStyle::Success)
            ),
            ButtonInfo::new(
                "yes",
                CreateButton::new("yes")
                    .label("Yes")
                    .style(ButtonStyle::Danger)
            ),
        ]
    );

    if let Some(id) = message.send().await?.run_interaction(10).await? {
        let embed = match id.as_str() {
            "yes" => 
                {
                    AccountController::new(params.state.get_model_controller(), AccountQueryKey::id(account.id))
                        .delete_account()
                        .await
                        .map_err(|e| Error::Client(ClientError::FailedToDeleteAccount(Box::new(e))))?;
                    base_embed(&EmbedCtx::from_params(&params), ColorType::Failure).description("Successfully deleted your account.")
                },
            "no" => base_embed(&EmbedCtx::from_params(&params), ColorType::Success).description("Account not deleted."), 
            _ => {return Err(Error::InvalidInteractionId)},
        };

        tokio::spawn(async move {message.disable_buttons().await});
        send_embed(embed, &SendCtx::from_params(&params)).await?;
    }

    Ok(())
}