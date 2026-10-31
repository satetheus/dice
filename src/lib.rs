#! /usr/bin/env rustc
use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Debug)]
pub struct Dice(VecDeque<Token>);

impl From<Tokens> for Dice {
    fn from(input: Tokens) -> Self {
        if !validate(&input) {
            println!("Because of the above error, the supplied dice notation");
            println!("could not be parsed. We took the liberty of supplying a");
            println!("1d20, in case that is what you wanted.");
            return Dice(Tokens::from("1d20").0);
        }

        Dice(input.0)
    }
}

fn validate(input: &Tokens) -> bool {
    // todo! more than this check will eventually be needed, but it's is a start
    if !input.0.contains(&Token::Operator('d')) {
        println!("Supplied dice notation does not contain a dice symbol: 'd'");
        return false;
    }

    true
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Token {
    Value(i64),
    Operator(char),
    UnaryOp(char),
    OpenParenthesis,
    CloseParenthesis,
    None,
}

impl Token {
    fn get_order(&self) -> i16 {
        match self {
            Token::Value(_) => -1,
            Token::Operator(op) => match op {
                '+' | '-' => 1,
                '*' | '/' => 2,
                '^' => 3,
                'd' => 4,
                _ => todo!(),
            },
            Token::OpenParenthesis | Token::CloseParenthesis => -1,
            // UnaryOp order is subject to change, but must be higher than d &
            // the other standard math operators
            Token::UnaryOp(_) => 10,
            Token::None => 0,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Tokens(VecDeque<Token>);

impl From<&str> for Tokens {
    fn from(input: &str) -> Self {
        let tokens = tokenize(input);

        Tokens(shunting_yard(tokens))
    }
}

fn tokenize(input: &str) -> VecDeque<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens: VecDeque<Token> = VecDeque::new();

    while let Some(&current_char) = chars.peek() {
        let new_token = match current_char {
            '0'..='9' => {
                let prev_digit = tokens.pop_back_if(|x| matches!(*x, Token::Value { .. }));
                let new_digit = current_char.to_digit(10).unwrap_or(0) as i64;
                match prev_digit {
                    Some(Token::Value(n)) => Token::Value(10 * n + new_digit),
                    _ => Token::Value(new_digit),
                }
            }
            'd' | '*' | '/' => Token::Operator(current_char),
            '+' | '-' => {
                if let Some(op) = tokens.back() {
                    match op {
                        Token::Operator(_) | Token::OpenParenthesis => Token::UnaryOp(current_char),
                        _ => Token::Operator(current_char),
                    }
                } else {
                    Token::UnaryOp(current_char)
                }
            }
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            _ => {
                println!("skipping character '{}'", current_char);
                Token::None
            }
        };
        tokens.push_back(new_token);
        chars.next();
    }

    tokens
}

fn shunting_yard(mut input: VecDeque<Token>) -> VecDeque<Token> {
    let mut op_stack: Vec<Token> = vec![];
    let mut out_stack: VecDeque<Token> = VecDeque::new();

    while let Some(token) = input.pop_front() {
        match token {
            Token::Value(_) => out_stack.push_back(token),
            Token::Operator(_) | Token::UnaryOp(_) => {
                while !op_stack.is_empty() {
                    let popped_op = op_stack.pop_if(|x| x.get_order() > token.get_order());
                    match popped_op {
                        Some(op) => {
                            out_stack.push_back(op);
                        }
                        None => break,
                    }
                }
                op_stack.push(token);
            }
            Token::OpenParenthesis => op_stack.push(token),
            Token::CloseParenthesis => {
                while !op_stack.is_empty() {
                    let popped_op = op_stack.pop();
                    match popped_op {
                        Some(op) => {
                            if op == Token::OpenParenthesis {
                                break;
                            }
                            out_stack.push_back(op);
                        }
                        None => break,
                    }
                }
            }
            Token::None => {}
        }
    }

    if !op_stack.is_empty() {
        for _ in 0..op_stack.len() {
            out_stack.push_back(op_stack.pop().expect("this should always work"));
        }
    }

    out_stack
}

fn roll(dice: &Dice) -> Vec<Token> {
    let mut op_stack = dice.0.clone();
    let mut solve_stack: Vec<Token> = Vec::new();

    //TODO this no long needs to pop, & op_stack is unnecessary
    while let Some(token) = op_stack.pop_front() {
        let solved_value = match token {
            Token::Operator(n) => {
                let values = solve_stack.split_off(op_stack.len().saturating_sub(2));
                do_operation(values, n)
            }
            Token::UnaryOp(n) => {
                let values = solve_stack.split_off(op_stack.len().saturating_sub(1));
                do_operation(values, n)
            }
            Token::Value(n) => n,
            _ => panic!("Invalid Token found"),
        };
        solve_stack.push(Token::Value(solved_value));
    }

    solve_stack
}

fn do_operation(mut values: Vec<Token>, operation: char) -> i64 {
    //TODO there has got to be a better way to do this.
    // likely need to implement a way to access Token inner values.
    let val2 = if let Some(Token::Value(n)) = values.pop() {
        n
    } else {
        0
    };
    let val1 = if let Some(Token::Value(n)) = values.pop() {
        n
    } else {
        if operation == 'd' {
            1
        } else {
            0
        }
    };

    match operation {
        'd' => {
            let mut rng = rand::thread_rng();
            let die_size = Uniform::from(1..=val2);
            let mut output: i64 = 0;

            for _ in 1..=val1 {
                output += die_size.sample(&mut rng)
            }

            output
        }
        '*' => val1 * val2,
        '/' => val1 / val2,
        '+' => val1 + val2,
        '-' => val1 - val2,
        _ => panic!("unrecognized operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        assert_eq!(
            Tokens::from("1 d10"),
            Tokens(VecDeque::from([
                Token::Value(1),
                Token::Value(10),
                Token::Operator('d'),
            ]))
        );
        assert_eq!(
            Tokens::from("1d20"),
            Tokens(VecDeque::from([
                Token::Value(1),
                Token::Value(20),
                Token::Operator('d'),
            ]))
        );
        assert_eq!(
            Tokens::from("1d20+1"),
            Tokens(VecDeque::from([
                Token::Value(1),
                Token::Value(20),
                Token::Operator('d'),
                Token::Value(1),
                Token::Operator('+'),
            ]))
        );
        assert_eq!(
            Tokens::from("-1+2d4*(5-1)"),
            Tokens(VecDeque::from([
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
    }

    #[test]
    fn test_shunting_yard() {
        assert_eq!(
            shunting_yard(VecDeque::from([
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(10),
            ])),
            VecDeque::from([Token::Value(1), Token::Value(10), Token::Operator('d'),])
        );
        assert_eq!(
            shunting_yard(VecDeque::from([
                Token::Value(2),
                Token::Operator('d'),
                Token::Value(6),
                Token::Operator('+'),
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(4),
                Token::Operator('-'),
                Token::Value(2),
            ])),
            VecDeque::from([
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
    }
    #[test]
    fn test_shunting_yard_parenthesis() {
        assert_eq!(
            shunting_yard(VecDeque::from([
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
            ])),
            VecDeque::from([
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
            shunting_yard(VecDeque::from([
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
            ])),
            VecDeque::from([
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
    }

    #[test]
    fn test_dice_from() {
        assert_eq!(
            Dice::from(Tokens::from("-1+2d4*(5-1)")),
            Dice(VecDeque::from([
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
            Dice::from(Tokens::from("1+1")),
            Dice(VecDeque::from([
                Token::Value(1),
                Token::Value(20),
                Token::Operator('d'),
            ]))
        );
    }

    #[test]
    fn test_roll() {
        let test_dice = Dice::from(Tokens::from("1d4+1"));
        let test_roll = roll(&test_dice);
        assert!(test_roll >= [Token::Value(2)].to_vec() && test_roll <= [Token::Value(5)].to_vec());
    }

    #[test]
    fn test_do_operation() {
        assert!(do_operation(vec![Token::Value(1), Token::Value(4)], 'd') <= 4);
        assert!(do_operation(vec![Token::Value(1), Token::Value(4)], 'd') >= 1);
        assert!(do_operation(vec![Token::Value(20)], 'd') >= 1);
        assert!(do_operation(vec![Token::Value(20)], 'd') <= 20);

        assert_eq!(do_operation(vec![Token::Value(2), Token::Value(2)], '*'), 4);
        assert_eq!(
            do_operation(vec![Token::Value(-2), Token::Value(2)], '*'),
            -4
        );

        assert_eq!(
            do_operation(vec![Token::Value(15), Token::Value(5)], '/'),
            3
        );
        assert_eq!(do_operation(vec![Token::Value(7), Token::Value(2)], '/'), 3);

        assert_eq!(do_operation(vec![Token::Value(1), Token::Value(1)], '+'), 2);
        assert_eq!(do_operation(vec![Token::Value(1)], '+'), 1);

        assert_eq!(do_operation(vec![Token::Value(1), Token::Value(1)], '-'), 0);
        assert_eq!(do_operation(vec![Token::Value(1)], '-'), -1);
    }
}
