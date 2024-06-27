mod error;
mod commands;
mod bot;
mod command;
mod model;

pub use self::error::{Error, Result};

use std::env;
use bot::Bot;
use command::Command;
use serenity::{all::GatewayIntents, Client};
use dotenv::dotenv;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()>{
    color_eyre::install().expect("Failed to install color_eyre");
    dotenv().expect("Failed to load environment variables");

    //initiate bot with prefix
    let bot = Bot::new("dev ")
        .register(
            Command::new(
                commands::misc::test, 
                vec_of_strings!["test", "test2"], 
                command::CommandType::RootCommand { category: (command::CommandCategory::Test) },
            )
        );

    let token = env::var("DEV_TOKEN").expect("Invalid environment token");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(bot).await.expect("Error creating client");

    client.start().await?;

    Ok(())
}
