// Methods for finding variables
use std::collections::hash_map::HashMap;
use crate::codegen::codegen::Codegen;

impl Codegen {
    pub fn find_var (&self, name: &String) -> isize {
        for map in self.var_context.iter().rev() {
            if map.contains_key(name) {
                return map[name];
            }
        }

        panic!("Unresolved variable reference \"{}\"", name)
    }

    pub fn begin_var_scope (&mut self) {
        self.var_context.push(HashMap::new());
    }

    // This is emitted when curly braces end
    pub fn end_compiletime_var_scope (&mut self) {
        self.var_context.pop();
    }
    // This is emitted when a function returns at any time
    // Depth is how many compound statements deep we are at the time of returning.
    // Eg. in an if statement within a function is depth: 2
    pub fn end_runtime_var_scope (&mut self, mutate_stack_offset: bool) {
        let mut dealloc_bytes = 0;

        for i in (0..self.var_context.len()).rev() {
            let scope = &self.var_context[i];
            // Each stack item is 64 bits (8 bytes)
            // We need to dealloc this scope by moving up
            // the stack pointer so future vars are alloced higher
            dealloc_bytes += scope.len() * 8;
        }

        self.emit(format!("addq ${}, %rsp", dealloc_bytes));
        if mutate_stack_offset {
            self.stack_offset += dealloc_bytes as isize;
        }
    }

    pub fn var_alloc_from_arbitrary_offset (&mut self, name: &String, offset: isize) {
        let latest = self.var_context.len() - 1;
        let map = &mut self.var_context[latest];

        map.insert(name.clone(), offset);
    }
    pub fn end_var_scope_without_dealloc (&mut self) {
        self.var_context.pop();
    }

    pub fn emit_var_alloc_from_location(&mut self, name: &String, location: &str) {
        self.emit(format!("push {}", location));
        self.stack_offset -= 8;

        let latest = self.var_context.len() - 1;
        let map = &mut self.var_context[latest];

        if map.contains_key(name) {
            panic!("Redefinition of \"{}\" in the same scope", name);
        }

        map.insert(name.clone(), self.stack_offset);
    }

    pub fn emit_var_alloc_from_eax (&mut self, name: &String) {
        self.emit_var_alloc_from_location(name, "%rax")
    }
}
