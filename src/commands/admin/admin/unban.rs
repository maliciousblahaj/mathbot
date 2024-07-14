use mathbot::{Result, command::CommandParams, error::ClientError, get_current_timestamp_secs_i64, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, SendCtx};

pub async fn unban(params: CommandParams) -> Result<()> {
    let Some(account) = params.require_admin() else {return Ok(());};
    let Some(target) = params.args.get(0)
        else {return send_help(params).await;};

    let Some(targetaccount) = params.get_account_by_user_input(target).await
        else {return Err(Error::Client(ClientError::AdminBanInvalidAccount));};

    let newtime = get_current_timestamp_secs_i64()?;

    sqlx::query!("UPDATE Accounts SET banned=? WHERE id=?", newtime, targetaccount.id)
        .execute(params.state.get_model_controller().get_database())
        .await
        .map_err(|e| Error::FailedToUnbanAccount(e))?;

    send_embed(
        base_embed(&EmbedCtx::from_account(&account), ColorType::Admin)
            .title("Success")
            .description(format!("Successfully unbanned `@{}`!", targetaccount.username)), 
        &SendCtx::from_params(&params)
    ).await?;

    Ok(())
}