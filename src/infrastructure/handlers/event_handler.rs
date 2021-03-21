use serenity::prelude::*;
use serenity::{
    async_trait,
    model::{
        event::ResumedEvent,
        gateway::Ready
    }
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}
