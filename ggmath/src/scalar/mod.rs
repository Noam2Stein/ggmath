//! Scalars are mathamatical types that have magnitude but no direction.
//! - [f32] and [bool] are scalars.
//! - [Vec3](crate::vec::Vec3) is not a scalar.

mod primitive_impls;

pub use crate::vector::interfaces::scalar_traits::*;

pub use ggmath_proc_macros::scalar_aliases;

pub trait ScalarNum:
    Scalar
    + From<usize>
    + ScalarAdd<Self, Output = Self>
    + ScalarSub<Self, Output = Self>
    + ScalarMul<Self, Output = Self>
    + ScalarDiv<Self, Output = Self>
    + ScalarRem<Self, Output = Self>
    + PartialOrd
{
}
