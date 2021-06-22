use super::types::Type;
use super::ast_printer::print_at_depth;
use super::types::IntegerTypeMetadata;

pub fn print_type (the_type: &Type, depth: isize) {
    match the_type {
        Type::Char(meta) => {
            print_at_depth("Type: char".to_string(), depth);
            print_int_meta(meta, depth + 1);
        },
        Type::Short(meta) => {
            print_at_depth("Type: short".to_string(), depth);
            print_int_meta(meta, depth + 1);
        },
        Type::Int(meta) => {
            print_at_depth("Type: int".to_string(), depth);
            print_int_meta(meta, depth + 1);
        },
        Type::LongLongInt(meta) => {
            print_at_depth("Type: long long int".to_string(), depth);
            print_int_meta(meta, depth + 1);
        },
        Type::Pointer(meta) => {
            print_at_depth("Type: Pointer to:".to_string(), depth);
            print_type(&meta.points_to, depth + 1);
        }
    }
}

fn print_int_meta (meta: &IntegerTypeMetadata, depth: isize) {
    let signed_string = if meta.signed { "yes" } else { "no" };
    print_at_depth(format!("Signed: {}", signed_string), depth);
}