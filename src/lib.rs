#! /usr/bin/env rustc
use rand::distr::{Distribution, Uniform};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Dice(pub Vec<Token>);

//TODO! move Token to its own file & set up a direct conversion from
// &str to dice
#[derive(Clone, Eq, PartialEq, PartialOrd, Debug)]
pub enum Token {
    Value(i64),
    Operator(char),
    UnaryOp(char),
    OpenParenthesis,
    CloseParenthesis,
    None,
}

impl TryFrom<&str> for Dice {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if !input.contains('d') {
            return Err("Supplied dice notation does not contain a dice symbol: 'd'".to_string());
        }

        let tokens = tokenize(input);

        Ok(Self(tokens.shunting_yard()))
    }
}

impl Dice {
    #[must_use]
    pub fn roll(&self) -> Vec<Token> {
        let mut solve_stack: Vec<Token> = Vec::new();

        for token in &self.0 {
            let solved_value = match token {
                Token::Operator(n) => {
                    let values = solve_stack.split_off(solve_stack.len().saturating_sub(2));
                    Self::do_operation(values, *n).unwrap_or(0)
                }
                Token::UnaryOp(n) => {
                    let values = solve_stack.split_off(solve_stack.len().saturating_sub(1));
                    Self::do_operation(values, *n).unwrap_or(0)
                }
                Token::Value(n) => *n,
                other_token => {
                    println!("Invalid Token found: {other_token:?}");
                    0
                }
            };
            solve_stack.push(Token::Value(solved_value));
        }

        solve_stack
    }

    #[must_use]
    fn do_operation(mut values: Vec<Token>, operation: char) -> Option<i64> {
        let val1_default = i64::from(operation == 'd');
        let val2 = get_value(values.pop().as_ref(), 0);
        let val1 = get_value(values.pop().as_ref(), val1_default);

        match operation {
            'd' => {
                let mut rng = rand::rng();
                let die_size = Uniform::new_inclusive(1, val2);
                match die_size {
                    Ok(size) => {
                        let mut output: i64 = 0;

                        for _ in 1..=val1 {
                            output = overflow_op('+', output, size.sample(&mut rng));
                        }

                        Some(output)
                    }
                    Err(e) => {
                        println!("Not a valid dice range: {e}");
                        None
                    }
                }
            }
            '*' => Some(overflow_op('*', val1, val2)),
            '/' => Some(overflow_op('/', val1, val2)),
            '+' => Some(overflow_op('+', val1, val2)),
            '-' => Some(overflow_op('-', val1, val2)),
            other => {
                println!("unrecognized operation: {other}");
                None
            }
        }
    }
}

fn overflow_op(op: char, val1: i64, val2: i64) -> i64 {
    let overflow = match op {
        '*' => val1.overflowing_mul(val2),
        '/' => val1.overflowing_div(val2), // can div even overflow?
        '+' => val1.overflowing_add(val2),
        '-' => val1.overflowing_sub(val2),
        _ => {
            println!("'{op}' not able to overflow");
            (0, false)
        }
    };
    if overflow.1 {
        println!("WARNING: operation overflowed. Output is likely wrong.");
    }
    overflow.0
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(&current_char) = chars.peek() {
        let new_token = match current_char {
            '0'..='9' => {
                let prev_digit = tokens.pop_if(|x| matches!(*x, Token::Value { .. }));
                let new_digit = i64::from(current_char.to_digit(10).unwrap_or(0));
                match prev_digit {
                    Some(Token::Value(n)) => {
                        Token::Value(overflow_op('+', overflow_op('*', 10, n), new_digit))
                    }
                    _ => Token::Value(new_digit),
                }
            }
            '*' | '/' => Token::Operator(current_char),
            'd' | '+' | '-' => tokens
                .last()
                .map_or(Token::UnaryOp(current_char), |op| match op {
                    Token::Operator(_) | Token::OpenParenthesis => Token::UnaryOp(current_char),
                    _ => Token::Operator(current_char),
                }),
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            _ => {
                println!("skipping character '{current_char}'");
                Token::None
            }
        };
        tokens.push(new_token);
        chars.next();
    }

    tokens
}

trait ShuntingYardExt {
    fn shunting_yard(self) -> Self;
}

impl ShuntingYardExt for Vec<Token> {
    fn shunting_yard(self) -> Self {
        let mut op_stack: Self = Self::new();
        let mut out_stack: Self = Self::new();

        for token in self {
            match token {
                Token::Value(_) => out_stack.push(token),
                Token::Operator(_) | Token::UnaryOp(_) => {
                    while let Some(op) = op_stack.pop_if(|x| x.get_order() > token.get_order()) {
                        out_stack.push(op);
                    }
                    op_stack.push(token);
                }
                Token::OpenParenthesis => op_stack.push(token),
                Token::CloseParenthesis => {
                    while let Some(op) = op_stack.pop() {
                        if op == Token::OpenParenthesis {
                            break;
                        }
                        out_stack.push(op);
                    }
                }
                Token::None => {}
            }
        }

        while let Some(op) = op_stack.pop() {
            out_stack.push(op);
        }

        out_stack
    }
}

impl Token {
    const fn get_order(&self) -> i16 {
        match self {
            Self::Operator(op) => match op {
                '+' | '-' => 1,
                '*' | '/' => 2,
                '^' => 3,
                'd' => 4,
                _ => -1,
            },
            // UnaryOp order is subject to change, but must be higher than d &
            // the other standard math operators
            Self::UnaryOp(_) => 10,
            _ => -1,
        }
    }
}

const fn get_value(input: Option<&Token>, default: i64) -> i64 {
    match input {
        Some(Token::Value(n)) => *n,
        _ => default,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Dice, ShuntingYardExt, Token};

    #[test]
    fn test_tokenization() {
        assert_eq!(
            Dice::try_from("1 d10"),
            Ok(Dice(Vec::from([
                Token::Value(1),
                Token::Value(10),
                Token::Operator('d'),
            ])))
        );
        assert_eq!(
            Dice::try_from("1d20"),
            Ok(Dice(Vec::from([
                Token::Value(1),
                Token::Value(20),
                Token::Operator('d'),
            ])))
        );
        assert_eq!(
            Dice::try_from("1d20+1"),
            Ok(Dice(Vec::from([
                Token::Value(1),
                Token::Value(20),
                Token::Operator('d'),
                Token::Value(1),
                Token::Operator('+'),
            ])))
        );
        assert_eq!(
            Dice::try_from("-1+2d4*(5-1)"),
            Ok(Dice(Vec::from([
                Token::Value(1),
                Token::UnaryOp('-'),
                Token::Value(2),
                Token::Value(4),
                Token::Operator('d'),
                Token::Value(5),
                Token::Value(1),
                Token::Operator('-'),
                Token::Operator('*'),
                Token::Operator('+'),
            ])))
        );
        assert_eq!(
            Dice::try_from("d10-1+(-1)"),
            Ok(Dice(Vec::from([
                Token::Value(10),
                Token::UnaryOp('d'),
                Token::Value(1),
                Token::Value(1),
                Token::UnaryOp('-'),
                Token::Operator('+'),
                Token::Operator('-'),
            ])))
        );
        assert_eq!(
            Dice::try_from("d20"),
            Ok(Dice(Vec::from([Token::Value(20), Token::UnaryOp('d'),])))
        );
    }

    #[test]
    fn test_shunting_yard() {
        assert_eq!(
            Vec::from([Token::Value(1), Token::Operator('d'), Token::Value(10),]).shunting_yard(),
            Vec::from([Token::Value(1), Token::Value(10), Token::Operator('d'),])
        );
        assert_eq!(
            Vec::from([
                Token::Value(2),
                Token::Operator('d'),
                Token::Value(6),
                Token::Operator('+'),
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(4),
                Token::Operator('-'),
                Token::Value(2),
            ])
            .shunting_yard(),
            Vec::from([
                Token::Value(2),
                Token::Value(6),
                Token::Operator('d'),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Value(2),
                Token::Operator('-'),
                Token::Operator('+'),
            ])
        );
        assert_eq!(
            Dice::try_from("5*d6").unwrap(),
            Dice(Vec::from([
                Token::Value(5),
                Token::Value(6),
                Token::UnaryOp('d'),
                Token::Operator('*'),
            ]))
        );
    }
    #[test]
    fn test_shunting_yard_parenthesis() {
        assert_eq!(
            Vec::from([
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(4),
                Token::Operator('*'),
                Token::OpenParenthesis,
                Token::Value(5),
                Token::Operator('-'),
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(4),
                Token::CloseParenthesis,
            ])
            .shunting_yard(),
            Vec::from([
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Value(5),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Operator('-'),
                Token::Operator('*'),
            ])
        );
        assert_eq!(
            Vec::from([
                Token::OpenParenthesis,
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(4),
                Token::CloseParenthesis,
                Token::Operator('*'),
                Token::OpenParenthesis,
                Token::OpenParenthesis,
                Token::Value(5),
                Token::Operator('-'),
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(4),
                Token::CloseParenthesis,
                Token::CloseParenthesis,
            ])
            .shunting_yard(),
            Vec::from([
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Value(5),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Operator('-'),
                Token::Operator('*'),
            ])
        );
        assert_eq!(
            Vec::from([
                Token::Value(3),
                Token::Operator('*'),
                Token::OpenParenthesis,
                Token::UnaryOp('d'),
                Token::Value(6),
                Token::Operator('+'),
                Token::Value(4),
                Token::CloseParenthesis,
            ])
            .shunting_yard(),
            Vec::from([
                Token::Value(3),
                Token::Value(6),
                Token::UnaryOp('d'),
                Token::Value(4),
                Token::Operator('+'),
                Token::Operator('*'),
            ])
        );
    }

    #[test]
    fn test_dice_from() {
        assert_eq!(
            Dice::try_from("-1+2d4*(5-1)").unwrap(),
            Dice(Vec::from([
                Token::Value(1),
                Token::UnaryOp('-'),
                Token::Value(2),
                Token::Value(4),
                Token::Operator('d'),
                Token::Value(5),
                Token::Value(1),
                Token::Operator('-'),
                Token::Operator('*'),
                Token::Operator('+'),
            ]))
        );
        assert_eq!(
            Dice::try_from("d20").unwrap(),
            Dice(Vec::from([Token::Value(20), Token::UnaryOp('d'),]))
        );
    }

    #[test]
    fn test_dice_from_fails_missing_d() {
        assert_eq!(
            Dice::try_from("1+1"),
            Err("Supplied dice notation does not contain a dice symbol: 'd'".to_string())
        );
    }

    #[test]
    fn test_roll() {
        let test_dice = Dice::try_from("1d4+1").unwrap();
        let test_roll = &test_dice.roll();
        assert!(
            *test_roll >= [Token::Value(2)].to_vec() && *test_roll <= [Token::Value(5)].to_vec()
        );

        let test_dice = Dice::try_from("d20").unwrap();
        let test_roll = &test_dice.roll();
        assert!(
            *test_roll >= [Token::Value(1)].to_vec() && *test_roll <= [Token::Value(20)].to_vec()
        );

        let test_dice = Dice::try_from("3*(d6+4)").unwrap();
        let test_roll = &test_dice.roll();
        assert!(
            *test_roll >= [Token::Value(15)].to_vec() && *test_roll <= [Token::Value(30)].to_vec()
        );
    }

    #[test]
    fn test_do_operation() {
        assert!(Dice::do_operation(vec![Token::Value(1), Token::Value(4)], 'd') <= Some(4));
        assert!(Dice::do_operation(vec![Token::Value(1), Token::Value(4)], 'd') >= Some(1));
        assert!(Dice::do_operation(vec![Token::Value(20)], 'd') >= Some(1));
        assert!(Dice::do_operation(vec![Token::Value(20)], 'd') <= Some(20));

        assert_eq!(
            Dice::do_operation(vec![Token::Value(2), Token::Value(2)], '*'),
            Some(4)
        );
        assert_eq!(
            Dice::do_operation(vec![Token::Value(-2), Token::Value(2)], '*'),
            Some(-4)
        );

        assert_eq!(
            Dice::do_operation(vec![Token::Value(15), Token::Value(5)], '/'),
            Some(3)
        );
        assert_eq!(
            Dice::do_operation(vec![Token::Value(7), Token::Value(2)], '/'),
            Some(3)
        );

        assert_eq!(
            Dice::do_operation(vec![Token::Value(1), Token::Value(1)], '+'),
            Some(2)
        );
        assert_eq!(Dice::do_operation(vec![Token::Value(1)], '+'), Some(1));

        assert_eq!(
            Dice::do_operation(vec![Token::Value(1), Token::Value(1)], '-'),
            Some(0)
        );
        assert_eq!(Dice::do_operation(vec![Token::Value(1)], '-'), Some(-1));
    }

    #[test]
    fn test_do_operation_d_unary() {
        let test_op = Dice::do_operation(vec![Token::Value(20)], 'd');
        assert!(test_op > Some(0) && test_op < Some(21));
    }
}
