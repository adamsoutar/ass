use crate::parser::parser::Parser;
use crate::parser::tokens::*;

impl Parser {
    pub fn expect_punctuation(&mut self, c: char) {
        let tk = self.tokeniser.read();
        if !match tk {
            Token::Punctuation(pnc) => pnc == c,
            _ => false
        } {
            panic!("Expected punctution '{}'", c)
        }
    }

    pub fn is_next_punctuation(&self, c: char) -> bool {
        match self.tokeniser.peek() {
            Token::Punctuation(pnc) => pnc == c,
            _ => false
        }
    }

    pub fn is_next_operator(&self, s: &str) -> bool {
        match self.tokeniser.peek() {
            Token::Operator(op) => &op[..] == s,
            _ => false
        }
    }
}
