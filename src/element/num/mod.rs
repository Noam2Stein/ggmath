use num_traits::*;

use super::{ops::*, *};

mod convert;
mod float;
mod int;
mod signed;
mod unsigned;
pub use convert::*;
pub use float::*;
pub use int::*;
pub use signed::*;
pub use unsigned::*;

pub trait NumElement:
    Element
    + ElementNumConvert
    + Num
    + ElementAdd
    + ElementAddAssign
    + ElementSub
    + ElementSubAssign
    + ElementMul
    + ElementMulAssign
    + ElementDiv
    + ElementDivAssign
    + ElementRem
    + ElementRemAssign
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_trait {
    ($ty:ident) => {
        impl NumElement for $ty {
            const ZERO: Self = 0 as $ty;
            const ONE: Self = 1 as $ty;
        }
    };
}
impl_trait!(f32);
impl_trait!(f64);
impl_trait!(u8);
impl_trait!(u16);
impl_trait!(u32);
impl_trait!(u64);
impl_trait!(u128);
impl_trait!(usize);
impl_trait!(i8);
impl_trait!(i16);
impl_trait!(i32);
impl_trait!(i64);
impl_trait!(i128);
impl_trait!(isize);
