use lazy_regex::regex;
use mathbot::{command::CommandParams, error::ClientError, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};

pub async fn update_pronouns(params: CommandParams) -> Result<()>{
    let account = params.require_account()?;
    let Some(pronouns) = params.args.get(0) else {return send_help(params).await;};
    let newpronouns = match pronouns.as_str() {
        "remove" => None, 
        p => {
            if !valid_pronouns(p) { return Err(Error::Client(ClientError::UpdatePronounsInvalid));}
            Some(p.to_string())
        }
    };
    
    
    sqlx::query!("UPDATE Accounts SET pronouns=? WHERE id=?", newpronouns, account.id)
        .execute(&params.state.get_model_controller().get_database().clone())
        .await
        .map_err(|e| Error::FailedToUpdateAccountPronouns(e))?;

    let embed = base_embed(&EmbedCtx::from_account(account), ColorType::Success)
        .title("Success")
        .description("Successfully updated your pronouns");
    send_embed(embed, &SendCtx::from_params(&params)).await?;
    Ok(())
}


fn valid_pronouns(pronouns: &str) -> bool {
    if !matches!(pronouns.len(), 3..=20){ return false; }
    let regex = regex!("^[A-Za-zÅÄÖåäö]+(/[A-Za-zÅÄÖåäö]+)?$");
    regex.is_match(pronouns)
}