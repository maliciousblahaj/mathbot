use mathbot::ui::embed::{ColorType, EmbedCtx};
use mathbot::{command::CommandParams, ui::embed::base_embed};
use mathbot::{format_f64, send_embed, Result, SendCtx};

pub async fn balance(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let mut viewaccount = account.clone();

    if let Some(user) = params.args.get(0) {
        if let Some(account) = params.get_account_by_user_input(user).await {
            viewaccount = account;
        }
    }

    send_embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Currency)
            .title(format!("@{}'s balance", viewaccount.username))
            .description(format!("`{}MTC$`", format_f64(&viewaccount.balance))), 
    &SendCtx::from_params(&params)
    ).await?;
    Ok(())
}