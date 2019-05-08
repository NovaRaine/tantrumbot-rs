command!(ping(_ctx, msg) {
    let _ = msg.channel_id.say("Pong!");
    let _ = msg.react("❌");
});

command!(test(_ctx, msg) {
    let reply = "Hello";

    let _ = msg.channel_id.send_message(|m| m
        .content(reply)
        .embed(|e| e
            .title("Title")
            .description("Description")
            .footer(|f| f
                .text("Footer"))
            .colour((100, 100, 200))
        ));
});

command!(good_bot(_ctx, msg) {
    let _ = msg.channel_id.say(":smiley:");
});
