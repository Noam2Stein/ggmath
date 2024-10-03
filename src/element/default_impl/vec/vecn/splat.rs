use super::*;

impl<T: ElementDefaultImpl> ElementVecSplat for T {
    #[inline(always)]
    fn vec2_splat(value: Self) -> Self::InnerVec2 {
        [value; 2]
    }
    #[inline(always)]
    fn vec3_splat(value: Self) -> Self::InnerVec3 {
        [value; 4]
    }
    #[inline(always)]
    fn vec4_splat(value: Self) -> Self::InnerVec4 {
        [value; 4]
    }
}
