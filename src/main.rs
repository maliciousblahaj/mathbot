mod commands;


use std::env;
use mathbot::bot::BotBuilder;
use mathbot::command::{self, Command, CommandHelp};
use commands::{fun, info, math};
use mathbot::vec_of_strings;
use serenity::{all::GatewayIntents, Client};
use dotenv::dotenv;

const DATABASE_PATH: &'static str = "db/mathbot.db";
const BOT_PREFIX: &'static str = "dev ";
const DISCORD_TOKEN_ENV_VAR_NAME: &'static str = "DEV_TOKEN";

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()>{
    color_eyre::install().expect("Failed to install color_eyre");
    dotenv().expect("Failed to load environment variables");

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5) //TODO: Look into these settings more instead of just copy pasting
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(DATABASE_PATH)
                .create_if_missing(true),
        )
        .await?;

    sqlx::migrate!("./migrations").run(&database).await?;

    let bot = BotBuilder::new(BOT_PREFIX, database)?
        .register(info::commands())?
        .register(math::commands())?
        .register(fun::commands())?
        .register_single(
            Command::new(
                commands::test::test,
                vec_of_strings!["test", "test2", "t"], 
                command::CommandType::RootCommand { category: (command::CommandCategory::Test) },
                CommandHelp::new("responds with hello world.", ""),
            )
        )?
        .build()
        ;

    let token = env::var(DISCORD_TOKEN_ENV_VAR_NAME).expect("Invalid environment token");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(bot).await.expect("Error creating client");

    client.start().await?;

    Ok(())
}