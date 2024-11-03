use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::application::Interaction;
use serenity::prelude::*;

#[async_trait]
pub trait Command: Send + Sync {
    async fn handle(&self, ctx: &Context, msg: &Message, input: &str);

    async fn handle_interaction(&self, ctx: &Context, interaction: &Interaction);
    
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub mod help;
pub mod mastery;
pub mod about;
pub mod profile;
pub mod region;