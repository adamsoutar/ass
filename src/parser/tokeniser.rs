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

        let mut c = self.code.read();

        while self.is_comment_coming(c) {
            self.eat_comment();
            c = self.code.read();
            // Edge-case where code ends with a comment
            if self.code.eof {
                self.eof = true;
                return
            }
        }

        if is_number(&c) {
            self.current = self.read_number(c);
        } else if is_identifier_start(&c) {
            self.current = self.read_identifier(c);
        } else if is_punctuation(&c) {
            self.current = Token::Punctuation(c);
        } else if is_operator_char(&c) {
            self.current = self.read_operator(c);
        } else if c == '\'' {
            self.current = self.read_character_literal();
        } else if c == '"' {
            self.current = self.read_string_literal();
        } else {
            panic!("Unrecognised character \"{}\"", c)
        }
    }

    fn read_string_literal (&mut self) -> Token {
        let mut str_vec = vec![];
        while !self.code.eof && self.code.peek() != '"' {
            str_vec.push(self.code.read());
        }
        self.code.read();

        Token::String(str_vec.iter().collect())
    }

    fn read_character_literal (&mut self) -> Token {
        let char_val = self.code.read();
        self.expect_char('\'');
        Token::Character(char_val)
    }

    fn read_operator (&mut self, first: char) -> Token {
        let mut oper = vec![first];

        // * is a special case - we always finish when reading it once.
        // This is to help with reading double pointers like char**
        if first != '*' {
            while !self.code.eof && is_operator_char(&self.code.peek()) {
                oper.push(self.code.read())
            }
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

    fn expect_char (&mut self, ch: char) {
        let next = self.code.read();
        if next != ch {
            panic!("Expected '{}' but got '{}'", ch, next)
        }
    }

    fn is_comment_coming (&self, first: char) -> bool {
        let next = self.code.peek();
        first == '/' && (next == '/' || next == '*')
    }

    fn eat_comment (&mut self) {
        let second_char = self.code.read();
        if second_char == '*' {
            while !self.code.eof {
                let c = self.code.read();
                if c == '*' && self.code.peek() == '/' {
                    self.code.read();
                    break;
                }
            }
        } else {
            while !self.code.eof && self.code.peek() != '\n' {
                self.code.read();
            }
        }
        self.eat_whitespace();
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
