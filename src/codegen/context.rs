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
    pub fn end_var_scope (&mut self) {
        let popped_scope = self.var_context.pop().unwrap();
        // Each stack item is 64 bits (8 bytes)
        // We need to dealloc this scope by moving up
        // the stack pointer so future vars are alloced higher
        let dealloc_bytes = popped_scope.len() * 8;
        self.emit(format!("addq ${}, %rsp", dealloc_bytes));
        self.stack_offset += dealloc_bytes as isize;
    }

    pub fn emit_var_alloc_from_eax (&mut self, name: &String) {
        self.emit_str("push %rax");
        self.stack_offset -= 8;

        let latest = self.var_context.len() - 1;
        let map = &mut self.var_context[latest];

        if map.contains_key(name) {
            panic!("Redefinition of \"{}\" in the same scope", name);
        }

        map.insert(name.clone(), self.stack_offset);
    }
}
