#! /usr/bin/env rustc
use rand::distributions::{Distribution, Uniform};
use regex::Regex;
use std::env;

fn parse_dice(dice_spec: &str) -> (i32, i32, i32) {
    let dice_regex = Regex::new(r"(?<count>\d+)*d(?<sides>\d+)\+*(?<plus>-*\d+)*").unwrap();
    let dice: Vec<(i32, i32, i32)> = dice_regex
        .captures_iter(dice_spec)
        .map(|c| {
            let count: i32 = match c.name("count") {
                Some(string) => string.as_str().parse::<i32>().expect(""),
                None => 1,
            };
            let sides: i32 = c.name("sides").unwrap().as_str().parse::<i32>().expect("");
            let plus: i32 = match c.name("plus") {
                Some(string) => string.as_str().parse::<i32>().expect(""),
                None => 0,
            };
            (count, sides, plus)
        })
        .collect();

    dice[0]
}

fn roll_dice(dice: (i32, i32, i32)) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let die_size = Uniform::from(1..=dice.1);
    let mut rolls = vec![];

    for _ in 1..=dice.0 {
        rolls.push(die_size.sample(&mut rng) + dice.2);
    }

    rolls
}

fn advantage(rolls: &[i32]) -> i32 {
    *rolls.iter().max().unwrap()
}

fn disadvantage(rolls: &[i32]) -> i32 {
    *rolls.iter().min().unwrap()
}

fn sum_rolls(rolls: &[i32]) -> i32 {
    rolls.iter().sum::<i32>()
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        args.push("this will trigger the invalid dice format error".to_owned());
    }

    let re = Regex::new(r"\d*d\d+\+*\d*").unwrap();
    let dice_spec = match re.is_match(&args[1]) {
        true => parse_dice(&args[1]),
        false => {
            println!(
                "1st parameter should be a valid dice format. Ex.: 1d20+1 or d10. \
                     We rolled a d20 for you in case that's what you wanted."
            );
            parse_dice("1d20")
        }
    };

    let rolls = roll_dice(dice_spec);

    if args.len() > 2 {
        match args[2].as_str() {
            "adv" => println!("{:?}", advantage(&rolls)),
            "dis" => println!("{:?}", disadvantage(&rolls)),
            "sum" => println!("{:?}", sum_rolls(&rolls)),
            _ => println!("{:?}", &rolls),
        }
    } else {
        println!("{:?}", &rolls);
    }
}

#[cfg(test)]
mod tests {
    use crate::{advantage, disadvantage, parse_dice, roll_dice, sum_rolls};

    #[test]
    fn test_parse_dice() {
        assert_eq!((1, 10, 0), parse_dice("d10"));
        assert_eq!((1, 10, 0), parse_dice("1d10"));
        assert_eq!((1, 10, 0), parse_dice("1d10+0"));
        assert_eq!((10, 6, 3), parse_dice("10d6+3"));
        assert_eq!((10, 6, 3), parse_dice("10d6+3+5"));
    }

    #[test]
    fn test_roll_dice() {
        // check upper range
        assert!(!roll_dice((100000, 10, 0)).contains(&0));
        // check lower range
        assert!(!roll_dice((100000, 10, 0)).contains(&11));
    }

    #[test]
    fn test_adv() {
        assert_eq!(10, advantage(&[10, 3, 5]));
        assert_eq!(99, advantage(&[99, 50, 32, 27]));
        assert_eq!(5, advantage(&[5]));
        assert_eq!(19, advantage(&[19, 19]));
        assert_eq!(19, advantage(&[19, 13]));
    }

    #[test]
    fn test_dis() {
        assert_eq!(3, disadvantage(&[10, 3, 5]));
        assert_eq!(27, disadvantage(&[99, 50, 32, 27]));
        assert_eq!(5, disadvantage(&[5]));
        assert_eq!(19, disadvantage(&[19, 19]));
        assert_eq!(13, disadvantage(&[19, 13]));
    }

    #[test]
    fn test_sum() {
        assert_eq!(18, sum_rolls(&[10, 3, 5]));
        assert_eq!(208, sum_rolls(&[99, 50, 32, 27]));
        assert_eq!(5, sum_rolls(&[5]));
        assert_eq!(38, sum_rolls(&[19, 19]));
        assert_eq!(32, sum_rolls(&[19, 13]));
    }
}
