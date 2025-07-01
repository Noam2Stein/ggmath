use super::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub fn intersects(self, other: Rectangle<N, T, impl VecAlignment, impl RectRepr>) -> bool {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => match self.resolve_repr() {
                ReprResolvedRectangle::Centered(other) => {
                    let abs_diff = T::rect_vector_abs_diff(rect.center(), other.center());
                    let extents_sum = rect.extents() + other.extents();

                    (0..N).all(|i| abs_diff[i] < extents_sum[i])
                }
                ReprResolvedRectangle::Cornered(other) => other.intersects(rect),
                ReprResolvedRectangle::MinMaxed(other) => other.intersects(rect),
            },
            ReprResolvedRectangle::Cornered(rect) => {
                (0..N).all(|i| rect.min()[i] < other.max()[i] && other.min()[i] < rect.max()[i])
            }
            ReprResolvedRectangle::MinMaxed(rect) => {
                (0..N).all(|i| rect.min()[i] < other.max()[i] && other.min()[i] < rect.max()[i])
            }
        }
    }

    pub fn intersection(
        self,
        other: Rectangle<N, T, impl VecAlignment, impl RectRepr>,
    ) -> Option<Self> {
        if self.intersects(other) {
            Some(Rectangle::from_min_max(
                T::rect_vector_max(self.min(), other.min()),
                T::rect_vector_min(self.max(), other.max()),
            ))
        } else {
            None
        }
    }
}
