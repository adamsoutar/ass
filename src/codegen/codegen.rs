use std::cmp::min;
use std::collections::hash_map::HashMap;
use crate::parser::ast_utils::*;
use crate::parser::tokens::*;
use crate::parser::ast_printer::print_ast_node;

// AMD64 assembly codegen

// In the x86-64 System V calling convention macOS uses,
// args are passed in certain registers and then on the stack
static ARGUMENT_LOCATIONS: &[&str] = &[
    "%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"
];
static MAX_ARGS: usize = ARGUMENT_LOCATIONS.len();

pub struct Codegen {
    pub ast: Vec<ASTNode>,
    pub generated: String,
    // Used to generate unique assembly jump labels
    pub label_counter: usize,
    // A stack of hashmaps of local var names to stack offsets
    pub var_context: Vec<HashMap<String, isize>>,
    pub stack_offset: isize
}

impl Codegen {
    pub fn generate (&mut self) {
        self.generated = String::from("");

        for node in self.ast.clone() {
            self.emit_for_node(&node)
        }
    }

    fn emit_for_block (&mut self, block: &Vec<ASTNode>) {
        self.begin_var_scope();
        for node in block { self.emit_for_node(node) }
        self.end_var_scope();
    }

    fn emit_for_node (&mut self, node: &ASTNode) {
        match node {
            ASTNode::IntegerLiteral(int) => {
                self.emit(format!("movl ${}, %eax", int))
            },
            ASTNode::ReturnStatement(ret) => {
                self.emit_for_node(&ret);
                self.emit_function_epilogue(false);
            },
            ASTNode::UnaryOperation(unar) => {
                self.emit_for_unary_operation(unar)
            },
            ASTNode::BinaryOperation(bin) => {
                self.emit_for_binary_operation(bin)
            },
            ASTNode::VariableDeclaration(var) => {
                self.emit_for_variable_declaration(var)
            },
            ASTNode::Identifier(ident) => {
                let offset = self.find_var(ident);
                self.emit(format!("movq {}(%rbp), %rax", offset))
            }
            ASTNode::BlockStatement(stmts) => {
                self.emit_for_block(stmts)
            },
            ASTNode::IfStatement(if_stmt) => {
                self.emit_for_if_statement(if_stmt)
            },
            ASTNode::FunctionDefinition(func) => {
                self.emit_for_function_definition(func)
            },
            ASTNode::FunctionCall(func_call) => {
                self.emit_for_function_call(func_call)
            },
            _ => {
                print_ast_node(node, 0);
                panic!("Node not supported in codegen")
            }
        }
    }

    fn emit_for_function_call (&mut self, func_call: &ASTFunctionCall) {
        // TODO: On macOS, the stack needs to be 16-bit aligned before calling
        //       a function.

        // First 6 args are put into registers
        let reg_args = min(func_call.args.len(), MAX_ARGS);
        for i in 0..reg_args {
            let arg = &func_call.args[i];
            let arg_loc = ARGUMENT_LOCATIONS[i];
            self.emit_for_node(arg);
            self.emit(format!("movq %rax, {}", arg_loc));
        }

        // Additional args are pushed on to the stack in reverse order
        for i in (MAX_ARGS..func_call.args.len()).rev() {
            let arg = &func_call.args[i];
            self.emit_for_node(arg);
            self.emit_str("push %rax");
        }

        self.emit(format!("call _{}", func_call.name));
    }

    fn emit_for_function_definition (&mut self, func: &ASTFunctionDefinition) {
        self.emit(format!(".globl _{}", func.name));

        if let Some(body) = &func.body {
            self.emit(format!("_{}:", func.name));
            self.emit_function_prologue();

            // Alloc arguments
            self.var_context.push(HashMap::new());

            let reg_args = min(func.params.len(), MAX_ARGS);
            for i in 0..reg_args {
                let arg = &func.params[i];
                let arg_loc = ARGUMENT_LOCATIONS[i];
                self.emit_var_alloc_from_location(arg, arg_loc);
            }

            // Additional args are in the stack in reverse order
            let mut offset: isize = 16;
            for i in (MAX_ARGS..func.params.len()).rev() {
                let arg = &func.params[i];
                self.var_alloc_from_arbitrary_offset(arg, offset);
                offset += 8;
            }

            self.emit_for_block(body);

            // Dealloc args
            self.end_var_scope_without_dealloc();

            self.emit_function_epilogue(true);
        }
    }

    fn emit_for_if_statement (&mut self, if_stmt: &ASTIfStatement) {
        self.emit_for_node(&if_stmt.condition);
        self.emit_str("cmpl $0, %eax");

        let skip_label = self.get_unique_label("if_skip");
        let else_label = self.get_unique_label("else");
        self.emit(format!("je {}", else_label));

        self.emit_for_node(&if_stmt.body);
        self.emit(format!("jmp {}", skip_label));

        self.emit(format!("{}:", else_label));

        if let Some(else_node) = &if_stmt.else_stmt {
            self.emit_for_node(else_node);
        }

        self.emit(format!("{}:", skip_label));
    }

    fn emit_for_variable_declaration (&mut self, var: &ASTVariableDeclaration) {
        // If there's an initial value, we'll put it in eax, if not we'll
        // shove whatever random vallue we were last using in there (it's UB)
        if let Some(init) = &var.initial_value {
            self.emit_for_node(init);
        }

        self.emit_var_alloc_from_eax(&var.identifier);
    }

    fn emit_for_binary_operation (&mut self, bin: &ASTBinaryOperation) {
        if is_binary_stack_operator(&bin.operator) {
            // Emit stack precursor
            self.emit_for_node(&bin.left_side);
            self.emit_str("push %rax");
            self.emit_for_node(&bin.right_side);
            self.emit_str("pop %rcx");

            match &bin.operator[..] {
                "+" => self.emit_str("addl %ecx, %eax"),
                "-" => {
                    self.emit_str("subl %eax, %ecx");
                    self.emit_str("movl %ecx, %eax")
                },
                "*" => self.emit_str("imul %ecx, %eax"),
                "/" => {
                    self.emit_str("movl %eax, %r8d");
                    self.emit_str("movl %ecx, %eax");
                    self.emit_str("cdq");
                    self.emit_str("idivl %r8d");
                },
                "==" => {
                    self.emit_for_comparison_precursor();
                    self.emit_str("sete %al");
                },
                "!=" => {
                    self.emit_for_comparison_precursor();
                    self.emit_str("setne %al");
                },
                ">" => {
                    self.emit_for_comparison_precursor();
                    self.emit_str("setg %al");
                },
                "<" => {
                    self.emit_for_comparison_precursor();
                    self.emit_str("setl %al");
                },
                ">=" => {
                    self.emit_for_comparison_precursor();
                    self.emit_str("setge %al");
                },
                "<=" => {
                    self.emit_for_comparison_precursor();
                    self.emit_str("setle %al");
                },
                _ => unimplemented!("\"{}\" stack operator", bin.operator)
            }

            return;
        }

        match &bin.operator[..] {
            // Short-circuiting OR implementation
            "||" => {
                self.emit_for_node(&bin.left_side);

                let skip_label = self.get_unique_label("skip");
                let end_label = self.get_unique_label("end");

                self.emit_str("cmpl $0, %eax");
                // If exp1 was false, we need to jump to evaluating exp2
                self.emit(format!("je {}", skip_label));

                // Otherwise, we set eax to true and skip to the end
                self.emit_str("movl $1, %eax");
                self.emit(format!("jmp {}", end_label));

                self.emit(format!("{}:", skip_label));
                self.emit_for_node(&bin.right_side);

                // Now we compare eax to 0 and set eax to the opposite of that comparison
                self.emit_str("cmpl $0, %eax");
                self.emit_str("movl $0, %eax");
                self.emit_str("setne %al");

                self.emit(format!("{}:", end_label));
            },
            // Short circuiting AND implementation, very similar to OR
            "&&" => {
                self.emit_for_node(&bin.left_side);

                let skip_label = self.get_unique_label("skip");
                let end_label = self.get_unique_label("end");

                self.emit_str("cmpl $0, %eax");
                self.emit(format!("jne {}", skip_label));
                self.emit(format!("jmp {}", end_label));

                self.emit(format!("{}:", skip_label));
                self.emit_for_node(&bin.right_side);

                self.emit_str("cmpl $0, %eax");
                self.emit_str("movl $0, %eax");
                self.emit_str("setne %al");

                self.emit(format!("{}:", end_label))
            },
            // Assignemnts (remember these are expressions with a value!)
            "=" => {
                let ass_offset = self.find_assignable(&bin.left_side);
                self.emit_for_node(&bin.right_side);
                self.emit(format!("movq %rax, {}(%rbp)", ass_offset));
            },
            _ => unimplemented!("\"{}\" non-stack operator", bin.operator)
        }
    }

    fn find_assignable (&self, node: &ASTNode) -> isize {
        match &node {
            ASTNode::Identifier(ident) => self.find_var(ident),
            _ => panic!("Cannot resolve non-identifier assignable")
        }
    }

    fn emit_for_comparison_precursor (&mut self) {
        self.emit_str("cmpl %eax, %ecx");
        self.emit_str("movl $0, %eax");
    }

    fn emit_for_unary_operation (&mut self, unar: &ASTUnaryOperation) {
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

    fn emit_function_prologue (&mut self) {
        // Save the old base pointer
        self.emit_str("push %rbp");
        // The stack head is the new base
        self.emit_str("movq %rsp, %rbp");
    }

    fn emit_function_epilogue (&mut self, gen_return_value: bool) {
        if gen_return_value {
            // Functions without a return statement return 0
            self.emit_str("movq $0, %rax");
        }

        // Stack head is the base
        self.emit_str("movq %rbp, %rsp");
        // Restore the old base
        self.emit_str("pop %rbp");
        // Jump out of func
        self.emit_str("ret");
    }

    fn get_unique_label (&mut self, comment: &str) -> String {
        self.label_counter += 1;
        format!("_{}_{}", comment, self.label_counter)
    }

    pub fn emit_str (&mut self, st: &str) {
        self.emit(st.to_string())
    }

    pub fn emit (&mut self, st: String) {
        self.generated = format!("{}{}\n", self.generated, st)
    }

    pub fn new (ast: Vec<ASTNode>) -> Codegen {
        Codegen {
            ast,
            generated: String::from(""),
            label_counter: 0,
            var_context: vec![],
            stack_offset: 0
        }
    }
}
