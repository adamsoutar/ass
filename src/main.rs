mod parser;

use parser::char_stream::CharStream;
use parser::tokeniser::Tokeniser;
use parser::parser::Parser;
use parser::ast_printer::print_ast_node;

fn main() {
    let code = "
int main () {
    return 2;
}
    ".to_string();

    let stream = CharStream::new(code);
    let tokeniser = Tokeniser::new(stream);
    let mut parser = Parser::new(tokeniser);

    let ast = parser.generate_ast();

    for node in ast {
        print_ast_node(&node, 0);
    }
}
