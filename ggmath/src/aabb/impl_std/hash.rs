use std::hash::Hash;

use super::*;

impl<const N: usize, T: AabbScalar + Hash, A: VecAlignment, R: AabbRepr> Hash for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.resolve() {
            ResolvedAabb::Cornered(rect) => rect.hash(state),
            ResolvedAabb::Centered(rect) => rect.hash(state),
            ResolvedAabb::MinMaxed(rect) => rect.hash(state),
        }
    }
}
