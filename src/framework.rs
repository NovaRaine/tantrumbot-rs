use std::collections::HashSet;
use std::env;
use crate::commands;
use serenity::{
    framework::StandardFramework,
    http
};

pub fn get_framework() -> StandardFramework {
    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why)
    };

    let prefix = env::var("TANTRUM-RS_PREFIX");
    let prefix = prefix
        .as_ref()
        .map(String::as_str)
        .unwrap_or("!");

    let f = StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix(&prefix)
            .on_mention(true)
            .allow_whitespace(true))
        .command("ping", |c| c.cmd(commands::meta::ping))
        .command("test", |c| c.cmd(commands::meta::test))
        .command("good bot", |c| c.cmd(commands::meta::good_bot))
        .command("quit", |c| c
            .cmd(commands::owner::quit)
            .owners_only(true));

    f
}