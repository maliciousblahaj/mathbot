mod commands;


use std::{env, str::FromStr};
use mathbot::bot::BotBuilder;
use commands::{admin, currency, fun, info, math, hidden, user};
use serenity::{all::GatewayIntents, Client};
use dotenv::dotenv;

//const BOT_PREFIX: &'static str = "dev ";
//const DISCORD_TOKEN_ENV_VAR_NAME: &'static str = "DEV_TOKEN";

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()>{
    color_eyre::install().expect("Failed to install color_eyre");
    dotenv().expect("Failed to load environment variables");

    let database_url = env::var("DATABASE_URL")?;
    let bot_prefix = env::var("BOT_PREFIX")?;
    let token = env::var("DISCORD_TOKEN").expect("Invalid environment token");

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5) //TODO: Look into these settings more instead of just copy pasting
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(database_url.as_str())?
                .create_if_missing(true),
        )
        .await?;

    sqlx::migrate!("./migrations").run(&database).await?;

    let bot = BotBuilder::new(bot_prefix, database)?
        .register(info::commands())?
        .register(user::commands())?
        .register(currency::commands())?
        .register(fun::commands())?
        .register(math::commands())?
        .register(admin::commands())?
        .register(hidden::commands())?
        .build()
        ;

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(bot).await.expect("Error creating client");

    client.start().await?;

    Ok(())
}