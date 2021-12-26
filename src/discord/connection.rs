use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    model::prelude::*,
    utils::MessageBuilder
};
pub struct DiscordConnection;

#[async_trait]
impl EventHandler for DiscordConnection {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected discord!", ready.user.name);
    }
}


struct DiscordConnection2;
#[async_trait]
impl EventHandler for DiscordConnection2 {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }

    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut webhook = ctx.http.get_webhook_with_token(782282707253854328, "89A7nQnBOUrqz9o3b4orizWJNn_CP-F4T3z9ragsAx5k_7oiSPZz2QW0fB2EA9Z9BGsE").await.expect("Err get Webhook");
        let response = MessageBuilder::new()
                .push("test消息~~~~ <@!724827488588660837>")
                .push_bold_safe("粗体")
                .push(" used the 'ping' command in the ")
                // .mention(&channel)
                .push(" channel")
                .build();
        println!("{}", response);

        match webhook.execute(ctx.http, false, |mut m| {
            m.avatar_url("https://cdn.discordapp.com/avatars/724827488588660837/71919445a77c9076e3915da81028a305.webp?size=80");
            m.username("serenity");
            m.content(response);
            m
        }).await {
            Ok(mes) => {
                println!("发送成功: {:?}", mes);
            },
            Err(err) => {
                println!("Error sending message: {:?}", err);
            }
        }

        // if let Err(why) = ctx.http.send_message(ctx.channel_id, response).await {
        //     println!("Error sending message: {:?}", why);
        // }
        println!("{} is connected!", ready.user.name);
    }
}
