use super::ast_utils::*;
use super::tokeniser::Tokeniser;
use super::tokens::*;
use super::token_printer::print_token;
use super::types::Type;
use crate::parser::types::{IntegerTypeMetadata, PointerTypeMetadata};

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
        let mut node = self.parse_atom();

        while !self.tokeniser.eof {
            let (was_call, call_node) = self.maybe_call(node);
            node = call_node;

            if !was_call { break; }
        }

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

    fn maybe_call (&mut self, me: ASTNode) -> (bool, ASTNode) {
        if !self.is_next_punctuation('(') {
            return (false, me);
        }
        self.tokeniser.read();

        let func_name = match me {
            ASTNode::Identifier(ident) => ident,
            _ => panic!("Function call must be an identifier (eg. not 3.14() )")
        };

        let mut args = vec![];
        while !self.tokeniser.eof {
            if self.is_next_punctuation(')') {
                self.tokeniser.read();
                break;
            }

            args.push(self.parse_component(0));

            // TODO: This doesn't quite follow the standard
            if self.is_next_punctuation(',') {
                self.tokeniser.read();
            }
        }

        (true, ASTNode::FunctionCall(ASTFunctionCall {
            name: func_name,
            args
        }))
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
                "return" => return self.parse_return_statement(),
                "if" => return self.parse_if_statement(),
                "while" => return self.parse_while_loop(),
                "for" => return self.parse_for_loop(),
                _ if is_builtin_type_name(kwd) => return self.parse_declaration(kwd),
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

    // Similar to an if statement without 'else'
    fn parse_while_loop (&mut self) -> ASTNode {
        self.expect_punctuation('(');
        let condition = Box::new(self.parse_component(0));
        self.expect_punctuation(')');
        let body = Box::new(self.parse_component(0));

        ASTNode::WhileLoop(ASTWhileLoop {
            condition,
            body
        })
    }

    // Quite complicated
    fn parse_for_loop (&mut self) -> ASTNode {
        self.expect_punctuation('(');

        let mut declaration = None;
        if !self.is_next_punctuation(';') {
            declaration = Some(Box::new(self.parse_component(0)));
        } else { self.tokeniser.read(); }

        let mut condition = None;
        if !self.is_next_punctuation(';') {
            condition = Some(Box::new(self.parse_component(0)));
        } else { self.tokeniser.read(); }

        let mut modification = None;
        if !self.is_next_punctuation(')') {
            modification = Some(Box::new(self.parse_component(0)));
        }

        self.expect_punctuation(')');

        let body = Box::new(self.parse_component(0));

        ASTNode::ForLoop(ASTForLoop {
            declaration,
            condition,
            modification,
            body
        })
    }

    fn parse_return_statement (&mut self) -> ASTNode {
        let ret_val = self.parse_component(0);
        // self.expect_punctuation(';');
        ASTNode::ReturnStatement(Box::new(ret_val))
    }

    fn parse_type (&mut self, start_keyword: &String) -> Type {
        // TODO: Modifiers like long/unsigned
        let mut the_type = match &start_keyword[..] {
            "char" => Type::Char(IntegerTypeMetadata { signed: true }),
            "short" => Type::Short(IntegerTypeMetadata { signed: true }),
            "int" => Type::Int(IntegerTypeMetadata { signed: true }),
            _ => unimplemented!("Type {}", start_keyword)
        };

        while self.is_next_operator("*") {
            self.tokeniser.read();
            the_type = Type::Pointer(PointerTypeMetadata {
                points_to: Box::new(the_type)
            })
        }

        the_type
    }

    // Declarations of variables and functions start the same (with a type)
    fn parse_declaration (&mut self, type_start_keyword: &String) -> ASTNode {
        let var_type = self.parse_type(type_start_keyword);

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

            // Parse parameters
            let mut params = vec![];
            while self.is_next_builtin_type_name() {
                let tk = self.tokeniser.read();
                let param_type = match tk {
                    Token::Keyword(kw) => self.parse_type(&kw),
                    _ => panic!("Expected a type for function param")
                };

                // For now, we only support named parameters
                let param_name = match self.tokeniser.read() {
                    Token::Identifier(ident) => ident,
                    _ => panic!("Function parameters must be identifiers")
                };
                params.push(ASTFunctionParameter {
                    name: param_name,
                    param_type
                });

                // NOTE: This doesn't quite match the standard
                if self.is_next_punctuation(',') {
                    self.tokeniser.read();
                }
            }

            self.expect_punctuation(')');

            let mut body = None;
            if self.is_next_punctuation('{') {
                let body_node = self.parse_block_statement(true, true);
                body = match body_node {
                    ASTNode::BlockStatement(stmts) => Some(stmts),
                    _ => unreachable!()
                };
            }

            return ASTNode::FunctionDefinition(ASTFunctionDefinition {
                name,
                return_type: var_type,
                body,
                params
            })
        } else {
            // This is a variable declaration
            let mut initial_value = None;

            if self.is_next_operator("=") {
                // It has an initial value
                self.tokeniser.read();
                initial_value = Some(Box::new(self.parse_component(0)));
            }

            ASTNode::VariableDeclaration(ASTVariableDeclaration {
                identifier: name,
                var_type,
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
