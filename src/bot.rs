use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use std::error::Error;

#[derive(Debug, serde::Deserialize)]
struct Summoner {
    puuid: String,
    gameName: String,
    tagLine: String,
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!summoner ") {
            let summoner_name = msg.content[10..].trim();

            match get_summoner_info(summoner_name).await {
                Ok(summoner) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Summoner Name: {}, Tagline: {}", summoner.gameName, summoner.tagLine)).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(why) => {
                    println!("Error fetching summoner info: {:?}", why);
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Failed to fetch summoner info.").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }
}

async fn get_summoner_info(summoner_name: &str) -> Result<Summoner, Box<dyn Error + Send + Sync>> {
    let token = env::var("RIOT_API_KEY").expect("Expected a Riot API token in the environment");

    // Разделяем имя призывателя на имя игры и тег
    let parts: Vec<&str> = summoner_name.split('#').collect();
    if parts.len() != 2 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid summoner name format. Expected format: 'GameName#TagLine'")));
    }
    let game_name = parts[0];
    let tag_line = parts[1];

    // Формируем URL
    let account_url = format!(
        "https://europe.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
        game_name, tag_line
    );

    let mut headers = HeaderMap::new();
    headers.insert("X-Riot-Token", HeaderValue::from_str(&token).unwrap());

    let response = reqwest::Client::new()
        .get(&account_url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let summoner: Summoner = response.json().await?;
        Ok(summoner)
    } else {
        let error_message = response.text().await?;
        println!("Error: {:?}", error_message);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Request failed")));
    }
}
