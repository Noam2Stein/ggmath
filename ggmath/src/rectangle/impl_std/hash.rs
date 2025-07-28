use std::hash::Hash;

use super::*;

impl<const N: usize, T: RectScalar + Hash, A: VecAlignment, R: RectRepr> Hash
    for Rectangle<N, T, A, R>
where
    Usize<N>: VecLen,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.resolve() {
            ResolvedRectangle::Cornered(rect) => rect.hash(state),
            ResolvedRectangle::Centered(rect) => rect.hash(state),
            ResolvedRectangle::MinMaxed(rect) => rect.hash(state),
        }
    }
}
