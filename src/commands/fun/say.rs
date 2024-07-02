use mathbot::{command::CommandParams, error::ClientError, send_text, Error, Result, SendCtx};

pub async fn say(params: CommandParams) -> Result<()> {
    if params.args.is_empty() {
        return Err(Error::Client(ClientError::NoSayContent));
    }

    let searchcontent = params.msg.content[params.bot_prefix.len()..].to_string();
    let sayalias = &params.aliassequence[0];
    let content = &searchcontent
        [
        searchcontent.find(sayalias)
        .ok_or(Error::SayAliasNotFoundInMessageContent)? + sayalias.len() + 1 
        ..];
    send_text(content, &SendCtx::from_params(&params)).await?;
    Ok(())
}