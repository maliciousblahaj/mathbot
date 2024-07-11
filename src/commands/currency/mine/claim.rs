use mathbot::{command::CommandParams, error::ClientError, model::account::{AccountController, AccountQueryKey, MineClaimType}, send_embed, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};

pub async fn claim(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    if account.mine_slots < 1 {
        return Err(Error::Client(ClientError::MineClaimNotOpenedYet));
    }
    let tax = 0.0;
    let mut ac = AccountController::new(params.state.get_model_controller(), AccountQueryKey::id(account.id));
    let amount = ac.claim_mine(MineClaimType::Bank, tax).await?;

    send_embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Success)
            .title("Success")
            .description(format!("Successfully claimed `{amount:.1}MTC$` from your mine after `{tax:.1}%` tax")),        
        &SendCtx::from_params(&params),
    ).await?;

    Ok(())   
}