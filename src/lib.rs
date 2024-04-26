#! /usr/bin/env rustc
pub mod roll_dice {
    use rand::distributions::{Distribution, Uniform};
    use regex::Regex;

    #[derive(PartialEq, Debug)]
    pub struct Dice {
        pub count: i32,
        pub sides: i32,
        pub plus: i32,
    }

    #[derive(Debug)]
    pub struct Rolls {
        pub results: Vec<i32>,
        pub max: i32,
        pub min: i32,
    }

    impl From<&str> for Dice {
        fn from(dice_spec: &str) -> Self {
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

            Dice {
                count: dice[0].0,
                sides: dice[0].1,
                plus: dice[0].2,
            }
        }
    }

    impl Dice {
        pub fn roll(&self) -> Rolls {
            let mut rng = rand::thread_rng();
            let die_size = Uniform::from(1..=self.sides);
            let mut rolls = vec![];

            for _ in 1..=self.count {
                rolls.push(die_size.sample(&mut rng) + self.plus);
            }

            Rolls {
                results: rolls,
                max: self.sides + self.plus,
                min: 1 + self.plus,
            }
        }
    }

    pub fn advantage(rolls: &Rolls) -> i32 {
        *rolls.results.iter().max().unwrap()
    }

    pub fn disadvantage(rolls: &Rolls) -> i32 {
        *rolls.results.iter().min().unwrap()
    }

    pub fn sum_rolls(rolls: &Rolls) -> i32 {
        rolls.results.iter().sum::<i32>()
    }

    pub fn explode_critical(mut rolls: Rolls, dice: Dice) -> Rolls {
        rolls.results.iter_mut().for_each(|x| {
            if *x == rolls.max {
                *x += dice.roll().results[0]
            }
        });

        rolls
    }

    pub fn explode_fumble(mut rolls: Rolls, dice: Dice) -> Rolls {
        rolls.results.iter_mut().for_each(|x| {
            if *x == rolls.min {
                *x -= dice.roll().results[0]
            }
        });

        rolls
    }
}

#[cfg(test)]
mod tests {
    use crate::roll_dice::{
        advantage, disadvantage, explode_critical, explode_fumble, sum_rolls, Dice, Rolls,
    };

    #[test]
    fn test_dice_from_str() {
        assert_eq!(
            Dice {
                count: 1,
                sides: 10,
                plus: 0
            },
            Dice::from("d10")
        );
        assert_eq!(
            Dice {
                count: 1,
                sides: 10,
                plus: 0
            },
            Dice::from("1d10")
        );
        assert_eq!(
            Dice {
                count: 1,
                sides: 10,
                plus: 0
            },
            Dice::from("1d10+0")
        );
        assert_eq!(
            Dice {
                count: 10,
                sides: 6,
                plus: 3
            },
            Dice::from("10d6+3")
        );
        assert_eq!(
            Dice {
                count: 10,
                sides: 6,
                plus: 3
            },
            Dice::from("10d6+3+5")
        );
    }

    #[test]
    fn test_dice_roll() {
        // check upper range
        assert!(!Dice::from("100000d10").roll().results.contains(&0));
        // check lower range
        assert!(!Dice::from("100000d10").roll().results.contains(&11));
        // check roll count
        assert_eq!(Dice::from("100000d10").roll().results.len(), 100000);
    }

    #[test]
    fn test_adv() {
        assert_eq!(
            10,
            advantage(&Rolls {
                results: vec![10, 3, 5],
                max: 10,
                min: 1
            })
        );
        assert_eq!(
            99,
            advantage(&Rolls {
                results: vec![99, 50, 32, 27],
                max: 100,
                min: 1
            })
        );
        assert_eq!(
            5,
            advantage(&Rolls {
                results: vec![5],
                max: 6,
                min: 1
            })
        );
        assert_eq!(
            19,
            advantage(&Rolls {
                results: vec![19, 19],
                max: 20,
                min: 1
            })
        );
        assert_eq!(
            19,
            advantage(&Rolls {
                results: vec![19, 13],
                max: 20,
                min: 1
            })
        );
    }

    #[test]
    fn test_dis() {
        assert_eq!(
            3,
            disadvantage(&Rolls {
                results: vec![10, 3, 5],
                max: 10,
                min: 1
            })
        );
        assert_eq!(
            27,
            disadvantage(&Rolls {
                results: vec![99, 50, 32, 27],
                max: 100,
                min: 1
            })
        );
        assert_eq!(
            5,
            disadvantage(&Rolls {
                results: vec![5],
                max: 6,
                min: 1
            })
        );
        assert_eq!(
            19,
            disadvantage(&Rolls {
                results: vec![19, 19],
                max: 20,
                min: 1
            })
        );
        assert_eq!(
            13,
            disadvantage(&Rolls {
                results: vec![19, 13],
                max: 20,
                min: 1
            })
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            18,
            sum_rolls(&Rolls {
                results: vec![10, 3, 5],
                max: 10,
                min: 1
            })
        );
        assert_eq!(
            208,
            sum_rolls(&Rolls {
                results: vec![99, 50, 32, 27],
                max: 100,
                min: 1
            })
        );
        assert_eq!(
            5,
            sum_rolls(&Rolls {
                results: vec![5],
                max: 6,
                min: 1
            })
        );
        assert_eq!(
            38,
            sum_rolls(&Rolls {
                results: vec![19, 19],
                max: 20,
                min: 1
            })
        );
        assert_eq!(
            32,
            sum_rolls(&Rolls {
                results: vec![19, 13],
                max: 20,
                min: 1
            })
        );
    }

    #[test]
    fn test_explode_critical() {
        let rolls = Rolls {
            results: vec![1, 2, 3, 4, 5, 6],
            max: 6,
            min: 1,
        };
        assert!(
            explode_critical(rolls, Dice::from("1d6"))
                .results
                .last()
                .unwrap()
                > &6
        );
    }

    #[test]
    fn test_explode_fumble() {
        let rolls = Rolls {
            results: vec![1, 2, 3, 4, 5, 6],
            max: 6,
            min: 1,
        };
        assert!(
            explode_fumble(rolls, Dice::from("1d6"))
                .results
                .first()
                .unwrap()
                < &1
        );
    }
}
