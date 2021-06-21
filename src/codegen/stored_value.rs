use super::codegen::Codegen;

// These are values that can be referred to by identifiers in the scope_context
#[derive(Clone)]
pub enum StoredValue {
    Stack(isize), // Local vars as offsets from the stack ptr
    Global(String), // Global vars as assembly identifiers
}

impl Codegen {
    pub fn get_stored_value_location (&self, value: &StoredValue) -> String {
        match value {
            StoredValue::Stack(offset) => {
                format!("{}(%rbp)", offset)
            },
            StoredValue::Global(ident) => {
                let label = self.get_global_var_label(ident);
                format!("{}(%rip)", label)
            }
        }
    }

    pub fn emit_for_stored_value_access (&mut self, value: &StoredValue) {
        self.emit(format!("movq {}, %rax", self.get_stored_value_location(value)))
    }
}
