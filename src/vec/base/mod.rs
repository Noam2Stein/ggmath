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

trait Seal {}
impl<T: Element> Seal for Vec2<T> {}
impl<T: Element> Seal for Vec3<T> {}
impl<T: Element> Seal for Vec4<T> {}

#[allow(private_bounds)]
pub trait VecN<T: Element, const N: usize>:
    Seal
    + std::fmt::Debug
    + Copy
    + PartialEq
    + PartialOrd
    + Default
    + std::fmt::Display
    + VecNInner
    + VecNArray<T, N>
    + VecNConstGet<T, N>
    + VecNConstGetMut<T, N>
    + VecNConstWith<T, N>
    + VecNConstSet<T, N>
    + VecNSplat<T>
{
    type WithT<T2: Element>: VecN<T2, N>;
}
impl<T: Element> VecN<T, 2> for Vec2<T> {
    type WithT<T2: Element> = Vec2<T2>;
}
impl<T: Element> VecN<T, 3> for Vec3<T> {
    type WithT<T2: Element> = Vec3<T2>;
}
impl<T: Element> VecN<T, 4> for Vec4<T> {
    type WithT<T2: Element> = Vec4<T2>;
}

pub trait ElementVec:
    ElementVecInner + ElementVecDefault + ElementVecConstSwizzle + ElementVecSwizzle + ElementVecSplat
{
}
