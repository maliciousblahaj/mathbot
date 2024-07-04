use mathbot::{command::CommandParams, error::ClientError, send_embed, ui::embed::{base_embed, ColorType}, Error, Result, SendCtx};

pub async fn answer(params: CommandParams) -> Result<()> {
    let mut smpanswers = params.state.get_smp_answers().lock().await;

    let Some(correct) = smpanswers.get(&params.msg.channel_id.into()) else 
        {return Err(Error::Client(ClientError::AnswerNoProblemInChannel(params.bot_prefix)));};

    let Some(answer) = params.args.get(0) else {return Ok(());};
    let Ok(answer) = answer.parse::<i64>() else {return Ok(());};
    if &answer != correct {return Ok(());}

    smpanswers.remove(&params.msg.channel_id.into());
    drop(smpanswers);

    let message = match &params.account {
        None => format!("That is correct! Well done!\nPlease create an account to save your statistics and rewards in the future! To do so, execute `{}account create`.", params.bot_prefix),
        Some(acc) if acc.is_banned()? => format!("That is correct! Well done!\nYou did not recieve any rewards since you're banned. You'll get unbanned <t:{}:R>", acc.banned),
        Some(acc) => {
            sqlx::query!("UPDATE Accounts SET smps_solved = smps_solved + 1, balance = balance + 10 WHERE id=?", acc.id).execute(params.state.get_model_controller().lock().await.get_database())
                .await.map_err(|e| Error::FailedToIncrementSmpsSolved(e))?;
            format!("That is correct! Well done!\nTotal smp's solved: `{}`\nYou earned `{}MTC$`", acc.smps_solved+1, 10)
        }
    };

    send_embed(base_embed(&params.get_embed_ctx(), ColorType::Fun).description(message), &SendCtx::from_params(&params)).await?;

    Ok(())
}