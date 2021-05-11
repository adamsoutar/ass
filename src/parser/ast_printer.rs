use crate::parser::ast_utils::*;

fn print_at_depth (s: String, depth: isize) {
    let mut str = String::from("");
    for _ in 0..depth * 4 {
        str += &String::from(" ");
    }
    str += &s;
    println!("{}", str);
}

pub fn print_ast_node (node: &ASTNode, depth: isize) {
    match node {
        ASTNode::IntegerLiteral(int) => {
            print_at_depth(format!("Integer literal: {}", int), depth)
        },
        ASTNode::Identifier(ident) => {
            print_at_depth(format!("Identifier: {}", ident), depth)
        },
        ASTNode::ReturnStatement(ret_stmt) => {
            print_at_depth(format!("Return:"), depth);
            print_ast_node(ret_stmt.as_ref(), depth + 1)
        },
        ASTNode::BlockStatement(block) => {
            print_at_depth("Block:".to_string(), depth);
            for stmt in block {
                print_ast_node(stmt, depth + 1)
            }
        },
        ASTNode::FunctionDefinition(func) => {
            print_at_depth(format!("Function: {}", func.name), depth);
            for stmt in &func.body {
                print_ast_node(stmt, depth + 1)
            }
        }
        ASTNode::UnaryOperation(unar) => {
            print_at_depth(format!("Unary operation: {}", unar.operator), depth);
            print_ast_node(&unar.operand, depth + 1);
        }
    }
}
