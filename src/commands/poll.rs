use poise::serenity_prelude::ReactionType;

use super::super::utility::global::{Context, Error};
use super::super::utility::to_keycap_unicode::to_keycap_unicode;

#[poise::command(slash_command, prefix_command)]
pub async fn poll(
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
