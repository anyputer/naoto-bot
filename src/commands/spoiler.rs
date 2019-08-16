use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use serenity::utils::ContentModifier::Spoiler;

use unicode_segmentation::UnicodeSegmentation;

#[command]
fn spoiler(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    // delete message if possible
    if let Some(guild) = msg.guild(&ctx) {
        if guild
            .read()
            .member(&ctx, &ctx.cache.read().user)?
            .permissions(&ctx)?
            .manage_messages()
        {
            msg.delete(&ctx)?;
        }
    }

    msg.channel_id.say(
        &ctx,
        args.rest()
            .graphemes(true)
            .map(|g| (Spoiler + g).to_string())
            .collect::<String>(),
    )?;

    Ok(())
}
