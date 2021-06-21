// This allows us to try and get constants from sets of ASTNodes
// Useful, for example, for emmitting compile-time constant globals.
use crate::parser::ast_utils::{ASTNode, ASTBinaryOperation};

pub fn get_constant_value_from_node(node: &ASTNode) -> isize {
    match node {
        ASTNode::IntegerLiteral(int) => *int,
        ASTNode::BinaryOperation(bin) => resolve_binary_operation(bin),
        _ => panic!("Constant propagation was not sophisticated enough to determine a value.
OR you attempted to declare a constant value with a non-constant expression.")
    }
}

fn resolve_binary_operation (bin: &ASTBinaryOperation) -> isize {
    let left = get_constant_value_from_node(&bin.left_side);
    let right = get_constant_value_from_node(&bin.right_side);
    match &bin.operator[..] {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        "%" => left % right,
        _ => panic!("Binary operator {} unknown to constant propagation", bin.operator)
    }
}
