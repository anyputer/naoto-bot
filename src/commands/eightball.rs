use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;

use serenity::utils::Colour;

use std::fmt;

#[command("8ball")]
#[aliases("eightball")]
fn eightball(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let answer: Answer = rand::random();

    let question = voca_rs::case::upper_first(args.rest())
        .replace(" i ", " I ")
        .replace(" i'm ", " I'm ");
    let mut question_vec = question.chars().collect::<Vec<char>>();

    if let Some(c) = question_vec.last_mut() {
        match *c {
            '.' | '!' => *c = '?',
            ch @ _ if ch != '?' && ch != '>' => {
                question_vec.push('?');
            }
            _ => (),
        }
    }

    let question = question_vec.into_iter().collect::<String>();

    msg.channel_id.send_message(&ctx, |m| m
        .embed(|e| e
            .author(|a| a
                .name(&msg.author.name)
                .icon_url(msg.author.face())
            )
            .description(question)
            .field("Answer", answer, false)
            .colour(*answer.color())
            .thumbnail("https://cdn.discordapp.com/attachments/447786789563006986/594264152147623954/8ball.png")
        )
    )?;

    Ok(())
}

#[derive(Debug, Copy, Clone)]
pub enum Answer<'a> {
    Affirmative(&'a str),
    NonCommittal(&'a str),
    Negative(&'a str),
}

impl<'a> Answer<'a> {
    const POSSIBLE_ANSWERS: [Answer<'a>; 10 + 5 + 5] = [
        Answer::Affirmative("It is certain."),
        Answer::Affirmative("It is decidedly so."),
        Answer::Affirmative("Without a doubt."),
        Answer::Affirmative("Yes - definitely."),
        Answer::Affirmative("You may rely on it."),
        Answer::Affirmative("As I see it, yes."),
        Answer::Affirmative("Most likely."),
        Answer::Affirmative("Outlook good."),
        Answer::Affirmative("Yes."),
        Answer::Affirmative("Signs point to yes."),
        Answer::NonCommittal("Reply hazy, try again."),
        Answer::NonCommittal("Ask again later."),
        Answer::NonCommittal("Better not tell you now."),
        Answer::NonCommittal("Cannot predict now."),
        Answer::NonCommittal("Concentrate and ask again."),
        Answer::Negative("Don't count on it."),
        Answer::Negative("My reply is no."),
        Answer::Negative("My sources say no."),
        Answer::Negative("Outlook not so good."),
        Answer::Negative("Very doubtful."),
    ];

    pub fn as_inner(self) -> &'a str {
        use Answer::*;

        match self {
            Affirmative(s) | NonCommittal(s) | Negative(s) => s,
        }
    }

    pub fn color(self) -> &'static Colour {
        use Answer::*;

        match self {
            Affirmative(_) => &Colour::DARK_GREEN,
            NonCommittal(_) => &Colour::GOLD,
            Negative(_) => &Colour::RED,
        }
    }
}

impl Distribution<Answer<'static>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Answer<'static> {
        *Answer::POSSIBLE_ANSWERS.choose(rng).unwrap()
    }
}

impl<'a> fmt::Display for Answer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_inner())
    }
}
