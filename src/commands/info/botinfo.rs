use mathbot::{appearance::embed::{base_embed, ColorType}, command::CommandParams, get_current_timestamp_secs};
use mathbot::{send_embed, Error, Result, BOT_VERSION};

pub async fn botinfo(params: CommandParams) -> Result<()> {
    let stateguard = params.state.lock().map_err(|_| Error::PoisonedStateMutex)?;
    let starttime = stateguard.get_start_time().clone();
    drop(stateguard);

    let prefix = &params.bot_prefix;

    let time = get_current_timestamp_secs()? - starttime;

    let embed = base_embed(&params, ColorType::Settings)
        .field("Bot Version", format!("`{BOT_VERSION}`"), false)
        .field("Bot Uptime", format!("`{time}s`"), false)
        .field("Bot Prefix", format!("`{prefix}`"), false);

    send_embed(embed, &params).await?;
    Ok(())
}