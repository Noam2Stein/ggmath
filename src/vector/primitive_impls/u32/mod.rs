use crate::{Alignment, Length, ScalarBackend, SupportedLength};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u32
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [u32; N];
}
