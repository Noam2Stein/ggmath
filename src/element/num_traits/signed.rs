use super::*;

pub use ::num::Signed;

pub trait SignedElement: NumElement + Signed + ElementNeg<Output = Self> {
    const NEG_ONE: Self;
}

macro_rules! impl_trait {
    ($ty:ident) => {
        impl SignedElement for $ty {
            const NEG_ONE: Self = -1 as $ty;
        }
    };
}
impl_trait!(f32);
impl_trait!(f64);
impl_trait!(i8);
impl_trait!(i16);
impl_trait!(i32);
impl_trait!(i64);
impl_trait!(i128);
impl_trait!(isize);
