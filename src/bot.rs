use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use std::error::Error;

#[derive(Debug, serde::Deserialize)]
struct RiotAccount {
    puuid: String,
    #[serde(rename = "gameName")]
    game_name: Option<String>,
    #[serde(rename = "tagLine")]
    tag_line: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct Summoner {
    id: String, 
    #[serde(rename = "accountId")]
    account_id: Option<String>,
    puuid: String,
    #[serde(rename = "profileIconId")]
    profile_icon_id: Option<i32>,
    #[serde(rename = "summonerLevel")]
    summoner_level: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
struct LeagueEntry {
    #[serde(rename = "leagueId")]
    league_id: Option<String>,
    
    #[serde(rename = "summonerId")]
    summoner_id: Option<String>,
    
    #[serde(rename = "queueType")]
    queue_type: Option<String>,
    
    tier: Option<String>,
    rank: Option<String>,
    
    #[serde(rename = "leaguePoints")]
    league_points: Option<i32>,
    
    wins: Option<i32>,
    losses: Option<i32>,
    
    #[serde(rename = "hotStreak")]
    hot_streak: Option<bool>,
    #[serde(rename = "veteran")]
    veteran: Option<bool>,
    #[serde(rename = "freshBlood")]
    fresh_blood: Option<bool>,
    #[serde(rename = "inactive")]
    inactive: Option<bool>,
    
    
    mini_series: Option<MiniSeries>,
}

#[derive(Debug, serde::Deserialize)]
struct MiniSeries {
    wins: Option<i32>,
    losses: Option<i32>,
    
    #[serde(rename = "target")]
    target: Option<i32>,
    
    progress: Option<String>,
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!summoner ") {
            let summoner_input = msg.content[10..].trim();
            let parts: Vec<&str> = summoner_input.split('#').collect();
            let game_name = parts[0];
            let tag_line = if parts.len() > 1 { parts[1] } else { "" };

            match get_riot_account_info(game_name, tag_line).await {
                Ok(account) => {
                    println!("Fetched Riot Account Info: {:?}", account); // Debug output for RiotAccount

                    match get_summoner_info(&account.puuid).await {
                        Ok(summoner) => {
                            println!("Fetched Summoner Info: {:?}", summoner); // Debug output for Summoner

                            match get_league_info(&summoner.id).await {
                                Ok(league_entries) => {
                                    println!("League entries: {:?}", league_entries); // Debug output for LeagueEntry

                                    if league_entries.is_empty() {
                                        let response = format!(
                                            "No ranked info found for {}",
                                            account.game_name.as_deref().unwrap_or("this player")
                                        );
                                        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                                            println!("Error sending message: {:?}", why);
                                        }
                                        return;
                                    }

                                    let mut response = String::new();

                                    for league_entry in league_entries {
                                        let total_games = league_entry.wins.unwrap_or(0) + league_entry.losses.unwrap_or(0);
                                        let winrate = if total_games > 0 {
                                            (league_entry.wins.unwrap_or(0) as f32 / total_games as f32) * 100.0
                                        } else {
                                            0.0
                                        };

                                        response.push_str(&format!(
                                            "Rank: {} {}\nLeague Points: {}\nWins: {}\nLosses: {}\nWinrate: {:.2}%\n\n",
                                            league_entry.tier.as_deref().unwrap_or("Unranked"),
                                            league_entry.rank.as_deref().unwrap_or("Unranked"),
                                            league_entry.league_points.unwrap_or(0),
                                            league_entry.wins.unwrap_or(0),
                                            league_entry.losses.unwrap_or(0),
                                            winrate
                                        ));
                                    }

                                    if !response.is_empty() {
                                        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                                            println!("Error sending message: {:?}", why);
                                        }
                                    }
                                }
                                Err(why) => {
                                    println!("Error fetching league info: {:?}", why);
                                    let message = "Failed to fetch league info.";
                                    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                                        println!("Error sending message: {:?}", why);
                                    }
                                }
                            }
                        }
                        Err(why) => {
                            println!("Error fetching summoner info: {:?}", why);
                            let message = "Failed to fetch summoner info.";
                            if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    }
                }
                Err(why) => {
                    println!("Error fetching Riot account info: {:?}", why);
                    let message = "Failed to fetch Riot account info.";
                    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }
}

async fn get_riot_account_info(game_name: &str, tag_line: &str) -> Result<RiotAccount, Box<dyn Error + Send + Sync>> {
    let token = env::var("RIOT_API_KEY").expect("Expected a Riot API token in the environment");
    let account_url = format!(
        "https://europe.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
        game_name,
        tag_line
    );

    let mut headers = HeaderMap::new();
    headers.insert("X-Riot-Token", HeaderValue::from_str(&token).unwrap());

    let response = reqwest::Client::new()
        .get(&account_url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let account: RiotAccount = response.json().await?;
        Ok(account)
    } else {
        let error_message = response.text().await?;
        println!("Error fetching account info: {:?}", error_message); // Error handling modified to show message
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Request failed")))
    }
}

async fn get_summoner_info(puuid: &str) -> Result<Summoner, Box<dyn Error + Send + Sync>> {
    let token = env::var("RIOT_API_KEY").expect("Expected a Riot API token in the environment");
    let summoner_url = format!(
        "https://ru.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}",
        puuid
    );

    let mut headers = HeaderMap::new();
    headers.insert("X-Riot-Token", HeaderValue::from_str(&token).unwrap());

    let response = reqwest::Client::new()
        .get(&summoner_url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let summoner: Summoner = response.json().await?;
        Ok(summoner)
    } else {
        let error_message = response.text().await?;
        println!("Error fetching summoner info: {:?}", error_message); // Error handling modified to show message
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Request failed")))
    }
}

async fn get_league_info(summoner_id: &str) -> Result<Vec<LeagueEntry>, Box<dyn Error + Send + Sync>> {
    let token = env::var("RIOT_API_KEY").expect("Expected a Riot API token in the environment");
    let league_url = format!(
        "https://ru.api.riotgames.com/lol/league/v4/entries/by-summoner/{}",
        summoner_id
    );

    let mut headers = HeaderMap::new();
    headers.insert("X-Riot-Token", HeaderValue::from_str(&token).unwrap());

    let response = reqwest::Client::new()
        .get(&league_url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let league_entries: Vec<LeagueEntry> = response.json().await?;
        Ok(league_entries)
    } else {
        let error_message = response.text().await?;
        println!("Error fetching league info: {:?}", error_message); // Error handling modified to show message
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Request failed")))
    }
}