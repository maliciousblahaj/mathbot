use mathbot::{command::CommandParams, error::ClientError, send_embed, ui::embed::{base_embed, ColorType}, Error, Result, SendCtx};

pub async fn solution(params: CommandParams) -> Result<()> {
    let mut smpanswers = params.state.get_smp_answers().lock().await;

    let Some(correct) = smpanswers.get(&params.msg.channel_id.into()).map(|c| c.clone()) else 
        {return Err(Error::Client(ClientError::SolutionNoProblemInChannel(params.bot_prefix)));};

    smpanswers.remove(&params.msg.channel_id.into());
    drop(smpanswers);

    send_embed(
        base_embed(&params.get_embed_ctx(), ColorType::Fun)
            .description(format!("The correct answer is {correct}")),
        &SendCtx::from_params(&params)
    ).await?;
    Ok(())
}