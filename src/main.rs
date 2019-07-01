mod commands;
mod utils;

use crate::commands::{
    about::ABOUT_COMMAND, coinflip::COINFLIP_COMMAND, dice::DICE_COMMAND,
    eightball::EIGHTBALL_COMMAND, ferrissays::FERRISSAYS_COMMAND, nekoslife::NEKOSLIFE_COMMAND,
    ping::PING_COMMAND, pride::PRIDE_COMMAND, zip::ZIP_COMMAND,
};

use std::collections::HashSet;
use std::sync::Arc;

use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, Delimiter, HelpOptions, StandardFramework,
};

use serenity::model::{
    channel::Message,
    gateway::{Activity, Ready},
    guild::{Guild, PartialGuild},
    id::{GuildId, UserId},
};

use serenity::prelude::*;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in as {}.", ready.user.tag());

        utils::update_activity(&ctx);
    }

    fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        ctx.set_activity(Activity::listening(&format!("{} servers", guilds.len())));
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
    commands: [about, ping, coinflip, dice, nekoslife, zip, ferrissays, pride, eightball],
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = utils::read_config()?;

    let mut client = Client::new(&config.token, Handler)?;

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.prefix(&config.prefix.unwrap_or_else(|| String::from(":-")))
                    .on_mention(Some(serenity::model::id::UserId(494_235_198_582_423_552)))
                    .no_dm_prefix(true)
            })
            .group(&GENERAL_GROUP)
            .help(&HELP_COMMAND)
            .prefix_only(|ctx, msg| {
                (ABOUT_COMMAND.fun)(ctx, msg, Args::new("", &[Delimiter::Single(' ')]))
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
