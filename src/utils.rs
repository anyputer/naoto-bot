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

use enum_iterator::IntoEnumIterator;

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

#[derive(Debug, Copy, Clone, IntoEnumIterator)]
/// An LGBTQIA+ pride flag.
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

    // these aren't in any real order (yet)
    Androphilia,
    Gynephilia,
    Bear,
    GenderCreative,
    Bigender,
    Trigender,
    Polygender,
    Pangender,
    Genderflux,
    Demiguy,
    Demigirl,
    Autosexual,
    Ceterosexual, // not sure if this should stick around
    Androgyne,
    Lithromantic,
    Abrosexual,
    Transmasculine,
    Transfeminine,
    Aroflux,
}

#[derive(Debug)]
/// The type of flag.
pub enum FlagType {
    /// The stripes go from top to bottom.
    Vertical,

    /// The stripes go from left to right.
    Horizontal,
}

impl PrideFlag {
    #[allow(clippy::unreadable_literal)]
    /// Returns the colors of the pride flag.
    pub fn colors(self) -> &'static [u32] {
        use PrideFlag::*;

        match self {
            // wikipedia
            Rainbow => &[0xE40303, 0xFF8C00, 0xFFED00, 0x008026, 0x004DFF, 0x750787],
            // pride.tar.xz
            Lesbian => &[
                0xD62C00, /*0xF07528,*/ 0xFF9B57, 0xEDEDEB, 0xD162A6,
                /*0xB75592,*/ 0xA50162,
            ],
            // wikipedia
            Bi => &[0xD60270, 0xD60270, 0x9B4F96, 0x0038A8, 0x0038A8],
            // wikipedia
            Pan => &[0xFF218C, 0xFFD800, 0x21B1FF],
            // wikipedia
            Poly => &[0xF61CB9, 0x07D569, 0x1C92F6],
            // wikipedia
            Trans => &[0x5BCEFA, 0xF5A9B8, 0xFFFFFF, 0xF5A9B8, 0x5BCEFA],
            // wikipedia
            Genderqueer => &[0xB57EDC, 0xFFFFFF, 0x4A8123],
            // wikipedia
            NonBinary => &[0xFFF430, 0xFFFFFF, 0x9C59D1, 0x000000],
            // wikipedia
            Genderfluid => &[0xFF75A2, 0xFFFFFF, 0xBE18D6, 0x000000, 0x333EBD],
            // wikipedia
            Agender => &[
                0x000000, 0xB9B9B9, 0xFFFFFF, 0xB8F483, 0xFFFFFF, 0xB9B9B9, 0x000000,
            ],
            // gender wikia
            Neutrois => &[0xFFFFFF, 0x22B14C, 0x000000],
            // wikipedia
            Asexual => &[0x000000, 0xA3A3A3, 0xFFFFFF, 0x800080],
            // wikipedia
            Aromantic => &[0x3DA542, 0xA7D379, 0xFFFFFF, 0xA9A9A9, 0x000000],

            // https://www.deviantart.com/savvyred/journal/Pride-Flags-Colors-explained-379547414
            Androphilia => &[0x000000, 0xC0C0C0, 0xFFFFFF, 0x008000],
            // https://www.deviantart.com/savvyred/journal/Pride-Flags-Colors-explained-379547414
            Gynephilia => &[0x000000, 0xC0C0C0, 0xFFFFFF, 0xFF0079], // unsure about the last color
            // wikipedia
            Bear => &[
                0x623804, 0xD56300, 0xFEDD63, 0xFEE6B8, 0xFFFFFF, 0x555555, 0x000000,
            ],
            // https://www.deviantart.com/pride-flags/art/Gender-Creative-628496779
            // +2 on every rgb channel
            GenderCreative => &[0x8C18B6, 0x8C18B6, 0xFFFFFF, 0x8C18B6, 0x8C18B6],
            // gender wikia
            Bigender => &[
                0xC479A0, 0xECA6CB, 0xD6C7E7, 0xFFFFFF, 0xD6C7E7, 0x9BC7E8, 0x6B83CF,
            ],
            // gender wikia
            Trigender => &[0xFF95C5, 0x9581FF, 0x67D966, 0x9581FF, 0xFF95C5],
            // gender wikia
            Polygender => &[0x000000, 0x939393, 0xED94C5, 0xF5ED81, 0x64BBE6],
            // gender wikia
            Pangender => &[
                0xFFEA7E, 0xFFC3B0, 0xFFD6F3, 0xFFFFFF, 0xFFD6F3, 0xFFC3B0, 0xFFEA7E,
            ],
            // gender wikia
            Genderflux => &[0xF47694, 0xF2A2B9, 0xCECECE, 0x7CE0F7, 0x3ECDF9, 0xFFF48D],
            // gender wikia
            Demiguy => &[
                0x7F7F7F, 0xC3C3C3, 0x99D9EA, 0xFFFFFF, 0x99D9EA, 0xC3C3C3, 0x7F7F7F,
            ],
            // gender wikia
            Demigirl => &[
                0x7F7F7F, 0xC3C3C3, 0xFFAEC9, 0xFFFFFF, 0xFFAEC9, 0xC3C3C3, 0x7F7F7F,
            ],
            // uses colors from the demiguy flag
            Autosexual => &[0x99D9EA, 0x7F7F7F],
            // rainbowpedia wikia
            Ceterosexual => &[0xFFFC01, 0x218D03, 0xF556E9, 0xFFFFFF, 0x000000],
            // gender wikia
            Androgyne => &[0xFE007F, 0x9832FF, 0x00B8E7],
            // aromantic wikia
            Lithromantic => &[0xF14952, 0xFE9E5F, 0xFFF547, 0xFFFFFF, 0x2A2A2A],
            // sexuality fandom
            Abrosexual => &[0x75CA91, 0xB3E4C7, 0xFFFFFF, 0xE695B5, 0xD9446C],
            // https://www.deviantart.com/pride-flags/art/Trans-Man-Transmasculine-1-543925972
            Transmasculine => &[
                0xFF8ABE, 0xCCF5FF, 0x99ECFF, 0x75DFFF, 0x99ECFF, 0xCCF5FF, 0xFF8ABE,
            ],
            // https://www.deviantart.com/pride-flags/art/Trans-Woman-Transfeminine-1-543925985
            Transfeminine => &[
                0x73DEFF, 0xFFE0ED, 0xFFB5D5, 0xFF8CBE, 0xFFB5D5, 0xFFE0ED, 0x73DEFF,
            ],
            // aromantic wikia
            Aroflux => &[0xE7516A, 0xD86D65, 0xB7A55D, 0xA3C95A, 0x92E454],
        }
    }

    /// Returns the type of flag.
    pub fn flag_type(self) -> FlagType {
        use FlagType::*;
        use PrideFlag::*;

        match self {
            Androgyne => Horizontal,
            _ => Vertical,
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
            "agender" | "genderblank" | "genderfree" | "gendervoid" => Ok(Agender),
            "neutrois" => Ok(Neutrois),
            "ace" | "asexual" => Ok(Asexual),
            "aromantic" => Ok(Aromantic),

            "androphilia" => Ok(Androphilia),
            "gynephilia" | "gynophilia" | "gynaecophilia" | "gynesexuality" => Ok(Gynephilia),
            "bear" => Ok(Bear),
            "gender-creative"
            | "gender-nonconforming"
            | "gendercreative"
            | "gendernonconforming" => Ok(GenderCreative),
            "bigender" => Ok(Bigender),
            "trigender" => Ok(Trigender),
            "polygender" => Ok(Polygender),
            "pangender" => Ok(Pangender),
            "genderflux" => Ok(Genderflux),
            "demiguy" | "demiboy" | "demiman" | "demimale" => Ok(Demiguy),
            "demigirl" | "demiwoman" | "demifemale" => Ok(Demigirl),
            "autosexual" => Ok(Autosexual),
            "ceterosexual" | "skoliosexual" => Ok(Ceterosexual),
            "androgyne" => Ok(Androgyne),
            "lithromantic" | "akoiromantic" | "apromantic" | "akoisexual" | "akoinesexual"
            | "lithsexual" => Ok(Lithromantic),
            "abrosexual" => Ok(Abrosexual),
            "transmasculine" => Ok(Transmasculine),
            "transfeminine" => Ok(Transfeminine),
            "aroflux" => Ok(Aroflux),

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
            Rainbow => "rainbow",
            Lesbian => "lesbian",
            Bi => "bisexual",
            Pan => "pansexual",
            Poly => "polysexual",
            Trans => "transgender",
            Genderqueer => "genderqueer",
            NonBinary => "non-binary",
            Genderfluid => "genderfluid",
            Agender => "agender",
            Neutrois => "neutrois",
            Asexual => "asexual",
            Aromantic => "aromantic",

            Androphilia => "androphilia",
            Gynephilia => "gynephilia",
            Bear => "bear",
            GenderCreative => "gender creative",
            Bigender => "bigender",
            Trigender => "trigender",
            Polygender => "polygender",
            Pangender => "pangender",
            Genderflux => "genderflux",
            Demiguy => "demiguy",
            Demigirl => "demigirl",
            Autosexual => "autosexual",
            Ceterosexual => "ceterosexual",
            Androgyne => "androgyne",
            Lithromantic => "lithromantic",
            Abrosexual => "abrosexual",
            Transmasculine => "transmasculine",
            Transfeminine => "transfeminine",
            Aroflux => "aroflux",
        })
    }
}

impl Default for PrideFlag {
    #[inline]
    fn default() -> PrideFlag {
        PrideFlag::Rainbow
    }
}

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
    (
        ((color >> 16) & 255) as u8,
        ((color >> 8) & 255) as u8,
        (color & 255) as u8,
    )
}
