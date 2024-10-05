use gomath_proc_macros::vec_cget_wrappers;

use super::*;

vec_cget_wrappers!(Vec2: x, y);
vec_cget_wrappers!(Vec3: x, y, z);
vec_cget_wrappers!(Vec4: x, y, z, w);

pub trait ElementVecConstGet<const N: usize>: ElementVecInner
where
    MaybeVecNum<N>: VecNum<N>,
{
    unsafe fn vec_cget<const X: usize>(vec: InnerVec<N, Self>) -> Self;
    unsafe fn vec_cget2<const X: usize, const Y: usize>(vec: InnerVec<N, Self>) -> Self::InnerVec2;
    unsafe fn vec_cget3<const X: usize, const Y: usize, const Z: usize>(
        vec: InnerVec<N, Self>,
    ) -> Self::InnerVec3;
    unsafe fn vec_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: InnerVec<N, Self>,
    ) -> Self::InnerVec4;
}

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    pub unsafe fn cget<const X: usize>(self) -> T {
        MaybeVecNum::<N>::cget::<T, X>(self)
    }
    pub unsafe fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T> {
        MaybeVecNum::<N>::cget2::<T, X, Y>(self)
    }
    pub unsafe fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T> {
        MaybeVecNum::<N>::cget3::<T, X, Y, Z>(self)
    }
    pub unsafe fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
    ) -> Vec4<T> {
        MaybeVecNum::<N>::cget4::<T, X, Y, Z, W>(self)
    }
}

pub(super) trait VecNumConstGet<const N: usize>
where
    MaybeVecNum<N>: VecNum<N>,
{
    unsafe fn cget<T: Element, const X: usize>(vec: VecN<N, T>) -> T;
    unsafe fn cget2<T: Element, const X: usize, const Y: usize>(vec: VecN<N, T>) -> Vec2<T>;
    unsafe fn cget3<T: Element, const X: usize, const Y: usize, const Z: usize>(
        vec: VecN<N, T>,
    ) -> Vec3<T>;
    unsafe fn cget4<T: Element, const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: VecN<N, T>,
    ) -> Vec4<T>;
}
impl VecNumConstGet<2> for MaybeVecNum<2> {
    unsafe fn cget<T: Element, const X: usize>(vec: VecN<2, T>) -> T {
        T::vec_cget(vec)
    }
    unsafe fn cget2<T: Element, const X: usize, const Y: usize>(vec: VecN<2, T>) -> Vec2<T> {
        VecN::from_inner(T::vec_cget2(vec))
    }
    unsafe fn cget3<T: Element, const X: usize, const Y: usize, const Z: usize>(
        vec: VecN<2, T>,
    ) -> Vec3<T> {
        VecN::from_inner(T::vec_cget3(vec))
    }
    unsafe fn cget4<T: Element, const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: VecN<2, T>,
    ) -> Vec4<T> {
        VecN::from_inner(T::vec_cget4(vec))
    }
}
impl VecNumConstGet<3> for MaybeVecNum<3> {
    unsafe fn cget<T: Element, const X: usize>(vec: VecN<3, T>) -> T {
        T::vec_cget(vec)
    }
    unsafe fn cget2<T: Element, const X: usize, const Y: usize>(vec: VecN<3, T>) -> Vec2<T> {
        VecN::from_inner(T::vec_cget2(vec))
    }
    unsafe fn cget3<T: Element, const X: usize, const Y: usize, const Z: usize>(
        vec: VecN<3, T>,
    ) -> Vec3<T> {
        VecN::from_inner(T::vec_cget3(vec))
    }
    unsafe fn cget4<T: Element, const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: VecN<3, T>,
    ) -> Vec4<T> {
        VecN::from_inner(T::vec_cget4(vec))
    }
}
impl VecNumConstGet<4> for MaybeVecNum<4> {
    unsafe fn cget<T: Element, const X: usize>(vec: VecN<4, T>) -> T {
        T::vec_cget(vec)
    }
    unsafe fn cget2<T: Element, const X: usize, const Y: usize>(vec: VecN<4, T>) -> Vec2<T> {
        VecN::from_inner(T::vec_cget2(vec))
    }
    unsafe fn cget3<T: Element, const X: usize, const Y: usize, const Z: usize>(
        vec: VecN<4, T>,
    ) -> Vec3<T> {
        VecN::from_inner(T::vec_cget3(vec))
    }
    unsafe fn cget4<T: Element, const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: VecN<4, T>,
    ) -> Vec4<T> {
        VecN::from_inner(T::vec_cget4(vec))
    }
}
