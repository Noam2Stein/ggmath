use crate::element::*;

use gomath_proc_macros::vecnum_trait;
use inner::*;

pub mod array;
pub mod from_split;
pub mod inner;
pub mod num;
pub mod ops;
pub mod splat;
pub mod std_impl;
pub mod swizzle_dynamic;
pub mod swizzle_static;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VecN<const N: usize, T: Element>
where
    MaybeVecNum<N>: VecNum<N>,
{
    inner: InnerVecN<N, T>,
}
pub type Vec2<T = f32> = VecN<2, T>;
pub type Vec3<T = f32> = VecN<3, T>;
pub type Vec4<T = f32> = VecN<4, T>;

pub trait ElementVec:
    ElementVecInner
    + std_impl::ElementVecDefault
    + swizzle_static::ElementVecConstSwizzle
    + swizzle_dynamic::ElementVecSwizzle
    + splat::ElementVecSplat
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MaybeVecNum<const VALUE: usize>;

vecnum_trait!(
    pub trait VecNum:
        VecNumInner + swizzle_dynamic::VecNumSwizzle + swizzle_static::VecNumConstSwizzle
    {
    }
);

pub type VecNOrOne<const N: usize, T: Element> = <MaybeVecNum<N> as VecNumOrOne>::VecOrOne<T>;
pub trait VecNumOrOne: Seal {
    type VecOrOne<T: Element>: 'static
        + std::fmt::Debug
        + Copy
        + PartialEq
        + PartialOrd
        + Default
        + std::fmt::Display;
}
impl VecNumOrOne for MaybeVecNum<1> {
    type VecOrOne<T: Element> = T;
}
impl VecNumOrOne for MaybeVecNum<2> {
    type VecOrOne<T: Element> = Vec2<T>;
}
impl VecNumOrOne for MaybeVecNum<3> {
    type VecOrOne<T: Element> = Vec3<T>;
}
impl VecNumOrOne for MaybeVecNum<4> {
    type VecOrOne<T: Element> = Vec4<T>;
}

trait Seal {}
impl<const N: usize> Seal for MaybeVecNum<N> {}
