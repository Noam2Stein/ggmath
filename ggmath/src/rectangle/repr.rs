use crate::construct::InnerConstruct;

use super::*;

pub struct RectCornered;
pub struct RectCentered;
pub struct RectMinMaxed;

pub trait RectRepr: Sized {
    type InnerRectangle<const N: usize, T: ScalarNum, A: VecAlignment>: InnerConstruct + PartialEq
    where
        ScalarCount<N>: VecLen<N>;

    fn from_min_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_max_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_center_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_min_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_max_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_center_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_min_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_min_center<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        center: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_center_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;

    fn min<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
    fn max<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
    fn center<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
    fn size<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
    fn extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
}

impl RectRepr for RectCornered {
    type InnerRectangle<const N: usize, T: ScalarNum, A: VecAlignment> = (Vector<N, T, A>, Vector<N, T, A>) where ScalarCount<N>: VecLen<N>;

    #[inline(always)]
    fn from_min_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle { inner: (min, size) }
    }
    #[inline(always)]
    fn from_max_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (max - size, size),
        }
    }
    #[inline(always)]
    fn from_center_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center - size / T::from(2), size),
        }
    }
    #[inline(always)]
    fn from_min_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (min, extents * T::from(2)),
        }
    }
    #[inline(always)]
    fn from_max_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (max - extents * T::from(2), extents * T::from(2)),
        }
    }
    #[inline(always)]
    fn from_center_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center - extents, extents),
        }
    }
    #[inline(always)]
    fn from_min_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (min, max - min),
        }
    }
    #[inline(always)]
    fn from_min_center<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        center: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (min, (center - min) * T::from(2)),
        }
    }
    #[inline(always)]
    fn from_center_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center - (max - center), (max - center) * T::from(2)),
        }
    }

    #[inline(always)]
    fn min<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.0
    }
    #[inline(always)]
    fn max<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.0 + rect.inner.1
    }
    #[inline(always)]
    fn center<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.0 + rect.inner.1 * T::from(2)
    }
    #[inline(always)]
    fn size<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.1
    }
    #[inline(always)]
    fn extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.1 / T::from(2)
    }
}

impl RectRepr for RectCentered {
    type InnerRectangle<const N: usize, T: ScalarNum, A: VecAlignment> =
        (Vector<N, T, A>, Vector<N, T, A>) where ScalarCount<N>: VecLen<N>;

    #[inline(always)]
    fn from_min_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (min + size / T::from(2), size / T::from(2)),
        }
    }
    #[inline(always)]
    fn from_max_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (max - size / T::from(2), size / T::from(2)),
        }
    }
    #[inline(always)]
    fn from_center_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center, size / T::from(2)),
        }
    }
    #[inline(always)]
    fn from_min_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (min + extents, extents),
        }
    }
    #[inline(always)]
    fn from_max_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (max - extents, extents),
        }
    }
    #[inline(always)]
    fn from_center_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center, extents),
        }
    }
    #[inline(always)]
    fn from_min_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: ((min + max) / T::from(2), (max - min) / T::from(2)),
        }
    }
    #[inline(always)]
    fn from_min_center<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        center: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center, center - min),
        }
    }
    #[inline(always)]
    fn from_center_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center, max - center),
        }
    }

    #[inline(always)]
    fn min<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.0 - rect.inner.1
    }
    #[inline(always)]
    fn max<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.0 + rect.inner.1
    }
    #[inline(always)]
    fn center<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.0
    }
    #[inline(always)]
    fn size<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.1 * T::from(2)
    }
    #[inline(always)]
    fn extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.1
    }
}

impl RectRepr for RectMinMaxed {
    type InnerRectangle<const N: usize, T: ScalarNum, A: VecAlignment> = (Vector<N, T, A>, Vector<N, T, A>) where ScalarCount<N>: VecLen<N>;

    #[inline(always)]
    fn from_min_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (min, min + size),
        }
    }
    #[inline(always)]
    fn from_max_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (max - size, size),
        }
    }
    #[inline(always)]
    fn from_center_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center - size / T::from(2), center + size / T::from(2)),
        }
    }
    #[inline(always)]
    fn from_min_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (min, min + extents * T::from(2)),
        }
    }
    #[inline(always)]
    fn from_max_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (max - extents * T::from(2), max),
        }
    }
    #[inline(always)]
    fn from_center_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center - extents, center + extents),
        }
    }
    #[inline(always)]
    fn from_min_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle { inner: (min, max) }
    }
    #[inline(always)]
    fn from_min_center<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        center: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (min, center + (center - min)),
        }
    }
    #[inline(always)]
    fn from_center_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>,
    {
        Rectangle {
            inner: (center - (max - center), max),
        }
    }

    #[inline(always)]
    fn min<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.0
    }
    #[inline(always)]
    fn max<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.1
    }
    #[inline(always)]
    fn center<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        (rect.inner.0 + rect.inner.1) / T::from(2)
    }
    #[inline(always)]
    fn size<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        rect.inner.1 - rect.inner.0
    }
    #[inline(always)]
    fn extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>,
    {
        (rect.inner.1 - rect.inner.0) / T::from(2)
    }
}
