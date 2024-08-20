use crate::{Context, Error};
use poise::serenity_prelude as serenity;


/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

// #[poise::command(prefix_command, slash_command)]
// async fn message(
//     ctx: Context<'_>,
//     #[description = "Message to send"] message: String,    
// ) -> Result<(), Error> {
//     let mut args = msg.content.splitn(2, ' ');

//     if let (Some("~setgame"), Some(game)) = (args.next(), args.next()) {
//         ctx.set_activity(Some(ActivityData::playing(game)));
//     }
// }

/// Add one to the count
///
/// Enter '>count` to add to the count
#[poise::command(prefix_command, slash_command)]
pub async fn vote(
    ctx: Context<'_>,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let (previous_count, total_count) = {
        let mut number = ctx.data().count.lock().unwrap();
        *number += 1;
        (*number - 1, *number)
    };
    let response = format!("Previous count was **{previous_count}**. Count is now **{total_count}**.");
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn get_votes(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let count = *ctx.data().count.lock().unwrap();
    ctx.say(format!("The current count is **{count}**.")).await?;
    Ok(())
}