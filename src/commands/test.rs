use mathbot::{command::CommandParams, send_message, Result, SendCtx};

pub async fn test(params: CommandParams) -> Result<()> {
    send_message("Hello World!", &SendCtx::from_params(&params)).await?;
    Ok(())
}