use super::*;

pub struct RectCentered;

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
            inner: (min + size / T::from(2).unwrap(), size / T::from(2).unwrap()),
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
            inner: (max - size / T::from(2).unwrap(), size / T::from(2).unwrap()),
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
            inner: (center, size / T::from(2).unwrap()),
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
            inner: (
                (min + max) / T::from(2).unwrap(),
                (max - min) / T::from(2).unwrap(),
            ),
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
        rect.inner.1 * T::from(2).unwrap()
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

    fn intersects<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        other: Rectangle<N, T, impl VecAlignment, impl RectRepr>,
    ) -> bool
    where
        ScalarCount<N>: VecLen<N>,
    {
        (0..N).all(|i| {
            rect.extents()[i] + other.extents()[i]
                < if rect.center()[i] > other.center()[i] {
                    rect.center()[i] - other.center()[i]
                } else {
                    other.center()[i] - rect.center()[i]
                }
        })
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

    fn debug_fmt<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result
    where
        ScalarCount<N>: VecLen<N>,
    {
        write!(
            f,
            "{{ center: {:?}, extents: {:?} }}",
            rect.center(),
            rect.extents()
        )
    }
    fn display_fmt<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result
    where
        ScalarCount<N>: VecLen<N>,
    {
        write!(
            f,
            "{{ center: {}, extents: {} }}",
            rect.center(),
            rect.extents()
        )
    }
}
