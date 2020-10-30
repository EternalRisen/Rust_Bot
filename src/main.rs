use serenity::{
    async_trait,
    framework::StandardFramework,
    model::gateway::{ Activity, Ready },
    prelude::*,
};

use serenity::framework::standard::macros::group;

// use tracing::{error, info};

mod commands;

use dotenv::dotenv;
use std::env;
use commands::{
    ping::*,
    rip::*
};

#[group]
#[commands(ping, rip)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::playing("What's my use?")).await;
        println!("{} is online!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occured while running the client: {:?}", why);
    }
}
