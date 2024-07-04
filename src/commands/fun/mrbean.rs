use mathbot::{command::CommandParams, send_embed, ui::embed::{base_embed, ColorType}, Result, SendCtx};

pub async fn mrbean(params: CommandParams) -> Result<()> {
    send_embed(base_embed(&params.get_embed_ctx(), ColorType::Fun).title("Bo’oh’o’wa’er"), &SendCtx::from_params(&params)).await?;
    Ok(())
}