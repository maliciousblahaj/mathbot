// bot rules:
//
// each command should always map to a response
// TODO

mod error;
mod commands;
mod bot;
mod command;

pub use self::error::{Error, Result};

use std::env;
use serenity::{all::{Context, EventHandler, GatewayIntents, Message}, async_trait, Client};
use dotenv::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()>{
    color_eyre::install()?;
    dotenv()?;

    let token = env::var("DEV_TOKEN").expect("Invalid environment token");
    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");

    client.start().await?;

    Ok(())
}
