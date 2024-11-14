use super::*;

pub struct RectMinMaxed;

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

    fn intersects<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        other: Rectangle<N, T, impl VecAlignment, impl RectRepr>,
    ) -> bool
    where
        ScalarCount<N>: VecLen<N>,
    {
        (0..N).all(|i| rect.min()[i] < other.max()[i] && other.min()[i] < rect.max()[i])
    }
    fn intersection<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        other: Rectangle<N, T, impl VecAlignment, impl RectRepr>,
    ) -> Option<Rectangle<N, T, A, Self>>
    where
        ScalarCount<N>: VecLen<N>,
    {
        if rect.intersects(other) {
            Some(Rectangle::from_min_max(
                Vector::from_array(array::from_fn(|i| {
                    if rect.min()[i] > other.min()[i] {
                        rect.min()[i]
                    } else {
                        other.min()[i]
                    }
                })),
                Vector::from_array(array::from_fn(|i| {
                    if rect.max()[i] < other.min()[i] {
                        rect.max()[i]
                    } else {
                        other.max()[i]
                    }
                })),
            ))
        } else {
            None
        }
    }

    fn display_fmt<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result
    where
        ScalarCount<N>: VecLen<N>,
    {
        write!(f, "{{ min: {}, max: {} }}", rect.min(), rect.max())
    }
}
