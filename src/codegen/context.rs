// Methods for finding variables
use std::collections::hash_map::HashMap;
use crate::codegen::codegen::Codegen;

impl Codegen {
    pub fn find_var (&self, name: &String) -> usize {
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
        self.var_context.pop();
    }

    pub fn emit_var_alloc_from_eax (&mut self, name: &String) {
        self.emit_str("push %rax");
        self.stack_offset += 8;

        let latest = self.var_context.len() - 1;
        self.var_context[latest].insert(name.clone(), self.stack_offset);
    }
}
