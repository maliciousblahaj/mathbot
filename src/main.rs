mod error;
mod commands;
mod bot;
mod command;
mod model;

pub use self::error::{Error, Result};

use std::{env, sync::{Arc, Mutex}};
use bot::Bot;
use model::State;
use serenity::{all::{Context, EventHandler, GatewayIntents, Message}, async_trait, Client};
use dotenv::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content == "dev test" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello world!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()>{
    color_eyre::install().expect("Failed to install color_eyre");
    dotenv().expect("Failed to load environment variables");

    //set prefix
    let bot = Bot::new("dev ");
    let global_state = Arc::new(Mutex::new(
        State::new(bot)
    ));

    let token = env::var("DEV_TOKEN").expect("Invalid environment token");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");

    client.start().await?;

    Ok(())
}
