use std::env;
use serenity::Client;
use serenity::model::gateway::GatewayIntents;
use dotenv::dotenv;

mod bot;
use bot::Handler;

#[tokio::main]
async fn main() {
    // Получаем токен бота из переменной окружения
    dotenv().ok(); // Загружаем переменные окружения из файла .env
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    // Устанавливаем необходимые намерения (Gateway Intents)
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Создаем экземпляр клиента с обработчиком событий
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Запускаем клиента
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
