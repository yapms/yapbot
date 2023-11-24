use std::env;

use dotenvy::dotenv;
use poise::serenity_prelude as serenity;

mod commands;
mod utility;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let token = env::var("DISCORD_TOKEN").expect("Failed to read DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let commands = vec![commands::poll::poll()];

    let options = poise::FrameworkOptions {
        commands,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(token)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(utility::global::Data {})
            })
        });

    framework.run().await.unwrap();
}
