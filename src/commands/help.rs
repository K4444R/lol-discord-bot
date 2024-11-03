use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::collections::HashMap;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};

use super::Command; 
use crate::commands::mastery::MasteryCommand; 
use crate::commands::about::AboutCommand;
use crate::commands::profile::ProfileCommand;
use crate::commands::region::RegionCommand;

pub struct HelpCommand;

impl HelpCommand {
    pub fn new() -> Self {
        HelpCommand
    }
}


fn get_all_commands() -> HashMap<String, Box<dyn Command + Send + Sync>> {
    let mut commands = HashMap::new();
    
    
    commands.insert("mastery".to_string(), Box::new(MasteryCommand::new()) as Box<dyn Command + Send + Sync>);
    commands.insert("about".to_string(), Box::new(AboutCommand::new()) as Box<dyn Command + Send + Sync>);
    commands.insert("help".to_string(), Box::new(HelpCommand::new()) as Box<dyn Command + Send + Sync>);
    commands.insert("profile".to_string(), Box::new(ProfileCommand::new()) as Box<dyn Command + Send + Sync>);
    commands.insert("region".to_string(), Box::new(RegionCommand::new()) as Box<dyn Command + Send + Sync>);
    
    commands
}

#[async_trait]
impl Command for HelpCommand {
    async fn handle(&self, ctx: &Context, msg: &Message, _input: &str) {
        let commands = get_all_commands(); 
        let mut response = String::from("To use the bot, write `/kir <command>`\nList of available commands:\n");

        for command in commands.values() {
            response.push_str(&format!("`{}` - {}\n", command.name(), command.description()));
        }

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            tracing::error!("Error sending message: {:?}", why);
        }
    }

    async fn handle_interaction(&self, ctx: &Context, interaction: &Interaction) {
        if let Some(command_interaction) = interaction.as_command() {
            let commands = get_all_commands(); 
            let mut response = String::from("List of available commands:\n");

            for command in commands.values() {
                response.push_str(&format!("`{}` - {}\n", command.name(), command.description()));
            }


            let create_response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::default().content(response));

            if let Err(why) = command_interaction.create_response(&ctx.http, create_response).await {
                tracing::error!("Error responding to interaction: {:?}", why);
            }
        } else {
            println!("This interaction is not a command interaction.");
        }
    }


    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Show the list of available commands."
    }
}
