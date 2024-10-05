use super::*;

pub trait SignedElement:
    NumElement
    + Signed
    + ElementNeg<Output = Self>
    + SignedElementVecFns<2>
    + SignedElementVecFns<3>
    + SignedElementVecFns<4>
{
    fn neg_one() -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool;
}
pub trait SignedElementVecFns<const N: usize>: NumElement
where
    MaybeVecNum<N>: VecNum,
{
    fn neg_one() -> VecN<N, Self>;
    fn abs(value: VecN<N, Self>) -> VecN<N, Self>;
    fn signum(value: VecN<N, Self>) -> VecN<N, Self>;
    fn are_positive(value: VecN<N, Self>) -> VecN<N, bool>;
    fn are_negative(value: VecN<N, Self>) -> VecN<N, bool>;
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
