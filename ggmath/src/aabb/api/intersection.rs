use super::*;

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub fn intersects(self, other: Aabb<N, T, impl VecAlignment, impl AabbRepr>) -> bool {
        match (self.resolve(), other.resolve()) {
            (ResolvedAabb::Centered(rect), ResolvedAabb::Centered(other)) => {
                let abs_diff = T::aabb_vector_abs_diff(rect.center(), other.center());
                let extents_sum = rect.extents() + other.extents();

                (0..N).all(|i| abs_diff[i] < extents_sum[i])
            }

            (ResolvedAabb::Centered(rect), _) => other.intersects(rect),

            (ResolvedAabb::Cornered(rect), _) => {
                (0..N).all(|i| rect.min()[i] < other.max()[i] && other.min()[i] < rect.max()[i])
            }

            (ResolvedAabb::MinMaxed(rect), _) => {
                (0..N).all(|i| rect.min()[i] < other.max()[i] && other.min()[i] < rect.max()[i])
            }
        }
    }

    #[inline(always)]
    pub fn intersection(self, other: Aabb<N, T, impl VecAlignment, impl AabbRepr>) -> Option<Self> {
        if self.intersects(other) {
            Some(Aabb::from_min_max(
                T::aabb_vector_max(self.min(), other.min()),
                T::aabb_vector_min(self.max(), other.max()),
            ))
        } else {
            None
        }
    }
}
