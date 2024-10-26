use crate::commands::Command; 
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub struct AboutCommand;

impl AboutCommand {
    pub fn new() -> Self {
        AboutCommand
    }
}


#[async_trait]
impl Command for AboutCommand {
    async fn handle(&self, ctx: &Context, msg: &Message, _input: &str) {
       
        let mut response = String::from(":page_with_curl:**Information**\n");
        response.push_str("Author: K4444R#RU1\n");
        response.push_str("Source Code\n");
        response.push_str("[Kir bot on GitHub](https://github.com/K4444R/lol-discord-bot)\n");
        

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    }

    fn name(&self) -> &str {
        "about"
    }

    fn description(&self) -> &str {
        "Show the information about bot."
    }
}