use super::*;

pub trait VecNWithT<const N: usize> {
    type WithT<T2: Element>: VecN<T2, N>;
}
impl<T: Element> VecNWithT<2> for Vec2<T> {
    type WithT<T2: Element> = Vec2<T2>;
}
impl<T: Element> VecNWithT<3> for Vec3<T> {
    type WithT<T2: Element> = Vec3<T2>;
}
impl<T: Element> VecNWithT<4> for Vec4<T> {
    type WithT<T2: Element> = Vec4<T2>;
}
