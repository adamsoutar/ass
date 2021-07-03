use crate::parser::tokens::*;

pub fn print_token (token: &Token) {
    match token {
        Token::Keyword(kwd) => eprintln!("Keyword: \"{}\"", kwd),
        Token::Identifier(ident) => eprintln!("Identifier: \"{}\"", ident),
        Token::Integer(int) => eprintln!("Integer literal: {}", int),
        Token::Punctuation(pnc) => eprintln!("Punctuation: {}", pnc),
        Token::Operator(op) => eprintln!("Operator: {}", op),
        Token::Character(ch) => eprint!("Character: {}", ch),
        Token::String(st) => eprintln!("String: \"{}\"", st)
    }
}
