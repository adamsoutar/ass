// This allows us to try and get constants from sets of ASTNodes
// Useful, for example, for emmitting compile-time constant globals.
use crate::parser::ast_utils::ASTNode;
use super::codegen::Codegen;

impl Codegen {
    pub fn get_constant_value_from_node(&self, node: &ASTNode) -> isize {
        match node {
            ASTNode::IntegerLiteral(int) => *int,
            _ => unimplemented!("Constant propagation was not sophisticated enough to determine a value.
OR you attempted to declare a constant value with a non-constant expression.")
        }
    }
}
