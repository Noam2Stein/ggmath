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
    #[inline(always)]
    pub fn to_layout<AOutput: VecAlignment, ROutput: AabbRepr>(
        self,
    ) -> Aabb<N, T, AOutput, ROutput> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => {
                Aabb::from_center_extents(rect.inner.center, rect.inner.extents)
            }
            ResolvedRectangle::Cornered(rect) => {
                Aabb::from_min_size(rect.inner.min, rect.inner.size)
            }
            ResolvedRectangle::MinMaxed(rect) => Aabb::from_min_max(rect.inner.min, rect.inner.max),
        }
    }
}
