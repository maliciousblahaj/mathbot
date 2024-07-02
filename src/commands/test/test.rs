use mathbot::{command::CommandParams, send_text, Result, SendCtx};

pub async fn test(params: CommandParams) -> Result<()> {
    send_text("Hello World!", &SendCtx::from_params(&params)).await?;
    Ok(())
}