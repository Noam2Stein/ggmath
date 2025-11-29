use crate::vector::{Alignment, Length, ScalarBackend, SupportedLength};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i32
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [i32; N];
}
