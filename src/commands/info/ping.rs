use std::time::{SystemTime, UNIX_EPOCH};

use crate::Result;
use crate::command::CommandParams;

pub async fn ping(params: CommandParams) -> Result<()> {
    let systemtime = SystemTime::now();
    let msg_snowflake = params.msg.id.get();
    let msg_timestamp_millis = ((msg_snowflake >> 22) + 1420070400000) as u128;
    let ping_time_ms = systemtime.duration_since(UNIX_EPOCH)?.as_millis() - msg_timestamp_millis;

    let message = format!("Pong! Bot latency: {ping_time_ms}ms");

    params.msg.channel_id.say(&params.ctx.http, message).await?;
    Ok(())
}
