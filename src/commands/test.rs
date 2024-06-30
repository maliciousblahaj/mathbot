use mathbot::{command::CommandParams, send_message, Result};

pub async fn test(params: CommandParams) -> Result<()> {
    send_message("Hello World!", &params).await?;
    Ok(())
}