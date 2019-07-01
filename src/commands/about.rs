use crate::utils;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

const TITLE: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"),);
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[command]
#[aliases("invite")]
#[description = "Sends some info about the bot."]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
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
