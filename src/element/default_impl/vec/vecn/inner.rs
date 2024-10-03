use super::*;

unsafe impl<T: ElementDefaultImpl> ElementVecInner for T {
    type InnerVec2 = [Self; 2];
    type InnerVec3 = [Self; 4];
    type InnerVec4 = [Self; 4];
}
