use crate::element::*;

use inner::*;

pub mod array;
pub mod const_swizzle;
pub mod from_split;
pub mod inner;
pub mod num;
pub mod ops;
pub mod splat;
pub mod std_impl;
pub mod swizzle;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VecN<const N: usize, T: Element>
where
    MaybeVecNum<N>: VecNum<N>,
{
    inner: InnerVec<N, T>,
}
pub type Vec2<T = f32> = VecN<2, T>;
pub type Vec3<T = f32> = VecN<3, T>;
pub type Vec4<T = f32> = VecN<4, T>;

pub trait ElementVec:
    ElementVecInner
    + std_impl::ElementVecDefault
    + const_swizzle::ElementVecConstSwizzle
    + swizzle::ElementVecSwizzle
    + splat::ElementVecSplat
{
}

pub trait VecNum<const N: usize>:
    VecNumInner + const_swizzle::VecNumConstSwizzle<N> + swizzle::VecNumSwizzle<N> + ops::VecNumOps<N>
where
    MaybeVecNum<N>: VecNum<N>,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeVecNum<const VALUE: usize>;

impl VecNum<2> for MaybeVecNum<2> {}
impl VecNum<3> for MaybeVecNum<3> {}
impl VecNum<4> for MaybeVecNum<4> {}
