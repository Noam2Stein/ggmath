use super::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    Usize<N>: VecLen,
{
    pub fn from_min_size(
        min: Vector<N, T, impl VecAlignment>,
        size: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Rectangle {
                inner: InnerCorneredRect {
                    min: min.to_layout(),
                    size: size.to_layout(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: (min + T::rect_div_vector_by_two(size)).to_layout(),
                    extents: T::rect_div_vector_by_two(size).to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: min.to_layout(),
                    max: (min + size).to_layout(),
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
                    min: (max - size).to_layout(),
                    size: size.to_layout(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: ((max - size) + T::rect_div_vector_by_two(size)).to_layout(),
                    extents: T::rect_div_vector_by_two(size).to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (max - size).to_layout(),
                    max: max.to_layout(),
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
                    min: (center - T::rect_div_vector_by_two(size)).to_layout(),
                    size: size.to_layout(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: center.to_layout(),
                    extents: T::rect_div_vector_by_two(size).to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (center - T::rect_div_vector_by_two(size)).to_layout(),
                    max: (center - T::rect_div_vector_by_two(size) + size).to_layout(),
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
                    min: min.to_layout(),
                    size: T::rect_mul_vector_by_two(extents.to_layout()),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: (min + extents).to_layout(),
                    extents: extents.to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: min.to_layout(),
                    max: (min + T::rect_mul_vector_by_two(extents)).to_layout(),
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
                    min: (max - T::rect_mul_vector_by_two(extents)).to_layout(),
                    size: (T::rect_mul_vector_by_two(extents)).to_layout(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: (max - extents).to_layout(),
                    extents: extents.to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (max - T::rect_mul_vector_by_two(extents)).to_layout(),
                    max: max.to_layout(),
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
                    min: (center - extents).to_layout(),
                    size: T::rect_mul_vector_by_two(extents).to_layout(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: center.to_layout(),
                    extents: extents.to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (center - extents).to_layout(),
                    max: (center + extents).to_layout(),
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
                    min: min.to_layout(),
                    size: (max - min).to_layout(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: (min + T::rect_div_vector_by_two(max - min)).to_layout(),
                    extents: T::rect_div_vector_by_two(max - min).to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: min.to_layout(),
                    max: max.to_layout(),
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
                    min: min.to_layout(),
                    size: T::rect_mul_vector_by_two(center - min).to_layout(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: center.to_layout(),
                    extents: (center - min).to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: min.to_layout(),
                    max: (center + (center - min)).to_layout(),
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
                    min: (center - (max - center)).to_layout(),
                    size: T::rect_mul_vector_by_two(max - center).to_layout(),
                },
            },
            Rectangle {
                inner: InnerCenteredRect {
                    center: center.to_layout(),
                    extents: (max - center).to_layout(),
                },
            },
            Rectangle {
                inner: InnerMinMaxedRect {
                    min: (center - (max - center)).to_layout(),
                    max: max.to_layout(),
                },
            },
        )
    }
}
