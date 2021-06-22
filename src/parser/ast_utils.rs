use super::types::Type;

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
    FunctionCall(ASTFunctionCall),
    WhileLoop(ASTWhileLoop),
    ForLoop(ASTForLoop)
}

#[derive(Clone, PartialEq)]
pub struct ASTFunctionDefinition {
    pub name: String,
    pub return_type: Type,
    // If this option is None, it's a function declaration without an implementation
    pub body: Option<Vec<ASTNode>>,
    pub params: Vec<ASTFunctionParameter>
}
#[derive(Clone, PartialEq)]
pub struct ASTFunctionParameter {
    pub name: String,
    pub param_type: Type
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
    pub var_type: Type,
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

#[derive(Clone, PartialEq)]
pub struct ASTWhileLoop {
    pub condition: Box<ASTNode>,
    pub body: Box<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct ASTForLoop {
    // Run once at start of loop
    pub declaration: Option<Box<ASTNode>>,
    // Checked each iteration
    pub condition: Option<Box<ASTNode>>,
    // Run at the end(!) of every iteration
    pub modification: Option<Box<ASTNode>>,
    pub body: Box<ASTNode>
}
