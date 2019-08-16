use crate::utils;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("nl")]
#[usage("<image category> <optional amount>")]
#[example("hug 3")]
fn nekoslife(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use enum_iterator::IntoEnumIterator;
    use itertools::Itertools;
    use nekos::{self, ImageCategory};
    use random_color::{Luminosity, RandomColor};

    let channel = msg.channel_id.to_channel(&ctx)?;
    let embed_color = utils::gen_random_color(RandomColor::new().luminosity(Luminosity::Light));

    if let Ok(category) = args.single::<ImageCategory>() {
        if category.is_sfw() || channel.is_nsfw() || msg.is_private() {
            msg.channel_id.broadcast_typing(&ctx)?;

            if let Ok(mut amount) = args.single::<usize>() {
                if amount > 50 {
                    amount = 50;
                }

                let mut output = String::new();
                for image in category.get_random_images(amount).into_iter() {
                    output += &image?.url;
                    output += "\n";
                }

                msg.channel_id.say(&ctx, output)?;
            } else {
                let image = category.get_random()?;

                msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.footer(|f| f.text(format!("Category: {}", category)))
                            .image(image.url)
                            .colour(embed_color)
                    })
                })?;
            }
        } else {
            msg.react(&ctx, "\u{1f51e}")?;
        }
    } else {
        let (sfw_categories, nsfw_categories): (Vec<ImageCategory>, Vec<ImageCategory>) =
            ImageCategory::into_enum_iter().partition(|&c| c.is_sfw());

        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Available Image Categories")
                    .description(sfw_categories.into_iter().join(utils::SEPERATOR))
                    .colour(embed_color);

                if channel.is_nsfw() || msg.is_private() {
                    e.field(
                        "\u{1f51e} NSFW \u{1f51e}",
                        nsfw_categories.into_iter().join(utils::SEPERATOR),
                        false,
                    )
                } else {
                    e
                }
            })
        })?;
    }

    Ok(())
}
