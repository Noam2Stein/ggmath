use std::mem::transmute_copy;

use super::*;

pub trait VecNWith<T: Element, const N: usize>: Sized {
    fn with(self, x: usize, value: T) -> Result<Self, &'static str>;
    fn with2(self, x: usize, y: usize, value: Vec2<T>) -> Result<Self, &'static str>;
    fn with3(self, x: usize, y: usize, z: usize, value: Vec3<T>) -> Result<Self, &'static str>;
    fn with4(
        self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        value: Vec4<T>,
    ) -> Result<Self, &'static str>;

    unsafe fn with_unchecked(self, x: usize, value: T) -> Self;
    unsafe fn with2_unchecked(self, x: usize, y: usize, value: Vec2<T>) -> Self;
    unsafe fn with3_unchecked(self, x: usize, y: usize, z: usize, value: Vec3<T>) -> Self;
    unsafe fn with4_unchecked(self, x: usize, y: usize, z: usize, w: usize, value: Vec4<T>)
        -> Self;
}

pub trait ElementVecWith: ElementVecInner {
    fn vec2_with(
        vec: Self::InnerVec2,
        x: usize,
        value: Self,
    ) -> Result<Self::InnerVec2, &'static str>;
    fn vec2_with2(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Result<Self::InnerVec2, &'static str>;

    fn vec3_with(
        vec: Self::InnerVec3,
        x: usize,
        value: Self,
    ) -> Result<Self::InnerVec3, &'static str>;
    fn vec3_with2(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Result<Self::InnerVec3, &'static str>;
    fn vec3_with3(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
        value: Self::InnerVec3,
    ) -> Result<Self::InnerVec3, &'static str>;

    fn vec4_with(
        vec: Self::InnerVec4,
        x: usize,
        value: Self,
    ) -> Result<Self::InnerVec4, &'static str>;
    fn vec4_with2(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Result<Self::InnerVec4, &'static str>;
    fn vec4_with3(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        value: Self::InnerVec3,
    ) -> Result<Self::InnerVec4, &'static str>;
    fn vec4_with4(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        value: Self::InnerVec4,
    ) -> Result<Self::InnerVec4, &'static str>;

    unsafe fn vec2_with_unchecked(vec: Self::InnerVec2, x: usize, value: Self) -> Self::InnerVec2;
    unsafe fn vec2_with2_unchecked(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Self::InnerVec2;

    unsafe fn vec3_with_unchecked(vec: Self::InnerVec3, x: usize, value: Self) -> Self::InnerVec3;
    unsafe fn vec3_with2_unchecked(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Self::InnerVec3;
    unsafe fn vec3_with3_unchecked(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
        value: Self::InnerVec3,
    ) -> Self::InnerVec3;

    unsafe fn vec4_with_unchecked(vec: Self::InnerVec4, x: usize, value: Self) -> Self::InnerVec4;
    unsafe fn vec4_with2_unchecked(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Self::InnerVec4;
    unsafe fn vec4_with3_unchecked(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        value: Self::InnerVec3,
    ) -> Self::InnerVec4;
    unsafe fn vec4_with4_unchecked(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        value: Self::InnerVec4,
    ) -> Self::InnerVec4;
}

impl<T: Element> VecNWith<T, 2> for Vec2<T> {
    #[inline(always)]
    fn with(self, x: usize, value: T) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec2_with(self.inner, x, value)) }
    }
    #[inline(always)]
    fn with2(self, x: usize, y: usize, value: Vec2<T>) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec2_with2(self.inner, x, y, value.inner)) }
    }
    #[inline(always)]
    fn with3(self, _x: usize, _y: usize, _z: usize, _value: Vec3<T>) -> Result<Self, &'static str> {
        Err("with3 is impossible on a vec2")
    }
    #[inline(always)]
    fn with4(
        self,
        _x: usize,
        _y: usize,
        _z: usize,
        _w: usize,
        _value: Vec4<T>,
    ) -> Result<Self, &'static str> {
        Err("with4 is impossible on a vec2")
    }

    #[inline(always)]
    unsafe fn with_unchecked(self, x: usize, value: T) -> Self {
        Self {
            inner: T::vec2_with_unchecked(self.inner, x, value),
        }
    }
    #[inline(always)]
    unsafe fn with2_unchecked(self, x: usize, y: usize, value: Vec2<T>) -> Self {
        Self {
            inner: T::vec2_with2_unchecked(self.inner, x, y, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn with3_unchecked(self, _x: usize, _y: usize, _z: usize, _value: Vec3<T>) -> Self {
        panic!("with3 is impossible on a vec2")
    }
    #[inline(always)]
    unsafe fn with4_unchecked(
        self,
        _x: usize,
        _y: usize,
        _z: usize,
        _w: usize,
        _value: Vec4<T>,
    ) -> Self {
        panic!("with4 is impossible on a vec2")
    }
}
impl<T: Element> VecNWith<T, 3> for Vec3<T> {
    #[inline(always)]
    fn with(self, x: usize, value: T) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec3_with(self.inner, x, value)) }
    }
    #[inline(always)]
    fn with2(self, x: usize, y: usize, value: Vec2<T>) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec3_with2(self.inner, x, y, value.inner)) }
    }
    #[inline(always)]
    fn with3(self, x: usize, y: usize, z: usize, value: Vec3<T>) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec3_with3(self.inner, x, y, z, value.inner)) }
    }
    #[inline(always)]
    fn with4(
        self,
        _x: usize,
        _y: usize,
        _z: usize,
        _w: usize,
        _value: Vec4<T>,
    ) -> Result<Self, &'static str> {
        Err("with4 is impossible on a vec3")
    }

    #[inline(always)]
    unsafe fn with_unchecked(self, x: usize, value: T) -> Self {
        Self {
            inner: T::vec3_with_unchecked(self.inner, x, value),
        }
    }
    #[inline(always)]
    unsafe fn with2_unchecked(self, x: usize, y: usize, value: Vec2<T>) -> Self {
        Self {
            inner: T::vec3_with2_unchecked(self.inner, x, y, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn with3_unchecked(self, x: usize, y: usize, z: usize, value: Vec3<T>) -> Self {
        Self {
            inner: T::vec3_with3_unchecked(self.inner, x, y, z, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn with4_unchecked(
        self,
        _x: usize,
        _y: usize,
        _z: usize,
        _w: usize,
        _value: Vec4<T>,
    ) -> Self {
        panic!("with4 is impossible on a vec3")
    }
}
impl<T: Element> VecNWith<T, 4> for Vec4<T> {
    #[inline(always)]
    fn with(self, x: usize, value: T) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec4_with(self.inner, x, value)) }
    }
    #[inline(always)]
    fn with2(self, x: usize, y: usize, value: Vec2<T>) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec4_with2(self.inner, x, y, value.inner)) }
    }
    #[inline(always)]
    fn with3(self, x: usize, y: usize, z: usize, value: Vec3<T>) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec4_with3(self.inner, x, y, z, value.inner)) }
    }
    #[inline(always)]
    fn with4(
        self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        value: Vec4<T>,
    ) -> Result<Self, &'static str> {
        unsafe { transmute_copy(&T::vec4_with4(self.inner, x, y, z, w, value.inner)) }
    }

    #[inline(always)]
    unsafe fn with_unchecked(self, x: usize, value: T) -> Self {
        Self {
            inner: T::vec4_with_unchecked(self.inner, x, value),
        }
    }
    #[inline(always)]
    unsafe fn with2_unchecked(self, x: usize, y: usize, value: Vec2<T>) -> Self {
        Self {
            inner: T::vec4_with2_unchecked(self.inner, x, y, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn with3_unchecked(self, x: usize, y: usize, z: usize, value: Vec3<T>) -> Self {
        Self {
            inner: T::vec4_with3_unchecked(self.inner, x, y, z, value.inner),
        }
    }
    #[inline(always)]
    unsafe fn with4_unchecked(
        self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        value: Vec4<T>,
    ) -> Self {
        Self {
            inner: T::vec4_with4_unchecked(self.inner, x, y, z, w, value.inner),
        }
    }
}
