use super::*;

impl<T: ElementDefaultImpl> ElementVecWith for T {
    #[inline(always)]
    unsafe fn vec2_cwith<const X: usize>(mut vec: Self::InnerVec2, value: Self) -> Self::InnerVec2 {
        *vec.get_unchecked_mut(X) = value;
        vec
    }
    #[inline(always)]
    unsafe fn vec2_cwith2<const X: usize, const Y: usize>(
        mut vec: Self::InnerVec2,
        value: Self::InnerVec2,
    ) -> Self::InnerVec2 {
        *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
        vec
    }

    #[inline(always)]
    unsafe fn vec3_cwith<const X: usize>(mut vec: Self::InnerVec3, value: Self) -> Self::InnerVec3 {
        *vec.get_unchecked_mut(X) = value;
        vec
    }
    #[inline(always)]
    unsafe fn vec3_cwith2<const X: usize, const Y: usize>(
        mut vec: Self::InnerVec3,
        value: Self::InnerVec2,
    ) -> Self::InnerVec3 {
        *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
        vec
    }
    #[inline(always)]
    unsafe fn vec3_cwith3<const X: usize, const Y: usize, const Z: usize>(
        mut vec: Self::InnerVec3,
        value: Self::InnerVec3,
    ) -> Self::InnerVec3 {
        *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
        *vec.get_unchecked_mut(Z) = *value.get_unchecked(2);
        vec
    }

    #[inline(always)]
    unsafe fn vec4_cwith<const X: usize>(mut vec: Self::InnerVec4, value: Self) -> Self::InnerVec4 {
        *vec.get_unchecked_mut(X) = value;
        vec
    }
    #[inline(always)]
    unsafe fn vec4_cwith2<const X: usize, const Y: usize>(
        mut vec: Self::InnerVec4,
        value: Self::InnerVec2,
    ) -> Self::InnerVec4 {
        *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
        vec
    }
    #[inline(always)]
    unsafe fn vec4_cwith3<const X: usize, const Y: usize, const Z: usize>(
        mut vec: Self::InnerVec4,
        value: Self::InnerVec3,
    ) -> Self::InnerVec4 {
        *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
        *vec.get_unchecked_mut(Z) = *value.get_unchecked(2);
        vec
    }
    #[inline(always)]
    unsafe fn vec4_cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        mut vec: Self::InnerVec4,
        value: Self::InnerVec4,
    ) -> Self::InnerVec4 {
        *vec.get_unchecked_mut(X) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(Y) = *value.get_unchecked(1);
        *vec.get_unchecked_mut(Z) = *value.get_unchecked(2);
        *vec.get_unchecked_mut(W) = *value.get_unchecked(3);
        vec
    }
}
