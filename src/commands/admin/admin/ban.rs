use color_eyre::owo_colors::OwoColorize;
use mathbot::{command::CommandParams, error::ClientError, get_current_timestamp_secs_i64, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};

pub async fn ban(params: CommandParams) -> Result<()> {
    let Some(account) = params.require_admin() else {return Ok(());};
    let (Some(target), Some(time)) = (params.args.get(0), params.args.get(1)) 
        else {return send_help(params).await;};

    let Some(targetaccount) = params.get_account_by_user_input(target).await
        else {return Err(Error::Client(ClientError::AdminBanInvalidAccount));};

    let Ok(time) = time.parse::<i64>() 
        else {return send_help(params).await;};

    let timemultiplier = match params.args.get(2) {
        Some(input) => get_time_multiplier(input.as_str()),
        None => 1,
    };

    let newtime = get_current_timestamp_secs_i64()? + time*timemultiplier;

    sqlx::query!("UPDATE Accounts SET banned=? WHERE id=?", newtime, targetaccount.id)
        .execute(params.state.get_model_controller().get_database())
        .await
        .map_err(|e| Error::FailedToBanAccount(e))?;

    send_embed(
        base_embed(&EmbedCtx::from_account(&account), ColorType::Admin)
            .title("Success")
            .description(format!("Successfully banned `@{}`. They will get unbanned <t:{newtime}:R>", targetaccount.username)), 
        &SendCtx::from_params(&params)
    ).await?;

    Ok(())
}

fn get_time_multiplier(input: &str) -> i64 {
    match input {
        "seconds" | "secs" | "second" | "s" => 1,
        "minutes" | "minute" | "mins" | "min" | "m"=> 60,
        "hours" | "hrs" | "h" => 3600,
        "days" | "day" | "d" => 86400,
        "weeks" | "week" | "w" => 604800,
        "months" | "month" => 2592000,
        "years" | "year" | "y" => 31536000,
        "decades" | "decade" => 315360000,
        "centuries" | "century" | "c" => 3153600000,
        "millenia" | "millenium" => 31536000000,
        _ => 1,
    }
}