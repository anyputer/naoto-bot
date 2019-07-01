use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
fn spoiler(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use serenity::utils::MessageBuilder;
    use unicode_segmentation::UnicodeSegmentation;

    let mut output = MessageBuilder::new();
    for grapheme in args.rest().graphemes(true) {
        output.push_spoiler(grapheme);
    }

    msg.channel_id.say(&ctx, output)?;

    Ok(())
}
