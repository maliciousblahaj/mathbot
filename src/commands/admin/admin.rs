use mathbot::{command::CommandParams, send_embed, send_text, ui::embed::{base_embed, ColorType, EmbedCtx}, Result, SendCtx};

pub async fn admin(params: CommandParams) -> Result<()> {
    let account = match &params.account {
        Some(account) if account.is_admin() => account,
        _ => {return Ok(());},
    };

    send_embed(base_embed(&EmbedCtx::from_account(account), ColorType::Admin).title("Admin panel").description("unfinished"), &SendCtx::from_params(&params)).await?;
    Ok(())
}