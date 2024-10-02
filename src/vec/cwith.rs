use gomath_proc_macros::vec_cwith_wrappers;

use super::*;

vec_cwith_wrappers!(Vec2: x, y);
vec_cwith_wrappers!(Vec3: x, y, z);
vec_cwith_wrappers!(Vec4: x, y, z, w);

pub trait VecNCWith<T: Element, const N: usize> {
    unsafe fn cwith<const X: usize>(self, value: T) -> Self;
    unsafe fn cwith2<const X: usize, const Y: usize>(self, value: Vec2<T>) -> Self;
    unsafe fn cwith3<const X: usize, const Y: usize, const Z: usize>(self, value: Vec3<T>) -> Self;
    unsafe fn cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
        value: Vec4<T>,
    ) -> Self;
}

pub trait ElementVecWith: ElementInnerVecs {
    unsafe fn vec2_cwith<const X: usize>(vec: Self::InnerVec2, value: Self) -> Self::InnerVec2;
    unsafe fn vec2_cwith2<const X: usize, const Y: usize>(
        vec: Self::InnerVec2,
        value: Self::InnerVec2,
    ) -> Self::InnerVec2;

    unsafe fn vec3_cwith<const X: usize>(vec: Self::InnerVec3, value: Self) -> Self::InnerVec3;
    unsafe fn vec3_cwith2<const X: usize, const Y: usize>(
        vec: Self::InnerVec3,
        value: Self::InnerVec2,
    ) -> Self::InnerVec3;
    unsafe fn vec3_cwith3<const X: usize, const Y: usize, const Z: usize>(
        vec: Self::InnerVec3,
        value: Self::InnerVec3,
    ) -> Self::InnerVec3;

    unsafe fn vec4_cwith<const X: usize>(vec: Self::InnerVec4, value: Self) -> Self::InnerVec4;
    unsafe fn vec4_cwith2<const X: usize, const Y: usize>(
        vec: Self::InnerVec4,
        value: Self::InnerVec2,
    ) -> Self::InnerVec4;
    unsafe fn vec4_cwith3<const X: usize, const Y: usize, const Z: usize>(
        vec: Self::InnerVec4,
        value: Self::InnerVec3,
    ) -> Self::InnerVec4;
    unsafe fn vec4_cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: Self::InnerVec4,
        value: Self::InnerVec4,
    ) -> Self::InnerVec4;
}

impl<T: Element> VecNCWith<T, 2> for Vec2<T> {
    #[inline(always)]
    unsafe fn cwith<const X: usize>(self, value: T) -> Self {
        Self {
            inner: T::vec2_cwith::<X>(self.inner, value),
        }
    }
    #[inline(always)]
    unsafe fn cwith2<const X: usize, const Y: usize>(self, value: Vec2<T>) -> Self {
        Self {
            inner: T::vec2_cwith2::<X, Y>(self.inner, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn cwith3<const X: usize, const Y: usize, const Z: usize>(
        self,
        _value: Vec3<T>,
    ) -> Self {
        panic!("'with3' can't be performed on a vec2 (3 > 2)")
    }
    #[inline(always)]
    unsafe fn cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
        _value: Vec4<T>,
    ) -> Self {
        panic!("'with4' can't be performed on a vec2 (4 > 2)")
    }
}
impl<T: Element> VecNCWith<T, 3> for Vec3<T> {
    #[inline(always)]
    unsafe fn cwith<const X: usize>(self, value: T) -> Self {
        Self {
            inner: T::vec3_cwith::<X>(self.inner, value),
        }
    }
    #[inline(always)]
    unsafe fn cwith2<const X: usize, const Y: usize>(self, value: Vec2<T>) -> Self {
        Self {
            inner: T::vec3_cwith2::<X, Y>(self.inner, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn cwith3<const X: usize, const Y: usize, const Z: usize>(self, value: Vec3<T>) -> Self {
        Self {
            inner: T::vec3_cwith3::<X, Y, Z>(self.inner, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
        _value: Vec4<T>,
    ) -> Self {
        panic!("'with4' can't be performed on a vec3 (4 > 3)")
    }
}
impl<T: Element> VecNCWith<T, 4> for Vec4<T> {
    #[inline(always)]
    unsafe fn cwith<const X: usize>(self, value: T) -> Self {
        Self {
            inner: T::vec4_cwith::<X>(self.inner, value),
        }
    }
    #[inline(always)]
    unsafe fn cwith2<const X: usize, const Y: usize>(self, value: Vec2<T>) -> Self {
        Self {
            inner: T::vec4_cwith2::<X, Y>(self.inner, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn cwith3<const X: usize, const Y: usize, const Z: usize>(self, value: Vec3<T>) -> Self {
        Self {
            inner: T::vec4_cwith3::<X, Y, Z>(self.inner, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
        value: Vec4<T>,
    ) -> Self {
        Self {
            inner: T::vec4_cwith4::<X, Y, Z, W>(self.inner, value.inner),
        }
    }
}
