// Methods for finding variables
use std::collections::hash_map::HashMap;
use super::codegen::Codegen;
use super::stored_value::StoredValue;

impl Codegen {
    pub fn find_var (&self, name: &String) -> &StoredValue {
        for map in self.var_context.iter().rev() {
            if map.contains_key(name) {
                return &map[name];
            }
        }

        panic!("Unresolved variable reference \"{}\"", name)
    }

    pub fn begin_var_scope (&mut self) {
        // println!(" = SCOPE BEGAN = ");
        self.var_context.push(HashMap::new());
    }

    // This is emitted when curly braces end
    pub fn end_compiletime_var_scope (&mut self) {
        // println!(" = COMPILETIME SCOPE ENDED = ");
        self.var_context.pop();
    }
    // This is emitted when a function returns at any time
    // Depth is how many compound statements deep we are at the time of returning.
    // Eg. in an if statement within a function is depth: 2
    pub fn end_runtime_var_scope (&mut self, mutate_stack_offset: bool) {
        let scope = &self.var_context[self.var_context.len() - 1];
        // Each stack item is 64 bits (8 bytes)
        // We need to dealloc this scope by moving up
        // the stack pointer so future vars are alloced higher
        let dealloc_bytes = scope.len() * 8;
        // println!("In this scope (stack_offset is {}):", self.stack_offset);
        // for varname in scope {
        //     println!(" - {} ({})", varname.0, varname.1);
        // }

        self.emit(format!("addq ${}, %rsp", dealloc_bytes));
        if mutate_stack_offset {
            self.stack_offset += dealloc_bytes as isize;
            // println!("runtime var stack deflation deallocced {} bytes ({})", dealloc_bytes, self.stack_offset);
        }

        // println!(" = RUNTIME SCOPE ENDED = (mutate: {})", mutate_stack_offset);
    }

    pub fn emit_global_alloc_from_constant (&mut self, name: &String, value: isize) {
        let label = self.get_global_var_label(name);
        self.emit(format!(".globl {}", label));
        self.emit_str(".data");
        self.emit_str(".align 3"); // 2 to the power of 3, 8 bytes - 64 bit
        self.emit(format!("{}:", label));
        self.emit(format!(".quad {}", value));
        self.emit_str(".text");

        let latest = self.var_context.len() - 1;
        let map = &mut self.var_context[latest];
        map.insert(name.clone(), StoredValue::Global(name.clone()));
    }

    pub fn stack_alloc_from_arbitrary_offset (&mut self, name: &String, offset: isize) {
        let latest = self.var_context.len() - 1;
        let map = &mut self.var_context[latest];

        map.insert(name.clone(), StoredValue::Stack(offset));
    }

    pub fn emit_stack_alloc_from_location(&mut self, name: &String, location: &str) {
        self.emit(format!("push {}", location));
        self.stack_offset -= 8;
        // println!("alloc from {} as \"{}\" ({})", location, name, self.stack_offset);

        let latest = self.var_context.len() - 1;
        let map = &mut self.var_context[latest];

        if map.contains_key(name) {
            panic!("Redefinition of \"{}\" in the same scope", name);
        }

        map.insert(name.clone(), StoredValue::Stack(self.stack_offset));
    }

    pub fn emit_stack_alloc_from_rax (&mut self, name: &String) {
        self.emit_stack_alloc_from_location(name, "%rax")
    }

    // NOTE: If you want to align when you're just about to push
    // some new stuff, but that stuff needs to be on top, provide
    // the future bytes eg. -8
    pub fn align_stack (&mut self, future_bytes: isize) {
        let total = self.stack_offset + future_bytes;
        if total % 16 != 0 {
            self.counter += 1;
            self.emit_stack_alloc_from_location(&format!("__ASS_ALIGN_{}", self.counter), "$0");
        }
    }

    pub fn get_global_var_label (&self, name: &String) -> String {
        // This isn't technically requried, we're just making sure you don't
        // accidentally name your variable after a jump label ass happens to
        // be using, as unlikely as that is.
        format!("_GLOBAL_VAR_{}", name)
    }
}
