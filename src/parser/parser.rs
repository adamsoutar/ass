use crate::parser::parser_helpers::*;
use crate::parser::ast_utils::*;
use crate::parser::tokeniser::Tokeniser;
use crate::parser::tokens::*;
use crate::parser::token_printer::print_token;

pub struct Parser {
    pub tokeniser: Tokeniser
}

impl Parser {
    pub fn generate_ast (&mut self) -> Vec<ASTNode> {
        let bs = self.parse_block_statement(false, false);

        match bs {
            ASTNode::BlockStatement(statements) => statements,
            _ => unreachable!()
        }
    }

    fn parse_block_statement (&mut self, expect_first: bool, expect_last: bool) -> ASTNode {
        if expect_first { self.expect_punctuation('{') }

        let mut statements = vec![];
        while !self.tokeniser.eof {
            if expect_last && self.is_next_punctuation('}') {
                break
            }

            statements.push(self.parse_component())
        }

        if expect_last { self.expect_punctuation('}') }
        ASTNode::BlockStatement(statements)
    }

    fn parse_component (&mut self) -> ASTNode {
        let mut node = self.parse_atom();

        // TODO: Check for binary operators, calls, etc.

        node
    }

    fn parse_atom (&mut self) -> ASTNode {
        let t = self.tokeniser.read();

        match t {
            Token::Integer(int) => return ASTNode::IntegerLiteral(int),
            _ => {}
        }

        self.parse_statement(t)
    }

    // NOTE: "int" is the only type for now
    fn parse_statement (&mut self, t: Token) -> ASTNode {
        if let Token::Keyword(kwd) = &t {
            let kwdstr = &kwd[..];
            match kwdstr {
                "int" => return self.parse_declaration(t),
                "return" => return self.parse_return_statement(),
                _ => panic!("Unexpected keyword \"{}\"", kwd)
            }
        }

        print_token(&t);
        panic!("Parser encountered an unexpected token")
    }

    fn parse_return_statement (&mut self) -> ASTNode {
        let ret_val = self.parse_component();
        self.expect_punctuation(';');
        ASTNode::ReturnStatement(Box::new(ret_val))
    }

    // Declarations of variables and functions start the same (with a type)
    fn parse_declaration (&mut self, _t: Token) -> ASTNode {
        let name_tk = self.tokeniser.read();
        let name = match name_tk {
            Token::Identifier(ident) => ident,
            _ => {
                print_token(&name_tk);
                panic!("Expected declaration identifier but didn't get one")
            }
        };

        if self.is_next_punctuation('(') {
            // This is a function declaration with a parameter list
            self.tokeniser.read();
            // For now, we only support 0-arg functions
            self.expect_punctuation(')');
            let body_node = self.parse_block_statement(true, true);
            let body = match body_node {
                ASTNode::BlockStatement(stmts) => stmts,
                _ => unreachable!()
            };

            return ASTNode::FunctionDefinition(ASTFunctionDefinition {
                name,
                body
            })
        } else {
            // This is a variable declaration
            panic!("Variables (\"{}\") are not yet implemented", name)
        }
    }

    pub fn new (tokeniser: Tokeniser) -> Parser {
        Parser {
            tokeniser
        }
    }
}
