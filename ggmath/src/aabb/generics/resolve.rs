use std::mem::{transmute, transmute_copy};

use super::*;

/// See [`Aabb::resolve`].
pub enum ResolvedAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    /// The aabb is represented by its minimum corner and size.
    Cornered(Aabb<N, T, A, AabbCornered>),
    /// The aabb is represented by its center and extents.
    Centered(Aabb<N, T, A, AabbCentered>),
    /// The aabb is represented by its minimum and maximum corners.
    MinMaxed(Aabb<N, T, A, AabbMinMaxed>),
}

/// See [`Aabb::resolve_ref`].
pub enum ResolvedAabbRef<'a, const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    /// The aabb is represented by its minimum corner and size.
    Cornered(&'a Aabb<N, T, A, AabbCornered>),
    /// The aabb is represented by its center and extents.
    Centered(&'a Aabb<N, T, A, AabbCentered>),
    /// The aabb is represented by its minimum and maximum corners.
    MinMaxed(&'a Aabb<N, T, A, AabbMinMaxed>),
}

/// See [`Aabb::resolve_repr_mut`].
pub enum ResolvedAabbMut<'a, const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    /// The aabb is represented by its minimum corner and size.
    Cornered(&'a mut Aabb<N, T, A, AabbCornered>),
    /// The aabb is represented by its center and extents.
    Centered(&'a mut Aabb<N, T, A, AabbCentered>),
    /// The aabb is represented by its minimum and maximum corners.
    MinMaxed(&'a mut Aabb<N, T, A, AabbMinMaxed>),
}

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    /// Splits a generic `Aabb` to an enum based on its `AabbRepr`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// fn foo(aabb: Aabb<2, f32, VecAligned, impl AabbRepr>) {
    ///     match aabb.resolve() {
    ///         ResolvedAabb::Cornered(aabb) => {
    ///             println!("aabb is represented by its minimum corner and size");
    ///         }
    ///         ResolvedAabb::Centered(aabb) => {
    ///             println!("aabb is represented by its center and extents");
    ///         }
    ///         ResolvedAabb::MinMaxed(aabb) => {
    ///             println!("aabb is represented by its minimum and maximum corners");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve(self) -> ResolvedAabb<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedAabb::Cornered(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbCornered>,
                >(&self)),

                AabbReprEnum::Centered => ResolvedAabb::Centered(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbCentered>,
                >(&self)),

                AabbReprEnum::MinMaxed => ResolvedAabb::MinMaxed(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbMinMaxed>,
                >(&self)),
            }
        }
    }

    /// Splits a generic `Aabb` reference to an enum based on its `AabbRepr`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// fn foo(aabb: &Aabb<2, f32, A, impl AabbRepr>) {
    ///     match aabb.resolve_ref() {
    ///         ResolvedAabbRef::Cornered(aabb) => {
    ///             println!("aabb is represented by its minimum corner and size");
    ///         }
    ///         ResolvedAabbRef::Centered(aabb) => {
    ///             println!("aabb is represented by its center and extents");
    ///         }
    ///         ResolvedAabbRef::MinMaxed(aabb) => {
    ///             println!("aabb is represented by its minimum and maximum corners");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve_ref(&self) -> ResolvedAabbRef<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedAabbRef::Cornered(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbCornered>,
                >(self)),

                AabbReprEnum::Centered => ResolvedAabbRef::Centered(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbCentered>,
                >(self)),

                AabbReprEnum::MinMaxed => ResolvedAabbRef::MinMaxed(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbMinMaxed>,
                >(self)),
            }
        }
    }

    /// Splits a generic `Aabb` mutable reference to an enum based on its `AabbRepr`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// fn foo(aabb: &mut Aabb<2, f32, A, impl AabbRepr>) {
    ///     match aabb.resolve_repr_mut() {
    ///         ResolvedAabbMut::Cornered(aabb) => {
    ///             println!("aabb is represented by its minimum corner and size");
    ///         }
    ///         ResolvedAabbMut::Centered(aabb) => {
    ///             println!("aabb is represented by its center and extents");
    ///         }
    ///         ResolvedAabbMut::MinMaxed(aabb) => {
    ///             println!("aabb is represented by its minimum and maximum corners");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve_repr_mut(&mut self) -> ResolvedAabbMut<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedAabbMut::Cornered(transmute::<
                    &mut Aabb<N, T, A, R>,
                    &mut Aabb<N, T, A, AabbCornered>,
                >(self)),

                AabbReprEnum::Centered => ResolvedAabbMut::Centered(transmute::<
                    &mut Aabb<N, T, A, R>,
                    &mut Aabb<N, T, A, AabbCentered>,
                >(self)),

                AabbReprEnum::MinMaxed => ResolvedAabbMut::MinMaxed(transmute::<
                    &mut Aabb<N, T, A, R>,
                    &mut Aabb<N, T, A, AabbMinMaxed>,
                >(self)),
            }
        }
    }

    /// Picks the correct value based on the `AabbRepr`.
    ///
    /// This can be used to specialize aabb initialization based on representation.
    #[inline(always)]
    pub const fn resolved(
        cornered: Aabb<N, T, A, AabbCornered>,
        centered: Aabb<N, T, A, AabbCentered>,
        min_maxed: Aabb<N, T, A, AabbMinMaxed>,
    ) -> Self {
        match R::ENUM {
            AabbReprEnum::Cornered => unsafe {
                transmute_copy::<Aabb<N, T, A, AabbCornered>, Aabb<N, T, A, R>>(&cornered)
            },

            AabbReprEnum::Centered => unsafe {
                transmute_copy::<Aabb<N, T, A, AabbCentered>, Aabb<N, T, A, R>>(&centered)
            },

            AabbReprEnum::MinMaxed => unsafe {
                transmute_copy::<Aabb<N, T, A, AabbMinMaxed>, Aabb<N, T, A, R>>(&min_maxed)
            },
        }
    }
}
