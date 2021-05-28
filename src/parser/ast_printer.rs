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

            if func.params.len() > 0 {
                print_at_depth("Parameters:".to_string(), depth + 1);
                for param in &func.params {
                    print_at_depth(format!("- \"{}\"", param), depth + 2);
                }
            }

            if let Some(body) = &func.body {
                print_at_depth("Body:".to_string(), depth + 1);
                for stmt in body {
                    print_ast_node(stmt, depth + 2)
                }
            }
        }
        ASTNode::UnaryOperation(unar) => {
            print_at_depth(format!("Unary operation: {}", unar.operator), depth);
            print_ast_node(&unar.operand, depth + 1);
        }
        ASTNode::BinaryOperation(bin) => {
            print_at_depth(format!("Binary operation: {}", bin.operator), depth);
            print_ast_node(&bin.left_side, depth + 1);
            print_ast_node(&bin.right_side, depth + 1)
        }
        ASTNode::VariableDeclaration(var) => {
            print_at_depth(format!("Variable declaration: {}", var.identifier), depth);
            if let Some(val) = &var.initial_value {
                print_ast_node(val, depth + 1);
            }
        }
        ASTNode::IfStatement(if_stmt) => {
            print_at_depth("If statement:".to_string(), depth);
            print_at_depth("Condition:".to_string(), depth + 1);
            print_ast_node(&if_stmt.condition, depth + 2);
            print_at_depth("Body:".to_string(), depth + 1);
            print_ast_node(&if_stmt.body, depth + 2);
            if let Some(else_stmt) = &if_stmt.else_stmt {
                print_at_depth("Else:".to_string(), depth + 1);
                print_ast_node(else_stmt, depth + 2);
            }
        },
        ASTNode::FunctionCall(func_call) => {
            print_at_depth(format!("Function call: {}", func_call.name), depth);
            for arg in &func_call.args {
                print_ast_node(arg, depth + 1);
            }
        }
    }
}
