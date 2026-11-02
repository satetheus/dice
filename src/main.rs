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
        .collect::<Vec<_>>()
        .join("");
    let dice = Dice::from(Tokens::from(joined.as_str()));
    println!("{:?}", dice.roll());
}
