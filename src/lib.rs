#! /usr/bin/env rustc
use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    Value(u64),
    Operator(char),
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
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Tokens(VecDeque<Token>);

impl From<&str> for Tokens {
    fn from(input: &str) -> Self {
        let mut tokens: VecDeque<Token> = VecDeque::new();
        let mut chars = input.chars().peekable();

        while let Some(&current_char) = chars.peek() {
            match current_char {
                '0'..='9' => {
                    tokens.push_back(tokenize_number(&mut chars));
                }
                'd' | '*' | '/' | '+' | '-' => {
                    tokens.push_back(Token::Operator(current_char));
                    chars.next();
                }
                _ => {
                    // !TODO this should really be handled properly
                    panic!("Couldn't parse dice notation")
                }
            }
        }

        Tokens(shunting_yard(tokens))
    }
}

fn tokenize_number(chars: &mut Peekable<Chars>) -> Token {
    let mut number: u64 = 0;

    while let Some(digit) = chars.next_if(|c| c.is_ascii_digit()) {
        number = number * 10 + (digit.to_digit(10).unwrap_or(0) as u64);
    }

    Token::Value(number)
}

fn shunting_yard(mut input: VecDeque<Token>) -> VecDeque<Token> {
    let mut op_stack: Vec<Token> = vec![];
    let mut out_stack: VecDeque<Token> = VecDeque::new();

    while let Some(token) = input.pop_front() {
        match token {
            Token::Value(_) => out_stack.push_back(token),
            Token::Operator(_) => {
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
        }
    }

    if !op_stack.is_empty() {
        for _ in 0..op_stack.len() {
            out_stack.push_back(op_stack.pop().expect("this should always work"));
        }
    }

    out_stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        assert_eq!(
            Tokens::from("1d10"),
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
}
