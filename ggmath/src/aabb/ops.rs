use super::*;

// Comparison

impl<
    const N: usize,
    T: AabbScalar + PartialEq<T2>,
    A: VecAlignment,
    R: AabbRepr,
    T2: AabbScalar + PartialEq,
    A2: VecAlignment,
> PartialEq<Aabb<N, T2, A2, R>> for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Aabb<N, T2, A2, R>) -> bool {
        match self.resolve() {
            ResolvedAabb::Cornered(aabb) => match other.resolve() {
                ResolvedAabb::Cornered(other) => {
                    aabb.inner.min == other.inner.min && aabb.inner.size == other.inner.size
                }
                _ => false,
            },
            ResolvedAabb::Centered(aabb) => match other.resolve() {
                ResolvedAabb::Centered(other) => {
                    aabb.inner.center == other.inner.center
                        && aabb.inner.extents == other.inner.extents
                }
                _ => false,
            },
            ResolvedAabb::MinMaxed(aabb) => match other.resolve() {
                ResolvedAabb::MinMaxed(other) => {
                    aabb.inner.min == other.inner.min && aabb.inner.max == other.inner.max
                }
                _ => false,
            },
        }
    }
}

impl<const N: usize, T: AabbScalar + Eq, A: VecAlignment, R: AabbRepr> Eq for Aabb<N, T, A, R> where
    Usize<N>: VecLen
{
}
