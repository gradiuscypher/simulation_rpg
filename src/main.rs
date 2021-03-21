mod commands;
mod infrastructure;

use crate::infrastructure::handlers::event_handler::Handler;
use crate::infrastructure::models::bot::command_group::*;

use dotenv;
use std::env;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{
        StandardFramework
    },
    http::Http,
    prelude::*,
};

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new_with_token(&token);

    let (owners, _) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("."))
            .group(&OWNER_GROUP)
            .group(&MISC_GROUP)
			.group(&NERD_GROUP);
    
    let mut client = Client::builder(&token)
        .cache_update_timeout(std::time::Duration::from_millis(500))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");
    
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
