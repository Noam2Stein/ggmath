use std::mem::transmute_copy;

use super::*;

pub trait VecNGet<T: Element, const N: usize> {
    fn get(self, x: usize) -> Result<T, &'static str>;
    fn get2(self, x: usize, y: usize) -> Result<Vec2<T>, &'static str>;
    fn get3(self, x: usize, y: usize, z: usize) -> Result<Vec3<T>, &'static str>;
    fn get4(self, x: usize, y: usize, z: usize, w: usize) -> Result<Vec4<T>, &'static str>;

    unsafe fn get_unchecked(self, x: usize) -> T;
    unsafe fn get2_unchecked(self, x: usize, y: usize) -> Vec2<T>;
    unsafe fn get3_unchecked(self, x: usize, y: usize, z: usize) -> Vec3<T>;
    unsafe fn get4_unchecked(self, x: usize, y: usize, z: usize, w: usize) -> Vec4<T>;
}

pub trait ElementVecGet: Sized + ElementVecInner {
    fn vec2_get(vec: Self::InnerVec2, x: usize) -> Result<Self, &'static str>;
    fn vec2_get2(vec: Self::InnerVec2, x: usize, y: usize)
        -> Result<Self::InnerVec2, &'static str>;
    fn vec2_get3(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        z: usize,
    ) -> Result<Self::InnerVec3, &'static str>;
    fn vec2_get4(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Result<Self::InnerVec4, &'static str>;

    fn vec3_get(vec: Self::InnerVec3, x: usize) -> Result<Self, &'static str>;
    fn vec3_get2(vec: Self::InnerVec3, x: usize, y: usize)
        -> Result<Self::InnerVec2, &'static str>;
    fn vec3_get3(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
    ) -> Result<Self::InnerVec3, &'static str>;
    fn vec3_get4(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Result<Self::InnerVec4, &'static str>;

    fn vec4_get(vec: Self::InnerVec4, x: usize) -> Result<Self, &'static str>;
    fn vec4_get2(vec: Self::InnerVec4, x: usize, y: usize)
        -> Result<Self::InnerVec2, &'static str>;
    fn vec4_get3(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
    ) -> Result<Self::InnerVec3, &'static str>;
    fn vec4_get4(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Result<Self::InnerVec4, &'static str>;

    unsafe fn vec2_get_unchecked(vec: Self::InnerVec2, x: usize) -> Self;
    unsafe fn vec2_get2_unchecked(vec: Self::InnerVec2, x: usize, y: usize) -> Self::InnerVec2;
    unsafe fn vec2_get3_unchecked(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        z: usize,
    ) -> Self::InnerVec3;
    unsafe fn vec2_get4_unchecked(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Self::InnerVec4;

    unsafe fn vec3_get_unchecked(vec: Self::InnerVec3, x: usize) -> Self;
    unsafe fn vec3_get2_unchecked(vec: Self::InnerVec3, x: usize, y: usize) -> Self::InnerVec2;
    unsafe fn vec3_get3_unchecked(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
    ) -> Self::InnerVec3;
    unsafe fn vec3_get4_unchecked(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Self::InnerVec4;

    unsafe fn vec4_get_unchecked(vec: Self::InnerVec4, x: usize) -> Self;
    unsafe fn vec4_get2_unchecked(vec: Self::InnerVec4, x: usize, y: usize) -> Self::InnerVec2;
    unsafe fn vec4_get3_unchecked(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
    ) -> Self::InnerVec3;
    unsafe fn vec4_get4_unchecked(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Self::InnerVec4;
}

impl<T: Element> VecNGet<T, 2> for Vec2<T> {
    #[inline(always)]
    fn get(self, x: usize) -> Result<T, &'static str> {
        unsafe { transmute_copy(&T::vec2_get(self.inner, x)) }
    }
    #[inline(always)]
    fn get2(self, x: usize, y: usize) -> Result<Vec2<T>, &'static str> {
        unsafe { transmute_copy(&T::vec2_get2(self.inner, x, y)) }
    }
    #[inline(always)]
    fn get3(self, x: usize, y: usize, z: usize) -> Result<Vec3<T>, &'static str> {
        unsafe { transmute_copy(&T::vec2_get3(self.inner, x, y, z)) }
    }
    #[inline(always)]
    fn get4(self, x: usize, y: usize, z: usize, w: usize) -> Result<Vec4<T>, &'static str> {
        unsafe { transmute_copy(&T::vec2_get4(self.inner, x, y, z, w)) }
    }

    #[inline(always)]
    unsafe fn get_unchecked(self, x: usize) -> T {
        T::vec2_get_unchecked(self.inner, x)
    }
    #[inline(always)]
    unsafe fn get2_unchecked(self, x: usize, y: usize) -> Vec2<T> {
        Vec2 {
            inner: T::vec2_get2_unchecked(self.inner, x, y),
        }
    }
    #[inline(always)]
    unsafe fn get3_unchecked(self, x: usize, y: usize, z: usize) -> Vec3<T> {
        Vec3 {
            inner: T::vec2_get3_unchecked(self.inner, x, y, z),
        }
    }
    #[inline(always)]
    unsafe fn get4_unchecked(self, x: usize, y: usize, z: usize, w: usize) -> Vec4<T> {
        Vec4 {
            inner: T::vec2_get4_unchecked(self.inner, x, y, z, w),
        }
    }
}
impl<T: Element> VecNGet<T, 3> for Vec3<T> {
    #[inline(always)]
    fn get(self, x: usize) -> Result<T, &'static str> {
        unsafe { transmute_copy(&T::vec3_get(self.inner, x)) }
    }
    #[inline(always)]
    fn get2(self, x: usize, y: usize) -> Result<Vec2<T>, &'static str> {
        unsafe { transmute_copy(&T::vec3_get2(self.inner, x, y)) }
    }
    #[inline(always)]
    fn get3(self, x: usize, y: usize, z: usize) -> Result<Vec3<T>, &'static str> {
        unsafe { transmute_copy(&T::vec3_get3(self.inner, x, y, z)) }
    }
    #[inline(always)]
    fn get4(self, x: usize, y: usize, z: usize, w: usize) -> Result<Vec4<T>, &'static str> {
        unsafe { transmute_copy(&T::vec3_get4(self.inner, x, y, z, w)) }
    }

    #[inline(always)]
    unsafe fn get_unchecked(self, x: usize) -> T {
        T::vec3_get_unchecked(self.inner, x)
    }
    #[inline(always)]
    unsafe fn get2_unchecked(self, x: usize, y: usize) -> Vec2<T> {
        Vec2 {
            inner: T::vec3_get2_unchecked(self.inner, x, y),
        }
    }
    #[inline(always)]
    unsafe fn get3_unchecked(self, x: usize, y: usize, z: usize) -> Vec3<T> {
        Vec3 {
            inner: T::vec3_get3_unchecked(self.inner, x, y, z),
        }
    }
    #[inline(always)]
    unsafe fn get4_unchecked(self, x: usize, y: usize, z: usize, w: usize) -> Vec4<T> {
        Vec4 {
            inner: T::vec3_get4_unchecked(self.inner, x, y, z, w),
        }
    }
}
impl<T: Element> VecNGet<T, 4> for Vec4<T> {
    #[inline(always)]
    fn get(self, x: usize) -> Result<T, &'static str> {
        unsafe { transmute_copy(&T::vec4_get(self.inner, x)) }
    }
    #[inline(always)]
    fn get2(self, x: usize, y: usize) -> Result<Vec2<T>, &'static str> {
        unsafe { transmute_copy(&T::vec4_get2(self.inner, x, y)) }
    }
    #[inline(always)]
    fn get3(self, x: usize, y: usize, z: usize) -> Result<Vec3<T>, &'static str> {
        unsafe { transmute_copy(&T::vec4_get3(self.inner, x, y, z)) }
    }
    #[inline(always)]
    fn get4(self, x: usize, y: usize, z: usize, w: usize) -> Result<Vec4<T>, &'static str> {
        unsafe { transmute_copy(&T::vec4_get4(self.inner, x, y, z, w)) }
    }

    #[inline(always)]
    unsafe fn get_unchecked(self, x: usize) -> T {
        T::vec4_get_unchecked(self.inner, x)
    }
    #[inline(always)]
    unsafe fn get2_unchecked(self, x: usize, y: usize) -> Vec2<T> {
        Vec2 {
            inner: T::vec4_get2_unchecked(self.inner, x, y),
        }
    }
    #[inline(always)]
    unsafe fn get3_unchecked(self, x: usize, y: usize, z: usize) -> Vec3<T> {
        Vec3 {
            inner: T::vec4_get3_unchecked(self.inner, x, y, z),
        }
    }
    #[inline(always)]
    unsafe fn get4_unchecked(self, x: usize, y: usize, z: usize, w: usize) -> Vec4<T> {
        Vec4 {
            inner: T::vec4_get4_unchecked(self.inner, x, y, z, w),
        }
    }
}
