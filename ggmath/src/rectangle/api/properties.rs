use super::*;

impl<const N: usize, T: ScalarRect, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn min(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner[0] - rect.inner[1],
            ReprResolvedRectangle::Cornered(rect) => rect.inner[0],
            ReprResolvedRectangle::MinMaxed(rect) => rect.inner[0],
        }
    }
    #[inline(always)]
    pub fn max(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner[0] + rect.inner[1],
            ReprResolvedRectangle::Cornered(rect) => rect.inner[0] + rect.inner[1],
            ReprResolvedRectangle::MinMaxed(rect) => rect.inner[1],
        }
    }
    #[inline(always)]
    pub fn center(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner[0],
            ReprResolvedRectangle::Cornered(rect) => rect.inner[0] + rect.inner[1] / T::u7(2),
            ReprResolvedRectangle::MinMaxed(rect) => (rect.inner[0] + rect.inner[1]) / T::u7(2),
        }
    }
    #[inline(always)]
    pub fn size(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner[1] + rect.inner[1],
            ReprResolvedRectangle::Cornered(rect) => rect.inner[1],
            ReprResolvedRectangle::MinMaxed(rect) => rect.inner[1] - rect.inner[0],
        }
    }
    #[inline(always)]
    pub fn extents(self) -> Vector<N, T, A> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => rect.inner[1],
            ReprResolvedRectangle::Cornered(rect) => rect.inner[1] / T::u7(2),
            ReprResolvedRectangle::MinMaxed(rect) => (rect.inner[1] - rect.inner[0]) / T::u7(2),
        }
    }
}
