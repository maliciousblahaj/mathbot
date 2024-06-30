pub mod error;
pub mod bot;
pub mod command;
pub mod model;
pub mod parser;
pub mod logging;
pub mod appearance;

#[cfg(test)]
mod tests;

pub const BOT_VERSION: &'static str = "3.0";

use command::CommandParams;
use serenity::all::{CreateEmbed, CreateMessage};

pub use error::{Error, Result};

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

use std::{fmt::Display, time::{Duration, SystemTime, UNIX_EPOCH}};

pub fn get_current_timestamp() -> Result<Duration> {
    Ok(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::FailedToGetSystemTimestamp)?
    )
}

pub fn get_current_timestamp_secs() -> Result<u64> {
    get_current_timestamp().map(|t| t.as_secs())
}

pub fn get_current_timestamp_millis() -> Result<u128> {
    get_current_timestamp().map(|t| t.as_millis())
}

pub async fn send_embed(embed: CreateEmbed, params: &CommandParams) -> Result<()> {
    params.msg.channel_id.send_message(
        &params.ctx.http, 
        CreateMessage::new().embed(embed)
    )
        .await
        .map_err(|_| Error::FailedToSendMessage)?;
    Ok(())
}

pub async fn send_message<S: AsRef<str> + Display>(message: S, params: &CommandParams) -> Result<()> {
    params.msg.channel_id.say(
        &params.ctx.http, 
        message.to_string(),
    )
        .await
        .map_err(|_| Error::FailedToSendMessage)?;
    Ok(())
}