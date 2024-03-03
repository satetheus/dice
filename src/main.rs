#! /usr/bin/env rustc
use std::env;
use rand::distributions::{Distribution, Uniform};
use regex::Regex;


fn parse_dice(dice_spec: &str) -> (i32, i32, i32) {
    //TODO this needs error handling
    let dice_regex = Regex::new(r"(?<count>\d+)*d(?<sides>\d+)\+*(?<plus>\d+)*").unwrap();
    let dice: Vec<(i32, i32, i32)> = dice_regex.captures_iter(dice_spec).map(|c| {
        let count: i32 = match c.name("count") {
            Some(string) => string.as_str().parse::<i32>().expect(""),
            None => 1
        };
        let sides: i32 = c.name("sides").unwrap().as_str().parse::<i32>()
            .expect("A valid dice number is required");
        let plus: i32 = match c.name("plus") {
            Some(string) => string.as_str().parse::<i32>().expect(""),
            None => 0
        };
        (count, sides, plus)
    }).collect();

    dice[0]
}


fn roll_dice(dice: (i32, i32, i32)) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let die_size = Uniform::from(1..=dice.1);
    let mut rolls = vec![];

    for _ in 1..=dice.0 {
        rolls.push(die_size.sample(&mut rng)+dice.2);
    }

    rolls
}


fn advantage(rolls: &Vec<i32>) -> i32 {
    *rolls.iter().max().unwrap()
}


fn disadvantage(rolls: &Vec<i32>) -> i32 {
    *rolls.iter().min().unwrap()
}


fn sum_rolls(rolls: &Vec<i32>) -> i32 {
    rolls.iter().sum::<i32>()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let dice_spec = parse_dice(&args[1]);
    let rolls = roll_dice(dice_spec);

    if args.len() > 2 {
        match args[2].as_str() {
            "adv" => println!("{:?}", advantage(&rolls)),
            "dis" => println!("{:?}", disadvantage(&rolls)),
            "sum" => println!("{:?}", sum_rolls(&rolls)),
            _ => println!("{:?}", &rolls)
        }
    }

    else {
        println!("{:?}", &rolls);
    }
}



#[cfg(test)]
mod tests {
    use crate::parse_dice;

    #[test]
    fn test_parse_dice() {
        assert_eq!((1,10,0), parse_dice("d10"));
        assert_eq!((1,10,0), parse_dice("1d10"));
        assert_eq!((1,10,0), parse_dice("1d10+0"));
        assert_eq!((10,6,3), parse_dice("10d6+3"));
        assert_eq!((10,6,3), parse_dice("10d6+3+5"));
    }
}
