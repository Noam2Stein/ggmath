use std::{
    mem::{transmute, transmute_copy},
    ops::*,
};

use derive_where::derive_where;

use super::*;

// Scalar

/// Trait required to put a type inside a `Aabb`.
///
/// This trait contains simple arithmetic operations that are used by the `Aabb` type,
/// like doubling or halving a vector.
pub trait AabbScalar:
    Scalar + Add<Output = Self> + Sub<Output = Self> + PartialEq + PartialOrd
{
    /// Returns `self * 2`.
    /// Used by `Rectangle` functions.
    fn aabb_mul_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Returns `self / 2`.
    /// Used by `Rectangle` functions.
    ///
    /// For ints this should floor the output.
    fn aabb_div_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Maps the vectors to the absolute difference of their components.
    /// Used by `Rectangle` functions.
    fn aabb_vector_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Maps the vectors to the minimum of their components.
    /// Used by `Rectangle` functions.
    fn aabb_vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Maps the vectors to the maximum of their components.
    /// Used by `Rectangle` functions.
    fn aabb_vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;
}

// Repr

/// A marker trait for the inner representation of an `Aabb`.
///
/// An aabb can be represented in different ways:
/// by its corners, center, size, etc.
///
/// This trait marks the inner representation of an `Aabb`,
/// and is implemented by `AabbCornered`, `AabbCentered`, and `AabbMinMaxed`.
///
/// `AabbCornered` represents the aabb by its minimum corner and size.
/// `AabbCentered` represents the aabb by its center and extents.
/// `AabbMinMaxed` represents the aabb by its minimum and maximum corners.
///
/// The inner representation of an aabb does not affect its API,
/// only the inner implementations of everything.
#[allow(private_bounds)]
pub unsafe trait AabbRepr: AabbReprPriv + Sized + 'static {}

/// Marks an `Aabb` as being represented by its minimum corner and size.
/// See [`AabbRepr`].
pub struct AabbCornered;

/// Marks an `Aabb` as being represented by its center and extents.
/// See [`AabbRepr`].
pub struct AabbCentered;

/// Marks an `Aabb` as being represented by its minimum and maximum corners.
/// See [`AabbRepr`].
pub struct AabbMinMaxed;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AabbReprEnum {
    Cornered,
    Centered,
    MinMaxed,
}

pub(crate) trait AabbReprPriv {
    const ENUM: AabbReprEnum;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>: Construct + PartialEq
    where
        Usize<N>: VecLen;
}

unsafe impl AabbRepr for AabbCornered {}
unsafe impl AabbRepr for AabbCentered {}
unsafe impl AabbRepr for AabbMinMaxed {}

impl AabbReprPriv for AabbCornered {
    const ENUM: AabbReprEnum = AabbReprEnum::Cornered;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerCorneredAabb<N, T, A>
    where
        Usize<N>: VecLen;
}
impl AabbReprPriv for AabbCentered {
    const ENUM: AabbReprEnum = AabbReprEnum::Centered;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerCenteredAabb<N, T, A>
    where
        Usize<N>: VecLen;
}
impl AabbReprPriv for AabbMinMaxed {
    const ENUM: AabbReprEnum = AabbReprEnum::MinMaxed;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerMinMaxedAabb<N, T, A>
    where
        Usize<N>: VecLen;
}

#[repr(C)]
#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(crate) struct InnerCorneredAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub size: Vector<N, T, A>,
}

#[repr(C)]
#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(crate) struct InnerMinMaxedAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub max: Vector<N, T, A>,
}

#[repr(C)]
#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(crate) struct InnerCenteredAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub center: Vector<N, T, A>,
    pub extents: Vector<N, T, A>,
}

// Resolve

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
    /// use ggmath::*;
    ///
    /// fn foo(aabb: Aabb<2, f32, impl VecAlignment, impl AabbRepr>) {
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
    /// use ggmath::*;
    ///
    /// fn foo(aabb: &Aabb<2, f32, impl VecAlignment, impl AabbRepr>) {
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
    pub const fn resolve_ref(&'_ self) -> ResolvedAabbRef<'_, N, T, A> {
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
    /// use ggmath::*;
    ///
    /// fn foo(aabb: &mut Aabb<2, f32, impl VecAlignment, impl AabbRepr>) {
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
    pub const fn resolve_repr_mut(&'_ mut self) -> ResolvedAabbMut<'_, N, T, A> {
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

    /// Maps the vector using the appropriate function based on the `AabbRepr`.
    #[inline(always)]
    pub fn resolved_map<const N2: usize, T2: AabbScalar, A2: VecAlignment>(
        self,
        f_cornered: impl FnOnce(Aabb<N, T, A, AabbCornered>) -> Aabb<N2, T2, A2, AabbCornered>,
        f_centered: impl FnOnce(Aabb<N, T, A, AabbCentered>) -> Aabb<N2, T2, A2, AabbCentered>,
        f_min_maxed: impl FnOnce(Aabb<N, T, A, AabbMinMaxed>) -> Aabb<N2, T2, A2, AabbMinMaxed>,
    ) -> Aabb<N2, T2, A2, R>
    where
        Usize<N2>: VecLen,
    {
        match self.resolve() {
            ResolvedAabb::Cornered(aabb) => unsafe {
                transmute_copy::<Aabb<N2, T2, A2, AabbCornered>, Aabb<N2, T2, A2, R>>(&f_cornered(
                    aabb,
                ))
            },

            ResolvedAabb::Centered(aabb) => unsafe {
                transmute_copy::<Aabb<N2, T2, A2, AabbCentered>, Aabb<N2, T2, A2, R>>(&f_centered(
                    aabb,
                ))
            },

            ResolvedAabb::MinMaxed(aabb) => unsafe {
                transmute_copy::<Aabb<N2, T2, A2, AabbMinMaxed>, Aabb<N2, T2, A2, R>>(&f_min_maxed(
                    aabb,
                ))
            },
        }
    }
}
