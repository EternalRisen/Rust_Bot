// use std::env;

mod bot;

fn main() {
    let bot = bot::Bot::new("token.", "!");
    println!("token: {}, prefix: {}", bot.token, bot.prefix);
}
