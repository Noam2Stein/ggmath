use super::*;

impl<T: ElementDefaultImpl> ElementVecConstGet for T {
    #[inline(always)]
    unsafe fn vec2_cget<const X: usize>(value: Self::InnerVec2) -> Self {
        *value.get_unchecked(X)
    }
    #[inline(always)]
    unsafe fn vec2_cget2<const X: usize, const Y: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec2 {
        [*value.get_unchecked(X), *value.get_unchecked(Y)]
    }
    #[inline(always)]
    unsafe fn vec2_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec3 {
        [
            *value.get_unchecked(X),
            *value.get_unchecked(Y),
            *value.get_unchecked(Z),
            T::default(),
        ]
    }
    #[inline(always)]
    unsafe fn vec2_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec4 {
        [
            *value.get_unchecked(X),
            *value.get_unchecked(Y),
            *value.get_unchecked(Z),
            *value.get_unchecked(W),
        ]
    }

    #[inline(always)]
    unsafe fn vec3_cget<const X: usize>(value: Self::InnerVec3) -> Self {
        *value.get_unchecked(X)
    }
    #[inline(always)]
    unsafe fn vec3_cget2<const X: usize, const Y: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec2 {
        [*value.get_unchecked(X), *value.get_unchecked(Y)]
    }
    #[inline(always)]
    unsafe fn vec3_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec3 {
        [
            *value.get_unchecked(X),
            *value.get_unchecked(Y),
            *value.get_unchecked(Z),
            T::default(),
        ]
    }
    #[inline(always)]
    unsafe fn vec3_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec4 {
        [
            *value.get_unchecked(X),
            *value.get_unchecked(Y),
            *value.get_unchecked(Z),
            *value.get_unchecked(W),
        ]
    }

    #[inline(always)]
    unsafe fn vec4_cget<const X: usize>(value: Self::InnerVec4) -> Self {
        *value.get_unchecked(X)
    }
    #[inline(always)]
    unsafe fn vec4_cget2<const X: usize, const Y: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec2 {
        [*value.get_unchecked(X), *value.get_unchecked(Y)]
    }
    #[inline(always)]
    unsafe fn vec4_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec3 {
        [
            *value.get_unchecked(X),
            *value.get_unchecked(Y),
            *value.get_unchecked(Z),
            T::default(),
        ]
    }
    #[inline(always)]
    unsafe fn vec4_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec4 {
        [
            *value.get_unchecked(X),
            *value.get_unchecked(Y),
            *value.get_unchecked(Z),
            *value.get_unchecked(W),
        ]
    }
}
