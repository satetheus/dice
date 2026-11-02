#! /usr/bin/env rustc
pub mod roll_dice {
    use rand::distributions::{Distribution, Uniform};
    use std::collections::VecDeque;

    #[derive(Clone, Eq, PartialEq, PartialOrd, Debug)]
    pub enum Token {
        Value(i64),
        Operator(char),
        UnaryOp(char),
        OpenParenthesis,
        CloseParenthesis,
        None,
    }

    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct Dice(pub VecDeque<Token>);

    impl From<Tokens> for Dice {
        fn from(input: Tokens) -> Self {
            assert!(!!&input.validate(), "Invalid input: {:?}", &input);

            Self(input.0)
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
                        Self::do_operation(values, n)
                    }
                    Token::UnaryOp(n) => {
                        let values = solve_stack.split_off(solve_stack.len().saturating_sub(1));
                        Self::do_operation(values, n)
                    }
                    Token::Value(n) => *n,
                    _ => panic!("Invalid Token found"),
                };
                solve_stack.push(Token::Value(solved_value));
            }

            solve_stack
        }

        //TODO this should be a private function. Its a helper function for roll
        #[must_use]
        pub fn do_operation(mut values: Vec<Token>, operation: &char) -> i64 {
            let val2 = values.pop().unwrap_or(Token::Value(0)).get_value();
            let val1 = values
                .pop()
                .unwrap_or_else(|| {
                    if operation == &'d' {
                        Token::Value(1)
                    } else {
                        Token::Value(0)
                    }
                })
                .get_value();

            match operation {
                'd' => {
                    let mut rng = rand::thread_rng();
                    let die_size = Uniform::from(1..=val2);
                    let mut output: i64 = 0;

                    for _ in 1..=val1 {
                        output += die_size.sample(&mut rng);
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
    }

    #[derive(Eq, PartialEq, Debug)]
    pub struct Tokens(pub VecDeque<Token>);

    impl From<&str> for Tokens {
        fn from(input: &str) -> Self {
            let tokens = Self::tokenize(input);

            tokens.shunting_yard()
        }
    }

    impl Tokens {
        fn tokenize(input: &str) -> Self {
            let mut chars = input.chars().peekable();
            let mut tokens: VecDeque<Token> = VecDeque::new();

            while let Some(&current_char) = chars.peek() {
                let new_token = match current_char {
                    '0'..='9' => {
                        let prev_digit = tokens.pop_back_if(|x| matches!(*x, Token::Value { .. }));
                        let new_digit = i64::from(current_char.to_digit(10).unwrap_or(0));
                        match prev_digit {
                            Some(Token::Value(n)) => Token::Value(10 * n + new_digit),
                            _ => Token::Value(new_digit),
                        }
                    }
                    //TODO 'd' can also be a UnaryOp (ex: d20)
                    'd' | '*' | '/' => Token::Operator(current_char),
                    '+' | '-' => {
                        tokens
                            .back()
                            .map_or(Token::UnaryOp(current_char), |op| match op {
                                Token::Operator(_) | Token::OpenParenthesis => {
                                    Token::UnaryOp(current_char)
                                }
                                _ => Token::Operator(current_char),
                            })
                    }
                    '(' => Token::OpenParenthesis,
                    ')' => Token::CloseParenthesis,
                    _ => {
                        println!("skipping character '{current_char}'");
                        Token::None
                    }
                };
                tokens.push_back(new_token);
                chars.next();
            }

            Self(tokens)
        }

        fn validate(&self) -> bool {
            // TODO! more than this check will eventually be needed, but it's is a start
            if !self.0.contains(&Token::Operator('d')) {
                println!("Supplied dice notation does not contain a dice symbol: 'd'");
                return false;
            }

            true
        }

        #[must_use]
        pub fn shunting_yard(mut self) -> Self {
            let mut op_stack: Vec<Token> = vec![];
            let mut out_stack: VecDeque<Token> = VecDeque::new();

            while let Some(token) = self.0.pop_front() {
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

            Self(out_stack)
        }
    }

    impl Token {
        fn get_order(&self) -> i16 {
            match self {
                Self::Operator(op) => match op {
                    '+' | '-' => 1,
                    '*' | '/' => 2,
                    '^' => 3,
                    'd' => 4,
                    _ => todo!(),
                },
                // UnaryOp order is subject to change, but must be higher than d &
                // the other standard math operators
                Self::UnaryOp(_) => 10,
                _ => -1,
            }
        }

        fn get_value(self) -> i64 {
            match self {
                Self::Value(n) => n,
                _ => panic!("not a value type"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::roll_dice::{Dice, Token, Tokens};
    use std::collections::VecDeque;

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
        assert_eq!(
            Tokens::from("-1+(-1)"),
            Tokens(VecDeque::from([
                Token::Value(1),
                Token::UnaryOp('-'),
                Token::Value(1),
                Token::UnaryOp('-'),
                Token::Operator('+'),
            ]))
        );
    }

    #[test]
    fn test_shunting_yard() {
        assert_eq!(
            Tokens(VecDeque::from([
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(10),
            ]))
            .shunting_yard(),
            Tokens(VecDeque::from([
                Token::Value(1),
                Token::Value(10),
                Token::Operator('d'),
            ]))
        );
        assert_eq!(
            Tokens(VecDeque::from([
                Token::Value(2),
                Token::Operator('d'),
                Token::Value(6),
                Token::Operator('+'),
                Token::Value(1),
                Token::Operator('d'),
                Token::Value(4),
                Token::Operator('-'),
                Token::Value(2),
            ]))
            .shunting_yard(),
            Tokens(VecDeque::from([
                Token::Value(2),
                Token::Value(6),
                Token::Operator('d'),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Value(2),
                Token::Operator('-'),
                Token::Operator('+'),
            ]))
        );
    }
    #[test]
    fn test_shunting_yard_parenthesis() {
        assert_eq!(
            Tokens(VecDeque::from([
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
            ]))
            .shunting_yard(),
            Tokens(VecDeque::from([
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Value(5),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Operator('-'),
                Token::Operator('*'),
            ]))
        );
        assert_eq!(
            Tokens(VecDeque::from([
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
            ]))
            .shunting_yard(),
            Tokens(VecDeque::from([
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Value(5),
                Token::Value(1),
                Token::Value(4),
                Token::Operator('d'),
                Token::Operator('-'),
                Token::Operator('*'),
            ]))
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
    }

    #[test]
    #[should_panic(expected = "Invalid input:")]
    fn test_dice_from_panics() {
        let _ = Dice::from(Tokens::from("1+1"));
    }

    #[test]
    fn test_roll() {
        let test_dice = Dice::from(Tokens::from("1d4+1"));
        let test_roll = &test_dice.roll();
        assert!(
            *test_roll >= [Token::Value(2)].to_vec() && *test_roll <= [Token::Value(5)].to_vec()
        );
    }

    #[test]
    fn test_do_operation() {
        assert!(Dice::do_operation(vec![Token::Value(1), Token::Value(4)], &'d') <= 4);
        assert!(Dice::do_operation(vec![Token::Value(1), Token::Value(4)], &'d') >= 1);
        assert!(Dice::do_operation(vec![Token::Value(20)], &'d') >= 1);
        assert!(Dice::do_operation(vec![Token::Value(20)], &'d') <= 20);

        assert_eq!(
            Dice::do_operation(vec![Token::Value(2), Token::Value(2)], &'*'),
            4
        );
        assert_eq!(
            Dice::do_operation(vec![Token::Value(-2), Token::Value(2)], &'*'),
            -4
        );

        assert_eq!(
            Dice::do_operation(vec![Token::Value(15), Token::Value(5)], &'/'),
            3
        );
        assert_eq!(
            Dice::do_operation(vec![Token::Value(7), Token::Value(2)], &'/'),
            3
        );

        assert_eq!(
            Dice::do_operation(vec![Token::Value(1), Token::Value(1)], &'+'),
            2
        );
        assert_eq!(Dice::do_operation(vec![Token::Value(1)], &'+'), 1);

        assert_eq!(
            Dice::do_operation(vec![Token::Value(1), Token::Value(1)], &'-'),
            0
        );
        assert_eq!(Dice::do_operation(vec![Token::Value(1)], &'-'), -1);
    }
}
