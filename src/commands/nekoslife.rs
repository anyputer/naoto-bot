use crate::utils;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("nl")]
#[usage("<image category> <optional amount>")]
#[example("hug 3")]
fn nekoslife(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use itertools::Itertools;
    use nekos::{self, ImageCategory};
    use random_color::{Luminosity, RandomColor};

    let channel = msg.channel_id.to_channel(&ctx)?;
    let client = nekos::Client::new();
    let embed_color = utils::gen_random_color(RandomColor::new().luminosity(Luminosity::Light));

    if let Ok(category) = args.single::<ImageCategory>() {
        if category.is_sfw() || channel.is_nsfw() || msg.is_private() {
            msg.channel_id.broadcast_typing(&ctx)?;

            if let Ok(mut amount) = args.single::<usize>() {
                if amount > 50 {
                    amount = 50;
                }

                let responses = client.get_random_images(category.clone(), amount);
                dbg!(responses.clone());
                let output = responses.iter().map(|img| &img.url).join("\n");

                msg.channel_id.say(&ctx, output)?;
            } else {
                let image = &client.get_random_images(category.clone(), 1)[0];

                msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.footer(|f| f.text(format!("Category: {}", category)))
                            .image(&image.url)
                            .colour(embed_color)
                    })
                })?;
            }
        } else {
            msg.channel_id.say(
                &ctx,
                "The image category isn't SFW, so you have to be in an NSFW channel to use it.",
            )?;
        }
    } else {
        let mut categories = client.get_image_categories()?;
        if !(channel.is_nsfw() || msg.is_private()) {
            categories.retain(|c| c.is_sfw());
        }

        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Available Image Categories")
                    .description(categories.iter().join(utils::SEPERATOR))
                    .colour(embed_color)
            })
        })?;
    }

    Ok(())
}
