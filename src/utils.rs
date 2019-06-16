use std::error;
use std::str::FromStr;
use std::{fmt, fs};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use serenity::model::gateway::Activity;
use serenity::prelude::*;
use serenity::utils::Colour;

use random_color::RandomColor;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Dice {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl Distribution<Dice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dice {
        match rng.gen_range(0, 6) {
            0 => Dice::One,
            1 => Dice::Two,
            2 => Dice::Three,
            3 => Dice::Four,
            4 => Dice::Five,
            _ => Dice::Six,
        }
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match self {
            Dice::One => "<:dice_1:501061465265143838>",
            Dice::Two => "<:dice_2:501061466552795137>",
            Dice::Three => "<:dice_3:501061466435092483>",
            Dice::Four => "<:dice_4:501061466900660233>",
            Dice::Five => "<:dice_5:501061467072888855>",
            Dice::Six => "<:dice_6:501061466926088202>",
        })
    }
}

#[derive(Debug)]
pub enum PrideFlag {
    Rainbow,
    Lesbian,
    Bi,
    Pan,
    Poly,
    Trans,
    Genderqueer,
    NonBinary,
    Genderfluid,
    Agender,
    Neutrois,
    Asexual,
    Aromantic,
}

impl PrideFlag {
    #![allow(clippy::unreadable_literal)]
    pub fn colors(&self) -> &'static [u32] {
        use crate::utils::PrideFlag::*;

        match self {
            Rainbow => &[0xE40303, 0xFF8C00, 0xFFED00, 0x008026, 0x004DFF, 0x750787],
            Lesbian => &[
                // 0xA40061, 0xB75592, 0xD063A6, 0xEDEDEB, 0xE4ACCF, 0xC54E54, 0x8A1E04,
                0xD62C00, /*0xF07528,*/ 0xFF9B57, 0xEDEDEB, 0xD162A6,
                /*0xB75592,*/ 0xA50162,
            ],
            Bi => &[0xD60270, 0xD60270, 0x9B4F96, 0x0038A8, 0x0038A8],
            Pan => &[0xFF218C, 0xFFD800, 0x21B1FF],
            Poly => &[0xF61CB9, 0x07D569, 0x1C92F6],
            Trans => &[0x5BCEFA, 0xF5A9B8, 0xFFFFFF, 0xF5A9B8, 0x5BCEFA],
            Genderqueer => &[0xB57EDC, 0xFFFFFF, 0x4A8123],
            NonBinary => &[0xFFF42C, 0xFFFFFF, 0x9D59D2, 0x000000],
            Genderfluid => &[0xFF75A2, 0xFFFFFF, 0xBE18D6, 0x000000, 0x333EBD],
            Agender => &[
                0x000000, 0xB9B9B9, 0xFFFFFF, 0xB8F483, 0xFFFFFF, 0xB9B9B9, 0x000000,
            ],
            Neutrois => &[0xFFFFFF, 0x22B14C, 0x000000],
            Asexual => &[0x000000, 0xA3A3A3, 0xFFFFFF, 0x800080],
            Aromantic => &[0x3DA542, 0xA7D379, 0xFFFFFF, 0xA9A9A9, 0x000000],
        }
    }
}

impl FromStr for PrideFlag {
    type Err = ParsePrideFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::utils::PrideFlag::*;

        match &s.to_lowercase()[..] {
            "gay" | "queer" | "rainbow" | "lgbt" | "lgbtq+" => Ok(Rainbow),
            "les" | "lesbian" | "lesbean" => Ok(Lesbian),
            "bi" | "bisexual" => Ok(Bi),
            "pan" | "pansexual" => Ok(Pan),
            "poly" | "polysexual" => Ok(Poly),
            "trans" | "transgender" | "icecream" => Ok(Trans),
            "genderqueer" => Ok(Genderqueer),
            "enby" | "nb" | "nonbinary" | "non-binary" => Ok(NonBinary),
            "genderfluid" => Ok(Genderfluid),
            "agender" => Ok(Agender),
            "neutrois" => Ok(Neutrois),
            "ace" | "asexual" => Ok(Asexual),
            "aromantic" => Ok(Aromantic),

            _ => Err(Self::Err {}),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsePrideFlagError;

impl fmt::Display for ParsePrideFlagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "provided string wasn't a valid pride flag")
    }
}

impl error::Error for ParsePrideFlagError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for PrideFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::utils::PrideFlag::*;

        f.pad(match self {
            Rainbow => "rainbow pride flag",
            Lesbian => "lesbian pride flag",
            Bi => "bisexual pride flag",
            Pan => "pansexual pride flag",
            Poly => "polysexual pride flag",
            Trans => "transgender pride flag",
            Genderqueer => "genderqueer pride flag",
            NonBinary => "non-binary pride flag",
            Genderfluid => "genderfluid pride flag",
            Agender => "agender pride flag",
            Neutrois => "neutrois pride flag",
            Asexual => "asexual pride flag",
            Aromantic => "aromantic pride flag",
        })
    }
}

impl Default for PrideFlag {
    #[inline]
    fn default() -> PrideFlag {
        PrideFlag::Rainbow
    }
}

/// Represents the bot's config.
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
