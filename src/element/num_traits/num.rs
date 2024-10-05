use super::*;

pub trait NumElement:
    Element
    + ElementNumConvert
    + ElementFromNum<f32>
    + ElementFromNum<f64>
    + ElementFromNum<u8>
    + ElementFromNum<u16>
    + ElementFromNum<u32>
    + ElementFromNum<u64>
    + ElementFromNum<u128>
    + ElementFromNum<usize>
    + ElementFromNum<i8>
    + ElementFromNum<i16>
    + ElementFromNum<i32>
    + ElementFromNum<i64>
    + ElementFromNum<i128>
    + ElementFromNum<isize>
    + Num
    + NumCast
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
