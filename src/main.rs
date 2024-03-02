#! /usr/bin/env rustc
use std::env;
use rand::distributions::{Distribution, Uniform};
use regex::Regex;
// accept user input of \d\+d\d\+{+\d\+}*, ex: "2d10" or "1d20+1"
// run random in range of the dice size, add the plus specified, the number of times specified
// return a vector of the random numbers
// create option for sum, advantage, & disadvantage


fn parse_dice(dice_spec: &str) -> (i32, i32, i32) {
    // input ex: 3d10+5, the +5 is optional
    //TODO this needs error handling
    let dice_regex = Regex::new(r#"(?<count>\d+)d(?<sides>\d+)\+*(?<plus>\d+)*"#).unwrap();
    let dice: Vec<(i32, i32, i32)> = dice_regex.captures_iter(dice_spec).map(|c| {
        //TODO this needs to replace None with 1
        let count: i32 = c.name("count").unwrap().as_str().parse::<i32>().expect("reason1");
        let sides: i32 = c.name("sides").unwrap().as_str().parse::<i32>().expect("reason2");
        //TODO this needs to replace None with 0
        let plus: i32 = c.name("plus").unwrap().as_str().parse::<i32>().expect("reason3");
        (count, sides, plus)
    }).collect();

    println!("{:?}", dice);
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


fn advantage(rolls: Vec<i32>) -> i32 {
    1
}


fn disadvantage(rolls: Vec<i32>) -> i32 {
    1
}


fn sum_rolls(rolls: Vec<i32>) -> i32 {
    1
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let dice_spec = parse_dice(&args[1]);
    let rolls = roll_dice(dice_spec);

    println!("{:?}", rolls);
}
