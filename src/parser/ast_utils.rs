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
    IfStatement(ASTIfStatement),
    FunctionCall(ASTFunctionCall)
}

#[derive(Clone, PartialEq)]
pub struct ASTFunctionDefinition {
    pub name: String,
    // If this option is None, it's a function declaration without an implementation
    pub body: Option<Vec<ASTNode>>,
    pub params: Vec<String>
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

#[derive(Clone, PartialEq)]
pub struct ASTIfStatement {
    pub condition: Box<ASTNode>,
    pub body: Box<ASTNode>,
    pub else_stmt: Option<Box<ASTNode>>
}

#[derive(Clone, PartialEq)]
pub struct ASTFunctionCall {
    pub name: String,
    pub args: Vec<ASTNode>
}
