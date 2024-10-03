use super::*;

pub trait VecNProduct {
    type Product;
    fn product(self) -> Self::Product;
}
impl<T: Element + Mul<Output = T>> VecNProduct for Vec2<T> {
    type Product = T::Output;
    #[inline(always)]
    fn product(self) -> Self::Product {
        self.x() * self.y()
    }
}
impl<T: Element + Mul<Output = T>> VecNProduct for Vec3<T> {
    type Product = T::Output;
    #[inline(always)]
    fn product(self) -> Self::Product {
        self.x() * self.y() * self.z()
    }
}
impl<T: Element + Mul<Output = T>> VecNProduct for Vec4<T> {
    type Product = T::Output;
    #[inline(always)]
    fn product(self) -> Self::Product {
        self.x() * self.y() * self.z() * self.w()
    }
}
