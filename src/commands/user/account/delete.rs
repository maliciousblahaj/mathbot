use mathbot::{command::CommandParams, error::ClientError, model::account::{AccountController, AccountQueryKey}, send_embed, ui::{embed::{base_embed, ColorType, EmbedCtx}, ButtonInfo, ButtonMessage}, Error, Result, SendCtx};
use rand::thread_rng;
use rand::seq::SliceRandom;
use serenity::all::{ButtonStyle, CreateButton, CreateMessage};

pub async fn delete(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let initmsg = CreateMessage::new()
        .embed(base_embed(&EmbedCtx::from_account(account), ColorType::Failure)
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
                    base_embed(&EmbedCtx::from_account(account), ColorType::Failure).description(get_delete_response(&account.username))
                },
            "no" => base_embed(&EmbedCtx::from_account(account), ColorType::Success).description("Account not deleted."), 
            _ => {return Err(Error::InvalidInteractionId)},
        };

        tokio::spawn(async move {message.disable_buttons().await});
        send_embed(embed, &SendCtx::from_params(&params)).await?;
    }

    Ok(())
}

fn get_delete_response(username: &String) -> String {
    DELETE_RESPONSES.choose(&mut thread_rng())
        .unwrap_or(&"USERNAME attempted to criticize the admins and disappeared under mysterious circumstances")
        .replace("USERNAME", format!("`@{username}`").as_str())
}

const DELETE_RESPONSES: [&'static str; 14] = [
    "USERNAME attempted to criticize the admins and disappeared under mysterious circumstances",
    "USERNAME went to North Korea and was never seen again",
    "USERNAME went to get milk",
    "USERNAME finally went to touch grass",
    "The admins accidentally sold too much of USERNAME's data to supreme leader Kim Jong-Ugn",
    "The admins decided to ban USERNAME without any reason given. Why? Because they can.",
    "USERNAME tried to escape the matrix and got arrested",
    "USERNAME commited alt + f4",
    "USERNAME tried to escape from the admins and was found with 31 shots in the back of their head. The admins suspect it was a suicide",
    "USERNAME actually intended to click the \"No\" button but for some reason their account was still deleted. When asked about this situation, the admins preferred not to comment anything",
    "USERNAME got teleported back to 1987 by the admins",
    "USERNAME got rickrolled too much and finally gave up",
    "USERNAME died of natural causes after recognizing Taiwan as it's own country",
    "USERNAME noclipped into the backrooms",
];