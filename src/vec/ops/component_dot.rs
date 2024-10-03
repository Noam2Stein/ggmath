use super::*;

pub trait VecNComponentProduct {
    type ComponentProduct;
    fn component_dot(self) -> Self::ComponentProduct;
}
impl<T: Element + Mul<Output = T>> VecNComponentProduct for Vec2<T> {
    type ComponentProduct = T::Output;
    #[inline(always)]
    fn component_dot(self) -> Self::ComponentProduct {
        self.x() * self.y()
    }
}
impl<T: Element + Mul<Output = T>> VecNComponentProduct for Vec3<T> {
    type ComponentProduct = T::Output;
    #[inline(always)]
    fn component_dot(self) -> Self::ComponentProduct {
        self.x() * self.y() * self.z()
    }
}
impl<T: Element + Mul<Output = T>> VecNComponentProduct for Vec4<T> {
    type ComponentProduct = T::Output;
    #[inline(always)]
    fn component_dot(self) -> Self::ComponentProduct {
        self.x() * self.y() * self.z() * self.w()
    }
}
