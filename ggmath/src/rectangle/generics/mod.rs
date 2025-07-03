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
                rect.center().to_storage(),
                rect.extents().to_storage(),
            ),
            ReprResolvedRectangle::Cornered(rect) => {
                Rectangle::from_min_size(rect.min().to_storage(), rect.size().to_storage())
            }
            ReprResolvedRectangle::MinMaxed(rect) => {
                Rectangle::from_min_max(rect.min().to_storage(), rect.max().to_storage())
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
