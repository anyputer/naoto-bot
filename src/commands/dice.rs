use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use serenity::utils::MessageBuilder;
use std::fmt;

#[command]
#[description = "Rolls up to 5 dice."]
fn dice(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let dices_amount = match args.single::<usize>() {
        Ok(x) if x < 5 && x > 0 => x,
        _ => 5,
    };

    let mut rng = thread_rng();

    let dice_vec = (0..dices_amount)
        .map(|_| rng.gen::<Dice>())
        .collect::<Vec<Dice>>();
    let dice_iter = dice_vec.iter().copied();

    let mut output = MessageBuilder::new();

    output.push_line(dice_iter.clone().join(" "));

    if dices_amount > 1 {
        output.push("The numbers are: ");
    } else {
        output.push("The number is: ");
    }

    for (i, dice) in dice_iter.clone().enumerate() {
        if i != (dices_amount - 1) {
            output.push_bold(dice as u8);

            if dices_amount != 2 {
                output.push(", ");
            } else {
                output.push(" ");
            }
        } else {
            if dices_amount > 1 {
                output.push("and ");
            }

            output.push_bold(dice as u8);
            output.push_line(".");
        }
    }

    if dices_amount > 1 {
        let sum: u8 = dice_iter.map(|d| d as u8).sum();

        output.push("The total is: ");
        output.push_bold(sum);
        output.push(".");
    }

    msg.channel_id.say(&ctx, output)?;

    Ok(())
}

#[derive(Debug, Copy, Clone)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
