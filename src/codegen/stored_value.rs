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

    // Will emit code that puts the address of the value into %rax
    // (used for &/* operators)
    pub fn emit_load_address_of_stored_value (&mut self, value: &StoredValue) {
        match &value.backing_store {
            ValueBackingStorage::Stack(offset) => {
                self.emit_str("movq %rbp, %rax");
                if *offset > 0 {
                    self.emit(format!("addq ${}, %rax", offset));
                } else {
                    self.emit(format!("subq ${}, %rax", offset.abs()));
                }
            },
            ValueBackingStorage::Global(ident) => {
                let label = self.get_global_var_label(ident);
                self.emit(format!("lea {}(%rip), %rax", label));
            }
        }
    }

    pub fn emit_for_stored_value_access (&mut self, value: &StoredValue) {
        let loc = self.get_stored_value_location(value);
        match &value.value_type {
            Type::Char(_) => {
                // NOTE: Little-endianness on x86 means that even though we're
                // 64-bit pushing chars, we can retrieve just the top byte of the stack
                // Byte moves don't auto-clear rax's high bits
                self.emit_str("mov $0, %rax");
                self.emit(format!("movb {}, %al", loc));
            },
            Type::Short(_) => {
                self.emit_str("mov $0, %rax");
                self.emit(format!("mov {}, %ax", loc));
            },
            Type::Int(_) => {
                self.emit(format!("movl {}, %eax", loc));
            },
            Type::LongLongInt(_) => {
                self.emit(format!("movq {}, %rax", loc));
            },
            Type::Pointer(_) => {
                self.emit(format!("movq {}, %rax", loc));
            }
        };
        // self.emit(format!(format_string, self.get_stored_value_location(value)))
    }
}
