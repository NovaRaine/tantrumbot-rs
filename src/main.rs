#[macro_use]
extern crate serenity;

pub mod commands;
mod framework;
use std::env;
use serenity::{
    model::gateway::Ready,
    prelude::*
};

fn main() {
    let token = env::var("DISCORD_TOKEN").expect(
        "Expected a token in the environment",
    );

    let mut client = Client::new(&token, Handler).expect("Oh no!");
    let framework = framework::get_framework();
    
    client.with_framework(framework);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}
