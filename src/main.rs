#! /usr/bin/env rustc
use dice::roll_dice::{Dice, Tokens};
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please supply a dice notation, such as 1d20");
        return;
    }

    args.remove(0);

    let joined = args
        .into_iter()
        .filter(|n| !n.contains("--"))
        .collect::<String>();
    let dice = Dice::try_from(Tokens::from(joined.as_str()));

    match dice {
        Ok(n) => {
            //TODO this needs to change because it relies on Debug
            println!("{:?}", n.roll());
        }
        Err(e) => {
            println!("There was an error parsing the dice notation: {e}");
        }
    }
}
