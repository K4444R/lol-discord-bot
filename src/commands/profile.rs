use anyhow::Result;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use riven::{RiotApi, consts::PlatformRoute};
use std::env;

use crate::commands::Command;
use crate::commands::region::RegionCommand;

pub struct ProfileCommand;

impl ProfileCommand {
    pub fn new() -> Self {
        ProfileCommand
    }
}

#[async_trait]
impl Command for ProfileCommand {
    async fn handle(&self, ctx: &Context, msg: &Message, input: &str) {
        let command_input = input.to_string();
        let parts: Vec<&str> = command_input.trim().split('#').collect();
        let game_name = parts[0];
        let tag_line = if parts.len() > 1 { parts[1] } else { "" };
        
        let region = RegionCommand::get_region();

        match get_summoner_stats(region, game_name, tag_line).await {
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

    async fn handle_interaction(&self, ctx: &Context, interaction: &Interaction) {
        if let Some(command_interaction) = interaction.as_command() {
            let input = command_interaction.data.options.iter()
                .find(|option| option.name == "input") // Предполагается, что вы передали параметр input
                .map(|option| option.value.as_str().unwrap_or(""))
                .unwrap_or("");
    
            let parts: Vec<&str> = input.trim().split('#').collect();
            let game_name = parts[0];
            let tag_line = if parts.len() > 1 { parts[1] } else { "" };
            
            let region = RegionCommand::get_region();
    
            match get_summoner_stats(region, game_name, tag_line).await {
                Ok(response) => {
                    let create_response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::default().content(response));
    
                    if let Err(why) = command_interaction.create_response(&ctx.http, create_response).await {
                        tracing::error!("Error responding to interaction: {:?}", why);
                    }
                }
                Err(why) => {
                    println!("Error fetching champion masteries: {:?}", why);
                    let error_response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::default().content("Failed to fetch champion masteries.".to_string()));
    
                    if let Err(why) = command_interaction.create_response(&ctx.http, error_response).await {
                        tracing::error!("Error responding to interaction: {:?}", why);
                    }
                }
            }
        } else {
            println!("This interaction is not a command interaction.");
        }
    }
    
    fn name(&self) -> &str {
        "profile"
    }

    fn description(&self) -> &str {
        "Show account and general statistics for the Disocrd user."
    }
}

async fn get_summoner_stats(platform: PlatformRoute, game_name: &str, tag_line: &str) -> Result<String> {
    let api_key = env::var("RIOT_API_KEY")?; 
    let riot_api = RiotApi::new(&api_key);


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

    let mut response = format!("\n**Riot ID**: {}#{} ({})\n", 
        account.game_name.unwrap_or_default(), 
        account.tag_line.unwrap_or_default(),
        platform.as_region_str());

    debug_log(&format!("Fetching summoner profile for {}#{}", game_name, tag_line));

    let summoner = match riot_api.summoner_v4().get_by_puuid(platform, &account.puuid).await {
        Ok(summoner) => summoner,
        Err(e) => {
            println!("Error fetching summoner profile: {:?}", e);
            return Ok("Failed to fetch summoner profile".to_string());
        }
    };

    // Get and add information
    let (rank_info, total_wins, total_losses, win_rate) = extract_league_info(&riot_api, platform, &summoner.id).await?;

    
    response.push_str("**Summoner Statistics**:\n```");
    response.push_str(&format!(
        "{: <20}: {}\n\
         {: <20}: {}\n\
         {: <20}: {}\n\
         {: <20}: {}\n\
         {: <20}: {:.2}%\n",
        "Summoner Level", summoner.summoner_level,
        "Rank", rank_info,
        "Total Wins", total_wins,
        "Total Losses", total_losses,
        "Win Rate", win_rate
    ));

    response.push_str("```\n"); 
    Ok(response)
}

async fn extract_league_info(riot_api: &RiotApi, platform: PlatformRoute, summoner_id: &str) -> Result<(String, u32, u32, f64)> {
    let leagues = match riot_api.league_v4().get_league_entries_for_summoner(platform, summoner_id).await {
        Ok(leagues) => leagues,
        Err(e) => {
            println!("Error fetching leagues: {:?}", e);
            return Ok(("Unknown Rank".to_string(), 0, 0, 0.0));
        }
    };

    let mut total_wins = 0;
    let mut total_losses = 0;
    let mut rank_info = String::new();

    for entry in leagues {
        total_wins += entry.wins;
        total_losses += entry.losses;

        rank_info.push_str(&format!(
            "{tier} {rank}: {lp} LP",
            tier = entry.tier.expect("No tier"),
            rank = entry.rank.expect("Unranked"),
            lp = entry.league_points,
        ));
    }

    let total_games = total_wins + total_losses;
    let win_rate = if total_games > 0 {
        (total_wins as f64 / total_games as f64 * 100.0).round()
    } else {
        0.0
    };

    Ok((rank_info, total_wins as u32, total_losses as u32, win_rate))
}



fn debug_log(message: &str) {
    println!("[DEBUG] {}", message);
}