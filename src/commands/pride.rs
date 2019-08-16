use crate::utils;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use serenity::http::AttachmentType;

use image::{
    imageops, png::PNGEncoder, ColorType, DynamicImage, FilterType, GenericImage, GenericImageView,
    ImageFormat,
};

// random is imported for a random color of the pride flag
use lenient_bool::LenientBool;
use rand::{seq::SliceRandom, thread_rng};
use random_color::{Luminosity, RandomColor};

use enum_iterator::IntoEnumIterator;
// use lazy_static::lazy_static;
// use serenity::model::id::GuildId;

use std::str::FromStr;
use std::{error, fmt};

// const PFE_GUILD_ID: GuildId = GuildId(592_009_826_687_647_786);

/*
lazy_static! {
    static ref EMOJIS: Result<Vec<Emoji>, String> = {
        if let Some(guild) = PFE_GUILD_ID.to_guild_cached(&ctx) {
            let mut emojis = guild.read().emojis.values();

            let mut vec: Vec<Emoji> = Vec::new();
            for pf in PrideFlag::into_enum_iter() {
                let emoji_name = pf.to_string().replace(" ", "_").replace("-", "");

                if let Some(emoji) = emojis.find(|e| e.name == emoji_name) {
                    vec.push(emoji);
                } else {
                    return Err(format!("pride flag emoji :{}: not found", emoji_name));
                }
            }

            Ok(vec)
        } else {
            Err("bot isn't on the pride flag emoji server".into())
        }
    };
}
*/

/*
#[command]
fn add_pfs(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Some(guild) = PFE_GUILD_ID.to_guild_cached(&ctx) {
        let pf_emoji_border = image::load_from_memory_with_format(
            include_bytes!("../../assets/flag-emoji-border.png"),
            ImageFormat::PNG,
        )?
        .to_rgba();

        msg.channel_id.say(&ctx, "adding?")?;
        for pf in PrideFlag::into_enum_iter() {
            let emoji_name = pf.to_string().replace(" ", "_").replace("-", "");

            let mut emoji_image = image::RgbaImage::new(112, 112);

            let pf_image = pf.to_image(255).resize_exact(112, 80, FilterType::Nearest);
            imageops::overlay(&mut emoji_image, &pf_image, 0, 16);
            imageops::overlay(&mut emoji_image, &pf_emoji_border, 0, 0);

            for pixel in emoji_image.pixels_mut() {
                if *pixel == image::Rgba([255, 0, 255, 255]) {
                    *pixel = image::Rgba([0, 0, 0, 0]);
                }
            }

            let mut output_bytes = vec![];
            PNGEncoder::new(&mut output_bytes).encode(
                &emoji_image.into_raw(),
                112,
                112,
                ColorType::RGBA(8),
            )?;

            let emoji = guild.read().create_emoji(&ctx, &emoji_name, &format!("data:image/png;base64,{}", base64::encode(&output_bytes)))?;

            msg.channel_id.say(&ctx, emoji)?;
        }

        msg.channel_id.say(&ctx, "done?")?;
    } else {
        msg.channel_id.say(&ctx, "sorry, but the bot isn't on the pride flag emoji server. cannot add the emojis.")?;
    }
    Ok(())
}
*/

#[command]
#[description = "Express your beautiful colors! Adds a flag of choice on top of your avatar. 🏳️‍🌈"]
#[usage("<pride flag> <gradient on?> <opacity from 20 to 80>")]
#[example("rainbow true 60")]
fn pride(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.current() != Some("list") {
        msg.channel_id.broadcast_typing(&ctx)?;

        let pf: PrideFlag = args.single().unwrap_or_default();

        let filter = if args.single::<LenientBool>().map(|lb| lb.0).unwrap_or(false) {
            FilterType::Gaussian
        } else {
            FilterType::Nearest
        };

        let a = match args.single::<u8>() {
            Ok(percent) if percent >= 20 && percent <= 80 => {
                ((f64::from(percent) / 100.0) * 255.0).trunc() as u8
            }
            _ => 127,
        };

        let mut image_bytes = vec![];
        reqwest::get(
            &msg.author
                .static_avatar_url()
                .ok_or("couldn't get static avatar url")?,
        )?
        .copy_to(&mut image_bytes)?;

        let mut image =
            image::load_from_memory_with_format(&image_bytes, ImageFormat::WEBP)?.to_rgba();

        let pf_image = pf
            .to_image(a)
            .resize_exact(image.width(), image.height(), filter);
        imageops::overlay(&mut image, &pf_image, 0, 0);

        let mut output_bytes = vec![];
        PNGEncoder::new(&mut output_bytes).encode(
            &image.into_raw(),
            pf_image.width(),
            pf_image.height(),
            ColorType::RGBA(8),
        )?;

        /*
        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.footer(|f| f.text(pf))
                    .colour(*pf.colors().choose(&mut thread_rng()).unwrap())
                    .image("attachment://output_pride.png")
            });
            m.add_file(AttachmentType::Bytes((&output_bytes, "output_pride.png")));
            m
        })?;
        */

        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(format!("{} + {} pride flag", msg.author.name, pf))
                        .icon_url(msg.author.face())
                })
                .colour(*pf.colors().choose(&mut thread_rng()).unwrap())
                .image("attachment://output_pride.png")
            });
            m.add_file(AttachmentType::Bytes((&output_bytes, "output_pride.png")));
            m
        })?;
    } else {
        /*
        if let Some(guild) = PFE_GUILD_ID.to_guild_cached(&ctx) {
            let guild = guild.read();

            for pf in PrideFlag::into_enum_iter() {
                let emoji_name = pf.to_string().replace(" ", "_").replace("-", "");

                if let Some(emoji) = guild.emojis.values().find(|e| e.name == emoji_name) {
                    println!("{:?} => \"{}\",", pf, emoji);
                }
            }
        }
        */

        use itertools::Itertools;

        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Available Pride Flags")
                    .description(
                        PrideFlag::into_enum_iter()
                            .map(|pf| format!("{} {}", prideflag_emoji(pf), pf))
                            .join("\n"),
                    )
                    .footer(|f| {
                        f.text(format!(
                            "Currently, there are *{}* unique pride flags to represent everyone. \
                             If you want to see a flag get added, please let @hyarsan#3653 know.",
                            PrideFlag::into_enum_iter().count()
                        ))
                    })
                    .colour(utils::gen_random_color(
                        RandomColor::new().luminosity(Luminosity::Bright),
                    ))
            })
        })?;
    }

    Ok(())
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

#[derive(Debug, Copy, Clone)]
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

    /// Returns an image of the pride flag that can be resized to fit your needs.
    pub fn to_image(self, a: u8) -> DynamicImage {
        let flag_type = self.flag_type();

        let color_count = self.colors().iter().count() as u32;
        let pf_size = match flag_type {
            FlagType::Vertical => (1, color_count),
            FlagType::Horizontal => (color_count, 1),
        };
        let mut pf_image = DynamicImage::new_rgba8(pf_size.0, pf_size.1);

        for (i, color) in self.colors().iter().enumerate() {
            let (r, g, b) = utils::hex_to_rgb(*color);

            let pixel_xy = match flag_type {
                FlagType::Vertical => (0, i as u32),
                FlagType::Horizontal => (i as u32, 0),
            };

            pf_image.put_pixel(pixel_xy.0, pixel_xy.1, image::Rgba([r, g, b, a]));
        }

        pf_image
    }
}

impl FromStr for PrideFlag {
    type Err = ParsePrideFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PrideFlag::*;

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provided string wasn't a valid pride flag")
    }
}

impl error::Error for ParsePrideFlagError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for PrideFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PrideFlag::*;

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

fn prideflag_emoji(pf: PrideFlag) -> &'static str {
    use PrideFlag::*;

    match pf {
        Rainbow => "<:rainbow:594974170140639245>",
        Lesbian => "<:lesbian:594974173034709002>",
        Bi => "<:bisexual:594974175882641441>",
        Pan => "<:pansexual:594974178361475093>",
        Poly => "<:polysexual:594974180550770712>",
        Trans => "<:transgender:594974195591675927>",
        Genderqueer => "<:genderqueer:594974198615638042>",
        NonBinary => "<:nonbinary:594974200486166535>",
        Genderfluid => "<:genderfluid:594974202403225603>",
        Agender => "<:agender:594974204336799774>",
        Neutrois => "<:neutrois:594974224330784778>",
        Asexual => "<:asexual:594974227371655218>",
        Aromantic => "<:aromantic:594974229305360405>",

        Androphilia => "<:androphilia:594974231872274453>",
        Gynephilia => "<:gynephilia:594974234829258768>",
        Bear => "<:bear:594974249748398112>",
        GenderCreative => "<:gender_creative:594974251564400671>",
        Bigender => "<:bigender:594974253573472269>",
        Trigender => "<:trigender:594974255167569921>",
        Polygender => "<:polygender:594974257566580779>",
        Pangender => "<:pangender:594974272691109909>",
        Genderflux => "<:genderflux:594974274679341266>",
        Demiguy => "<:demiguy:594974276772298860>",
        Demigirl => "<:demigirl:594974280995962880>",
        Autosexual => "<:autosexual:594974283399168002>",
        Ceterosexual => "<:ceterosexual:594974297756401833>",
        Androgyne => "<:androgyne:594974300142829580>",
        Lithromantic => "<:lithromantic:594974303787941900>",
        Abrosexual => "<:abrosexual:594974306652389386>",
        Transmasculine => "<:transmasculine:594974308267458573>",
        Transfeminine => "<:transfeminine:594974323337461787>",
        Aroflux => "<:aroflux:594974325379956736>",
    }
}
