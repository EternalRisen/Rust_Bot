use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::StandardFramework,
    http::Http,
    model::gateway::{ Activity, Ready },
    prelude::*,
};
use serenity::framework::standard::macros::group;
use tracing::{error, info};
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};

// use tracing::{error, info};

mod commands;

use dotenv::dotenv;
use std::{
    collections::HashSet,
    env,
    sync::Arc,
};
use commands::{
    ping::*,
    rip::*,
    quit::*
};

#[group]
#[commands(ping, rip, quit)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::playing("What's my use?")).await;
        info!("{} is online!", ready.user.name);
    }
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Failed to get token from environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .owners(owners)
                   .prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    if let Err(why) = client.start().await {
        error!("An error occured while running the client: {:?}", why);
    }
}
