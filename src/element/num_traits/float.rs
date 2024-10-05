use super::*;

pub use ::num::Float;

pub trait FloatElement: SignedElement + Float {}

macro_rules! impl_trait {
    ($ty:ident) => {
        impl FloatElement for $ty {}
    };
}
impl_trait!(f32);
impl_trait!(f64);
