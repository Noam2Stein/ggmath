use super::*;

pub trait SignedElement:
    NumElement
    + Signed
    + ElementNeg<Output = Self>
    + SignedElementVecFns<2, Vec2<Self>>
    + SignedElementVecFns<3, Vec3<Self>>
    + SignedElementVecFns<4, Vec4<Self>>
{
    fn neg_one() -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool;
}
pub trait SignedElementVecFns<const N: usize, V: VecN<Self, N>>: NumElement {
    fn neg_one() -> V;
    fn abs(value: V) -> V;
    fn signum(value: V) -> V;
    fn are_positive(value: V) -> V::WithT<bool>;
    fn are_negative(value: V) -> V::WithT<bool>;
}

macro_rules! validate {
    ($ty:ty) => {
        const _: () = {
            const fn validate<T: SignedElement>() {}

            validate::<$ty>()
        };
    };
}
validate!(f32);
validate!(f64);
validate!(i8);
validate!(i16);
validate!(i32);
validate!(i64);
validate!(i128);
validate!(isize);
