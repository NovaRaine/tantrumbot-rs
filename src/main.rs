#[macro_use]
extern crate serenity;

pub mod commands;
mod framework;
use std::env;
use serenity::{
    model::gateway::{Game, Ready},
    prelude::*
};

fn main() {
    let token = env::var("TANTRUM_RS_TOKEN").expect(
        "No Discord token in env",
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
    fn ready(&self, context: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        let prefix = env::var("TANTRUM-RS_PREFIX");
        let prefix = prefix
            .as_ref()
            .map(String::as_str)
            .unwrap_or("!");

        let g = Game::playing(&prefix);
        context.set_game(g);
    }
}
