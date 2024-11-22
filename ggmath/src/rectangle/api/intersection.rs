use super::*;

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn intersects(self, other: Rectangle<N, T, impl VecAlignment, impl RectRepr>) -> bool {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => match self.resolve_repr() {
                ReprResolvedRectangle::Centered(other) => (0..N).all(|i| {
                    rect.center()[i].abs_diff(&other.center()[i])
                        < rect.extents()[i] + other.extents()[i]
                }),
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

    #[inline(always)]
    pub fn intersection(
        self,
        other: Rectangle<N, T, impl VecAlignment, impl RectRepr>,
    ) -> Option<Self> {
        if self.intersects(other) {
            Some(Rectangle::from_min_max(
                self.min().max(other.min()),
                self.max().min(other.max()),
            ))
        } else {
            None
        }
    }
}
