use super::*;

mod resolve;
pub use resolve::*;

mod alignment;
mod rect_scalar;
mod repr;
pub use rect_scalar::*;
pub use repr::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub fn into_alignment<AOutput: VecAlignment>(self) -> Rectangle<N, T, AOutput, R> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => {
                Rectangle::from_center_extents(rect.center(), rect.extents())
            }
            ResolvedRectangle::Cornered(rect) => Rectangle::from_min_size(rect.min(), rect.size()),
            ResolvedRectangle::MinMaxed(rect) => Rectangle::from_min_max(rect.min(), rect.max()),
        }
    }

    #[inline(always)]
    pub fn to_storage<AOutput: VecAlignment, ROutput: RectRepr>(
        self,
    ) -> Rectangle<N, T, AOutput, ROutput> {
        self.into_alignment().into_repr()
    }
}
