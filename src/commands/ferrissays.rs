use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("ferris")]
#[description = "Makes Ferris the Crab say something."]
#[usage("<text to say>")]
#[example("Hello, world!")]
fn ferrissays(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use serenity::utils::MessageBuilder;

    let mut output = MessageBuilder::new();
    let mut ferris = Vec::new();

    ferris_says::say(args.rest().as_bytes(), 24, &mut ferris)?;
    output.push_codeblock(String::from_utf8(ferris)?, None);

    msg.channel_id.say(&ctx, output)?;

    Ok(())
}
