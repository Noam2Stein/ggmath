use super::*;

impl<const N: usize, T: AabbScalar + Eq, A: VecAlignment, R: AabbRepr> Eq for Aabb<N, T, A, R> where
    Usize<N>: VecLen
{
}
