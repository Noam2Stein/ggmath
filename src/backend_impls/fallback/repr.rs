use crate::{
    Alignment, Length, Scalar, ScalarRepr, SupportedLength,
    utils::{Repr2, Repr3, Repr4},
};

macro_rules! impl_scalar_repr {
    ($T:ty) => {
        unsafe impl ScalarRepr for $T {
            type VectorRepr<const N: usize, T, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr2<T>, Repr3<T>, Repr4<T>>
            where
                Length<N>: SupportedLength,
                T: Scalar;

            type MaskRepr<const N: usize, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr2<bool>, Repr3<bool>, Repr4<bool>>
            where
                Length<N>: SupportedLength;
        }
    };
}
impl_scalar_repr!(i32);
