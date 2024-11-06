use crate::scanner;

pub(crate) enum Token {
    Eof,
    Char(char),
    CharSet(char, char),
    Quantifier(usize, usize),
    LPathe
}

pub(crate) struct Tokenizer {
    scanner: scanner::Scanner,
}

impl Tokenizer {
    pub(crate) fn new(regex: &str) -> Tokenizer {
        Tokenizer {
            scanner: scanner::Scanner::new(regex),
        }
    }

    pub(crate) fn next(&self) -> Token {
        Token::Char('a')
    }
}
