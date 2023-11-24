use std::env;

use dotenvy::dotenv;
use poise::serenity_prelude::{self as serenity, ReactionType};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    _ctx: Context<'_>,
    #[description = "Selected User"] _user: Option<serenity::User>,
) -> Result<(), Error> {
    Ok(())
}

fn to_keycap_unicode(number: usize) -> String {
    match number {
        1 => "1️⃣",
        2 => "2️⃣",
        3 => "3️⃣",
        4 => "4️⃣",
        5 => "5️⃣",
        6 => "6️⃣",
        7 => "7️⃣",
        8 => "8️⃣",
        9 => "9️⃣",
        10 => "🔟",
        _ => "❓",
    }
    .to_string()
}

#[poise::command(slash_command, prefix_command)]
async fn poll(
    ctx: Context<'_>,
    #[description = "Question"] question: String,
    #[description = "Answer 1"] answer1: String,
    #[description = "Answer 2"] answer2: String,
    #[description = "Answer 3"] answer3: Option<String>,
    #[description = "Answer 4"] answer4: Option<String>,
    #[description = "Answer 5"] answer5: Option<String>,
    #[description = "Answer 6"] answer6: Option<String>,
    #[description = "Answer 7"] answer7: Option<String>,
    #[description = "Answer 8"] answer8: Option<String>,
    #[description = "Answer 9"] answer9: Option<String>,
    #[description = "Answer 10"] answer10: Option<String>,
) -> Result<(), Error> {
    let answers: Vec<_> = vec![
        Some(answer1),
        Some(answer2),
        answer3,
        answer4,
        answer5,
        answer6,
        answer7,
        answer8,
        answer9,
        answer10,
    ]
    .iter()
    .flatten()
    .enumerate()
    .map(|text| {
        (
            "",
            format!("{} — {}", to_keycap_unicode(text.0 + 1), text.1),
            false,
        )
    })
    .collect();

    let author_name = &ctx.author().name;
    let author_icon = ctx.author().avatar_url();

    let reply_handle = ctx
        .send(|builder| {
            builder.embed(|embed| {
                embed
                    .title(question)
                    .author(|author| {
                        author.name(author_name);
                        match author_icon {
                            Some(url) => author.icon_url(url),
                            _ => author,
                        }
                    })
                    .fields(answers.clone())
            })
        })
        .await?;

    let message = reply_handle.into_message().await?;

    for (index, _) in answers.iter().enumerate() {
        let emoji = ReactionType::Unicode(to_keycap_unicode(index + 1));
        message.react(ctx, emoji).await?;
    }

    message
        .channel_id
        .create_public_thread(ctx, message.id, |thread| thread.name("Discussion"))
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let token = env::var("DISCORD_TOKEN").expect("Failed to read DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let commands = vec![poll()];

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
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
