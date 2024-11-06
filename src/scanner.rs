// #[derive(Debug)]
// pub(crate) enum Token {
//     Alternate,        // |
//     OpenParentheses,  // (
//     CloseParentheses, // )
//     OpenBracket,      // [
//     CloseBracket,     // ]
//     OpenBrace,        // {
//     CloseBrace,       // }
//     Dash,             // -
//     Caret,            // ^
//     Dollar,           // $
//     Star,             // *
//     Plus,             // +
//     Option,           // ?
//     Concatenation,    // .
//     Any,              // .
//     Character(char),  // Any characters
// }

pub(crate) struct Scanner {
    buffer: String,
    offset: usize,
}

impl Scanner {
    pub(crate) fn new(regex: &str) -> Self {
        Scanner {
            buffer: regex.to_string(),
            offset: 0,
        }
    }

    fn next(&mut self, number: usize) -> String {
        let s = self
            .buffer
            .chars()
            .skip(self.offset)
            .take(self.offset + number)
            .collect();

        self.offset += number;
        s
    }

    fn peek(&self) -> Option<char> {
        self.buffer.chars().nth(self.offset)
    }

    fn consume(&mut self, ch: char) -> bool {
        let peek = self.peek();
        if let Some(c) = peek {
            if c == ch {
                self.advance_once();
                return true;
            }
        }
        return false;
    }

    fn advance(&mut self, number: usize) {
        self.offset += number;
    }

    fn advance_once(&mut self) {
        self.offset += 1;
    }

    fn retreat(&mut self, number: usize) {
        self.offset -= number;
    }

    fn retreat_once(&mut self) {
        self.offset -= 1;
    }
}
