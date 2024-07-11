use chrono::TimeDelta;
use mathbot::{command::CommandParams, get_current_timestamp_secs, ui::embed::{base_embed, ColorType}, SendCtx};
use mathbot::{send_embed, Result, BOT_VERSION};

pub async fn botinfo(params: CommandParams) -> Result<()> {
    let starttime = params.state.get_start_time().clone();

    let prefix = &params.bot_prefix;

    let timestr = get_timedelta_string(TimeDelta::seconds((get_current_timestamp_secs()? - starttime) as i64));

    let embed = base_embed(&params.get_embed_ctx(), ColorType::Settings)
        .title("Bot Information")
        .field("Bot Version", format!("`{BOT_VERSION}`"), false)
        .field("Bot Uptime", timestr, false)
        .field("Bot Prefix", format!("`{prefix}`"), false);

    send_embed(embed, &SendCtx::from_params(&params)).await?;
    Ok(())
}

fn get_timedelta_string(time: TimeDelta) -> String {
    let days = time.num_days();
    let hours = time.num_hours();
    let minutes = time.num_minutes();
    let seconds = time.num_seconds();

    format!("`{}{}:{:02}:{:02}`",
        match days {
            0 => "".to_string(),
            1 => format!("{days} day, "),
            days => format!("{days} days, "),
        },
        hours%24,
        minutes%60,
        seconds%60,
    )
}