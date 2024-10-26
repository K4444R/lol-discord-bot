use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::collections::HashMap;

use crate::commands::mastery::MasteryCommand;
use crate::commands::about::AboutCommand;
use crate::commands::help::HelpCommand;
use crate::commands::profile::ProfileCommand;
use crate::commands::Command;

pub struct CommandHandler {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

        
        commands.insert("mastery".to_string(), Box::new(MasteryCommand::new())); 
        commands.insert("about".to_string(), Box::new(AboutCommand::new())); 
        commands.insert("profile".to_string(), Box::new(ProfileCommand::new())); 
        commands.insert("help".to_string(), Box::new(HelpCommand::new())); 
        
        

        CommandHandler { commands }
    }

    pub async fn handle_command(&self, ctx: &Context, msg: &Message, command_input: &str) {
        let parts: Vec<&str> = command_input.split_whitespace().collect();
        if let Some(command_name) = parts.get(0).map(|s| *s) {
            if let Some(command) = self.commands.get(command_name) {
                command.handle(ctx, msg, &command_input[command_name.len()..].trim()).await;
            } else {
                let _ = msg.channel_id.say(&ctx.http, "Unknown command.").await;
            }
        }
    }
}

pub struct Handler {
    command_handler: CommandHandler,
}

impl Handler {
    pub fn new() -> Self {
        let command_handler = CommandHandler::new();
        Handler { command_handler }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("/kir ") {
            let command_input = &msg.content[5..].trim().to_string();
            self.command_handler.handle_command(&ctx, &msg, command_input).await;
        }
    }
}