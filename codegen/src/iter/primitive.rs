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
}

impl Int {
    pub fn iter() -> impl Iterator<Item = Self> {
        Sint::iter()
            .map(Self::Sint)
            .chain(Uint::iter().map(Self::Uint))
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
        match self {
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
        }
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sint(sint) => write!(f, "{sint}"),
            Self::Uint(uint) => write!(f, "{uint}"),
        }
    }
}

impl Display for Sint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::I128 => write!(f, "i128"),
            Self::Isize => write!(f, "isize"),
        }
    }
}

impl Display for Uint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::U128 => write!(f, "u128"),
            Self::Usize => write!(f, "usize"),
        }
    }
}

impl<L: Lang> FormatInto<L> for Primitive {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.to_string());
    }
}

impl<L: Lang> FormatInto<L> for Float {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.to_string());
    }
}

impl<L: Lang> FormatInto<L> for Int {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.to_string());
    }
}

impl<L: Lang> FormatInto<L> for Sint {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.to_string());
    }
}

impl<L: Lang> FormatInto<L> for Uint {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.to_string());
    }
}
