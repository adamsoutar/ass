#[derive(Clone, PartialEq)]
pub enum ASTNode {
    IntegerLiteral(isize),
    Identifier(String),
    ReturnStatement(Box<ASTNode>),
    BlockStatement(Vec<ASTNode>),
    FunctionDefinition(ASTFunctionDefinition),
    UnaryOperation(ASTUnaryOperation)
}

#[derive(Clone, PartialEq)]
pub struct ASTFunctionDefinition {
    pub name: String,
    pub body: Vec<ASTNode>,
    // Unused for now, everything is an int
    // pub returnType: Box<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct ASTUnaryOperation {
    pub operator: String,
    pub operand: Box<ASTNode>
}
