mod utils;

use std::collections::HashSet;
use std::sync::Arc;

use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, Delimiter, HelpOptions, StandardFramework,
};

use serenity::model::{
    channel::Message,
    gateway::Ready,
    guild::{Guild, PartialGuild},
    id::{GuildId, UserId},
};

use serenity::prelude::*;

const SEPERATOR: &str = " \u{2022} ";

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        let (name, discrim) = (ready.user.name, ready.user.discriminator);
        println!("Logged in as {}#{:04}.", name, discrim);

        utils::update_activity(&ctx);
    }

    fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        utils::update_activity(&ctx);
    }

    fn guild_create(&self, ctx: Context, _guild: Guild, is_new: bool) {
        if is_new {
            utils::update_activity(&ctx);
        }
    }

    fn guild_delete(&self, ctx: Context, _guild: PartialGuild, _full: Option<Arc<RwLock<Guild>>>) {
        utils::update_activity(&ctx);
    }
}

group!({
    name: "general",
    options: {},
    commands: [about, ping, coinflip, dice, nekoslife, zip, ferrissays, pride],
});

#[help]
fn help_command(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

#[command]
#[description = "Sends some info about the bot."]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    const TITLE: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"),);

    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

    let guild_count = utils::get_guild_count(&ctx);

    msg.channel_id.send_message(&ctx, |m| m
        .embed(|e| e
            .title(TITLE)
            .description(DESCRIPTION)
            .field("Bot Invite", "[Invite](https://discordapp.com/api/oauth2/authorize?client_id=494235198582423552&permissions=8&scope=bot)", true)
            .footer(|f| f
                .text(format!("Servers: {}", guild_count))
            )
        )
    )?;

    Ok(())
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "Pong!")?;

    Ok(())
}

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

#[command]
#[description = "Rolls up to 5 dice."]
fn dice(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use serenity::utils::MessageBuilder;

    use rand::{thread_rng, Rng};
    use utils::Dice;

    let dices_amount = match args.single::<usize>() {
        Ok(x) if x < 5 && x > 0 => x,
        _ => 5,
    };

    let mut dices = Vec::with_capacity(dices_amount);

    let mut rng = thread_rng();
    for _ in 0..dices_amount {
        dices.push(rng.gen::<Dice>())
    }

    let dices_iter = dices.iter();

    let mut output = MessageBuilder::new();

    for dice in dices_iter.clone() {
        output.push(&dice);
    }

    if dices_amount > 1 {
        output.push("\nThe numbers are: ");
    } else {
        output.push("\nThe number is: ");
    }

    for (i, dice) in dices_iter.clone().enumerate() {
        if i != (dices_amount - 1) {
            output.push_bold(dice.clone() as u8);

            if dices_amount != 2 {
                output.push(", ");
            } else {
                output.push(" ");
            }
        } else {
            if dices_amount > 1 {
                output.push("and ");
            }

            output.push_bold(dice.clone() as u8);
            output.push(".\n");
        }
    }

    if dices_amount > 1 {
        let sum: u8 = dices_iter.map(|d| d.clone() as u8).sum();
        output.push("The total is: ");
        output.push_bold(sum);
        output.push(".");
    }

    msg.channel_id.say(&ctx, output)?;

    Ok(())
}

#[command]
#[description = "Sends a `.zip` archive containing the attachments."]
fn zip(ctx: &mut Context, msg: &Message) -> CommandResult {
    use std::io::{Cursor, Write};
    use zip::{write::FileOptions, ZipWriter};

    msg.channel_id.broadcast_typing(&ctx)?;

    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));

    for attachment in msg.attachments.iter() {
        zip.start_file(&*attachment.filename, FileOptions::default())?;
        zip.write(&attachment.download()?)?;
    }

    msg.channel_id.send_files(
        &ctx,
        vec![(zip.finish()?.into_inner().as_slice(), "output.zip")],
        |m| m,
    )?;

    Ok(())
}

/*
#[command]
fn mcskin(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use percent_encoding::percent_decode;

    if let Some(username) = args.current() {
        let json: serde_json::Value = reqwest::get(&format!("https://api.mojang.com/users/profiles/minecraft/{}", username))
            .expect("couldn't retrieve user profile")
            .json()?;

        let uuid = json["id"].as_str().unwrap();
        let username = json["name"].as_str().unwrap();

        let json: serde_json::Value = reqwest::get(&format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", uuid))
            .expect("couldn't retrieve user profile")
            .json()?;

        let base64_value = json["properties"][0]["value"].as_str().unwrap();
        let json = base64::decode(base64_value).unwrap();
        let json = percent_decode(json.as_slice()).decode_utf8().unwrap();

        msg.channel_id.send_message(&ctx, |m| m
            .embed(|e| e
                .description(format!("{}", json))
                .footer(|f| f.text(username))
            )
        )?;
    } else {
        msg.channel_id.say(&ctx, "No username provided.")?;
    }

    Ok(())
}
*/

#[command]
#[aliases("nl")]
fn nekoslife(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use itertools::Itertools;
    use nekos::{self, ImageCategory};
    use random_color::{Luminosity, RandomColor};

    if let Some(category) = args.current() {
        if let Ok(category) = category.parse::<ImageCategory>() {
            if category.is_sfw() || msg.channel_id.to_channel(&ctx)?.is_nsfw() {
                let client = nekos::Client::new();

                msg.channel_id.broadcast_typing(&ctx)?;

                args.advance();
                if let Ok(amount) = args.single::<u8>() {
                    let mut output = String::new();
                    for _ in 0..amount {
                        output += &client.get_random_image(category.clone())?.url;
                        output += "\n";
                    }

                    msg.channel_id.say(&ctx, output)?;
                } else {
                    let image = client.get_random_image(category.clone())?;

                    let embed_color =
                        utils::gen_random_color(RandomColor::new().luminosity(Luminosity::Light));

                    msg.channel_id.send_message(&ctx, |m| {
                        m.embed(|e| {
                            e.footer(|f| f.text(format!("Category: {}", category)))
                                .image(image.url)
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
            msg.channel_id
                .say(&ctx, "Invalid image category specified.")?;
        }
    } else {
        let client = nekos::Client::new();

        let mut categories = client.get_image_categories()?;
        if !msg.channel_id.to_channel(&ctx)?.is_nsfw() {
            categories.retain(|c| c.is_sfw());
        }

        let description = categories.iter().join(SEPERATOR);

        let embed_color = utils::gen_random_color(RandomColor::new().luminosity(Luminosity::Light));

        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Available Image Categories")
                    .description(description)
                    .colour(embed_color)
            })
        })?;
    }

    Ok(())
}

#[command]
#[description = "Makes Ferris the Crab say something."]
fn ferrissays(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use serenity::utils::MessageBuilder;

    let mut output = MessageBuilder::new();
    let mut ferris = Vec::new();

    ferris_says::say(args.rest().as_bytes(), 24, &mut ferris)?;
    output.push_codeblock(String::from_utf8(ferris)?, None);

    msg.channel_id.say(&ctx, output)?;

    Ok(())
}

/*
#[command]
#[description = "Outputs the sum of the numbers passed in."]
fn sum(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use itertools::Itertools;

    let iter = args.iter::<f64>().filter_map(|n| n.ok());

    let foo = iter.join(" + ");
    let sum: f64 = iter.sum();

    msg.channel_id.say(&ctx, format!("{} = {}", foo, sum))?;

    Ok(())
}
*/

#[command]
#[description = "Makes your avatar all the more gay! 🏳️‍🌈"]
fn pride(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use crate::utils::PrideFlag;

    use serenity::http::AttachmentType;

    use image::{
        imageops, png::PNGEncoder, ColorType, DynamicImage, FilterType, GenericImage,
        GenericImageView, ImageFormat,
    };

    // random is imported for a random color of the pride flag
    use rand::{seq::SliceRandom, thread_rng};

    use lenient_bool::LenientBool;

    msg.channel_id.broadcast_typing(&ctx)?;

    let flag: PrideFlag = args.single().unwrap_or_default();

    let filter = match args.single::<LenientBool>() {
        Ok(lb) => lb.into(),
        _ => false,
    };
    let filter = if filter {
        FilterType::Gaussian
    } else {
        FilterType::Nearest
    };

    let a = match args.single::<u8>() {
        Ok(percent) if percent >= 20 && percent <= 80 => {
            ((percent as f64 / 100.0) * 255.0).trunc() as u8
        }
        _ => 127,
    };

    let mut image_bytes = vec![];
    reqwest::get(&msg.author.static_avatar_url().unwrap())?.copy_to(&mut image_bytes)?;

    let mut image = image::load_from_memory_with_format(&image_bytes, ImageFormat::WEBP)?.to_rgba();

    let mut prideflag_image = DynamicImage::new_rgba8(1, flag.colors().iter().count() as u32);
    for (i, color) in flag.colors().iter().enumerate() {
        let (r, g, b) = (
            ((*color >> 16) & 255) as u8,
            ((*color >> 8) & 255) as u8,
            (*color & 255) as u8,
        );

        prideflag_image.put_pixel(0, i as u32, image::Rgba([r, g, b, a]));
    }

    let prideflag_image = prideflag_image.resize_exact(image.width(), image.height(), filter);
    imageops::overlay(&mut image, &prideflag_image, 0, 0);

    let mut output_bytes = vec![];
    PNGEncoder::new(&mut output_bytes).encode(
        &image.into_raw(),
        prideflag_image.width(),
        prideflag_image.height(),
        ColorType::RGBA(8),
    )?;

    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.footer(|f| f.text(&flag))
                .colour(*flag.colors().choose(&mut thread_rng()).unwrap())
                .image("attachment://prideified.png")
        });
        m.add_file(AttachmentType::Bytes((&output_bytes, "prideified.png")));
        m
    })?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = utils::read_config()?;

    let mut client = Client::new(&config.token, Handler)?;

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.prefix(&config.prefix.unwrap_or(String::from(":-")))
                    .on_mention(Some(serenity::model::id::UserId(494_235_198_582_423_552)))
                    .no_dm_prefix(true)
            })
            .group(&GENERAL_GROUP)
            .help(&HELP_COMMAND)
            .prefix_only(|ctx, msg| {
                about(ctx, msg, Args::new("", &[Delimiter::Single(' ')]))
                    .expect("couldn't run the about command");
            })
            .after(|ctx, msg, command_name, command_result| {
                use serenity::utils::Colour;

                if let Err(command_error) = command_result {
                    msg.channel_id
                        .send_message(&ctx, |m| {
                            m.embed(|e| {
                                e.title("An error occurred")
                                    .description(format!("```{}```", command_error.0))
                                    .footer(|f| f.text(format!("Command: {}", command_name)))
                                    .colour(Colour::RED)
                            })
                        })
                        .expect("couldn't send message");
                }
            }),
    );

    if let Err(e) = client.start() {
        eprintln!("client error: {:?}", e);
    }

    Ok(())
}
