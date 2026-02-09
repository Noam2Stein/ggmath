use crate::{
    Alignment, Length, Scalar, ScalarRepr, SupportedLength,
    utils::{Repr2, Repr3, Repr4},
};

macro_rules! impl_scalar_repr {
    ($T:ty) => {
        // SAFETY: Look at the safety note for each associated type.
        unsafe impl ScalarRepr for $T {
            // SAFETY: Select chooses `ReprN` from `Repr2`, `Repr3`, and
            // `Repr4`. Each type is guaranteed to be a simple struct equivalent
            // to `[T; N]`. The vector is made out of consecutive values of `T`,
            // the vector is guaranteed to have the size and alignment of
            // `[T; N]`, and two scalars that share `$T` must have the same size
            // thus the vectors have the same size and element positions too.
            type VectorRepr<const N: usize, T, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr2<T>, Repr3<T>, Repr4<T>>
            where
                Length<N>: SupportedLength,
                T: Scalar;

            // SAFETY: All types are equivalent to nested vectors making
            // `[T; N * N]`.
            type MatrixRepr<const N: usize, T, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr4<T>, Repr3<Repr3<T>>, Repr4<Repr4<T>>>
            where
                Length<N>: SupportedLength,
                T: Scalar;

            // SAFETY: Select chooses `ReprN` from `Repr2`, `Repr3`, and
            // `Repr4`. Each type is guaranteed to be a simple struct equivalent
            // to `[bool; N]`. `[bool; N]` has no uninitialized bytes, and is
            // zeroable. Masks of `$T` have the same representation no matter
            // their `T` type.
            type MaskRepr<const N: usize, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr2<bool>, Repr3<bool>, Repr4<bool>>
            where
                Length<N>: SupportedLength;
        }
    };
}
impl_scalar_repr!(i32);
