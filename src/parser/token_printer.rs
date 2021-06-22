use crate::parser::tokens::*;

pub fn print_token (token: &Token) {
    match token {
        Token::Keyword(kwd) => println!("Keyword: \"{}\"", kwd),
        Token::Identifier(ident) => println!("Identifier: \"{}\"", ident),
        Token::Integer(int) => println!("Integer literal: {}", int),
        Token::Punctuation(pnc) => println!("Punctuation: {}", pnc),
        Token::Operator(op) => println!("Operator: {}", op)
    }
}
