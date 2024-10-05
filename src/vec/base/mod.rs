use super::*;

mod array;
mod const_swizzle;
mod from_split;
mod inner;
mod splat;
mod std_impl;
mod swizzle;
pub use array::*;
pub use const_swizzle::*;
pub use from_split::*;
pub use inner::*;
pub use splat::*;
pub use std_impl::*;
pub use swizzle::*;

pub trait ElementVec:
    ElementVecInner + ElementVecDefault + ElementVecConstSwizzle + ElementVecSwizzle + ElementVecSplat
{
}
