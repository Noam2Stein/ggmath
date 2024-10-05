use super::*;

pub use num_traits::PrimInt;

pub trait IntElement:
    NumElement
    + PrimInt
    + ElementBitAnd
    + ElementBitAndAssign
    + ElementBitOr
    + ElementBitOrAssign
    + ElementBitXor
    + ElementBitXorAssign
    + ElementShr
    + ElementShrAssign
    + ElementShl
    + ElementShlAssign
{
}

macro_rules! impl_trait {
    ($ty:ident) => {
        impl IntElement for $ty {}
    };
}
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
