use super::*;

impl<
        const N: usize,
        T: ScalarRect,
        A: VecAlignment,
        R: RectRepr,
        ARhs: VecAlignment,
        RRhs: RectRepr,
    > PartialEq<Rectangle<N, T, ARhs, RRhs>> for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Rectangle<N, T, ARhs, RRhs>) -> bool {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => match other.resolve_repr() {
                ReprResolvedRectangle::Centered(other) => rect.inner.eq(&other.inner),
                ReprResolvedRectangle::Cornered(other) => other.eq(&rect),
                ReprResolvedRectangle::MinMaxed(other) => other.eq(&rect),
            },
            ReprResolvedRectangle::Cornered(rect) => rect.inner.eq(&other.into_cornered().inner),
            ReprResolvedRectangle::MinMaxed(rect) => rect.inner.eq(&other.into_min_maxed().inner),
        }
    }
}

impl<const N: usize, T: ScalarRect + Eq, A: VecAlignment, R: RectRepr> Eq for Rectangle<N, T, A, R> where
    ScalarCount<N>: VecLen
{
}
