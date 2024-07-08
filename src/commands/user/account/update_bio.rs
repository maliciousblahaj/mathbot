use mathbot::{command::CommandParams, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};

pub async fn update_bio(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let Some(arg1) = params.args.get(0) else {return send_help(params).await;};
    let (newbio, description) = match arg1.as_str() {
        "remove" => (None, "Successfully removed your bio"),
        _ => {
            let searchcontent = params.msg.content[params.bot_prefix.len()..].to_string();
            let update_bio_alias = &params.aliassequence[1];
            (Some(
            searchcontent[
                searchcontent.find(update_bio_alias)
                .ok_or(Error::UpdateBioAliasNotFoundInMessageContent)? + update_bio_alias.len() + 1 
                ..].to_string()
            ), "Successfully updated your bio")
        }
    };
    sqlx::query!("UPDATE Accounts SET user_bio=? WHERE id=?", newbio, account.id)
        .execute(&params.state.get_model_controller().get_database().clone())
        .await
        .map_err(|e| Error::FailedToUpdateAccountBio(e))?;

    let embed = base_embed(&EmbedCtx::from_account(account), ColorType::Success)
        .title("Success")
        .description(description);
    send_embed(embed, &SendCtx::from_params(&params)).await?;
    Ok(())
}