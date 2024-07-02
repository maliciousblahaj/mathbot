pub mod error;
pub mod bot;
pub mod command;
pub mod model;
pub mod parser;
pub mod logging;
pub mod appearance;

#[cfg(test)]
#[allow(unused)]
mod tests;

pub const BOT_VERSION: &'static str = "MathBot 3.0";

use command::CommandParams;
use serenity::all::{ChannelId, CreateEmbed, CreateMessage, Message};

pub use error::{Error, Result};

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

use std::{fmt::Display, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

pub fn get_current_timestamp() -> Result<Duration> {
    Ok(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::FailedToGetSystemTimestamp(e))?
    )
}

pub fn get_current_timestamp_secs() -> Result<u64> {
    get_current_timestamp().map(|t| t.as_secs())
}

pub fn get_current_timestamp_millis() -> Result<u128> {
    get_current_timestamp().map(|t| t.as_millis())
}

pub struct SendCtx {
    channel_id: ChannelId,
    cache_http: Arc<serenity::http::Http>,
}

impl SendCtx {
    pub fn from_params(params: &CommandParams) -> Self {
        Self {
            channel_id: params.msg.channel_id,
            cache_http: params.ctx.http.clone(),
        }
    }
}

pub async fn send_embed(embed: CreateEmbed, ctx: &SendCtx) -> Result<Message> {
    ctx.channel_id.send_message(
        &ctx.cache_http, 
        CreateMessage::new().embed(embed)
    )
        .await
        .map_err(|e| Error::FailedToSendMessage(e))
}

pub async fn send_text<S: AsRef<str> + Display>(message: S, ctx: &SendCtx) -> Result<Message> {
    ctx.channel_id.say(
        &ctx.cache_http, 
        message.to_string(),
    )
        .await
        .map_err(|e| Error::FailedToSendMessage(e))
}

pub async fn send_message (message: CreateMessage, ctx: &SendCtx) -> Result<Message> {
    ctx.channel_id.send_message(
        &ctx.cache_http, 
        message
    )
        .await
        .map_err(|e| Error::FailedToSendMessage(e))
}