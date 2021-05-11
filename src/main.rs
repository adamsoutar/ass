use std::env;
use std::fs;

mod parser;
mod codegen;

use parser::char_stream::CharStream;
use parser::tokeniser::Tokeniser;
use parser::parser::Parser;
use parser::ast_printer::print_ast_node;
use codegen::codegen::Codegen;

fn main() {
    let filename = env::args().nth(1)
        .expect("Pass a C file path argument");
    let code = fs::read_to_string(filename)
        .expect("Failed to open code file for reading");

    let stream = CharStream::new(code);
    let tokeniser = Tokeniser::new(stream);
    let mut parser = Parser::new(tokeniser);

    let ast = parser.generate_ast();

    // for node in &ast {
    //     print_ast_node(node, 0);
    // }

    let mut codegen = Codegen::new(ast);
    codegen.generate();

    print!("{}", codegen.generated)
}
