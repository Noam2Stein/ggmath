use std::fmt::Display;

use genco::{Tokens, lang::Lang, tokens::FormatInto};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Primitive {
    Float(PrimitiveFloat),
    Int(PrimitiveInt),
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum PrimitiveFloat {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveInt {
    Sint(PrimitiveSint),
    Uint(PrimitiveUint),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum PrimitiveSint {
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum PrimitiveUint {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

#[expect(dead_code)]
impl Primitive {
    pub fn as_float(&self) -> Option<PrimitiveFloat> {
        match self {
            Primitive::Float(primitive) => Some(*primitive),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<PrimitiveInt> {
        match self {
            Primitive::Int(primitive) => Some(*primitive),
            _ => None,
        }
    }

    pub fn as_sint(&self) -> Option<PrimitiveSint> {
        match self {
            Primitive::Int(PrimitiveInt::Sint(primitive)) => Some(*primitive),
            _ => None,
        }
    }

    pub fn as_uint(&self) -> Option<PrimitiveUint> {
        match self {
            Primitive::Int(PrimitiveInt::Uint(primitive)) => Some(*primitive),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Primitive::Float(_))
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Primitive::Int(_))
    }

    pub fn is_sint(&self) -> bool {
        matches!(self, Primitive::Int(PrimitiveInt::Sint(_)))
    }

    pub fn is_uint(&self) -> bool {
        matches!(self, Primitive::Int(PrimitiveInt::Uint(_)))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Primitive::Bool)
    }

    pub fn is_primary(self) -> bool {
        match self {
            Primitive::Float(float) => float == PrimitiveFloat::F32,
            Primitive::Int(int) => int.is_primary(),
            Primitive::Bool => true,
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        PrimitiveFloat::iter()
            .map(Primitive::Float)
            .chain(PrimitiveInt::iter().map(Primitive::Int))
            .chain(std::iter::once(Primitive::Bool))
    }

    pub fn prefix_uppercase(&self) -> &'static str {
        match self {
            Primitive::Float(primitive) => primitive.prefix_uppercase(),
            Primitive::Int(primitive) => primitive.prefix_uppercase(),
            Primitive::Bool => "B",
        }
    }
}

impl PrimitiveFloat {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }

    pub fn prefix_uppercase(&self) -> &'static str {
        match self {
            PrimitiveFloat::F32 => "F",
            PrimitiveFloat::F64 => "D",
        }
    }
}

#[expect(dead_code)]
impl PrimitiveInt {
    pub fn as_sint(&self) -> Option<PrimitiveSint> {
        match self {
            PrimitiveInt::Sint(primitive) => Some(*primitive),
            _ => None,
        }
    }

    pub fn as_uint(&self) -> Option<PrimitiveUint> {
        match self {
            PrimitiveInt::Uint(primitive) => Some(*primitive),
            _ => None,
        }
    }

    pub fn is_sint(&self) -> bool {
        matches!(self, PrimitiveInt::Sint(_))
    }

    pub fn is_uint(&self) -> bool {
        matches!(self, PrimitiveInt::Uint(_))
    }

    pub fn is_primary(self) -> bool {
        match self {
            PrimitiveInt::Sint(sint) => sint == PrimitiveSint::I32,
            PrimitiveInt::Uint(uint) => uint == PrimitiveUint::U32,
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        PrimitiveSint::iter()
            .map(PrimitiveInt::Sint)
            .chain(PrimitiveUint::iter().map(PrimitiveInt::Uint))
    }

    pub fn prefix_uppercase(&self) -> &'static str {
        match self {
            PrimitiveInt::Sint(primitive) => primitive.prefix_uppercase(),
            PrimitiveInt::Uint(primitive) => primitive.prefix_uppercase(),
        }
    }
}

impl PrimitiveSint {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }

    pub fn prefix_uppercase(&self) -> &'static str {
        match self {
            PrimitiveSint::I8 => "I8",
            PrimitiveSint::I16 => "I16",
            PrimitiveSint::I32 => "I",
            PrimitiveSint::I64 => "I64",
            PrimitiveSint::I128 => "I128",
            PrimitiveSint::Isize => "Isize",
        }
    }
}

impl PrimitiveUint {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }

    pub fn prefix_uppercase(&self) -> &'static str {
        match self {
            PrimitiveUint::U8 => "U8",
            PrimitiveUint::U16 => "U16",
            PrimitiveUint::U32 => "U",
            PrimitiveUint::U64 => "U64",
            PrimitiveUint::U128 => "U128",
            PrimitiveUint::Usize => "Usize",
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Float(primitive) => write!(f, "{primitive}"),
            Primitive::Int(primitive) => write!(f, "{primitive}"),
            Primitive::Bool => write!(f, "bool"),
        }
    }
}

impl Display for PrimitiveFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveFloat::F32 => write!(f, "f32"),
            PrimitiveFloat::F64 => write!(f, "f64"),
        }
    }
}

impl Display for PrimitiveInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveInt::Sint(primitive) => write!(f, "{primitive}"),
            PrimitiveInt::Uint(primitive) => write!(f, "{primitive}"),
        }
    }
}

impl Display for PrimitiveSint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveSint::I8 => write!(f, "i8"),
            PrimitiveSint::I16 => write!(f, "i16"),
            PrimitiveSint::I32 => write!(f, "i32"),
            PrimitiveSint::I64 => write!(f, "i64"),
            PrimitiveSint::I128 => write!(f, "i128"),
            PrimitiveSint::Isize => write!(f, "isize"),
        }
    }
}

impl Display for PrimitiveUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveUint::U8 => write!(f, "u8"),
            PrimitiveUint::U16 => write!(f, "u16"),
            PrimitiveUint::U32 => write!(f, "u32"),
            PrimitiveUint::U64 => write!(f, "u64"),
            PrimitiveUint::U128 => write!(f, "u128"),
            PrimitiveUint::Usize => write!(f, "usize"),
        }
    }
}

impl<L: Lang> FormatInto<L> for Primitive {
    fn format_into(self, tokens: &mut Tokens<L>) {
        self.to_string().format_into(tokens);
    }
}

impl<L: Lang> FormatInto<L> for PrimitiveFloat {
    fn format_into(self, tokens: &mut Tokens<L>) {
        self.to_string().format_into(tokens);
    }
}

impl<L: Lang> FormatInto<L> for PrimitiveInt {
    fn format_into(self, tokens: &mut Tokens<L>) {
        self.to_string().format_into(tokens);
    }
}

impl<L: Lang> FormatInto<L> for PrimitiveSint {
    fn format_into(self, tokens: &mut Tokens<L>) {
        self.to_string().format_into(tokens);
    }
}

impl<L: Lang> FormatInto<L> for PrimitiveUint {
    fn format_into(self, tokens: &mut Tokens<L>) {
        self.to_string().format_into(tokens);
    }
}
