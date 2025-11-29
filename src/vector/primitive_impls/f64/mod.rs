use crate::vector::{
    Alignment, Length, ScalarBackend, SupportedLength, primitive_apis::f64::FloatBackend,
};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for f64
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [f64; N];
}

impl<const N: usize, A: Alignment> FloatBackend<N, A> for f64 where Length<N>: SupportedLength {}
