use crate::{Error, Result, command::CommandParams};

pub async fn test(params: CommandParams) -> Result<()> {
    if let Err(E) = params.msg.channel_id.say(&params.ctx.http, "Hello world!").await {
        return Err(Error::FailedToSendMessage);
    }
    Ok(())
}