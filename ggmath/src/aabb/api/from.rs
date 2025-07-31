use super::*;

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    /// Creates an `Aabb` from its minimum corner and size.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_min_size(
        min: Vector<N, T, impl VecAlignment>,
        size: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: min.to_layout(),
                    size: size.to_layout(),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: (min + T::aabb_div_vector_by_two(size)).to_layout(),
                    extents: T::aabb_div_vector_by_two(size).to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: min.to_layout(),
                    max: (min + size).to_layout(),
                },
            },
        )
    }

    /// Creates an `Aabb` from its maximum corner and size.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_max_size(
        max: Vector<N, T, impl VecAlignment>,
        size: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: (max - size).to_layout(),
                    size: size.to_layout(),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: ((max - size) + T::aabb_div_vector_by_two(size)).to_layout(),
                    extents: T::aabb_div_vector_by_two(size).to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: (max - size).to_layout(),
                    max: max.to_layout(),
                },
            },
        )
    }

    /// Creates an `Aabb` from its center and size.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_center_size(
        center: Vector<N, T, impl VecAlignment>,
        size: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: (center - T::aabb_div_vector_by_two(size)).to_layout(),
                    size: size.to_layout(),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: center.to_layout(),
                    extents: T::aabb_div_vector_by_two(size).to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: (center - T::aabb_div_vector_by_two(size)).to_layout(),
                    max: (center - T::aabb_div_vector_by_two(size) + size).to_layout(),
                },
            },
        )
    }

    /// Creates an `Aabb` from its minimum corner and extents.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_min_extents(
        min: Vector<N, T, impl VecAlignment>,
        extents: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: min.to_layout(),
                    size: T::aabb_mul_vector_by_two(extents.to_layout()),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: (min + extents).to_layout(),
                    extents: extents.to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: min.to_layout(),
                    max: (min + T::aabb_mul_vector_by_two(extents)).to_layout(),
                },
            },
        )
    }

    /// Creates an `Aabb` from its maximum corner and extents.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_max_extents(
        max: Vector<N, T, impl VecAlignment>,
        extents: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: (max - T::aabb_mul_vector_by_two(extents)).to_layout(),
                    size: (T::aabb_mul_vector_by_two(extents)).to_layout(),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: (max - extents).to_layout(),
                    extents: extents.to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: (max - T::aabb_mul_vector_by_two(extents)).to_layout(),
                    max: max.to_layout(),
                },
            },
        )
    }

    /// Creates an `Aabb` from its center and extents.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_center_extents(
        center: Vector<N, T, impl VecAlignment>,
        extents: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: (center - extents).to_layout(),
                    size: T::aabb_mul_vector_by_two(extents).to_layout(),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: center.to_layout(),
                    extents: extents.to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: (center - extents).to_layout(),
                    max: (center + extents).to_layout(),
                },
            },
        )
    }

    /// Creates an `Aabb` from its minimum and maximum corners.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_min_max(
        min: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: min.to_layout(),
                    size: (max - min).to_layout(),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: (min + T::aabb_div_vector_by_two(max - min)).to_layout(),
                    extents: T::aabb_div_vector_by_two(max - min).to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: min.to_layout(),
                    max: max.to_layout(),
                },
            },
        )
    }

    /// Creates an `Aabb` from its minimum corner and center.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_min_center(
        min: Vector<N, T, impl VecAlignment>,
        center: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: min.to_layout(),
                    size: T::aabb_mul_vector_by_two(center - min).to_layout(),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: center.to_layout(),
                    extents: (center - min).to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: min.to_layout(),
                    max: (center + (center - min)).to_layout(),
                },
            },
        )
    }

    /// Creates an `Aabb` from its center and maximum corner.
    ///
    /// Note:
    /// Sometimes int-aabbs can loose precision when initializing them depending on their `AabbRepr`.
    /// The exact behviour of the rounding is not specified yet.
    #[inline(always)]
    pub fn from_center_max(
        center: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        Self::resolved(
            Aabb {
                inner: InnerCorneredAabb {
                    min: (center - (max - center)).to_layout(),
                    size: T::aabb_mul_vector_by_two(max - center).to_layout(),
                },
            },
            Aabb {
                inner: InnerCenteredAabb {
                    center: center.to_layout(),
                    extents: (max - center).to_layout(),
                },
            },
            Aabb {
                inner: InnerMinMaxedAabb {
                    min: (center - (max - center)).to_layout(),
                    max: max.to_layout(),
                },
            },
        )
    }
}
