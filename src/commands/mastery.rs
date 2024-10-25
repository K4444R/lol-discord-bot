use anyhow::Result;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use riven::{RiotApi, consts::PlatformRoute};
use std::env;

use crate::commands::Command;

pub struct MasteryCommand;

impl MasteryCommand {
    pub fn new() -> Self {
        MasteryCommand
    }
}

#[async_trait]
impl Command for MasteryCommand {
    async fn handle(&self, ctx: &Context, msg: &Message, input: &str) {
        let command_input = input.to_string();
        let parts: Vec<&str> = command_input.trim().split('#').collect();
        let game_name = parts[0];
        let tag_line = if parts.len() > 1 { parts[1] } else { "" };

        match get_champion_masteries(game_name, tag_line).await {
            Ok(response) => {
                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            Err(why) => {
                println!("Error fetching champion masteries: {:?}", why);
                if let Err(why) = msg.channel_id.say(&ctx.http, "Failed to fetch champion masteries.".to_string()).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    fn name(&self) -> &str {
        "mastery"
    }

    fn description(&self) -> &str {
        "Fetch champion masteries for the provided game name and tag."
    }
}

async fn get_champion_masteries(game_name: &str, tag_line: &str) -> Result<String> { 
    let api_key = env::var("RIOT_API_KEY")?; 
    let riot_api = RiotApi::new(&api_key);
    let platform = PlatformRoute::RU; // Make sure this is the correct region

    debug_log(&format!("Fetching account for {}#{}", game_name, tag_line));

    // Get the account by name and tag
    let account = match riot_api.account_v1().get_by_riot_id(platform.to_regional(), game_name, tag_line).await {
        Ok(account) => account.expect("There is no summoner with that name."),
        Err(e) => {
            println!("Error fetching account: {:?}", e);
            return Ok("Failed to fetch account.".to_string());
        }
    };

    debug_log(&format!("Account details: {:?}, {:?}", account.game_name, account.tag_line));

    let mut response = format!("\n{}#{} Champion Masteries:\n```", 
        account.game_name.unwrap_or_default(), 
        account.tag_line.unwrap_or_default());

    debug_log("Fetching champion masteries...");

    let masteries = match riot_api.champion_mastery_v4().get_all_champion_masteries_by_puuid(platform, &account.puuid).await {
        Ok(masteries) => masteries,
        Err(e) => {
            println!("Error fetching champion masteries: {:?}", e);
            return Ok("Failed to fetch champion masteries.".to_string());
        }
    };

    if masteries.is_empty() {
        debug_log("No champion masteries found.");
        response.push_str("No champion masteries found.\n");
    } else {
        for (i, mastery) in masteries.iter().take(10).enumerate() {
            response.push_str(&format!("{: >2}) {: <9}    {: >7} ({})\n", 
                i + 1,
                mastery.champion_id.name().unwrap_or("UNKNOWN"),
                mastery.champion_points, 
                mastery.champion_level));
        }
    }

    response.push_str("```\n"); // Закрывающая обратная кавычка блока
    Ok(response)
}

fn debug_log(message: &str) {
    println!("[DEBUG] {}", message);
}