use gomath_proc_macros::impl_vec_cget_shortnames;

use super::*;

impl_vec_cget_shortnames!(Vec2: x, y);
impl_vec_cget_shortnames!(Vec3: x, y, z);
impl_vec_cget_shortnames!(Vec4: x, y, z, w);

pub trait VecNCGet<T: Element, const N: usize> {
    fn cget<const X: usize>(self) -> T;
    fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T>;
    fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T>;
    fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(self) -> Vec4<T>;
}

pub trait ElementVecGet: ElementInnerVecs {
    fn vec2_cget<const X: usize>(value: Self::InnerVec2) -> Self;
    fn vec2_cget2<const X: usize, const Y: usize>(value: Self::InnerVec2) -> Self::InnerVec2;
    fn vec2_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec3;
    fn vec2_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec4;

    fn vec3_cget<const X: usize>(value: Self::InnerVec3) -> Self;
    fn vec3_cget2<const X: usize, const Y: usize>(value: Self::InnerVec3) -> Self::InnerVec2;
    fn vec3_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec3;
    fn vec3_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec4;

    fn vec4_cget<const X: usize>(value: Self::InnerVec4) -> Self;
    fn vec4_cget2<const X: usize, const Y: usize>(value: Self::InnerVec4) -> Self::InnerVec2;
    fn vec4_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec3;
    fn vec4_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec4;
}

impl<T: Element> VecNCGet<T, 2> for Vec2<T> {
    #[inline(always)]
    fn cget<const X: usize>(self) -> T {
        T::vec2_cget::<X>(self.inner)
    }
    #[inline(always)]
    fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T> {
        Vec2 {
            inner: T::vec2_cget2::<X, Y>(self.inner),
        }
    }
    #[inline(always)]
    fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T> {
        Vec3 {
            inner: T::vec2_cget3::<X, Y, Z>(self.inner),
        }
    }
    #[inline(always)]
    fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(self) -> Vec4<T> {
        Vec4 {
            inner: T::vec2_cget4::<X, Y, Z, W>(self.inner),
        }
    }
}
impl<T: Element> VecNCGet<T, 3> for Vec3<T> {
    #[inline(always)]
    fn cget<const X: usize>(self) -> T {
        T::vec3_cget::<X>(self.inner)
    }
    #[inline(always)]
    fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T> {
        Vec2 {
            inner: T::vec3_cget2::<X, Y>(self.inner),
        }
    }
    #[inline(always)]
    fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T> {
        Vec3 {
            inner: T::vec3_cget3::<X, Y, Z>(self.inner),
        }
    }
    #[inline(always)]
    fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(self) -> Vec4<T> {
        Vec4 {
            inner: T::vec3_cget4::<X, Y, Z, W>(self.inner),
        }
    }
}
impl<T: Element> VecNCGet<T, 4> for Vec4<T> {
    #[inline(always)]
    fn cget<const X: usize>(self) -> T {
        T::vec4_cget::<X>(self.inner)
    }
    #[inline(always)]
    fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T> {
        Vec2 {
            inner: T::vec4_cget2::<X, Y>(self.inner),
        }
    }
    #[inline(always)]
    fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T> {
        Vec3 {
            inner: T::vec4_cget3::<X, Y, Z>(self.inner),
        }
    }
    #[inline(always)]
    fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(self) -> Vec4<T> {
        Vec4 {
            inner: T::vec4_cget4::<X, Y, Z, W>(self.inner),
        }
    }
}
