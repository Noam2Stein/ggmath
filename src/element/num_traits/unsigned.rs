use super::*;

pub use ::num::Unsigned;

pub trait UnsignedElement: NumElement + Unsigned {}

macro_rules! impl_trait {
    ($ty:ident) => {
        impl UnsignedElement for $ty {}
    };
}
impl_trait!(u8);
impl_trait!(u16);
impl_trait!(u32);
impl_trait!(u64);
impl_trait!(u128);
impl_trait!(usize);
