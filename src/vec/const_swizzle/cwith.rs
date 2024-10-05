use gomath_proc_macros::vec_cwith_wrappers;

use super::*;

vec_cwith_wrappers!(Vec2: x, y);
vec_cwith_wrappers!(Vec3: x, y, z);
vec_cwith_wrappers!(Vec4: x, y, z, w);

pub trait ElementVecConstWith<const N: usize>: ElementVecInner
where
    MaybeVecNum<N>: VecNum<N>,
{
    unsafe fn vec_cwith<const X: usize>(vec: InnerVec<N, Self>, value: Self) -> InnerVec<N, Self>;
    unsafe fn vec_cwith2<const X: usize, const Y: usize>(
        vec: InnerVec<N, Self>,
        value: Self::InnerVec2,
    ) -> InnerVec<N, Self>;
    unsafe fn vec_cwith3<const X: usize, const Y: usize, const Z: usize>(
        vec: InnerVec<N, Self>,
        value: Self::InnerVec3,
    ) -> InnerVec<N, Self>;
    unsafe fn vec_cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: InnerVec<N, Self>,
        value: Self::InnerVec4,
    ) -> InnerVec<N, Self>;
}

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    pub unsafe fn cwith<const X: usize>(self, value: T) -> Self {
        MaybeVecNum::<N>::cwith::<T, X>(self, value)
    }
    pub unsafe fn cwith2<const X: usize, const Y: usize>(self, value: Vec2<T>) -> Self {
        MaybeVecNum::<N>::cwith2::<T, X, Y>(self, value)
    }
    pub unsafe fn cwith3<const X: usize, const Y: usize, const Z: usize>(
        self,
        value: Vec3<T>,
    ) -> Self {
        MaybeVecNum::<N>::cwith3::<T, X, Y, Z>(self, value)
    }
    pub unsafe fn cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
        value: Vec4<T>,
    ) -> Self {
        MaybeVecNum::<N>::cwith4::<T, X, Y, Z, W>(self, value)
    }
}

pub(super) trait VecNumConstWith<const N: usize>
where
    MaybeVecNum<N>: VecNum<N>,
{
    unsafe fn cwith<T: Element, const X: usize>(vec: VecN<N, T>, value: T) -> VecN<N, T>;
    unsafe fn cwith2<T: Element, const X: usize, const Y: usize>(
        vec: VecN<N, T>,
        value: Vec2<T>,
    ) -> VecN<N, T>;
    unsafe fn cwith3<T: Element, const X: usize, const Y: usize, const Z: usize>(
        vec: VecN<N, T>,
        value: Vec3<T>,
    ) -> VecN<N, T>;
    unsafe fn cwith4<T: Element, const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: VecN<N, T>,
        value: Vec4<T>,
    ) -> VecN<N, T>;
}
impl VecNumConstWith<2> for MaybeVecNum<2> {
    unsafe fn cwith<T: Element, const X: usize>(vec: VecN<2, T>, value: T) -> VecN<2, T> {
        T::vec_cwith(vec, value)
    }
    unsafe fn cwith2<T: Element, const X: usize, const Y: usize>(
        vec: VecN<2, T>,
        value: Vec2<T>,
    ) -> VecN<2, T> {
        T::vec_cwith2(vec, value.inner)
    }
    unsafe fn cwith3<T: Element, const X: usize, const Y: usize, const Z: usize>(
        vec: VecN<2, T>,
        value: Vec3<T>,
    ) -> VecN<2, T> {
        T::vec_cwith3(vec, value.inner)
    }
    unsafe fn cwith4<T: Element, const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: VecN<2, T>,
        value: Vec4<T>,
    ) -> VecN<2, T> {
        T::vec_cwith4(vec, value.inner)
    }
}
impl VecNumConstWith<3> for MaybeVecNum<3> {
    unsafe fn cwith<T: Element, const X: usize>(vec: VecN<3, T>, value: T) -> VecN<3, T> {
        T::vec_cwith(vec, value)
    }
    unsafe fn cwith2<T: Element, const X: usize, const Y: usize>(
        vec: VecN<3, T>,
        value: Vec2<T>,
    ) -> VecN<3, T> {
        T::vec_cwith2(vec, value.inner)
    }
    unsafe fn cwith3<T: Element, const X: usize, const Y: usize, const Z: usize>(
        vec: VecN<3, T>,
        value: Vec3<T>,
    ) -> VecN<3, T> {
        T::vec_cwith3(vec, value.inner)
    }
    unsafe fn cwith4<T: Element, const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: VecN<3, T>,
        value: Vec4<T>,
    ) -> VecN<3, T> {
        T::vec_cwith4(vec, value.inner)
    }
}
impl VecNumConstWith<4> for MaybeVecNum<4> {
    unsafe fn cwith<T: Element, const X: usize>(vec: VecN<4, T>, value: T) -> VecN<4, T> {
        T::vec_cwith(vec, value)
    }
    unsafe fn cwith2<T: Element, const X: usize, const Y: usize>(
        vec: VecN<4, T>,
        value: Vec2<T>,
    ) -> VecN<4, T> {
        T::vec_cwith2(vec, value.inner)
    }
    unsafe fn cwith3<T: Element, const X: usize, const Y: usize, const Z: usize>(
        vec: VecN<4, T>,
        value: Vec3<T>,
    ) -> VecN<4, T> {
        T::vec_cwith3(vec, value.inner)
    }
    unsafe fn cwith4<T: Element, const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: VecN<4, T>,
        value: Vec4<T>,
    ) -> VecN<4, T> {
        T::vec_cwith4(vec, value.inner)
    }
}
