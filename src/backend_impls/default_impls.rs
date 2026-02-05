use crate::{
    Alignment, F64VectorBackend, I8VectorBackend, I16VectorBackend, I32VectorBackend,
    I64VectorBackend, I128VectorBackend, IsizeVectorBackend, Length, ScalarBackend,
    SupportedLength, U8VectorBackend, U16VectorBackend, U32VectorBackend, U64VectorBackend,
    U128VectorBackend, UsizeVectorBackend,
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
