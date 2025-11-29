use crate::vector::{Alignment, Length, ScalarBackend, SupportedLength};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u64
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [u64; N];
}
