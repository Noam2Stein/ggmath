// Length

pub const LENGTHS: &[usize] = &[2, 3, 4];
pub const LENGTH_NAMES: [&str; LENGTHS.len()] = ["Two", "Three", "Four"];
pub const COMPONENTS: [&str; MAX_LENGTH] = ["x", "y", "z", "w"];
pub const COMPONENT_ORDINALS: [&str; MAX_LENGTH] = ["1st", "2nd", "3rd", "4th"];

pub const MAX_LENGTH: usize = LENGTHS[LENGTHS.len() - 1];

// Primitives

pub const PRIMITIVES: &[&str] = &[
    "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
    "usize", "bool",
];
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

// Operators

pub const UNARY_OPS: &[&str] = &["Neg", "Not"];

pub const BINARY_OPS: &[&str] = &[
    "Add", "Sub", "Mul", "Div", "Rem", "Shl", "Shr", "BiAnd", "BiOr", "BiXor",
];
