use gomath_proc_macros::vec_cget_wrappers;

use super::*;

vec_cget_wrappers!(Vec2: x, y);
vec_cget_wrappers!(Vec3: x, y, z);
vec_cget_wrappers!(Vec4: x, y, z, w);

pub trait VecNConstGet<T: Element, const N: usize> {
    unsafe fn cget<const X: usize>(self) -> T;
    unsafe fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T>;
    unsafe fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T>;
    unsafe fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
    ) -> Vec4<T>;
}

pub trait ElementVecConstGet: ElementVecInner {
    unsafe fn vec2_cget<const X: usize>(value: Self::InnerVec2) -> Self;
    unsafe fn vec2_cget2<const X: usize, const Y: usize>(value: Self::InnerVec2)
        -> Self::InnerVec2;
    unsafe fn vec2_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec3;
    unsafe fn vec2_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec4;

    unsafe fn vec3_cget<const X: usize>(value: Self::InnerVec3) -> Self;
    unsafe fn vec3_cget2<const X: usize, const Y: usize>(value: Self::InnerVec3)
        -> Self::InnerVec2;
    unsafe fn vec3_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec3;
    unsafe fn vec3_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec4;

    unsafe fn vec4_cget<const X: usize>(value: Self::InnerVec4) -> Self;
    unsafe fn vec4_cget2<const X: usize, const Y: usize>(value: Self::InnerVec4)
        -> Self::InnerVec2;
    unsafe fn vec4_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec3;
    unsafe fn vec4_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec4;
}

impl<T: Element> VecNConstGet<T, 2> for Vec2<T> {
    #[inline(always)]
    unsafe fn cget<const X: usize>(self) -> T {
        T::vec2_cget::<X>(self.inner)
    }
    #[inline(always)]
    unsafe fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T> {
        Vec2 {
            inner: T::vec2_cget2::<X, Y>(self.inner),
        }
    }
    #[inline(always)]
    unsafe fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T> {
        Vec3 {
            inner: T::vec2_cget3::<X, Y, Z>(self.inner),
        }
    }
    #[inline(always)]
    unsafe fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
    ) -> Vec4<T> {
        Vec4 {
            inner: T::vec2_cget4::<X, Y, Z, W>(self.inner),
        }
    }
}
impl<T: Element> VecNConstGet<T, 3> for Vec3<T> {
    #[inline(always)]
    unsafe fn cget<const X: usize>(self) -> T {
        T::vec3_cget::<X>(self.inner)
    }
    #[inline(always)]
    unsafe fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T> {
        Vec2 {
            inner: T::vec3_cget2::<X, Y>(self.inner),
        }
    }
    #[inline(always)]
    unsafe fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T> {
        Vec3 {
            inner: T::vec3_cget3::<X, Y, Z>(self.inner),
        }
    }
    #[inline(always)]
    unsafe fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
    ) -> Vec4<T> {
        Vec4 {
            inner: T::vec3_cget4::<X, Y, Z, W>(self.inner),
        }
    }
}
impl<T: Element> VecNConstGet<T, 4> for Vec4<T> {
    #[inline(always)]
    unsafe fn cget<const X: usize>(self) -> T {
        T::vec4_cget::<X>(self.inner)
    }
    #[inline(always)]
    unsafe fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T> {
        Vec2 {
            inner: T::vec4_cget2::<X, Y>(self.inner),
        }
    }
    #[inline(always)]
    unsafe fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T> {
        Vec3 {
            inner: T::vec4_cget3::<X, Y, Z>(self.inner),
        }
    }
    #[inline(always)]
    unsafe fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
    ) -> Vec4<T> {
        Vec4 {
            inner: T::vec4_cget4::<X, Y, Z, W>(self.inner),
        }
    }
}
