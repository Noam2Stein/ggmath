use super::*;

impl<T: ElementDefaultImpl> ElementVecDefault for T {
    #[inline(always)]
    fn default_vec2() -> Self::InnerVec2 {
        [T::default(); 2]
    }
    #[inline(always)]
    fn default_vec3() -> Self::InnerVec3 {
        [T::default(); 4]
    }
    #[inline(always)]
    fn default_vec4() -> Self::InnerVec4 {
        [T::default(); 4]
    }
}
