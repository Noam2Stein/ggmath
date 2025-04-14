use super::*;

impl<
    const N: usize,
    T: RectScalar,
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
                ReprResolvedRectangle::Centered(other) => {
                    rect.inner.center == other.inner.center
                        && rect.inner.extents == other.inner.extents
                }
                ReprResolvedRectangle::Cornered(other) => other.eq(&rect),
                ReprResolvedRectangle::MinMaxed(other) => other.eq(&rect),
            },
            ReprResolvedRectangle::Cornered(rect) => {
                rect.inner.min == other.min() && rect.inner.size == other.size()
            }
            ReprResolvedRectangle::MinMaxed(rect) => {
                rect.inner.min == other.min() && rect.inner.max == other.max()
            }
        }
    }
}

impl<const N: usize, T: RectScalar + Eq, A: VecAlignment, R: RectRepr> Eq for Rectangle<N, T, A, R> where
    ScalarCount<N>: VecLen
{
}
