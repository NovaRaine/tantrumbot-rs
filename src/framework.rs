use std::collections::HashSet;
use crate::commands;
use serenity::{
    framework::StandardFramework,
    prelude::*,
    http,
};

pub fn get_framework() -> StandardFramework {
    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    let f = StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("!")
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