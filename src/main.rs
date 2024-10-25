use serenity::Client;
use serenity::model::gateway::GatewayIntents;
use std::env;
use std::error::Error;
use dotenvy::dotenv;

mod commands;
mod handler;

use handler::Handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the .env file
    dotenv()?;

    let token = env::var("DISCORD_TOKEN")
        .map_err(|_| "Expected a token in the environment")?;

    // Define intents for the bot
    let intents = GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT;


    let handler = Handler::new();

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .map_err(|_| "Error creating client")?;


    client.start().await.map_err(|err| {
        eprintln!("Client error: {:?}", err);
        "Client error"
    })?;

    Ok(())
}