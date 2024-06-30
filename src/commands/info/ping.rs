use mathbot::{get_current_timestamp_millis, send_message, Error, Result, SendCtx};
use mathbot::command::CommandParams;
use serenity::all::EditMessage;

// temptime = time.time()
//     msggg = await ctx.send("Pong!")
//     await msggg.edit(content=f"Pong! Bot latency: {round((time.time()-temptime)*1000)}ms")

pub async fn ping(params: CommandParams) -> Result<()> {
    //does not work when out of sync with discord's servers
        //let systemtime = get_current_timestamp_millis()?;
        //let msg_snowflake = params.msg.id.get();
        //let msg_timestamp_millis = ((msg_snowflake >> 22) + 1420070400000) as u128;
        //let ping_time_ms = systemtime - msg_timestamp_millis;
    
    let systemtime = get_current_timestamp_millis()?;
    let mut message = send_message("Pong!", &SendCtx::from_params(&params)).await?;
    let deltatime_ms = get_current_timestamp_millis()? - systemtime;

    message.edit(&params.ctx.http, 
        EditMessage::new()
            .content(format!("Pong! Bot latency: {deltatime_ms}ms"))
    ).await
    .map_err(|_| Error::FailedToEditMessage)?;
    Ok(())
}
