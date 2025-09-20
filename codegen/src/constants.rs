// Length

pub const LENGTHS: &[usize] = &[2, 3, 4];
pub const LENGTH_NAMES: [&str; LENGTHS.len()] = ["Two", "Three", "Four"];
pub const COMPONENTS: [&str; MAX_LENGTH] = ["x", "y", "z", "w"];
pub const COMPONENT_ORDINALS: [&str; MAX_LENGTH] = ["1st", "2nd", "3rd", "4th"];

pub const MAX_LENGTH: usize = LENGTHS[LENGTHS.len() - 1];

// Directions

pub const DIRECTIONS_A: &[&str] = &["Right", "Up", "Forwards"];
pub const DIRECTIONS_B: &[&str] = &["Left", "Down", "Backwards"];

// Primitives

pub const PRIMITIVES: &[&str] = &[
    "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
    "usize", "bool",
];
pub const PRIMARY_PRIMITIVES: &[&str] = &["f32", "i32", "u32", "bool"];
pub const NUM_PRIMITIVES: &[&str] = &[
    "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
    "usize",
];
pub const FLOAT_PRIMITIVES: &[&str] = &["f32", "f64"];
pub const INT_PRIMITIVES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
];
pub const SINT_PRIMITIVES: &[&str] = &["i8", "i16", "i32", "i64", "i128", "isize"];
pub const UINT_PRIMITIVES: &[&str] = &["u8", "u16", "u32", "u64", "u128", "usize"];

pub const PRIMITIVE_PREFIXES: [&str; PRIMITIVES.len()] = [
    "F", "D", "I8", "I16", "I", "I64", "I128", "Isize", "U8", "U16", "U", "U64", "U128", "Usize",
    "B",
];

// Operators

pub const UNARY_OPS: &[&str] = &["Neg", "Not"];

pub const BINARY_OPS: &[&str] = &[
    "Add", "Sub", "Mul", "Div", "Rem", "Shl", "Shr", "BitAnd", "BitOr", "BitXor",
];

pub const COMPARISON_OPS: &[&str] = &["eq", "ne", "lt", "le", "gt", "ge"];
pub const COMPARISON_OP_TOKENS: [&str; COMPARISON_OPS.len()] = ["==", "!=", "<", "<=", ">", ">="];
pub const COMPARISON_OP_TRAITS: [&str; COMPARISON_OPS.len()] = [
    "PartialEq",
    "PartialEq",
    "PartialOrd",
    "PartialOrd",
    "PartialOrd",
    "PartialOrd",
];
pub const COMPARISON_OP_DOCS: [&str; COMPARISON_OPS.len()] = [
    "equal to",
    "not equal to",
    "less than",
    "less than or equal to",
    "greater than",
    "greater than or equal to",
];
