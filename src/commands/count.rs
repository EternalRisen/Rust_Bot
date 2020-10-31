use serenity::{model::prelude::*, framework::standard::{CommandResult, macros::command}};
use serenity::prelude::*;

use crate::CommandCounter;

#[command]
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read().await;
    let counter = data.get::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");

    for (k, v) in counter {
        //writeln!(contents, "- {name}: {amount}", name=k, amount=v)?;
        contents += &format!("- {name}: {amount}\n", name=k, amount=v);
    }

    msg.channel_id.say(&ctx.http, &contents).await?;

    Ok(())
}