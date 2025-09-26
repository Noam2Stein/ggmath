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
