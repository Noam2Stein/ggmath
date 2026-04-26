use crate::{Alignment, Length, PrimitiveFloatVectorBackend, ScalarBackend, SupportedLength};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for f32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> PrimitiveFloatVectorBackend<N, A> for f32 where
    Length<N>: SupportedLength
{
}
