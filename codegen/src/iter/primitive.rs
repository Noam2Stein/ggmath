use std::{fmt::Display, iter::once};

use genco::{lang::Lang, tokens::FormatInto};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Primitive {
    Float(Float),
    Int(Int),
    Bool,
    Char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Float {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Int {
    Sint(Sint),
    Uint(Uint),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Sint {
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Uint {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

impl Primitive {
    pub fn iter() -> impl Iterator<Item = Self> {
        Float::iter()
            .map(Self::Float)
            .chain(Int::iter().map(Self::Int))
            .chain(once(Self::Bool))
            .chain(once(Self::Char))
    }

    pub fn snakecase(&self) -> &'static str {
        match self {
            Self::Float(float) => float.snakecase(),
            Self::Int(int) => int.snakecase(),
            Self::Bool => "bool",
            Self::Char => "char",
        }
    }

    #[expect(unused)]
    pub fn camelcase(&self) -> &'static str {
        match self {
            Self::Float(float) => float.camelcase(),
            Self::Int(int) => int.camelcase(),
            Self::Bool => "Bool",
            Self::Char => "Char",
        }
    }

    #[expect(unused)]
    pub fn lowercase_prefix(&self) -> &'static str {
        match self {
            Self::Float(float) => float.lowercase_prefix(),
            Self::Int(int) => int.lowercase_prefix(),
            Self::Bool => "b",
            Self::Char => "c",
        }
    }
}

impl Float {
    pub fn snakecase(&self) -> &'static str {
        match self {
            Self::F32 => "f32",
            Self::F64 => "f64",
        }
    }

    pub fn camelcase(&self) -> &'static str {
        match self {
            Self::F32 => "F32",
            Self::F64 => "F64",
        }
    }

    pub fn lowercase_prefix(&self) -> &'static str {
        match self {
            Self::F32 => "f",
            Self::F64 => "d",
        }
    }
}

impl Int {
    pub fn iter() -> impl Iterator<Item = Self> {
        Sint::iter()
            .map(Self::Sint)
            .chain(Uint::iter().map(Self::Uint))
    }

    pub fn snakecase(&self) -> &'static str {
        match self {
            Self::Sint(sint) => sint.snakecase(),
            Self::Uint(uint) => uint.snakecase(),
        }
    }

    pub fn camelcase(&self) -> &'static str {
        match self {
            Self::Sint(sint) => sint.camelcase(),
            Self::Uint(uint) => uint.camelcase(),
        }
    }

    pub fn lowercase_prefix(&self) -> &'static str {
        match self {
            Self::Sint(sint) => sint.lowercase_prefix(),
            Self::Uint(uint) => uint.lowercase_prefix(),
        }
    }
}

impl Sint {
    pub fn snakecase(&self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::Isize => "isize",
        }
    }

    pub fn camelcase(&self) -> &'static str {
        match self {
            Self::I8 => "I8",
            Self::I16 => "I16",
            Self::I32 => "I32",
            Self::I64 => "I64",
            Self::I128 => "I128",
            Self::Isize => "Isize",
        }
    }

    pub fn lowercase_prefix(&self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::Isize => "isize",
        }
    }
}

impl Uint {
    pub fn snakecase(&self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::Usize => "usize",
        }
    }

    pub fn camelcase(&self) -> &'static str {
        match self {
            Self::U8 => "U8",
            Self::U16 => "U16",
            Self::U32 => "U32",
            Self::U64 => "U64",
            Self::U128 => "U128",
            Self::Usize => "Usize",
        }
    }

    pub fn lowercase_prefix(&self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::Usize => "usize",
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(float) => write!(f, "{float}"),
            Self::Int(int) => write!(f, "{int}"),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),
        }
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.snakecase())
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.snakecase())
    }
}

impl Display for Sint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.snakecase())
    }
}

impl Display for Uint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.snakecase())
    }
}

impl<L: Lang> FormatInto<L> for Primitive {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.snakecase());
    }
}

impl<L: Lang> FormatInto<L> for Float {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.snakecase());
    }
}

impl<L: Lang> FormatInto<L> for Int {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.snakecase());
    }
}

impl<L: Lang> FormatInto<L> for Sint {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.snakecase());
    }
}

impl<L: Lang> FormatInto<L> for Uint {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.snakecase());
    }
}
