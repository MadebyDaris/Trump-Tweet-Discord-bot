use std::{env, fs::File, io::Read};

use serenity::{async_trait, client::EventHandler, framework::StandardFramework, model::channel::*, prelude::*, utils::MessageBuilder};
mod client;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!trump" {
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");

                    return;
                },
            };

        let tweet = client::generate();

        let response = MessageBuilder::new()
            .push("here you go: ").push_bold_line(&msg.author.name)
            .push_quote_line(tweet.contents).push_italic_line(tweet.date).build();

        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut token = String::new();
    let txt = File::open("TOKEN.txt").unwrap().read_to_string(&mut token);
    let framework = StandardFramework::new();
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).framework(framework).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}