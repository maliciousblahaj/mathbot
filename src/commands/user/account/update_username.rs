use mathbot::ui::embed::{base_embed, ColorType, EmbedCtx};
use mathbot::{get_current_timestamp_secs_i64, send_embed, SendCtx};
use mathbot::{command::CommandParams, error::ClientError, send_help, Error, Result};

pub async fn update_username(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let currenttime = get_current_timestamp_secs_i64()?;
    if !account.is_admin() && account.next_username_update > currenttime {return Err(Error::Client(ClientError::UpdateUsernameTooSoon(account.next_username_update)));}
    
    let username = match params.args.get(0) {
        Some(username) => {
            let 2..=20 = username.len() else {
                return Err(Error::Client(ClientError::UpdateUsernameInvalidLength));
            };
            //the user should be able to set their username as the same but with different case letters
            if username.to_lowercase() != account.username.to_lowercase() 
                && params.state.get_model_controller().username_exists(username).await? {
                return Err(Error::Client(ClientError::UpdateUsernameAlreadyExists(username.to_string())));
            }
            username
        },
        None => {return send_help(params).await;},
    };
    let newtime = currenttime + 600;
    sqlx::query!("UPDATE Accounts SET username=?, next_username_update=? WHERE id=?", username, newtime, account.id)
        .execute(&params.state.get_model_controller().get_database().clone())
        .await
        .map_err(|e| Error::FailedToUpdateAccountUsername(e))?;

    let embed = base_embed(&EmbedCtx::new(username.clone(), account.avatar_url.clone()), ColorType::Success)
        .title("Success")
        .description(format!("Successfully updated your username to `@{username}`"));
    send_embed(embed, &SendCtx::from_params(&params)).await?;
    Ok(())
}