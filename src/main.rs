#! /usr/bin/env rustc
use dice::roll_dice::{advantage, disadvantage, explode_critical, explode_fumble, sum_rolls, Dice};
use regex::Regex;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        args.push("this will trigger the invalid dice format error".to_owned());
    }

    let re = Regex::new(r"\d*d\d+\+*\d*").unwrap();
    let dice_spec = match re.is_match(&args[1]) {
        true => Dice::from(args[1].as_str()),
        false => {
            println!(
                "1st parameter should be a valid dice format. Ex.: 1d20+1 or d10. \
                     We rolled a d20 for you in case that's what you wanted."
            );
            Dice::from("1d20")
        }
    };

    let mut rolls = dice_spec.roll();

    if args.len() > 2 {
        if args.contains(&"crit".to_owned()) {
            let temp_dice = Dice {
                count: 1,
                ..dice_spec
            };
            rolls = explode_critical(rolls, temp_dice);
        }

        if args.contains(&"fumb".to_owned()) {
            let temp_dice = Dice {
                count: 1,
                ..dice_spec
            };
            rolls = explode_fumble(rolls, temp_dice);
        }

        if args.contains(&"adv".to_owned()) {
            println!("Advantage: {:?}", advantage(&rolls));
        }

        if args.contains(&"dis".to_owned()) {
            println!("Disadvantage: {:?}", disadvantage(&rolls));
        }

        if args.contains(&"sum".to_owned()) {
            println!("Sum: {:?}", sum_rolls(&rolls));
        }
    }

    if !args.contains(&"hide".to_owned()) {
        println!("{:?}", &rolls.results);
    }
}
