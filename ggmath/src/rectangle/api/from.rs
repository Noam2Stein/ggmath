use super::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    pub fn from_min_size(
        min: Vector<N, T, impl VecAlignment>,
        size: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: min.to_storage(),
                    size: size.to_storage(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: (min + T::rect_div_vector_by_two(size)).to_storage(),
                    extents: T::rect_div_vector_by_two(size).to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: min.to_storage(),
                    max: (min + size).to_storage(),
                },
            },
        )
    }

    pub fn from_max_size(
        max: Vector<N, T, impl VecAlignment>,
        size: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: (max - size).to_storage(),
                    size: size.to_storage(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: ((max - size) + T::rect_div_vector_by_two(size)).to_storage(),
                    extents: T::rect_div_vector_by_two(size).to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (max - size).to_storage(),
                    max: max.to_storage(),
                },
            },
        )
    }

    pub fn from_center_size(
        center: Vector<N, T, impl VecAlignment>,
        size: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: (center - T::rect_div_vector_by_two(size)).to_storage(),
                    size: size.to_storage(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: center.to_storage(),
                    extents: T::rect_div_vector_by_two(size).to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (center - T::rect_div_vector_by_two(size)).to_storage(),
                    max: (center - T::rect_div_vector_by_two(size) + size).to_storage(),
                },
            },
        )
    }

    pub fn from_min_extents(
        min: Vector<N, T, impl VecAlignment>,
        extents: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: min.to_storage(),
                    size: T::rect_mul_vector_by_two(extents.to_storage()),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: (min + extents).to_storage(),
                    extents: extents.to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: min.to_storage(),
                    max: (min + T::rect_mul_vector_by_two(extents)).to_storage(),
                },
            },
        )
    }

    pub fn from_max_extents(
        max: Vector<N, T, impl VecAlignment>,
        extents: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: (max - T::rect_mul_vector_by_two(extents)).to_storage(),
                    size: (T::rect_mul_vector_by_two(extents)).to_storage(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: (max - extents).to_storage(),
                    extents: extents.to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (max - T::rect_mul_vector_by_two(extents)).to_storage(),
                    max: max.to_storage(),
                },
            },
        )
    }

    pub fn from_center_extents(
        center: Vector<N, T, impl VecAlignment>,
        extents: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: (center - extents).to_storage(),
                    size: T::rect_mul_vector_by_two(extents).to_storage(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: center.to_storage(),
                    extents: extents.to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (center - extents).to_storage(),
                    max: (center + extents).to_storage(),
                },
            },
        )
    }

    pub fn from_min_max(
        min: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: min.to_storage(),
                    size: (max - min).to_storage(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: (min + T::rect_div_vector_by_two(max - min)).to_storage(),
                    extents: T::rect_div_vector_by_two(max - min).to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: min.to_storage(),
                    max: max.to_storage(),
                },
            },
        )
    }

    pub fn from_min_center(
        min: Vector<N, T, impl VecAlignment>,
        center: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: min.to_storage(),
                    size: T::rect_mul_vector_by_two(center - min).to_storage(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: center.to_storage(),
                    extents: (center - min).to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: min.to_storage(),
                    max: (center + (center - min)).to_storage(),
                },
            },
        )
    }

    pub fn from_center_max(
        center: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: (center - (max - center)).to_storage(),
                    size: T::rect_mul_vector_by_two(max - center).to_storage(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: center.to_storage(),
                    extents: (max - center).to_storage(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (center - (max - center)).to_storage(),
                    max: max.to_storage(),
                },
            },
        )
    }
}
