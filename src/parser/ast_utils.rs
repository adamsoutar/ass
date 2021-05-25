#[derive(Clone, PartialEq)]
pub enum ASTNode {
    IntegerLiteral(isize),
    Identifier(String),
    ReturnStatement(Box<ASTNode>),
    BlockStatement(Vec<ASTNode>),
    FunctionDefinition(ASTFunctionDefinition),
    UnaryOperation(ASTUnaryOperation),
    BinaryOperation(ASTBinaryOperation),
    VariableDeclaration(ASTVariableDeclaration),
}

#[derive(Clone, PartialEq)]
pub struct ASTFunctionDefinition {
    pub name: String,
    pub body: Vec<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct ASTUnaryOperation {
    pub operator: String,
    pub operand: Box<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct ASTBinaryOperation {
    pub left_side: Box<ASTNode>,
    pub operator: String,
    pub right_side: Box<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct ASTVariableDeclaration {
    pub identifier: String,
    pub initial_value: Option<Box<ASTNode>>
}
