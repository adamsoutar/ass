// This will get quite a bit more complicated when we introduce structs etc.

#[derive(Clone, PartialEq)]
pub enum Type {
    Char(IntegerTypeMetadata), // 8-bit
    Short(IntegerTypeMetadata), // 16-bit
    Int(IntegerTypeMetadata), // 32-bit (for both int and long int)
    LongLongInt(IntegerTypeMetadata), // 64-bit
    Pointer(PointerTypeMetadata)
}

#[derive(Clone, PartialEq)]
pub struct IntegerTypeMetadata {
    pub signed: bool,
}

#[derive(Clone, PartialEq)]
pub struct PointerTypeMetadata {
    pub points_to: Box<Type>
}


/*
 Eg: char** -> Pointer({ points_to: Pointer({ points_to: Char({ signed: true }) }) })
*/
