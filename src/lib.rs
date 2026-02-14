#! /usr/bin/env rustc
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    Value(u64),
    Operator(char),
}

#[derive(PartialEq, Debug)]
pub struct Tokens(Vec<Token>);

impl From<&str> for Tokens {
    fn from(input: &str) -> Self {
        let mut tokens: Vec<Token> = vec![];
        let mut chars = input.chars().peekable();

        while let Some(&current_char) = chars.peek() {
            match current_char {
                '0'..='9' => {
                    tokens.push(tokenize_number(&mut chars));
                }
                'd' | '*' | '/' | '+' | '-' => {
                    tokens.push(Token::Operator(current_char));
                    chars.next();
                }
                _ => {
                    // !TODO this should really be handled properly
                    panic!("Couldn't parse dice notation")
                }
            }
        }

        Tokens(tokens)
    }
}

fn tokenize_number(chars: &mut Peekable<Chars>) -> Token {
    let mut number: u64 = 0;

    while let Some(digit) = chars.next_if(|c| c.is_ascii_digit()) {
        number = number * 10 + (digit.to_digit(10).unwrap_or(0) as u64);
    }

    Token::Value(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        assert_eq!(
            Tokens::from("1d10"),
            Tokens(vec![
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(10)
            ])
        );
        assert_eq!(
            Tokens::from("1d20"),
            Tokens(vec![
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(20)
            ])
        );
        assert_eq!(
            Tokens::from("1d20+1"),
            Tokens(vec![
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(20),
                Token::Operator('+'),
                Token::Value(1)
            ])
        );
    }
}
