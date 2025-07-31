use super::*;

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    /// Checks if the aabb intersects with another aabb.
    ///
    /// Note:
    /// 2 aabbs that are exactly touching are NOT considered intersecting.
    ///
    /// Note:
    /// Currently if the 2 aabbs don't share the same `AabbRepr`,
    /// the function could have slight precision mistakes with int-aabbs.
    /// This will be fixed in the future.
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

    /// Returns the intersection of the aabb with another aabb.
    ///
    /// Note:
    /// This function is consistent `intersects` in terms of `Some` or `None`.
    /// This means that like in `intersects`, 2 aabbs that are exactly touching are NOT considered intersecting.
    /// So this function will return `None`, not an empty aabb.
    ///
    /// Note:
    /// Currently if the 2 aabbs don't share the same `AabbRepr`,
    /// the function could have slight precision mistakes with int-aabbs.
    /// This will be fixed in the future.
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
