use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args,
    CommandResult,
    macros::command,
};
use rand::prelude::*;
use rand::rngs::StdRng;
use std::format;

#[command]
async fn rip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    const RWEEP: [&str; 14] = [
        ";w;",
        ";",
        ";;",
        ";-;",
        ":(",
        "T-T",
        ":'‑(",
        ":'(",
        ";(",
        ":{",
        ":c",
        "D:",
        ":<",
        "🦀"
    ];

    const ALIGNMENT: [&str; 12] = [
        "a good person",
        "a bad person",
        "very caring",
        "an asshole",
        "a wonderful person",
        "ok... i guess...",
        "tolerable",
        "honestly not that important",
        "a pretty bad person",
        "the literal worst",
        "the best",
        "my crush"
    ];

    const CAUSE: [&str; 8] = [
        "in that car accident",
        "in that nuclear bomb that North Korea set off",
        "in that fatal computer explosion",
        "from ... wait, how did they die? oh well...",
        "from COVID",
        "in a fire",
        "while petting a kitty",
        "from being thrown out a window"
    ];

    const F: [&str; 7] = [
        "RIP",
        "F",
        "Press f for",
        "rest in piss",
        "no one liked you anyway",
        "unfortunate,",
        "take me with you"
    ];

    let mut rng = StdRng::from_entropy();

    let name = args.rest();

    let ripmsg = format!("{name} was {alignment}\nWhy did they have to die {cause}\n{f} {name}\n{weep}", name = name, alignment = ALIGNMENT.choose(&mut rng).unwrap(), cause = CAUSE.choose(&mut rng).unwrap(), f = F.choose(&mut rng).unwrap(), weep = RWEEP.choose(&mut rng).unwrap());

    if name == "" {
        msg.reply(&ctx, "No name given.").await?;
    } else {
        msg.channel_id.say(&ctx, ripmsg).await?;
    }

    Ok(())
}
