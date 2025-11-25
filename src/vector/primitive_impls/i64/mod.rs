use crate::{Alignment, Length, ScalarBackend, SupportedLength};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i64
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [i64; N];
}
