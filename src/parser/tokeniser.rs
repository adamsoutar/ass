use crate::parser::char_stream::CharStream;
use crate::parser::tokens::*;
use std::iter::FromIterator;

pub struct Tokeniser {
    pub code: CharStream,
    pub current: Token,
    pub eof: bool
}

impl Tokeniser {
    fn read_next (&mut self) {
        self.eat_whitespace();

        if self.code.eof {
            self.eof = true;
            return
        }

        let c = self.code.read();

        if is_number(&c) {
            self.current = self.read_number(c);
        } else if is_identifier_start(&c) {
            self.current = self.read_identifier(c);
        } else if is_punctuation(&c) {
            self.current = Token::Punctuation(c);
        } else if is_operator_char(&c) {
            self.current = self.read_operator(c);
        } else {
            panic!("Unrecognised character \"{}\"", c)
        }
    }

    fn read_operator (&mut self, first: char) -> Token {
        let mut oper = vec![first];
        while !self.code.eof && is_operator_char(&self.code.peek()) {
            oper.push(self.code.read())
        }
        let op_str = String::from_iter(oper);

        if !is_operator(&op_str) {
            panic!("\"{}\" is not an operator", op_str)
        }

        Token::Operator(op_str)
    }

    fn read_identifier (&mut self, first: char) -> Token {
        let mut ident = vec![first];
        while !self.code.eof && is_identifier(&self.code.peek()) {
            ident.push(self.code.read())
        }
        let st = String::from_iter(ident);

        if is_keyword(&st) {
            Token::Keyword(st)
        } else {
            Token::Identifier(st)
        }
    }

    fn read_number (&mut self, first: char) -> Token {
        let mut vc = vec![first];
        while !self.code.eof && is_number(&self.code.peek()) {
            vc.push(self.code.read())
        }
        let st = String::from_iter(vc);
        Token::Integer(st.parse().unwrap())
    }

    fn eat_whitespace (&mut self) {
        while !self.code.eof && is_whitespace(&self.code.peek()) {
            self.code.read();
        }
    }

    pub fn peek (&self) -> Token {
        self.current.clone()
    }

    pub fn read (&mut self) -> Token {
        let tk = self.current.clone();
        self.read_next();
        tk
    }

    pub fn new (code: CharStream) -> Tokeniser {
        let mut tok = Tokeniser {
            code,
            current: Token::Integer(0),
            eof: false
        };
        tok.read_next();
        tok
    }
}
