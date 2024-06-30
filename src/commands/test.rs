use crate::{Result, command::CommandParams};

pub async fn test(params: CommandParams) -> Result<()> {
    params.msg.channel_id.say(&params.ctx.http, "Hello world!").await?;
    Ok(())
}