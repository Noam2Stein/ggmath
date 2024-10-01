use super::*;

impl<T: ElementDefaultImpl> ElementVecGet for T {
    #[inline(always)]
    fn vec2_cget<const X: usize>(value: Self::InnerVec2) -> Self {
        value[X]
    }
    #[inline(always)]
    fn vec2_cget2<const X: usize, const Y: usize>(value: Self::InnerVec2) -> Self::InnerVec2 {
        [value[X], value[Y]]
    }
    #[inline(always)]
    fn vec2_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec3 {
        [value[X], value[Y], value[Z], T::default()]
    }
    #[inline(always)]
    fn vec2_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec2,
    ) -> Self::InnerVec4 {
        [value[X], value[Y], value[Z], value[W]]
    }

    #[inline(always)]
    fn vec3_cget<const X: usize>(value: Self::InnerVec3) -> Self {
        value[X]
    }
    #[inline(always)]
    fn vec3_cget2<const X: usize, const Y: usize>(value: Self::InnerVec3) -> Self::InnerVec2 {
        [value[X], value[Y]]
    }
    #[inline(always)]
    fn vec3_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec3 {
        [value[X], value[Y], value[Z], value[Z + 1]]
    }
    #[inline(always)]
    fn vec3_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec3,
    ) -> Self::InnerVec4 {
        [value[X], value[Y], value[Z], value[W]]
    }

    #[inline(always)]
    fn vec4_cget<const X: usize>(value: Self::InnerVec4) -> Self {
        value[X]
    }
    #[inline(always)]
    fn vec4_cget2<const X: usize, const Y: usize>(value: Self::InnerVec4) -> Self::InnerVec2 {
        [value[X], value[Y]]
    }
    #[inline(always)]
    fn vec4_cget3<const X: usize, const Y: usize, const Z: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec3 {
        [value[X], value[Y], value[Z], value[Z + 1]]
    }
    #[inline(always)]
    fn vec4_cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        value: Self::InnerVec4,
    ) -> Self::InnerVec4 {
        [value[X], value[Y], value[Z], value[W]]
    }
}
