use super::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    pub fn from_min_size(min: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect { min, size },
            },
            || Rectangle {
                inner: InnerCenteredRect {
                    center: min + T::rect_div_vector_by_two(size),
                    extents: T::rect_div_vector_by_two(size),
                },
            },
            || Rectangle {
                inner: InnerMinMaxedRect {
                    min,
                    max: min + size,
                },
            },
        )
    }
    pub fn from_max_size(max: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect {
                    min: max - size,
                    size,
                },
            },
            || Rectangle {
                inner: InnerCenteredRect {
                    center: (max - size) + T::rect_div_vector_by_two(size),
                    extents: T::rect_div_vector_by_two(size),
                },
            },
            || Rectangle {
                inner: InnerMinMaxedRect {
                    min: max - size,
                    max,
                },
            },
        )
    }
    pub fn from_center_size(center: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect {
                    min: center - T::rect_div_vector_by_two(size),
                    size,
                },
            },
            || Rectangle {
                inner: InnerCenteredRect {
                    center,
                    extents: T::rect_div_vector_by_two(size),
                },
            },
            || Rectangle {
                inner: InnerMinMaxedRect {
                    min: center - T::rect_div_vector_by_two(size),
                    max: center - T::rect_div_vector_by_two(size) + size,
                },
            },
        )
    }

    pub fn from_min_extents(min: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect {
                    min,
                    size: T::rect_mul_vector_by_two(extents),
                },
            },
            || Rectangle {
                inner: InnerCenteredRect {
                    center: min + extents,
                    extents,
                },
            },
            || Rectangle {
                inner: InnerMinMaxedRect {
                    min,
                    max: min + T::rect_mul_vector_by_two(extents),
                },
            },
        )
    }
    pub fn from_max_extents(max: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect {
                    min: max - T::rect_mul_vector_by_two(extents),
                    size: T::rect_mul_vector_by_two(extents),
                },
            },
            || Rectangle {
                inner: InnerCenteredRect {
                    center: max - extents,
                    extents,
                },
            },
            || Rectangle {
                inner: InnerMinMaxedRect {
                    min: max - T::rect_mul_vector_by_two(extents),
                    max,
                },
            },
        )
    }
    pub fn from_center_extents(center: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect {
                    min: center - extents,
                    size: T::rect_mul_vector_by_two(extents),
                },
            },
            || Rectangle {
                inner: InnerCenteredRect { center, extents },
            },
            || Rectangle {
                inner: InnerMinMaxedRect {
                    min: center - extents,
                    max: center + extents,
                },
            },
        )
    }

    pub fn from_min_max(min: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect {
                    min,
                    size: max - min,
                },
            },
            || Rectangle {
                inner: InnerCenteredRect {
                    center: min + T::rect_div_vector_by_two(max - min),
                    extents: T::rect_div_vector_by_two(max - min),
                },
            },
            || Rectangle {
                inner: InnerMinMaxedRect { min, max },
            },
        )
    }
    pub fn from_min_center(min: Vector<N, T, A>, center: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect {
                    min,
                    size: T::rect_mul_vector_by_two(center - min),
                },
            },
            || Rectangle {
                inner: InnerCenteredRect {
                    center,
                    extents: center - min,
                },
            },
            || Rectangle {
                inner: InnerMinMaxedRect {
                    min,
                    max: center + (center - min),
                },
            },
        )
    }
    pub fn from_center_max(center: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        Self::from_resolved_repr_fns(
            || Rectangle {
                inner: InnerCorneredRect {
                    min: center - (max - center),
                    size: T::rect_mul_vector_by_two(max - center),
                },
            },
            || Rectangle {
                inner: InnerCenteredRect {
                    center,
                    extents: max - center,
                },
            },
            || Rectangle {
                inner: InnerMinMaxedRect {
                    min: center - (max - center),
                    max,
                },
            },
        )
    }
}
