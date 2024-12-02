use std::{char, fmt::Display, os::unix::raw::pid_t, string, vec};

use crate::scanner;

#[derive(PartialEq, PartialOrd, Clone)]
pub(crate) enum Token {
    Eof,

    // characters
    Char(char),
    CharSet(char, char),

    // operators
    Or,
    Concatenate,
    Quantifier(usize, usize),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Char(c) => {
                write!(f, "Char({})", c.to_string())
            }
            Token::CharSet(l, r) => {
                write!(f, "CharSet({}, {})", l.to_string(), r.to_string())
            }
            Token::Or => {
                write!(f, "Or")
            }
            Token::Quantifier(min, max) => {
                write!(f, "Quantifier({}, {})", min.to_string(), max.to_string())
            }
            Token::Concatenate => {
                write!(f, "Concatenate")
            }
            _ => {
                write!(f, "")
            }
        }
    }
}

pub(crate) struct PostfixParser {
    operators: vec::Vec<Token>,
    scanner: scanner::Scanner,
}

impl PostfixParser {
    pub(crate) fn new(regex: &str) -> PostfixParser {
        PostfixParser {
            scanner: scanner::Scanner::new(regex),
            operators: vec::Vec::new(),
        }
    }

    pub(crate) fn parse(&mut self) -> Vec<Token> {
        let mut postfixs: vec::Vec<Token> = vec::Vec::new();
        let mut char_pushed = false;

        while let Some(c) = self.scanner.consume() {
            if c == '|' {
                self.push_operator(&mut postfixs, Token::Or);
                char_pushed = false;
                continue;
            }

            postfixs.push(Token::Char(c));
            if char_pushed {
                self.push_operator(&mut postfixs, Token::Concatenate);
            }
            char_pushed = true;
        }

        while !self.operators.is_empty() {
            postfixs.push(self.operators.pop().unwrap());
        }

        return postfixs;
    }

    fn push_operator(&mut self, postfixs: &mut Vec<Token>, token: Token) {
        while !self.operators.is_empty() {
            let top = self.operators.last().unwrap();

            if *top < token {
                self.operators.push(token);
                return;
            }

            postfixs.push(self.operators.pop().unwrap());
        }

        self.operators.push(token);
    }
}
