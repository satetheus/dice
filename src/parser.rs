#! /usr/bin/env rustc
#[allow(dead_code, unused_variables)]
pub mod parse_dice {
    use nom::character::complete::{char, digit1};
    use nom::combinator::{map_res, opt};
    use nom::sequence::separated_pair;
    use nom::IResult;
    use nom::Parser;

    #[derive(PartialEq, Debug)]
    pub struct Dice {
        pub count: i32,
        pub sides: Sides,
    }

    #[derive(PartialEq, Debug)]
    pub enum Sides {
        Number(i32),
        Symbols(Vec<String>),
    }

    enum Rolls {
        Numbers(Vec<i32>),
        Symbols(Vec<String>),
    }

    pub fn parse_digit(input: &str) -> IResult<&str, i32> {
        let (remaining, matched) = opt(map_res(digit1, str::parse::<i32>)).parse(input)?;

        Ok((remaining, matched.unwrap_or(1)))
    }

    pub fn parse_dice(input: &str) -> IResult<&str, Dice> {
        let mut dice_parser = separated_pair(parse_digit, char('d'), parse_digit);
        // todo! eventually needs to handle Sides::Symbols()
        let Ok((remaining, (count, sides))) = dice_parser.parse(input) else {
            todo!() // needs proper error handling
        };

        Ok((
            remaining,
            Dice {
                count,
                sides: Sides::Number(sides),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::parse_dice::{parse_dice, parse_digit, Dice, Sides};

    #[test]
    fn test_parse_digit() {
        assert_eq!(parse_digit("1d10"), Ok(("d10", 1,)));
        assert_eq!(parse_digit("100d10"), Ok(("d10", 100,)));
        assert_eq!(parse_digit("asdf100"), Ok(("asdf100", 1,)));
        assert_eq!(parse_digit("1000"), Ok(("", 1000,)));
    }

    #[test]
    fn test_parse_dice() {
        assert_eq!(
            parse_dice("1d10+27"),
            Ok((
                "+27",
                Dice {
                    count: 1,
                    sides: Sides::Number(10)
                }
            ))
        );
        assert_eq!(
            parse_dice("57d300-1"),
            Ok((
                "-1",
                Dice {
                    count: 57,
                    sides: Sides::Number(300)
                }
            ))
        );
        assert_eq!(
            parse_dice("d10*2"),
            Ok((
                "*2",
                Dice {
                    count: 1,
                    sides: Sides::Number(10)
                }
            ))
        );
    }
}
