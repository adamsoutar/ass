use crate::parser::ast_utils::*;
use crate::parser::ast_printer::print_ast_node;

// AMD64 assembly codegen

pub struct Codegen {
    pub ast: Vec<ASTNode>,
    pub generated: String
}

impl Codegen {
    pub fn generate (&mut self) {
        self.generated = String::from("");

        let first_node = self.ast[0].clone();
        let main_func = match first_node {
            ASTNode::FunctionDefinition(func) => func,
            _ => panic!("Only a main() declaration is supported")
        };

        self.emit_program_prologue();
        self.emit_for_block(&main_func.body)
    }

    fn emit_for_block (&mut self, block: &Vec<ASTNode>) {
        for node in block { self.emit_for_node(node) }
    }

    fn emit_for_node (&mut self, node: &ASTNode) {
        match node {
            ASTNode::IntegerLiteral(int) => {
                self.emit(format!("movl ${}, %eax", int))
            },
            ASTNode::ReturnStatement(ret) => {
                // Value will be in EAX
                self.emit_for_node(&ret);
                self.emit_str("ret")
            },
            ASTNode::UnaryOperation(unar) => {
                self.emit_for_unary_operator(unar)
            },
            _ => {
                print_ast_node(node, 0);
                panic!("Node not supported in codegen")
            }
        }
    }

    fn emit_for_unary_operator (&mut self, unar: &ASTUnaryOperation) {
        self.emit_for_node(&unar.operand);

        match &unar.operator[..] {
            "-" => {
                self.emit_str("neg %eax")
            },
            "~" => {
                self.emit_str("not %eax")
            },
            "!" => {
                self.emit_str("cmpl $0, %eax");
                self.emit_str("movl $0, %eax");
                self.emit_str("setz %al");
            },
            _ => panic!("Codegen unimplemented for unary operator \"{}\"", unar.operator)
        }
    }

    fn emit_program_prologue (&mut self) {
        // NOTE: "_main" is a macOS thing.
        // For Linux support, just use "main"
        self.emit(".globl _main".to_string());
        self.emit("_main:".to_string())
    }

    fn emit_str (&mut self, st: &str) {
        self.emit(st.to_string())
    }

    fn emit (&mut self, st: String) {
        self.generated = format!("{}{}\n", self.generated, st)
    }

    pub fn new (ast: Vec<ASTNode>) -> Codegen {
        Codegen {
            ast,
            generated: String::from("")
        }
    }
}
