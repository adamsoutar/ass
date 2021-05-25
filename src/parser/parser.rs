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

            statements.push(self.parse_component(0))
        }

        if expect_last { self.expect_punctuation('}') }
        ASTNode::BlockStatement(statements)
    }

    fn parse_component (&mut self, precedence: usize) -> ASTNode {
        let node = self.parse_atom();

        // TODO: Check for calls, array access, etc.

        let bin = self.maybe_binary_operation(node, precedence);
        self.allow_expression_statement();
        bin
    }

    // It might be something we didn't expect to be a statement.
    // eg. 3 + 3; or a = 1;
    fn allow_expression_statement (&mut self) {
        if self.is_next_punctuation(';') {
            self.tokeniser.read();
        }
    }

    fn maybe_binary_operation (&mut self, me: ASTNode, my_precedence: usize) -> ASTNode {
        let t = self.tokeniser.peek().clone();

        if let Token::Operator(op) = t {
            if is_binary_operator(&op) {
                let their_prec = get_operator_precedence(&op);

                if their_prec > my_precedence {
                    self.tokeniser.read();

                    let them = self.parse_component(their_prec);

                    let node = ASTNode::BinaryOperation(ASTBinaryOperation {
                        left_side: Box::new(me),
                        operator: op,
                        right_side: Box::new(them)
                    });

                    return self.maybe_binary_operation(node, my_precedence)
                }
            }

            // Assignment operators have right-to-left associativity
            if is_assignment_operator(&op) {
                self.tokeniser.read();

                let them = self.parse_component(0);

                let node = ASTNode::BinaryOperation(ASTBinaryOperation {
                    left_side: Box::new(me),
                    operator: op,
                    right_side: Box::new(them)
                });

                return node;
            }
        }

        me
    }

    fn parse_atom (&mut self) -> ASTNode {
        let t = self.tokeniser.read();

        if let Token::Punctuation(pnc) = t {
            // Bracketed expressions
            if pnc == '(' {
                let contents = self.parse_component(0);
                self.expect_punctuation(')');
                return contents
            }

            // Compound statements
            if pnc == '{' {
                let contents = self.parse_block_statement(false, true);
                return contents
            }
        }

        match t {
            Token::Integer(int) => return ASTNode::IntegerLiteral(int),
            Token::Identifier(ident) => return ASTNode::Identifier(ident),
            Token::Operator(oper) => return self.parse_unary_operation(oper),
            _ => {}
        }

        self.parse_statement(t)
    }

    fn parse_unary_operation (&mut self, oper: String) -> ASTNode {
        if !is_unary_operator(&oper) {
            panic!("\"{}\" was used as a unary operator but it isn't one", oper);
        }

        let operand = self.parse_atom();
        ASTNode::UnaryOperation(ASTUnaryOperation {
            operator: oper,
            operand: Box::new(operand)
        })
    }

    // NOTE: "int" is the only type for now
    fn parse_statement (&mut self, t: Token) -> ASTNode {
        if let Token::Keyword(kwd) = &t {
            let kwdstr = &kwd[..];
            match kwdstr {
                "int" => return self.parse_declaration(t),
                "return" => return self.parse_return_statement(),
                "if" => return self.parse_if_statement(),
                _ => panic!("Unexpected keyword \"{}\"", kwd)
            }
        }

        print_token(&t);
        panic!("Parser encountered an unexpected token")
    }

    fn parse_if_statement (&mut self) -> ASTNode {
        self.expect_punctuation('(');
        let condition = Box::new(self.parse_component(0));
        self.expect_punctuation(')');

        let body = Box::new(self.parse_component(0));

        let mut else_stmt = None;
        if self.is_next_keyword("else") {
            self.tokeniser.read();
            else_stmt = Some(
                Box::new(self.parse_component(0))
            );
        }

        ASTNode::IfStatement(ASTIfStatement {
            condition,
            body,
            else_stmt
        })
    }

    fn parse_return_statement (&mut self) -> ASTNode {
        let ret_val = self.parse_component(0);
        // self.expect_punctuation(';');
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
            let mut initial_value = None;

            if self.is_next_operator("=") {
                // It has an initial value
                self.tokeniser.read();
                initial_value = Some(Box::new(self.parse_component(0)));
            }

            // self.expect_punctuation(';');

            ASTNode::VariableDeclaration(ASTVariableDeclaration {
                identifier: name,
                initial_value
            })
        }
    }

    pub fn new (tokeniser: Tokeniser) -> Parser {
        Parser {
            tokeniser
        }
    }
}
