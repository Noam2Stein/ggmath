use super::*;

impl<const N: usize, T: ScalarRect, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn from_min_size(min: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle { inner: [min, size] },
            || Rectangle::from_min_extents(min, size / T::u7(2)),
            || Rectangle::from_min_max(min, min + size),
        )
    }
    #[inline(always)]
    pub fn from_max_size(max: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle::from_min_size(max - size, size),
            || Rectangle::from_min_size(max - size, size),
            || Rectangle::from_min_max(max - size, max),
        )
    }
    #[inline(always)]
    pub fn from_center_size(center: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle::from_max_size(center + size / T::u7(2), size),
            || Rectangle::from_center_extents(center, size / T::u7(2)),
            || Rectangle::from_max_size(center + size / T::u7(2), size),
        )
    }

    #[inline(always)]
    pub fn from_min_extents(min: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle::from_min_size(min, extents + extents),
            || Rectangle::from_center_extents(min + extents, extents),
            || Rectangle::from_min_size(min, extents + extents),
        )
    }
    #[inline(always)]
    pub fn from_max_extents(max: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle::from_max_size(max, extents + extents),
            || Rectangle::from_center_extents(max - extents, extents),
            || Rectangle::from_max_size(max, extents + extents),
        )
    }
    #[inline(always)]
    pub fn from_center_extents(center: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle::from_min_size(center - extents, extents + extents),
            || Rectangle {
                inner: [center, extents],
            },
            || Rectangle::from_min_max(center - extents, center + extents),
        )
    }

    #[inline(always)]
    pub fn from_min_max(min: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle::from_min_size(min, max - min),
            || Rectangle::from_min_size(min, max - min),
            || Rectangle { inner: [min, max] },
        )
    }
    #[inline(always)]
    pub fn from_min_center(min: Vector<N, T, A>, center: Vector<N, T, A>) -> Self {
        Rectangle::from_center_extents(center, center - min)
    }
    #[inline(always)]
    pub fn from_center_max(center: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        Rectangle::from_center_extents(center, max - center)
    }
}
