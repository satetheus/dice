#! /usr/bin/env rustc
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    Dice,
    Number(u64),
    Operator(char),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&current_char) = chars.peek() {
        dbg!(&chars);
        match current_char {
            'd' => {
                tokens.push(Token::Dice);
                chars.next();
            }
            '0'..='9' => {
                tokens.push(Token::Number(tokenize_number(&mut chars)));
            }
            '*' | '/' | '+' | '-' => {
                tokens.push(Token::Operator(current_char));
                chars.next();
            }
            _ => {
                panic!("Couldn't parse dice notation")
            }
        }
    }

    tokens
}

fn tokenize_number(chars: &mut Peekable<Chars>) -> u64 {
    let mut result = String::new();

    while let Some(&current_char) = chars.peek() {
        match current_char {
            '0'..='9' => {
                result.push(current_char);
                chars.next();
            }
            _ => break,
        }
    }
    result.parse::<u64>().expect("issue parsing number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("1d10"),
            vec![Token::Number(1), Token::Dice, Token::Number(10)]
        );
        assert_eq!(
            tokenize("1d20"),
            vec![Token::Number(1), Token::Dice, Token::Number(20)]
        );
        assert_eq!(
            tokenize("1d20+1"),
            vec![
                Token::Number(1),
                Token::Dice,
                Token::Number(20),
                Token::Operator('+'),
                Token::Number(1)
            ]
        );
    }
}
