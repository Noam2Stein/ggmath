use super::*;

pub trait VecNComponentSum {
    type ComponentSum;
    fn component_sum(self) -> Self::ComponentSum;
}
impl<T: Element + Add<Output = T>> VecNComponentSum for Vec2<T> {
    type ComponentSum = T::Output;
    #[inline(always)]
    fn component_sum(self) -> Self::ComponentSum {
        self.x() + self.y()
    }
}
impl<T: Element + Add<Output = T>> VecNComponentSum for Vec3<T> {
    type ComponentSum = T::Output;
    #[inline(always)]
    fn component_sum(self) -> Self::ComponentSum {
        self.x() + self.y() + self.z()
    }
}
impl<T: Element + Add<Output = T>> VecNComponentSum for Vec4<T> {
    type ComponentSum = T::Output;
    #[inline(always)]
    fn component_sum(self) -> Self::ComponentSum {
        self.x() + self.y() + self.z() + self.w()
    }
}
