use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[async_trait]
pub trait Command: Send + Sync {
    async fn handle(&self, ctx: &Context, msg: &Message, input: &str);
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub mod help;
pub mod mastery;
pub mod about;