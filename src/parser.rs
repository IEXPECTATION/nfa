use std::{char, fmt::Display, usize, vec};

use crate::scanner;

#[derive(PartialEq, PartialOrd, Clone)]
pub(crate) enum CharSet {
    Signal(char),
    Range(char, char),
}

#[derive(PartialEq, PartialOrd, Clone)]
pub(crate) enum Token {
    // characters
    Char(char),
    CharSets { sets: Vec<CharSet>, excluded: bool },

    // operators
    Or,
    Concatenate,
    Quantifier(usize, usize),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Char(c) => {
                write!(f, "Char({})", c.to_string())
            }
            Token::CharSets { sets, excluded } => /* match char_set {
                CharSet::Signal(c) => {
                    write!(f, "CharSets({})", c.to_string())
                }
                CharSet::Range(l, r) => {
                    write!(f, "CharSets({}, {})", l.to_string(), r.to_string())
                } */
               {todo!()
            },
            Token::Or => {
                write!(f, "Or")
            }
            Token::Quantifier(min, max) => {
                write!(f, "Quantifier({}, {})", min.to_string(), max.to_string())
            }
            Token::Concatenate => {
                write!(f, "Concatenate")
            }
        }
    }
}

pub(crate) struct PostfixParser {
    scanner: scanner::Scanner,
}

impl PostfixParser {
    pub(crate) fn new(regex: &str) -> PostfixParser {
        PostfixParser {
            scanner: scanner::Scanner::new(regex),
        }
    }

    pub(crate) fn parse(&mut self) -> Vec<Token> {
        let mut postfixs: vec::Vec<Token> = vec::Vec::new();
        let mut operators: vec::Vec<Token> = vec::Vec::new();
        let mut char_pushed = false;
        let mut parenthnesses: vec::Vec<usize> = vec::Vec::new();

        while let Some(c) = self.scanner.consume() {
            if c == '|' {
                PostfixParser::push_operator(&mut postfixs, &mut operators, Token::Or);
                char_pushed = false;
                continue;
            }

            if c == '*' {
                PostfixParser::push_operator(
                    &mut postfixs,
                    &mut operators,
                    Token::Quantifier(0, usize::MAX),
                );
                char_pushed = true;
                continue;
            }

            if c == '+' {
                PostfixParser::push_operator(
                    &mut postfixs,
                    &mut operators,
                    Token::Quantifier(1, usize::MAX),
                );
                char_pushed = true;
                continue;
            }

            if c == '?' {
                PostfixParser::push_operator(
                    &mut postfixs,
                    &mut operators,
                    Token::Quantifier(0, 1),
                );
                char_pushed = true;
                continue;
            }

            if c == '(' {
                parenthnesses.push(operators.len());
                char_pushed = false;
                continue;
            }

            if c == ')' {
                let top = parenthnesses.pop().unwrap();

                while operators.len() > top {
                    let last = operators.pop().unwrap();
                    postfixs.push(last);
                }
                char_pushed = true;
                continue;
            }

            if char_pushed {
                Self::push_operator(&mut postfixs, &mut operators, Token::Concatenate);
            }
            postfixs.push(Token::Char(c));
            char_pushed = true;
        }

        while !operators.is_empty() {
            postfixs.push(operators.pop().unwrap());
        }

        return postfixs;
    }

    fn push_operator(postfixs: &mut Vec<Token>, operators: &mut vec::Vec<Token>, token: Token) {
        while !operators.is_empty() {
            let top = operators.last().unwrap();

            if *top < token {
                operators.push(token);
                return;
            }

            postfixs.push(operators.pop().unwrap());
        }

        operators.push(token);
    }
}
