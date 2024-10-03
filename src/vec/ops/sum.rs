use super::*;

pub trait VecNSum {
    type Sum;
    fn sum(self) -> Self::Sum;
}
impl<T: Element + Add<Output = T>> VecNSum for Vec2<T> {
    type Sum = T::Output;
    #[inline(always)]
    fn sum(self) -> Self::Sum {
        self.x() + self.y()
    }
}
impl<T: Element + Add<Output = T>> VecNSum for Vec3<T> {
    type Sum = T::Output;
    #[inline(always)]
    fn sum(self) -> Self::Sum {
        self.x() + self.y() + self.z()
    }
}
impl<T: Element + Add<Output = T>> VecNSum for Vec4<T> {
    type Sum = T::Output;
    #[inline(always)]
    fn sum(self) -> Self::Sum {
        self.x() + self.y() + self.z() + self.w()
    }
}
