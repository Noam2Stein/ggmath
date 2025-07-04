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
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Rectangle<N, T, ARhs, RRhs>) -> bool {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => match other.resolve() {
                ResolvedRectangle::Centered(other) => {
                    rect.inner.center == other.inner.center
                        && rect.inner.extents == other.inner.extents
                }
                ResolvedRectangle::Cornered(other) => other.eq(&rect),
                ResolvedRectangle::MinMaxed(other) => other.eq(&rect),
            },
            ResolvedRectangle::Cornered(rect) => {
                rect.inner.min == other.min() && rect.inner.size == other.size()
            }
            ResolvedRectangle::MinMaxed(rect) => {
                rect.inner.min == other.min() && rect.inner.max == other.max()
            }
        }
    }
}

impl<const N: usize, T: RectScalar + Eq, A: VecAlignment, R: RectRepr> Eq for Rectangle<N, T, A, R> where
    MaybeVecLen<N>: VecLen
{
}
