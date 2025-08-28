//! Module for the aabb type.

use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::*,
};

use super::*;

mod construct;
mod convert;
mod intersect;
mod properties;

/// Trait required to put a type inside a `Aabb`.
///
/// This trait contains simple arithmetic operations that are used by the `Aabb` type,
/// like doubling or halving a vector.
pub trait AabbScalar:
    Scalar + Add<Output = Self> + Sub<Output = Self> + PartialEq + PartialOrd
{
    /// Return `vec * 2`.
    /// Used by aabb functions.
    fn aabb_vec_double<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Return `vec / 2`.
    /// Used by aabb functions.
    ///
    /// For ints this should FLOOR the output.
    fn aabb_vec_half<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Maps the vectors to the absolute difference of their components.
    /// Used by aabb functions.
    fn aabb_vec_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Maps the vectors to the minimum of their components.
    /// Used by aabb functions.
    fn aabb_vec_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Maps the vectors to the maximum of their components.
    /// Used by aabb functions.
    fn aabb_vec_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;
}

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
pub unsafe trait AabbRepr: Sized + 'static {
    const ENUM: AabbReprEnum;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>: Construct + PartialEq
    where
        Usize<N>: VecLen;

    fn inner_hash<const N: usize, T: AabbScalar + Hash, A: VecAlignment, H: Hasher>(
        inner: &Self::InnerAabb<N, T, A>,
        state: &mut H,
    ) where
        Usize<N>: VecLen;

    fn inner_debug<const N: usize, T: AabbScalar + Debug, A: VecAlignment>(
        inner: &Self::InnerAabb<N, T, A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Usize<N>: VecLen;

    fn inner_display<const N: usize, T: AabbScalar + Display, A: VecAlignment>(
        inner: &Self::InnerAabb<N, T, A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Usize<N>: VecLen;
}

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
pub enum AabbReprEnum {
    Cornered,
    Centered,
    MinMaxed,
}

unsafe impl AabbRepr for AabbCornered {
    const ENUM: AabbReprEnum = AabbReprEnum::Cornered;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerCorneredAabb<N, T, A>
    where
        Usize<N>: VecLen;

    fn inner_hash<const N: usize, T: AabbScalar + Hash, A: VecAlignment, H: Hasher>(
        inner: &Self::InnerAabb<N, T, A>,
        state: &mut H,
    ) where
        Usize<N>: VecLen,
    {
        inner.min.hash(state);
        inner.size.hash(state);
    }

    fn inner_debug<const N: usize, T: AabbScalar + Debug, A: VecAlignment>(
        inner: &Self::InnerAabb<N, T, A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Usize<N>: VecLen,
    {
        write!(f, "{{ min: {:?}, size: {:?} }}", inner.min, inner.size)
    }

    fn inner_display<const N: usize, T: AabbScalar + Display, A: VecAlignment>(
        inner: &Self::InnerAabb<N, T, A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Usize<N>: VecLen,
    {
        write!(f, "{{ min: {}, size: {} }}", inner.min, inner.size)
    }
}

unsafe impl AabbRepr for AabbCentered {
    const ENUM: AabbReprEnum = AabbReprEnum::Centered;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerCenteredAabb<N, T, A>
    where
        Usize<N>: VecLen;

    fn inner_hash<const N: usize, T: AabbScalar + Hash, A: VecAlignment, H: Hasher>(
        inner: &Self::InnerAabb<N, T, A>,
        state: &mut H,
    ) where
        Usize<N>: VecLen,
    {
        inner.center.hash(state);
        inner.extents.hash(state);
    }

    fn inner_debug<const N: usize, T: AabbScalar + Debug, A: VecAlignment>(
        inner: &Self::InnerAabb<N, T, A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Usize<N>: VecLen,
    {
        write!(
            f,
            "{{ center: {:?}, extents: {:?} }}",
            inner.center, inner.extents
        )
    }

    fn inner_display<const N: usize, T: AabbScalar + Display, A: VecAlignment>(
        inner: &Self::InnerAabb<N, T, A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Usize<N>: VecLen,
    {
        write!(
            f,
            "{{ center: {}, extents: {} }}",
            inner.center, inner.extents
        )
    }
}

unsafe impl AabbRepr for AabbMinMaxed {
    const ENUM: AabbReprEnum = AabbReprEnum::MinMaxed;

    type InnerAabb<const N: usize, T: AabbScalar, A: VecAlignment>
        = InnerMinMaxedAabb<N, T, A>
    where
        Usize<N>: VecLen;

    fn inner_hash<const N: usize, T: AabbScalar + Hash, A: VecAlignment, H: Hasher>(
        inner: &Self::InnerAabb<N, T, A>,
        state: &mut H,
    ) where
        Usize<N>: VecLen,
    {
        inner.min.hash(state);
        inner.max.hash(state);
    }

    fn inner_debug<const N: usize, T: AabbScalar + Debug, A: VecAlignment>(
        inner: &Self::InnerAabb<N, T, A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Usize<N>: VecLen,
    {
        write!(f, "{{ min: {:?}, max: {:?} }}", inner.min, inner.max)
    }

    fn inner_display<const N: usize, T: AabbScalar + Display, A: VecAlignment>(
        inner: &Self::InnerAabb<N, T, A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Usize<N>: VecLen,
    {
        write!(f, "{{ min: {}, max: {} }}", inner.min, inner.max)
    }
}

/// Axis-aligned bounding box.
///
/// In most cases, you can use the type aliases of this type (`Rect`, `Aabb3`, etc.).
/// See [`crate::aabb`].
///
/// This type is generic over the number of dimensions,
/// the scalar type, the vector alignment, which all follow `Vector`'s generics,
/// and the representation of the bounding box.
///
/// Aabbs can be represented in different ways:
/// - by their corners,
/// - by their center and size,
/// - any combination of these.
///
/// The type parameter `R` is the representation of the bounding box.
/// - `AabbCornered` represents the bounding box by its minimum corner and size.
/// - `AabbCentered` represents the bounding box by its center and extents.
/// - `AabbMinMaxed` represents the bounding box by its minimum and maximum corners.
#[repr(transparent)]
pub struct Aabb<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr>
where
    Usize<N>: VecLen,
{
    inner: R::InnerAabb<N, T, A>,
}

/// Type alias to [`Aabb<2, T, VecAligned, AabbCornered>`].
pub type Rect<T> = Aabb<2, T, VecAligned, AabbCornered>;

/// Type alias to [`Aabb<2, T, VecAligned, AabbCentered>`].
pub type RectC<T> = Aabb<2, T, VecAligned, AabbCentered>;

/// Type alias to [`Aabb<2, T, VecAligned, AabbMinMaxed>`].
pub type RectM<T> = Aabb<2, T, VecAligned, AabbMinMaxed>;

/// Type alias to [`Aabb<2, T, VecPacked, AabbCornered>`].
pub type RectP<T> = Aabb<2, T, VecPacked, AabbCornered>;

/// Type alias to [`Aabb<2, T, VecPacked, AabbCentered>`].
pub type RectCP<T> = Aabb<2, T, VecPacked, AabbCentered>;

/// Type alias to [`Aabb<2, T, VecPacked, AabbMinMaxed>`].
pub type RectMP<T> = Aabb<2, T, VecPacked, AabbMinMaxed>;

/// Type alias to [`Aabb<3, T, VecAligned, AabbCornered>`].
pub type Aabb3<T> = Aabb<3, T, VecAligned, AabbCornered>;

/// Type alias to [`Aabb<3, T, VecAligned, AabbCentered>`].
pub type Aabb3C<T> = Aabb<3, T, VecAligned, AabbCentered>;

/// Type alias to [`Aabb<3, T, VecAligned, AabbMinMaxed>`].
pub type Aabb3M<T> = Aabb<3, T, VecAligned, AabbMinMaxed>;

/// Type alias to [`Aabb<3, T, VecPacked, AabbCornered>`].
pub type Aabb3P<T> = Aabb<3, T, VecPacked, AabbCornered>;

/// Type alias to [`Aabb<3, T, VecPacked, AabbCentered>`].
pub type Aabb3CP<T> = Aabb<3, T, VecPacked, AabbCentered>;

/// Type alias to [`Aabb<3, T, VecPacked, AabbMinMaxed>`].
pub type Aabb3MP<T> = Aabb<3, T, VecPacked, AabbMinMaxed>;

/// Type alias to [`Aabb<4, T, VecAligned, AabbCornered>`].
pub type Aabb4<T> = Aabb<4, T, VecAligned, AabbCornered>;

/// Type alias to [`Aabb<4, T, VecAligned, AabbCentered>`].
pub type Aabb4C<T> = Aabb<4, T, VecAligned, AabbCentered>;

/// Type alias to [`Aabb<4, T, VecAligned, AabbMinMaxed>`].
pub type Aabb4M<T> = Aabb<4, T, VecAligned, AabbMinMaxed>;

/// Type alias to [`Aabb<4, T, VecPacked, AabbCornered>`].
pub type Aabb4P<T> = Aabb<4, T, VecPacked, AabbCornered>;

/// Type alias to [`Aabb<4, T, VecPacked, AabbCentered>`].
pub type Aabb4CP<T> = Aabb<4, T, VecPacked, AabbCentered>;

/// Type alias to [`Aabb<4, T, VecPacked, AabbMinMaxed>`].
pub type Aabb4MP<T> = Aabb<4, T, VecPacked, AabbMinMaxed>;

#[repr(C)]
struct InnerCorneredAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub size: Vector<N, T, A>,
}

#[repr(C)]
struct InnerMinMaxedAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub max: Vector<N, T, A>,
}

#[repr(C)]
struct InnerCenteredAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub center: Vector<N, T, A>,
    pub extents: Vector<N, T, A>,
}

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Clone for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Copy for Aabb<N, T, A, R> where
    Usize<N>: VecLen
{
}
impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> PartialEq for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<const N: usize, T: AabbScalar, A: VecAlignment> Clone for InnerCorneredAabb<N, T, A>
where
    Usize<N>: VecLen,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const N: usize, T: AabbScalar, A: VecAlignment> Copy for InnerCorneredAabb<N, T, A> where
    Usize<N>: VecLen
{
}
impl<const N: usize, T: AabbScalar, A: VecAlignment> PartialEq for InnerCorneredAabb<N, T, A>
where
    Usize<N>: VecLen,
{
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.size == other.size
    }
}

impl<const N: usize, T: AabbScalar, A: VecAlignment> Clone for InnerCenteredAabb<N, T, A>
where
    Usize<N>: VecLen,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const N: usize, T: AabbScalar, A: VecAlignment> Copy for InnerCenteredAabb<N, T, A> where
    Usize<N>: VecLen
{
}
impl<const N: usize, T: AabbScalar, A: VecAlignment> PartialEq for InnerCenteredAabb<N, T, A>
where
    Usize<N>: VecLen,
{
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.extents == other.extents
    }
}

impl<const N: usize, T: AabbScalar, A: VecAlignment> Clone for InnerMinMaxedAabb<N, T, A>
where
    Usize<N>: VecLen,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const N: usize, T: AabbScalar, A: VecAlignment> Copy for InnerMinMaxedAabb<N, T, A> where
    Usize<N>: VecLen
{
}
impl<const N: usize, T: AabbScalar, A: VecAlignment> PartialEq for InnerMinMaxedAabb<N, T, A>
where
    Usize<N>: VecLen,
{
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl<const N: usize, T: AabbScalar + Eq, A: VecAlignment, R: AabbRepr> Eq for Aabb<N, T, A, R> where
    Usize<N>: VecLen
{
}
impl<const N: usize, T: AabbScalar + Hash, A: VecAlignment, R: AabbRepr> Hash for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        R::inner_hash(&self.inner, state);
    }
}

impl<const N: usize, T: AabbScalar + Debug, A: VecAlignment, R: AabbRepr> Debug for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        R::inner_debug(&self.inner, f)
    }
}

impl<const N: usize, T: AabbScalar + Display, A: VecAlignment, R: AabbRepr> Display
    for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        R::inner_display(&self.inner, f)
    }
}
