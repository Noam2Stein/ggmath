use super::*;

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn from_min_size(min: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle { inner: [min, size] },
            || Rectangle {
                inner: [min + size / T::from(2).unwrap(), size / T::from(2).unwrap()],
            },
            || Rectangle {
                inner: [min, min + size],
            },
        )
    }
    #[inline(always)]
    pub fn from_max_size(max: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: [max - size, size],
            },
            || Rectangle {
                inner: [max - size / T::from(2).unwrap(), size / T::from(2).unwrap()],
            },
            || Rectangle {
                inner: [max - size, max],
            },
        )
    }
    #[inline(always)]
    pub fn from_center_size(center: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_center_extents(center, size / T::from(2).unwrap())
    }

    #[inline(always)]
    pub fn from_min_extents(min: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: [min, extents + extents],
            },
            || Rectangle {
                inner: [min + extents, extents],
            },
            || Rectangle {
                inner: [min, min + extents + extents],
            },
        )
    }
    #[inline(always)]
    pub fn from_max_extents(max: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: [max - extents - extents, extents + extents],
            },
            || Rectangle {
                inner: [max - extents, extents],
            },
            || Rectangle {
                inner: [max - extents - extents, max],
            },
        )
    }
    #[inline(always)]
    pub fn from_center_extents(center: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: [center - extents, extents + extents],
            },
            || Rectangle {
                inner: [center, extents],
            },
            || Rectangle {
                inner: [center - extents, center + extents],
            },
        )
    }

    #[inline(always)]
    pub fn from_min_max(min: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: [min, max - min],
            },
            || Rectangle {
                inner: [
                    (min + max) / T::from(2).unwrap(),
                    (max - min) / T::from(2).unwrap(),
                ],
            },
            || Rectangle { inner: [min, max] },
        )
    }
    #[inline(always)]
    pub fn from_min_center(min: Vector<N, T, A>, center: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: [min, center - min + center - min],
            },
            || Rectangle {
                inner: [center, center - min],
            },
            || Rectangle {
                inner: [min, center + center - min],
            },
        )
    }
    #[inline(always)]
    pub fn from_center_max(center: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: [center + center - max, max - center + max - center],
            },
            || Rectangle {
                inner: [center, max - center],
            },
            || Rectangle {
                inner: [center + center - max, center + max - center],
            },
        )
    }
}
