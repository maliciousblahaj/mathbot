mod commands;


use std::env;
use mathbot::bot::BotBuilder;
use mathbot::command::{self, Command, CommandHelp};
use commands::{fun, info, math};
use mathbot::vec_of_strings;
use serenity::{all::GatewayIntents, Client};
use dotenv::dotenv;


#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()>{
    color_eyre::install().expect("Failed to install color_eyre");
    dotenv().expect("Failed to load environment variables");

    //initiate bot with prefix
    let bot = BotBuilder::new("dev ")?
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

    let token = env::var("DEV_TOKEN").expect("Invalid environment token");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(bot).await.expect("Error creating client");

    client.start().await?;

    Ok(())
}
