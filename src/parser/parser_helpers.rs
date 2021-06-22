use super::parser::Parser;
use super::tokens::*;

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

    pub fn is_next_keyword(&self, s: &str) -> bool {
        match self.tokeniser.peek() {
            Token::Keyword(kw) => &kw[..] == s,
            _ => false
        }
    }

    pub fn is_next_builtin_type_name (&self) -> bool {
        match self.tokeniser.peek() {
            Token::Keyword(kw) => is_builtin_type_name(&kw),
            _ => false
        }
    }
}
