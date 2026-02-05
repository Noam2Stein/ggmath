use crate::{
    Alignment, F64VectorBackend, I8VectorBackend, I16VectorBackend, I32VectorBackend,
    I64VectorBackend, I128VectorBackend, IsizeVectorBackend, Length, Scalar, ScalarBackend,
    ScalarRepr, SupportedLength, U8VectorBackend, U16VectorBackend, U32VectorBackend,
    U64VectorBackend, U128VectorBackend, UsizeVectorBackend,
    utils::{Repr2, Repr3, Repr4},
};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for f64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i8 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i16 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i128 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for isize where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u8 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u16 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u128 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for usize where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for bool where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> F64VectorBackend<N, A> for f64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I8VectorBackend<N, A> for i8 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I16VectorBackend<N, A> for i16 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I32VectorBackend<N, A> for i32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I64VectorBackend<N, A> for i64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I128VectorBackend<N, A> for i128 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> IsizeVectorBackend<N, A> for isize where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> U8VectorBackend<N, A> for u8 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> U16VectorBackend<N, A> for u16 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> U32VectorBackend<N, A> for u32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> U64VectorBackend<N, A> for u64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> U128VectorBackend<N, A> for u128 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> UsizeVectorBackend<N, A> for usize where
    Length<N>: SupportedLength
{
}

macro_rules! impl_scalar_repr {
    ($T:ty) => {
        unsafe impl ScalarRepr for $T {
            type VectorRepr<const N: usize, T, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr2<T>, Repr3<T>, Repr4<T>>
            where
                Length<N>: SupportedLength,
                T: Scalar;
        }
    };
}
impl_scalar_repr!(());
impl_scalar_repr!(i8);
impl_scalar_repr!(i16);
impl_scalar_repr!(i64);
impl_scalar_repr!(i128);
