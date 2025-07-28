use super::*;

impl<const N: usize, T: RectScalar + Eq, A: VecAlignment, R: RectRepr> Eq for Rectangle<N, T, A, R> where
    Usize<N>: VecLen
{
}
