use super::codegen::Codegen;
use crate::parser::types::Type;

// These are values that can be referred to by identifiers in the scope_context
#[derive(Clone)]
pub struct StoredValue {
    pub backing_store: ValueBackingStorage,
    pub value_type: Type
}
#[derive(Clone)]
pub enum ValueBackingStorage {
    Stack(isize), // Local vars as offsets from the stack ptr
    Global(String), // Global vars as assembly identifiers
}



impl Codegen {
    pub fn get_stored_value_location (&self, value: &StoredValue) -> String {
        match &value.backing_store {
            ValueBackingStorage::Stack(offset) => {
                format!("{}(%rbp)", offset)
            },
            ValueBackingStorage::Global(ident) => {
                let label = self.get_global_var_label(ident);
                format!("{}(%rip)", label)
            }
        }
    }

    pub fn emit_for_stored_value_access (&mut self, value: &StoredValue) {
        self.emit(format!("mov {}, %rax", self.get_stored_value_location(value)))
    }
}
