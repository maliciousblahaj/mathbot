use mathbot::{get_current_timestamp_millis, send_message, Result};
use mathbot::command::CommandParams;

pub async fn ping(params: CommandParams) -> Result<()> {
    let systemtime = get_current_timestamp_millis()?;
    let msg_snowflake = params.msg.id.get();
    let msg_timestamp_millis = ((msg_snowflake >> 22) + 1420070400000) as u128;
    let ping_time_ms = systemtime - msg_timestamp_millis;

    let message = format!("Pong! Bot latency: {ping_time_ms}ms");

    send_message(message, &params).await?;
    Ok(())
}
