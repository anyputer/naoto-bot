use std::fs;

use serenity::model::gateway::Activity;
use serenity::prelude::*;
use serenity::utils::Colour;

use random_color::RandomColor;

use serde::Deserialize;

pub const SEPERATOR: &str = " \u{2022} ";

/// The bot's config.
#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: Option<String>,
}

/// Reads the bot's configuration file.
pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("config.toml")?;

    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

/// Gets the bot's guild count.
#[inline]
pub fn get_guild_count(ctx: &Context) -> usize {
    ctx.cache.read().guilds.len()
}

/// Updates the bot's activity.
pub fn update_activity(ctx: &Context) {
    let guild_count = get_guild_count(ctx);

    ctx.set_activity(Activity::listening(&format!("{} servers", guild_count)));
}

/// Generates a random `serenity::utils::Colour` from a `random_color::RandomColor`.
pub fn gen_random_color(color: &mut RandomColor) -> Colour {
    let [r, g, b] = color.to_rgb_array();

    (r as u8, g as u8, b as u8).into()
}

/// Extracts the RGB components from a hex color.
#[inline]
pub fn hex_to_rgb(color: u32) -> (u8, u8, u8) {
    let [_, r, g, b] = color.to_be_bytes();

    (r, g, b)
}
