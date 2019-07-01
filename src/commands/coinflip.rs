use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[description = "Flips a coin."]
fn coinflip(ctx: &mut Context, msg: &Message) -> CommandResult {
    if rand::random() {
        msg.channel_id.say(&ctx, "The coin landed on heads.")?;
    } else {
        msg.channel_id.say(&ctx, "The coin landed on tails.")?;
    }

    Ok(())
}
