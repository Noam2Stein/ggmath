use super::*;

mod resolve;
pub use resolve::*;

mod aabb_scalar;
mod alignment;
mod repr;
pub use aabb_scalar::*;
pub use repr::*;

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    /// Converts the aabb to the specified memory-layout.
    ///
    /// This can swap the alignment and representation of the aabb.
    #[inline(always)]
    pub fn to_layout<AOutput: VecAlignment, ROutput: AabbRepr>(
        self,
    ) -> Aabb<N, T, AOutput, ROutput> {
        match self.resolve() {
            ResolvedAabb::Centered(rect) => {
                Aabb::from_center_extents(rect.inner.center, rect.inner.extents)
            }
            ResolvedAabb::Cornered(rect) => Aabb::from_min_size(rect.inner.min, rect.inner.size),
            ResolvedAabb::MinMaxed(rect) => Aabb::from_min_max(rect.inner.min, rect.inner.max),
        }
    }
}
