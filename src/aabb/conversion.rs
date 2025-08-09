use std::mem::transmute_copy;

use super::*;

// T

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    /// Converts the aabb to a different scalar type using the `From` trait.
    #[inline(always)]
    pub fn to_t<T2: AabbScalar + From<T>>(self) -> Aabb<N, T2, A, R> {
        self.resolved_map(
            |aabb| Aabb {
                inner: InnerCorneredAabb {
                    min: aabb.inner.min.to_t(),
                    size: aabb.inner.size.to_t(),
                },
            },
            |aabb| Aabb {
                inner: InnerCenteredAabb {
                    center: aabb.inner.center.to_t(),
                    extents: aabb.inner.extents.to_t(),
                },
            },
            |aabb| Aabb {
                inner: InnerMinMaxedAabb {
                    min: aabb.inner.min.to_t(),
                    max: aabb.inner.max.to_t(),
                },
            },
        )
    }
}

// A

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    /// Aligns the aabb to `VecAligned`.
    #[inline(always)]
    pub const fn align(self) -> Aabb<N, T, VecAligned, R> {
        match self.resolve() {
            ResolvedAabb::Centered(rect) => unsafe {
                transmute_copy::<Aabb<N, T, VecAligned, AabbCentered>, Aabb<N, T, VecAligned, R>>(
                    &Aabb {
                        inner: InnerCenteredAabb {
                            center: rect.inner.center.align(),
                            extents: rect.inner.extents.align(),
                        },
                    },
                )
            },
            ResolvedAabb::Cornered(rect) => unsafe {
                transmute_copy::<Aabb<N, T, VecAligned, AabbCornered>, Aabb<N, T, VecAligned, R>>(
                    &Aabb {
                        inner: InnerCorneredAabb {
                            min: rect.inner.min.align(),
                            size: rect.inner.size.align(),
                        },
                    },
                )
            },
            ResolvedAabb::MinMaxed(rect) => unsafe {
                transmute_copy::<Aabb<N, T, VecAligned, AabbMinMaxed>, Aabb<N, T, VecAligned, R>>(
                    &Aabb {
                        inner: InnerMinMaxedAabb {
                            min: rect.inner.min.align(),
                            max: rect.inner.max.align(),
                        },
                    },
                )
            },
        }
    }

    /// Unaligns the aabb to `VecPacked`.
    #[inline(always)]
    pub const fn unalign(self) -> Aabb<N, T, VecPacked, R> {
        match self.resolve() {
            ResolvedAabb::Centered(rect) => unsafe {
                transmute_copy::<Aabb<N, T, VecPacked, AabbCentered>, Aabb<N, T, VecPacked, R>>(
                    &Aabb {
                        inner: InnerCenteredAabb {
                            center: rect.inner.center.unalign(),
                            extents: rect.inner.extents.unalign(),
                        },
                    },
                )
            },
            ResolvedAabb::Cornered(rect) => unsafe {
                transmute_copy::<Aabb<N, T, VecPacked, AabbCornered>, Aabb<N, T, VecPacked, R>>(
                    &Aabb {
                        inner: InnerCorneredAabb {
                            min: rect.inner.min.unalign(),
                            size: rect.inner.size.unalign(),
                        },
                    },
                )
            },
            ResolvedAabb::MinMaxed(rect) => unsafe {
                transmute_copy::<Aabb<N, T, VecPacked, AabbMinMaxed>, Aabb<N, T, VecPacked, R>>(
                    &Aabb {
                        inner: InnerMinMaxedAabb {
                            min: rect.inner.min.unalign(),
                            max: rect.inner.max.unalign(),
                        },
                    },
                )
            },
        }
    }
}

// R

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    /// Converts the aabb to `AabbCornered`.
    /// This means the aabb will be represented by its minimum corner and size.
    #[inline(always)]
    pub fn cornered(self) -> Aabb<N, T, A, AabbCornered> {
        self.to_layout()
    }

    /// Converts the aabb to `AabbCentered`.
    /// This means the aabb will be represented by its center and extents.
    #[inline(always)]
    pub fn centered(self) -> Aabb<N, T, A, AabbCentered> {
        self.to_layout()
    }

    /// Converts the aabb to `AabbMinMaxed`.
    /// This means the aabb will be represented by its minimum and maximum corners.
    #[inline(always)]
    pub fn min_maxed(self) -> Aabb<N, T, A, AabbMinMaxed> {
        self.to_layout()
    }
}

// Layout

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    /// Converts the aabb to the specified memory-layout.
    ///
    /// This can swap the alignment and representation of the aabb.
    #[inline(always)]
    pub fn to_layout<AOutput: VecAlignment, ROutput: AabbRepr>(
        self,
    ) -> Aabb<N, T, AOutput, ROutput> {
        match self.resolve() {
            ResolvedAabb::Centered(rect) => {
                Aabb::from_center_extents(rect.inner.center, rect.inner.extents)
            }
            ResolvedAabb::Cornered(rect) => Aabb::from_min_size(rect.inner.min, rect.inner.size),
            ResolvedAabb::MinMaxed(rect) => Aabb::from_min_max(rect.inner.min, rect.inner.max),
        }
    }
}
