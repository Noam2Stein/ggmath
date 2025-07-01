use super::*;

mod repr;
mod scalar;
pub use repr::*;
pub use scalar::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub fn into_aligned(self) -> Rectangle<N, T, VecAligned, R> {
        self.into_alignment()
    }
    #[inline(always)]
    pub fn into_packed(self) -> Rectangle<N, T, VecPacked, R> {
        self.into_alignment()
    }
    #[inline(always)]
    pub fn into_alignment<AOutput: VecAlignment>(self) -> Rectangle<N, T, AOutput, R> {
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => Rectangle::from_center_extents(
                rect.center().into_alignment(),
                rect.extents().into_alignment(),
            ),
            ReprResolvedRectangle::Cornered(rect) => {
                Rectangle::from_min_size(rect.min().into_alignment(), rect.size().into_alignment())
            }
            ReprResolvedRectangle::MinMaxed(rect) => {
                Rectangle::from_min_max(rect.min().into_alignment(), rect.max().into_alignment())
            }
        }
    }

    #[inline(always)]
    pub fn into_storage<AOutput: VecAlignment, ROutput: RectRepr>(
        self,
    ) -> Rectangle<N, T, AOutput, ROutput> {
        self.into_alignment().into_repr()
    }
}
