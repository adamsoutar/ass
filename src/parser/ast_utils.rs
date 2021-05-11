#[derive(Clone, PartialEq)]
pub enum ASTNode {
    IntegerLiteral(isize),
    Identifier(String),
    ReturnStatement(Box<ASTNode>),
    BlockStatement(Vec<ASTNode>),
    FunctionDefinition(ASTFunctionDefinition)
}

#[derive(Clone, PartialEq)]
pub struct ASTFunctionDefinition {
    pub name: String,
    pub body: Box<ASTNode>,
    // Unused for now, everything is an int
    // pub returnType: Box<ASTNode>
}
