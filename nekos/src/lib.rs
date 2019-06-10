use std::error;
use std::fmt;
use std::str::FromStr;

use reqwest;
// use serde::Deserialize;
use serde_json;

use regex::Regex;

const BASE_URL_API_V2: &str = "https://nekos.life/api/v2";

/*
 * NOTE:
 * There is `bj` and `blowjob` which both mean
 * the same thing, but result in different URLs.
 * These are just represented as `Bj` and `Blowjob`.
 */

/// Enum that represents the various image categories
#[derive(Debug, PartialEq, Clone)]
pub enum ImageCategory {
    Femdom,
    Tickle,
    Classic,
    NekoGif,
    EroFeet,
    Meow,
    EroK,
    Poke,
    Lesbian,
    /// URL always leads to 403 Forbidden.
    V3,
    HoloLewd,
    NekoApiV3_1,
    LewdK,
    Keta,
    FeetGif,
    NsfwNekoGif,
    EroYuri,
    Kiss,
    EightBall,
    Kuni,
    Tits,
    PussyJpg,
    CumJpg,
    Pussy,
    LewdKemo,
    Lizard,
    Slap,
    LewdNeko,
    Cum,
    Cuddle,
    Spank,
    // Always returns https://cdn.nekos.life/smallboobs/404.png.
    SmallBoobs,
    Goose,
    RandomHentaiGif,
    Avatar,
    FoxGirl,
    NsfwAvatar,
    Hug,
    Gecg,
    Boobs,
    Pat,
    Feet,
    Smug,
    Kemonomimi,
    SoloGif,
    Holo,
    Wallpaper,
    Bj,
    Woof,
    Yuri,
    Trap,
    Anal,
    Baka,
    Blowjob,
    HoloEro,
    Feed,
    Neko,
    Gasm,
    Hentai,
    Futanari,
    Ero,
    Solo,
    Waifu,
    PwankGif,
    EroNeko,
    EroKemo,

    Other(String),
}

/// Enum that represents the level of NSFW of the image category.
///
/// This is subjective.
#[derive(Debug, PartialEq)]
pub enum NsfwRating {
    /// All images in the image category are Safe For Work.
    Sfw,
    /// Image category contains (some) nudity, naked feet etc.
    Questionable,
    /// Image category contains (some) nudity, naked feet, female nipples etc.
    QuestionableNipples,
    /// Image category contains Not Safe For Work content, including nudity, erotica etc.
    Nsfw,
    /// The NSFW rating for `ImageCategory::Other` and unknown image categories.
    Unknown,
}

impl ImageCategory {
    /// Returns the NSFW rating for the image category.
    pub fn nsfw_rating(&self) -> NsfwRating {
        use ImageCategory::*;
        use NsfwRating::*;

        match self {
            Femdom => Nsfw,
            Tickle => Sfw,
            Classic => Nsfw,
            NekoGif => Questionable,
            EroFeet => Questionable,
            Meow => Sfw,
            EroK => QuestionableNipples,
            Poke => Sfw,
            Lesbian => Nsfw,
            V3 => Unknown,
            HoloLewd => Nsfw,
            NekoApiV3_1 => Unknown,
            LewdK => Nsfw,
            Keta => Nsfw,
            FeetGif => Nsfw,
            NsfwNekoGif => Nsfw,
            EroYuri => Questionable,
            Kiss => Sfw,
            EightBall => Sfw,
            Kuni => Nsfw,
            Tits => Nsfw,
            PussyJpg => Nsfw,
            CumJpg => Nsfw,
            Pussy => Nsfw,
            LewdKemo => Nsfw,
            Lizard => Sfw,
            Slap => Sfw,
            LewdNeko => Nsfw,
            Cum => Nsfw,
            Cuddle => Sfw,
            Spank => Nsfw,
            SmallBoobs => Unknown,
            Goose => Sfw,
            RandomHentaiGif => Nsfw,
            Avatar => Questionable,
            FoxGirl => Sfw,
            NsfwAvatar => QuestionableNipples,
            Hug => Sfw,
            Gecg => Sfw,
            Boobs => Nsfw,
            Pat => Sfw,
            Feet => Nsfw,
            Smug => Sfw,
            Kemonomimi => Questionable,
            SoloGif => Nsfw,
            Holo => Questionable,
            Wallpaper => Questionable,
            Bj => Nsfw,
            Woof => Sfw,
            Yuri => Nsfw,
            Trap => Nsfw,
            Anal => Nsfw,
            Baka => Sfw,
            Blowjob => Nsfw,
            HoloEro => QuestionableNipples,
            Feed => Sfw,
            Neko => Questionable,
            Gasm => Questionable, // Ahegao is questionable
            Hentai => Nsfw,
            Futanari => Nsfw,
            Ero => QuestionableNipples,
            Solo => Nsfw,
            Waifu => Sfw,
            PwankGif => Nsfw,
            EroNeko => QuestionableNipples,
            EroKemo => QuestionableNipples,

            Other(_) => Unknown,
        }
    }

    /// Returns whether or not the image category is Safe For Work.
    pub fn is_sfw(&self) -> bool {
        self.nsfw_rating() == NsfwRating::Sfw
    }
}

impl fmt::Display for ImageCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImageCategory::*;

        let s = match self {
            Femdom => "femdom",
            Tickle => "tickle",
            Classic => "classic",
            NekoGif => "ngif",
            EroFeet => "erofeet",
            Meow => "meow",
            EroK => "erok",
            Poke => "poke",
            Lesbian => "les",
            V3 => "v3",
            HoloLewd => "hololewd",
            NekoApiV3_1 => "nekoapi_v3.1",
            LewdK => "lewdk",
            Keta => "keta",
            FeetGif => "feetg",
            NsfwNekoGif => "nsfw_neko_gif",
            EroYuri => "eroyuri",
            Kiss => "kiss",
            EightBall => "8ball",
            Kuni => "kuni",
            Tits => "tits",
            PussyJpg => "pussy_jpg",
            CumJpg => "cum_jpg",
            Pussy => "pussy",
            LewdKemo => "lewdkemo",
            Lizard => "lizard",
            Slap => "slap",
            LewdNeko => "lewd",
            Cum => "cum",
            Cuddle => "cuddle",
            Spank => "spank",
            SmallBoobs => "smallboobs",
            Goose => "goose",
            RandomHentaiGif => "Random_hentai_gif",
            Avatar => "avatar",
            FoxGirl => "fox_girl",
            NsfwAvatar => "nsfw_avatar",
            Hug => "hug",
            Gecg => "gecg",
            Boobs => "boobs",
            Pat => "pat",
            Feet => "feet",
            Smug => "smug",
            Kemonomimi => "kemonomimi",
            SoloGif => "solog",
            Holo => "holo",
            Wallpaper => "wallpaper",
            Bj => "bj",
            Woof => "woof",
            Yuri => "yuri",
            Trap => "trap",
            Anal => "anal",
            Baka => "baka",
            Blowjob => "blowjob",
            HoloEro => "holoero",
            Feed => "feed",
            Neko => "neko",
            Gasm => "gasm",
            Hentai => "hentai",
            Futanari => "futanari",
            Ero => "ero",
            Solo => "solo",
            Waifu => "waifu",
            PwankGif => "pwankg",
            EroNeko => "eron",
            EroKemo => "erokemo",

            ImageCategory::Other(s) => s,
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseImageCategoryError;

impl fmt::Display for ParseImageCategoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "provided string wasn't a valid image category")
    }
}

impl error::Error for ParseImageCategoryError {
    fn description(&self) -> &str {
        "provided string wasn't a valid image category"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// `ImageCategory` by design, does not parse into `Other`
/// if the image category doesn't exist. Instead it
/// returns a `Result` being `Ok` or `Err` depending
/// on whether or not the image category actually exists.
///
/// Simply do something like the following if you want the behaviour above:
/// ```
/// use nekos::ImageCategory;
///
/// let foo = "nimbat";
/// let bar: ImageCategory = foo.parse().unwrap_or(ImageCategory::Other(String::from(foo)));
/// assert_eq!(ImageCategory::Other(String::from("nimbat")), bar);
/// ```
impl FromStr for ImageCategory {
    type Err = ParseImageCategoryError;

    fn from_str(s: &str) -> Result<Self, ParseImageCategoryError> {
        use ImageCategory::*;

        match s {
            "femdom" => Ok(Femdom),
            "tickle" => Ok(Tickle),
            "classic" => Ok(Classic),
            "ngif" => Ok(NekoGif),
            "erofeet" => Ok(EroFeet),
            "meow" => Ok(Meow),
            "erok" => Ok(EroK),
            "poke" => Ok(Poke),
            "les" => Ok(Lesbian),
            "v3" => Ok(V3),
            "hololewd" => Ok(HoloLewd),
            "nekoapi_v3.1" => Ok(NekoApiV3_1),
            "lewdk" => Ok(LewdK),
            "keta" => Ok(Keta),
            "feetg" => Ok(FeetGif),
            "nsfw_neko_gif" => Ok(NsfwNekoGif),
            "eroyuri" => Ok(EroYuri),
            "kiss" => Ok(Kiss),
            "8ball" => Ok(EightBall),
            "kuni" => Ok(Kuni),
            "tits" => Ok(Tits),
            "pussy_jpg" => Ok(PussyJpg),
            "cum_jpg" => Ok(CumJpg),
            "pussy" => Ok(Pussy),
            "lewdkemo" => Ok(LewdKemo),
            "lizard" => Ok(Lizard),
            "slap" => Ok(Slap),
            "lewd" => Ok(LewdNeko),
            "cum" => Ok(Cum),
            "cuddle" => Ok(Cuddle),
            "spank" => Ok(Spank),
            "smallboobs" => Ok(SmallBoobs),
            "goose" => Ok(Goose),
            "Random_hentai_gif" => Ok(RandomHentaiGif),
            "avatar" => Ok(Avatar),
            "fox_girl" => Ok(FoxGirl),
            "nsfw_avatar" => Ok(NsfwAvatar),
            "hug" => Ok(Hug),
            "gecg" => Ok(Gecg),
            "boobs" => Ok(Boobs),
            "pat" => Ok(Pat),
            "feet" => Ok(Feet),
            "smug" => Ok(Smug),
            "kemonomimi" => Ok(Kemonomimi),
            "solog" => Ok(SoloGif),
            "holo" => Ok(Holo),
            "wallpaper" => Ok(Wallpaper),
            "bj" => Ok(Bj),
            "woof" => Ok(Woof),
            "yuri" => Ok(Yuri),
            "trap" => Ok(Trap),
            "anal" => Ok(Anal),
            "baka" => Ok(Baka),
            "blowjob" => Ok(Blowjob),
            "holoero" => Ok(HoloEro),
            "feed" => Ok(Feed),
            "neko" => Ok(Neko),
            "gasm" => Ok(Gasm),
            "hentai" => Ok(Hentai),
            "futanari" => Ok(Futanari),
            "ero" => Ok(Ero),
            "solo" => Ok(Solo),
            "waifu" => Ok(Waifu),
            "pwankg" => Ok(PwankGif),
            "eron" => Ok(EroNeko),
            "erokemo" => Ok(EroKemo),
            _ => Err(ParseImageCategoryError),
        }
    }
}

pub struct Image {
    pub url: String,
    pub category: ImageCategory,
}

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
        }
    }

    /// Gets a list of usable image categories.
    pub fn get_image_categories(&self) -> Result<Vec<ImageCategory>, reqwest::Error> {
        let mut resp = self
            .client
            .get("https://nekos.life/api/v2/endpoints")
            .send()?;
        let json: serde_json::Value = resp.json()?;

        let re = Regex::new(r"'(.+?)'").unwrap();
        let s = json[11].as_str().unwrap().to_owned();

        let result = re
            .find_iter(&s)
            .map(|m| m.as_str())
            .map(|s| s.trim_matches('\'').to_string())
            .map(|s| {
                s.parse::<ImageCategory>()
                    .unwrap_or(ImageCategory::Other(s))
            })
            .collect::<Vec<ImageCategory>>();

        /*
        let pattern: &[_] = &['<', '>', '\''];
        let result = s
            .trim_start_matches("HEAD,GET,OPTIONS     /api/v2/img/")
            .trim_matches(pattern)
            .split("', '")
            .map(|s| s.parse::<ImageCategory>().unwrap_or(ImageCategory::Other(s)))
            .collect::<Vec<ImageCategory>>();
        */

        Ok(result)
    }

    /// Gets a random image from the category.
    pub fn get_random_image(&self, category: ImageCategory) -> Result<Image, reqwest::Error> {
        let mut resp = self
            .client
            .get(&format!("{}/img/{}", BASE_URL_API_V2, category))
            .send()?;
        let json: serde_json::Value = resp.json()?;

        let url = json["url"].as_str().unwrap().to_owned();

        Ok(Image { url, category })
    }
}

#[cfg(test)]
mod tests {
    use super::{ImageCategory, ImageCategory::*, NsfwRating::*, ParseImageCategoryError};

    #[test]
    fn image_category_from_str() {
        let foo = "neko".parse::<ImageCategory>();
        assert_eq!(foo, Ok(ImageCategory::Neko));

        let foo = "nimbat".parse::<ImageCategory>();
        assert_eq!(foo, Err(ParseImageCategoryError));
    }

    #[test]
    fn image_category_to_string() {
        assert_eq!(ImageCategory::Neko.to_string(), "neko");
        assert_eq!(
            ImageCategory::Other(String::from("nimbat")).to_string(),
            "nimbat"
        );
    }

    #[test]
    fn image_category_nsfw_rating() {
        assert_eq!(Hug.nsfw_rating(), Sfw);
        assert_eq!(Hentai.nsfw_rating(), Nsfw);
        assert_eq!(Other(String::from("nimbat")).nsfw_rating(), Unknown);
    }

    /*
    #[test]
    fn get_image_categories() {
        use crate::{Client, ImageCategory};

        let client = Client::new();

        assert!(client.get_image_categories().unwrap().contains(ImageCategory::Neko));
    }
    */
}
