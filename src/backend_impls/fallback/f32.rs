use crate::{Alignment, Length, PrimitiveFloatBackend, ScalarBackend, SupportedLength};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for f32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> PrimitiveFloatBackend<N, A> for f32 where
    Length<N>: SupportedLength
{
}
