#! /usr/bin/env rustc
pub mod parse_dice {
    use nom::character::complete::{char, digit0, digit1, one_of};
    use nom::multi::many0;
    use nom::sequence::{pair, separated_pair};
    use nom::IResult;
    use nom::Parser;

    pub fn dice_count(input: &str) -> IResult<&str, &str> {
        let (remaining, matched) = digit0(input)?;
        let matched = if matched.is_empty() { "1" } else { matched };

        Ok((remaining, matched))
    }

    pub fn basic_dice(
        input: &str,
    ) -> IResult<&str, (Vec<(&str, char)>, (&str, &str), Vec<(char, &str)>)> {
        // r"(?<count>\d+)*d(?<sides>\d+)\+*(?<plus>-*\d+)*"
        let mut dice = (
            many0(pair(number_value, operator)),
            separated_pair(dice_count, char('d'), number_value),
            many0(pair(operator, number_value)),
        );

        dice.parse(input)
    }

    pub fn number_value(input: &str) -> IResult<&str, &str> {
        // having a wrapper around digit1 is for later expansion of dice values.
        digit1(input)
    }

    pub fn operator(input: &str) -> IResult<&str, char> {
        one_of("+-*x/")(input)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_dice::{basic_dice, dice_count, number_value, operator};

    #[test]
    fn test_basic_dice() {
        assert_eq!(basic_dice("1d20"), Ok(("", (vec![], ("1", "20"), vec![]))));
        assert_eq!(basic_dice("d20"), Ok(("", (vec![], ("1", "20"), vec![]))));
        assert_eq!(
            basic_dice("1d20+5"),
            Ok(("", (vec![], ("1", "20"), vec![('+', "5")])))
        );
        assert_eq!(
            basic_dice("2d20-4"),
            Ok(("", (vec![], ("2", "20"), vec![('-', "4")])))
        );
        assert_eq!(
            basic_dice("1d20x2"),
            Ok(("", (vec![], ("1", "20"), vec![('x', "2")])))
        );
        assert_eq!(
            basic_dice("1d10/2"),
            Ok(("", (vec![], ("1", "10"), vec![('/', "2")])))
        );
        assert_eq!(
            basic_dice("10*1d20x2"),
            Ok(("", (vec![("10", '*')], ("1", "20"), vec![('x', "2")])))
        );
        assert_eq!(
            basic_dice("5+10*1d20x2-1-3/2"),
            Ok((
                "",
                (
                    vec![("5", '+'), ("10", '*')],
                    ("1", "20"),
                    vec![('x', "2"), ('-', "1"), ('-', "3"), ('/', "2")]
                )
            ))
        );
    }

    #[test]
    fn test_dice_count() {
        assert_eq!(dice_count("1d20"), Ok(("d20", "1")));
        assert_eq!(dice_count("d20"), Ok(("d20", "1")));
        assert_eq!(dice_count("30000d20+1"), Ok(("d20+1", "30000")));
        assert_eq!(dice_count("123451d"), Ok(("d", "123451")));
    }

    #[test]
    fn test_number_value() {
        assert_eq!(number_value("20"), Ok(("", "20")));
        assert!(number_value("").is_err());
        assert!(number_value("asdf").is_err());
        assert!(number_value("d20").is_err());
    }

    #[test]
    fn test_operator() {
        assert!(operator("1+").is_err());
        assert_eq!(operator("+2"), Ok(("2", '+')));
        assert_eq!(operator("x3"), Ok(("3", 'x')));
        assert_eq!(operator("*3"), Ok(("3", '*')));
        assert_eq!(operator("-d20"), Ok(("d20", '-')));
        assert_eq!(operator("/5"), Ok(("5", '/')));
    }
}
