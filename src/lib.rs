#! /usr/bin/env rustc
use rand::distr::{Distribution, Uniform};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Dice(pub Vec<Token>);

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug)]
pub enum Token {
    Value(i64),
    Operator(char, usize),
    UnaryMod(char),
    OpenParenthesis,
    CloseParenthesis,
    Keep(Keep),
    None,
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug)]
pub enum Keep {
    High(i64),
    Low(i64),
    All,
}

impl TryFrom<&str> for Dice {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if !input.contains('d') {
            return Err("Supplied dice notation does not contain a dice symbol: 'd'".to_string());
        }

        let tokens = tokenize(input);

        for i in 0..tokens.len() {
            let next = tokens.get(i.saturating_add(1)).unwrap_or(&Token::None);
            match tokens.get(i).unwrap_or(&Token::None) {
                Token::Operator(n, _)
                    if matches!(next, Token::Operator(_, 2))
                        | matches!(next, Token::CloseParenthesis)
                        | matches!(next, Token::None) =>
                {
                    return Err(format!("Operator '{n}' cannot be followed by {next:?}"));
                }
                _ => (),
            }
        }

        Ok(Self(tokens.shunting_yard()))
    }
}

impl Dice {
    #[must_use]
    pub fn roll(&self) -> Vec<Token> {
        let mut solve_stack: Vec<Token> = Vec::new();

        for token in &self.0 {
            match token {
                Token::Operator(n, vals) => {
                    let keep = solve_stack
                        .pop_if(|n| matches!(n, Token::Keep(_)))
                        .unwrap_or(Token::Keep(Keep::All));
                    let values = solve_stack.split_off(solve_stack.len().saturating_sub(*vals));
                    let val = if *n == 'd' {
                        Token::Value(Self::dice_operation(values, &keep).unwrap_or(0))
                    } else {
                        Token::Value(Self::do_operation(values, *n).unwrap_or(0))
                    };
                    solve_stack.push(val);
                }
                Token::Value(n) => {
                    solve_stack.push(Token::Value(*n));
                }
                Token::Keep(n) => {
                    solve_stack.push(Token::Keep(*n));
                }
                other_token => {
                    println!("Invalid Token found: {other_token:?}");
                }
            }
        }

        solve_stack
    }

    #[must_use]
    fn do_operation(mut values: Vec<Token>, operation: char) -> Option<i64> {
        let val2 = get_value(values.pop().as_ref(), 0);
        let val1 = get_value(values.pop().as_ref(), 0);

        match operation {
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

    fn dice_operation(mut values: Vec<Token>, keep: &Token) -> Option<i64> {
        let val2 = get_value(values.pop().as_ref(), 0);
        let val1 = get_value(values.pop().as_ref(), 1);

        let mut rng = rand::rng();
        let die_size = Uniform::new_inclusive(1, val2);
        match die_size {
            Ok(size) => {
                let mut output: i64 = 0;

                match keep {
                    Token::Keep(Keep::High(n)) => {
                        let mut highs = vec![0; usize::try_from(*n).unwrap_or(1)];
                        for _ in 1..=val1 {
                            let new_num = size.sample(&mut rng);
                            let highs_index: Option<usize> =
                                highs.iter().position(|&val| val < new_num);
                            if let Some(val) = highs_index {
                                let _ = std::mem::replace(&mut highs[val], new_num);
                            }
                        }
                        output = highs.into_iter().sum();
                    }
                    Token::Keep(Keep::Low(n)) => {
                        let mut lows = vec![val2; usize::try_from(*n).unwrap_or(1)];
                        for _ in 1..=val1 {
                            let new_num = size.sample(&mut rng);
                            let lows_index: Option<usize> =
                                lows.iter().position(|&val| val > new_num);
                            if let Some(val) = lows_index {
                                let _ = std::mem::replace(&mut lows[val], new_num);
                            }
                        }
                        output = lows.into_iter().sum();
                    }
                    _ => {
                        for _ in 1..=val1 {
                            output = overflow_op('+', output, size.sample(&mut rng));
                        }
                    }
                }

                Some(output)
            }
            Err(e) => {
                println!("Not a valid dice range: {e}");
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
        let new_token =
            match current_char {
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
                '*' | '/' => Token::Operator(current_char, 2),
                'd' | '+' | '-' => tokens
                    .last()
                    .map_or(Token::Operator(current_char, 1), |op| match op {
                        Token::Operator(_, _) | Token::OpenParenthesis => {
                            Token::Operator(current_char, 1)
                        }
                        _ => Token::Operator(current_char, 2),
                    }),
                '(' => Token::OpenParenthesis,
                ')' => Token::CloseParenthesis,
                'h' => match tokens
                    .pop_if(|val| matches!(*val, Token::Value { .. }))
                    .unwrap_or(Token::None)
                {
                    Token::Value(n) => Token::Keep(Keep::High(n)),
                    _ => Token::Keep(Keep::High(1)),
                },
                'l' => match tokens
                    .pop_if(|val| matches!(*val, Token::Value { .. }))
                    .unwrap_or(Token::None)
                {
                    Token::Value(n) => Token::Keep(Keep::Low(n)),
                    _ => Token::Keep(Keep::Low(1)),
                },
                'k' => {
                    // This is skipped because drop notation will not be
                    // implemented & the 'k' char is just a separator between the
                    // second dice operand & the number to keep.
                    Token::None
                }
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
                Token::Value(_) | Token::Keep(_) => out_stack.push(token),
                Token::Operator(_, _) | Token::UnaryMod(_) => {
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
            // UnaryOp order is subject to change, but must be higher than d &
            // the other standard math operators
            Self::Operator(_, 1) => 10,
            Self::Operator(op, _) => match op {
                '+' | '-' => 1,
                '*' | '/' => 2,
                '^' => 3,
                'd' => 4,
                _ => -1,
            },
            Self::Keep(_) => 5,
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
    use crate::{Dice, Keep, ShuntingYardExt, Token};

    #[test]
    fn test_tokenization() {
        assert_eq!(
            Dice::try_from("1 d10"),
            Ok(Dice(Vec::from([
                Token::Value(1),
                Token::Value(10),
                Token::Operator('d', 2),
            ])))
        );
        assert_eq!(
            Dice::try_from("1d20"),
            Ok(Dice(Vec::from([
                Token::Value(1),
                Token::Value(20),
                Token::Operator('d', 2),
            ])))
        );
        assert_eq!(
            Dice::try_from("1d20+1"),
            Ok(Dice(Vec::from([
                Token::Value(1),
                Token::Value(20),
                Token::Operator('d', 2),
                Token::Value(1),
                Token::Operator('+', 2),
            ])))
        );
        assert_eq!(
            Dice::try_from("-1+2d4*(5-1)"),
            Ok(Dice(Vec::from([
                Token::Value(1),
                Token::Operator('-', 1),
                Token::Value(2),
                Token::Value(4),
                Token::Operator('d', 2),
                Token::Value(5),
                Token::Value(1),
                Token::Operator('-', 2),
                Token::Operator('*', 2),
                Token::Operator('+', 2),
            ])))
        );
        assert_eq!(
            Dice::try_from("d10-1+(-1)"),
            Ok(Dice(Vec::from([
                Token::Value(10),
                Token::Operator('d', 1),
                Token::Value(1),
                Token::Value(1),
                Token::Operator('-', 1),
                Token::Operator('+', 2),
                Token::Operator('-', 2),
            ])))
        );
        assert_eq!(
            Dice::try_from("d20"),
            Ok(Dice(Vec::from(
                [Token::Value(20), Token::Operator('d', 1),]
            )))
        );
    }

    #[test]
    fn test_tokenization_negative_cases() {
        assert_eq!(
            Dice::try_from("1d"),
            Err("Operator 'd' cannot be followed by None".to_string()),
        );
        assert_eq!(
            Dice::try_from("1d10-"),
            Err("Operator '-' cannot be followed by None".to_string()),
        );
    }

    #[test]
    fn test_shunting_yard() {
        assert_eq!(
            Vec::from([Token::Value(1), Token::Operator('d', 2), Token::Value(10),])
                .shunting_yard(),
            Vec::from([Token::Value(1), Token::Value(10), Token::Operator('d', 2),])
        );
        assert_eq!(
            Vec::from([
                Token::Value(2),
                Token::Operator('d', 2),
                Token::Value(6),
                Token::Operator('+', 2),
                Token::Value(1),
                Token::Operator('d', 2),
                Token::Value(4),
                Token::Operator('-', 2),
                Token::Value(2),
            ])
            .shunting_yard(),
            Vec::from([
                Token::Value(2),
                Token::Value(6),
                Token::Operator('d', 2),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d', 2),
                Token::Value(2),
                Token::Operator('-', 2),
                Token::Operator('+', 2),
            ])
        );
        assert_eq!(
            Dice::try_from("5*d6").unwrap(),
            Dice(Vec::from([
                Token::Value(5),
                Token::Value(6),
                Token::Operator('d', 1),
                Token::Operator('*', 2),
            ]))
        );
    }
    #[test]
    fn test_shunting_yard_parenthesis() {
        assert_eq!(
            Vec::from([
                Token::Value(1),
                Token::Operator('d', 2),
                Token::Value(4),
                Token::Operator('*', 2),
                Token::OpenParenthesis,
                Token::Value(5),
                Token::Operator('-', 2),
                Token::Value(1),
                Token::Operator('d', 2),
                Token::Value(4),
                Token::CloseParenthesis,
            ])
            .shunting_yard(),
            Vec::from([
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d', 2),
                Token::Value(5),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d', 2),
                Token::Operator('-', 2),
                Token::Operator('*', 2),
            ])
        );
        assert_eq!(
            Vec::from([
                Token::OpenParenthesis,
                Token::Value(1),
                Token::Operator('d', 2),
                Token::Value(4),
                Token::CloseParenthesis,
                Token::Operator('*', 2),
                Token::OpenParenthesis,
                Token::OpenParenthesis,
                Token::Value(5),
                Token::Operator('-', 2),
                Token::Value(1),
                Token::Operator('d', 2),
                Token::Value(4),
                Token::CloseParenthesis,
                Token::CloseParenthesis,
            ])
            .shunting_yard(),
            Vec::from([
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d', 2),
                Token::Value(5),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d', 2),
                Token::Operator('-', 2),
                Token::Operator('*', 2),
            ])
        );
        assert_eq!(
            Vec::from([
                Token::Value(3),
                Token::Operator('*', 2),
                Token::OpenParenthesis,
                Token::Operator('d', 1),
                Token::Value(6),
                Token::Operator('+', 2),
                Token::Value(4),
                Token::CloseParenthesis,
            ])
            .shunting_yard(),
            Vec::from([
                Token::Value(3),
                Token::Value(6),
                Token::Operator('d', 1),
                Token::Value(4),
                Token::Operator('+', 2),
                Token::Operator('*', 2),
            ])
        );
    }

    #[test]
    fn test_dice_from() {
        assert_eq!(
            Dice::try_from("-1+2d4*(5-1)").unwrap(),
            Dice(Vec::from([
                Token::Value(1),
                Token::Operator('-', 1),
                Token::Value(2),
                Token::Value(4),
                Token::Operator('d', 2),
                Token::Value(5),
                Token::Value(1),
                Token::Operator('-', 2),
                Token::Operator('*', 2),
                Token::Operator('+', 2),
            ]))
        );
        assert_eq!(
            Dice::try_from("d20").unwrap(),
            Dice(Vec::from([Token::Value(20), Token::Operator('d', 1),]))
        );
    }

    #[test]
    fn test_dice_from_keep_notation() {
        assert_eq!(
            Dice::try_from("2d20kh").unwrap(),
            Dice(Vec::from([
                Token::Value(2),
                Token::Value(20),
                Token::Keep(Keep::High(1)),
                Token::Operator('d', 2),
            ]))
        );

        assert_eq!(
            Dice::try_from("2d20kl").unwrap(),
            Dice(Vec::from([
                Token::Value(2),
                Token::Value(20),
                Token::Keep(Keep::Low(1)),
                Token::Operator('d', 2),
            ]))
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
    fn test_roll_keep_notation() {
        let test_dice = Dice::try_from("2d20kh").unwrap();
        let test_roll = &test_dice.roll();
        assert!(
            *test_roll >= [Token::Value(1)].to_vec() && *test_roll <= [Token::Value(20)].to_vec()
        );

        let test_dice = Dice::try_from("2d20kl").unwrap();
        let test_roll = &test_dice.roll();
        assert!(
            *test_roll >= [Token::Value(1)].to_vec() && *test_roll <= [Token::Value(20)].to_vec()
        );
    }

    #[test]
    fn test_dice_operation() {
        assert!(
            Dice::dice_operation(
                vec![Token::Value(1), Token::Value(4)],
                &Token::Keep(Keep::All)
            ) <= Some(4)
        );
        assert!(
            Dice::dice_operation(
                vec![Token::Value(1), Token::Value(4)],
                &Token::Keep(Keep::All)
            ) >= Some(1)
        );
        assert!(Dice::dice_operation(vec![Token::Value(20)], &Token::Keep(Keep::All)) >= Some(1));
        assert!(Dice::dice_operation(vec![Token::Value(20)], &Token::Keep(Keep::All)) <= Some(20));
    }

    #[test]
    fn test_do_operation() {
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
    fn test_dice_operation_unary() {
        let test_op = Dice::dice_operation(vec![Token::Value(20)], &Token::Keep(Keep::All));
        assert!(test_op > Some(0) && test_op < Some(21));
    }
}
