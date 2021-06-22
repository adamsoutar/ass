/*
 Eg: char** -> Pointer({ points_to: Pointer({ points_to: Char({ signed: true }) }) })
*/
// This will get quite a bit more complicated when we introduce structs etc.

#[derive(Clone, PartialEq)]
pub enum Type {
    Char(IntegerTypeMetadata), // 8-bit
    Short(IntegerTypeMetadata), // 16-bit
    Int(IntegerTypeMetadata), // 32-bit (for both int and long int)
    LongLongInt(IntegerTypeMetadata), // 64-bit
    Pointer(PointerTypeMetadata) // 64-bit pointers
}

#[derive(Clone, PartialEq)]
pub struct IntegerTypeMetadata {
    pub signed: bool,
}

#[derive(Clone, PartialEq)]
pub struct PointerTypeMetadata {
    pub points_to: Box<Type>
}

pub fn size_in_bytes (the_type: &Type) -> isize {
    match the_type {
        Type::Char(_) => 1,
        Type::Short(_) => 2,
        Type::Int(_) => 4,
        Type::LongLongInt(_) => 8,
        Type::Pointer(_) => 8
    }
}
