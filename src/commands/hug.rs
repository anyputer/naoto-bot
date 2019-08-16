use crate::utils;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use serenity::model::id::UserId;
use serenity::utils::parse_username;

use nekos::ImageCategory;
use rand::seq::SliceRandom;
use random_color::{Luminosity, RandomColor};

const TEXT: [&'static str; 14] = [
    "soft~!",
    "aaaa warm",
    "how cute!!",
    "awwwww",
    "two cuties hugging eachother uwu",
    "uwu",
    "owo",
    "awa",
    ":3c",
    "^w^",
    "^~^",
    "sdjgiosghujsiohuj",
    "aaaaaaaaaa",
    "aaaa",
];

const TEXT_BOT: [&'static str; 11] = [
    "*clunk clunk*",
    "beep boop",
    "aaaaaaa beep beep",
    "bloop beep",
    "beepity boopy",
    "beep beeeeep",
    "*computer noise*",
    "*dial-up sound*",
    "<[o w o]>",
    "01110101 01110111 01110101",
    "01110110 01100001 01101100 01101001 01100100",
];

const TEXT_SELF: [&'static str; 5] = [
    "nooooo ;-;",
    "i.. wanted to give you a hug...",
    "aww ;c",
    "uwu..",
    "i'm here for you..",
];

const TEXT_NAOTO: [&'static str; 10] = [
    "aaaaaa thank you!!!",
    "i needed that!!!!",
    "skskskskssksksksks cutie",
    "yes please",
    "thanks uwu",
    "you're super warm and comfy uwu",
    "gimme that~ aaaa",
    "you're super cute and valid!!!",
    "*hugs back*",
    "^u^",
];

#[command]
fn hug(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let hugged_id = parse_username(args.current().ok_or("no args passed")?)
        .ok_or("arg passed isn't a valid user mention")?;

    let hugged = UserId(hugged_id).to_user(&ctx)?;

    let text = if msg.author == hugged {
        TEXT_SELF.choose(&mut rand::thread_rng())
    } else if hugged == ctx.cache.read().user.id.to_user(&ctx)? {
        TEXT_NAOTO.choose(&mut rand::thread_rng())
    } else if hugged.bot {
        TEXT_BOT.choose(&mut rand::thread_rng())
    } else {
        TEXT.choose(&mut rand::thread_rng())
    }
    .unwrap();

    let image = ImageCategory::Hug.get_random()?;

    let author_name = if msg.is_private() {
        msg.author.tag()
    } else {
        ctx.cache
            .read()
            .member(msg.guild_id.unwrap(), msg.author.id)
            .unwrap()
            .display_name()
            .into_owned()
    };

    let hugged_name = if msg.is_private() {
        hugged.tag()
    } else {
        ctx.cache
            .read()
            .member(msg.guild_id.unwrap(), hugged.id)
            .unwrap()
            .display_name()
            .into_owned()
    };

    let embed_color = utils::gen_random_color(RandomColor::new().luminosity(Luminosity::Light));

    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.author(|a| {
                a.name(format!(
                    "\u{1f917} {} hugs {}! {}",
                    author_name, hugged_name, text
                ))
                .icon_url(msg.author.face())
            })
            .image(image.url)
            .color(embed_color)
        })
    })?;
    Ok(())
}
