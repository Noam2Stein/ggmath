use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
        DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Mask, Scalar, ScalarBackend, ScalarRepr, SignedInteger,
    SupportedLength, Unaligned,
    constants::{False, Infinity, Max, Min, Nan, NegInfinity, NegOne, One, True, Zero},
    repr::{Repr2, Repr3, Repr4},
    specialize::specialize,
    transmute::{transmute_generic, transmute_mut, transmute_ref},
};

mod bool;
mod constructor;
mod float;
mod signed;
mod swizzle;
mod unsigned;
pub(crate) use float::*;
pub(crate) use signed::*;
pub(crate) use unsigned::*;

/// A generic vector type.
///
/// `Vector` is the generic form of:
///
/// - [`Vec2<T>`](crate::Vec2)
/// - [`Vec3<T>`](crate::Vec3)
/// - [`Vec4<T>`](crate::Vec4)
/// - [`Vec2U<T>`](crate::Vec2U)
/// - [`Vec3U<T>`](crate::Vec3U)
/// - [`Vec4U<T>`](crate::Vec4U)
///
/// `Vector` is generic over:
///
/// - `N`: Length (2, 3, or 4)
/// - `T`: Scalar type (see [`Scalar`])
/// - `A`: Alignment (see [`Alignment`])
///
/// # Guarantees
///
/// `Vector<N, T, A>` represents `N` consecutive values of `T` followed by
/// optional padding due to alignment.
///
/// Padding bytes are initialized and accept any bit-pattern. It is **sound** to
/// store any bit-pattern in padding, and it is **unsound** to assume that
/// padding contains valid values of `T` unless `T` accepts all bit-patterns.
///
/// - `Vector<N, T, Unaligned>`: has no padding, has no additional alignment.
///
/// - `Vector<2, T, Aligned>`: has no padding, may have additional alignment.
///
/// - `Vector<3, T, Aligned>`: may have one padding element, may have additional
///   alignment.
///
/// - `Vector<4, T, Aligned>`: has no padding, may have additional alignment.
///
/// Vectors of scalar types with the same [`Scalar::Repr`] are guaranteed to
/// have compatible memory layouts, unless `Repr = ()`. They are guaranteed to
/// have the same size and element positions, but their alignment may differ.
///
/// Types containing `Vector` are **not guaranteed** to have the same memory
/// layout as types containing equivalent arrays. For example, even though
/// `Vec2U<T>` and `[T; 2]` have the same memory layout, `Option<Vec2U<T>>` and
/// `Option<[T; 2]>` may not.
#[repr(transparent)]
pub struct Vector<const N: usize, T, A: Alignment>(
    pub(crate) <T::Repr as ScalarRepr>::VectorRepr<N, T, A>,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

/*
When the compiler is smart enough to understand type equality based on const
generic equality, many of the function implementations in this module should
be simplifyed.
*/

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero,
{
    /// All `0`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + One,
{
    /// All `1`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + NegOne,
{
    /// All `-1`.
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Min,
{
    /// All [`T::MIN`](Min::MIN).
    pub const MIN: Self = Self::splat(T::MIN);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Max,
{
    /// All [`T::MAX`](Max::MAX).
    pub const MAX: Self = Self::splat(T::MAX);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Nan,
{
    /// All NaN (Not a Number).
    pub const NAN: Self = Self::splat(T::NAN);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Infinity,
{
    /// All [`T::INFINITY`](Infinity::INFINITY).
    pub const INFINITY: Self = Self::splat(T::INFINITY);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + NegInfinity,
{
    /// All [`T::NEG_INFINITY`](NegInfinity::NEG_INFINITY).
    pub const NEG_INFINITY: Self = Self::splat(T::NEG_INFINITY);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + True,
{
    /// All `true`.
    pub const TRUE: Self = Self::splat(T::TRUE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + False,
{
    /// All `false`.
    pub const FALSE: Self = Self::splat(T::FALSE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Creates a vector from an array.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; N]) -> Self {
        match N {
            // SAFETY: Because `N == 2`, `Vector<N, T, A>` and `Vector<2, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    array[0], array[1],
                ))
            },

            // SAFETY: Because `N == 3`, `Vector<N, T, A>` and `Vector<3, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    array[0], array[1], array[2],
                ))
            },

            // SAFETY: Because `N == 4`, `Vector<N, T, A>` and `Vector<4, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    array[0], array[1], array[2], array[3],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Creates a vector with all components set to the given value.
    #[inline]
    #[must_use]
    pub const fn splat(value: T) -> Self {
        match N {
            // SAFETY: Because `N == 2`, `Vector<N, T, A>` and `Vector<2, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    value, value,
                ))
            },

            // SAFETY: Because `N == 3`, `Vector<N, T, A>` and `Vector<3, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    value, value, value,
                ))
            },

            // SAFETY: Because `N == 4`, `Vector<N, T, A>` and `Vector<4, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    value, value, value, value,
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Creates a vector by calling function `f` for each component index.
    ///
    /// Equivalent to `(f(0), f(1), f(2), ...)`.
    #[inline]
    #[must_use]
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        match N {
            // SAFETY: Because `N == 2`, `Vector<N, T, A>` and `Vector<2, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    f(0),
                    f(1),
                ))
            },

            // SAFETY: Because `N == 3`, `Vector<N, T, A>` and `Vector<3, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    f(0),
                    f(1),
                    f(2),
                ))
            },

            // SAFETY: Because `N == 4`, `Vector<N, T, A>` and `Vector<4, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    f(0),
                    f(1),
                    f(2),
                    f(3),
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Converts the vector to the specified alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(self) -> Vector<N, T, A2> {
        match N {
            // SAFETY: Vectors with length `2` and `4` are guaranteed to be made
            // out of `N` consecutive values of `T` with no padding. Meaning
            // they have compatible layouts between alignments.
            2 | 4 => unsafe { transmute_generic::<Vector<N, T, A>, Vector<N, T, A2>>(self) },

            // SAFETY: Because `N == 3`, `Vector<N, T, A2>` and
            // `Vector<3, T, A2>` are the same type.
            3 => unsafe {
                transmute_generic::<Vector<3, T, A2>, Vector<N, T, A2>>(Vector::<3, T, A2>::new(
                    self.as_array_ref()[0],
                    self.as_array_ref()[1],
                    self.as_array_ref()[2],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Converts the vector to [`Aligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn align(self) -> Vector<N, T, Aligned> {
        self.to_alignment()
    }

    /// Converts the vector to [`Unaligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn unalign(self) -> Vector<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Converts the vector to an array.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; N] {
        *self.as_array_ref()
    }

    /// Returns a reference to the vector's components.
    #[inline]
    #[must_use]
    pub const fn as_array_ref(&self) -> &[T; N] {
        // SAFETY: `Vector<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `T`.
        unsafe { transmute_ref::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns a mutable reference to the vector's components.
    #[inline]
    #[must_use]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        // SAFETY: `Vector<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `T`.
        unsafe { transmute_mut::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns an iterator over the vector's components.
    ///
    /// This method returns an iterator over `T` and not `&T`. to iterate over
    /// references use `vec.as_array_ref().iter()`.
    #[inline]
    #[must_use]
    pub fn iter(self) -> core::array::IntoIter<T, N> {
        self.to_array().into_iter()
    }

    /// Returns an iterator over mutable references to the vector's components.
    #[inline]
    #[must_use = "iterators are lazy and do nothing unless consumed"]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_array_mut().iter_mut()
    }

    /// Creates a vector by calling function `f` for each component of the input
    /// vector.
    ///
    /// Equivalent to `(f(vec.x), f(vec.y), f(vec.z), ...)`.
    ///
    /// # Example
    ///
    /// ```
    /// use ggmath::Vec3;
    ///
    /// let vec: Vec3<f32> = Vec3::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(vec.map(|x| x * 2.0), Vec3::new(2.0, 4.0, 6.0));
    ///
    /// assert_eq!(vec.map(|x| x.is_sign_negative()), Vec3::new(false, false, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn map<T2, F>(self, f: F) -> Vector<N, T2, A>
    where
        T2: Scalar,
        F: Fn(T) -> T2,
    {
        Vector::from_fn(|i| f(self[i]))
    }

    /// Returns the vector's components in reverse order.
    ///
    /// # Example
    ///
    /// ```
    /// use ggmath::{Vec3, vec3};
    ///
    /// let vec: Vec3<f32> = Vec3::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(vec.reverse(), Vec3::new(3.0, 2.0, 1.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn reverse(self) -> Self {
        Self::from_fn(|i| self[N - 1 - i])
    }

    /// Returns a mask where each component is `true` if the corresponding
    /// components of `self` and `other` are equal.
    ///
    /// Equivalent to `(self.x == other.x, self.y == other.y, ...)`.
    #[inline]
    #[must_use]
    pub fn eq_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialEq,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_eq_mask(self, other))
    }

    /// Returns a mask where each component is `true` if the corresponding
    /// components of `self` and `other` are not equal.
    ///
    /// Equivalent to `(self.x != other.x, self.y != other.y, ...)`.
    #[inline]
    #[must_use]
    pub fn ne_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialEq,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_ne_mask(self, other))
    }

    /// Returns a mask where each component is `true` if the corresponding
    /// component of `self` is less than the corresponding component of `other`.
    ///
    /// Equivalent to `(self.x < other.x, self.y < other.y, ...)`.
    #[inline]
    #[must_use]
    pub fn lt_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_lt_mask(self, other))
    }

    /// Returns a mask where each component is `true` if the corresponding
    /// component of `self` is greater than the corresponding component of
    /// `other`.
    ///
    /// Equivalent to `(self.x > other.x, self.y > other.y, ...)`.
    #[inline]
    #[must_use]
    pub fn gt_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_gt_mask(self, other))
    }

    /// Returns a mask where each component is `true` if the corresponding
    /// component of `self` is less than or equal to the corresponding component
    /// of `other`.
    ///
    /// Equivalent to `(self.x <= other.x, self.y <= other.y, ...)`.
    #[inline]
    #[must_use]
    pub fn le_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_le_mask(self, other))
    }

    /// Returns a mask where each component is `true` if the corresponding
    /// component of `self` is greater than or equal to the corresponding
    /// component of `other`.
    ///
    /// Equivalent to `(self.x >= other.x, self.y >= other.y, ...)`.
    #[inline]
    #[must_use]
    pub fn ge_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_ge_mask(self, other))
    }

    /// Reinterprets the bits of the vector to a different scalar type.
    ///
    /// The two scalar types must have compatible memory layouts. This is
    /// enforced via trait bounds in this function's signature.
    ///
    /// This function is used to make SIMD optimizations in implementations of [`Scalar`].
    ///
    /// # Safety
    ///
    /// The components of the input must be valid for the output vector type.
    ///
    /// For example, when converting vectors from `u8` to `bool` the
    /// input components must be either `0` or `1`.
    ///
    /// The optional padding does not need to be a valid value of `T2`.
    #[inline]
    #[must_use]
    #[expect(private_bounds)]
    pub const unsafe fn to_repr<T2>(self) -> Vector<N, T2, A>
    where
        T2: Scalar<Repr = T::Repr>,
        T::Repr: SignedInteger,
    {
        // SAFETY: Vectors of scalars with the same `Scalar::Repr` are
        // guaranteed to have compatible memory layouts if `Repr` is a signed
        // integer.
        unsafe { transmute_generic::<Vector<N, T, A>, Vector<N, T2, A>>(self) }
    }
}

impl<T, A: Alignment> Vector<2, T, A>
where
    T: Scalar + Zero + One,
{
    /// `(1, 0)`.
    pub const X: Self = Self::new(T::ONE, T::ZERO);

    /// `(0, 1)`.
    pub const Y: Self = Self::new(T::ZERO, T::ONE);
}

impl<T, A: Alignment> Vector<2, T, A>
where
    T: Scalar + Zero + NegOne,
{
    /// `(-1, 0)`.
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO);

    /// `(0, -1)`.
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE);
}

impl<T, A: Alignment> Vector<2, T, A>
where
    T: Scalar,
{
    /// Creates a 2-dimensional vector.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        // SAFETY: `Vector<2, T, A>` is guaranteed to be made out of 2
        // consecutive values of `T`, with no additional padding.
        unsafe { transmute_generic::<Repr2<T>, Vector<2, T, A>>(Repr2(x, y)) }
    }
}

impl<T, A: Alignment> Vector<3, T, A>
where
    T: Scalar + Zero + One,
{
    /// `(1, 0, 0)`.
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);

    /// `(0, 1, 0)`.
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);

    /// `(0, 0, 1)`.
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}

impl<T, A: Alignment> Vector<3, T, A>
where
    T: Scalar + Zero + NegOne,
{
    /// `(-1, 0, 0)`.
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO, T::ZERO);

    /// `(0, -1, 0)`.
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE, T::ZERO);

    /// `(0, 0, -1)`.
    pub const NEG_Z: Self = Self::new(T::ZERO, T::ZERO, T::NEG_ONE);
}

impl<T, A: Alignment> Vector<3, T, A>
where
    T: Scalar,
{
    /// Creates a 3-dimensional vector.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T, z: T) -> Self {
        match size_of::<Vector<3, T, A>>() / size_of::<T>() {
            // SAFETY: Because the vector has 3 values of `T` and no padding,
            // its equivalent to `Repr3<T>`.
            3 => unsafe { transmute_generic::<Repr3<T>, Vector<3, T, A>>(Repr3(x, y, z)) },

            // SAFETY: Because the vector has 3 values of `T` plus 1 padding
            // element, its equivalent to `Repr4<T>`.
            4 => unsafe { transmute_generic::<Repr4<T>, Vector<3, T, A>>(Repr4(x, y, z, z)) },

            _ => unreachable!(),
        }
    }
}

impl<T, A: Alignment> Vector<4, T, A>
where
    T: Scalar + Zero + One,
{
    /// `(1, 0, 0, 0)`.
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);

    /// `(0, 1, 0, 0)`.
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);

    /// `(0, 0, 1, 0)`.
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);

    /// `(0, 0, 0, 1)`.
    pub const W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T, A: Alignment> Vector<4, T, A>
where
    T: Scalar + Zero + NegOne,
{
    /// `(-1, 0, 0, 0)`.
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO);

    /// `(0, -1, 0, 0)`.
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO);

    /// `(0, 0, -1, 0)`.
    pub const NEG_Z: Self = Self::new(T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO);

    /// `(0, 0, 0, -1)`.
    pub const NEG_W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE);
}

impl<T, A: Alignment> Vector<4, T, A>
where
    T: Scalar,
{
    /// Creates a 4-dimensional vector.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        // SAFETY: `Vector<4, T, A>` is guaranteed to be made out of 4
        // consecutive values of `T`, with no additional padding.
        unsafe { transmute_generic::<Repr4<T>, Vector<4, T, A>>(Repr4(x, y, z, w)) }
    }
}

impl<const N: usize, T, A: Alignment> Clone for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> Index<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array_ref().index(index)
    }
}

impl<const N: usize, T, A: Alignment> IndexMut<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}

impl<const N: usize, T, A: Alignment> IntoIterator for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize, T, A: Alignment> IntoIterator for &Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize, T, A: Alignment> IntoIterator for &'a mut Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Vec2Fields<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
}

impl<T, A: Alignment> Deref for Vector<2, T, A>
where
    T: Scalar,
{
    type Target = Vec2Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Vector<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Xy<T>`.
        unsafe { transmute_ref::<Vector<2, T, A>, Vec2Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Vector<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Xy<T>`.
        unsafe { transmute_mut::<Vector<2, T, A>, Vec2Fields<T>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Vec3Fields<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
    /// The third element of the vector.
    pub z: T,
}

impl<T, A: Alignment> Deref for Vector<3, T, A>
where
    T: Scalar,
{
    type Target = Vec3Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Vector<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `T`, and so begin with `Xyz<T>`.
        unsafe { transmute_ref::<Vector<3, T, A>, Vec3Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Vector<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `T`, and so begin with `Xyz<T>`.
        unsafe { transmute_mut::<Vector<3, T, A>, Vec3Fields<T>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Vec4Fields<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
    /// The third element of the vector.
    pub z: T,
    /// The fourth element of the vector.
    pub w: T,
}

impl<T, A: Alignment> Deref for Vector<4, T, A>
where
    T: Scalar,
{
    type Target = Vec4Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Vector<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Xyzw<T>`.
        unsafe { transmute_ref::<Vector<4, T, A>, Vec4Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Vector<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Xyzw<T>`.
        unsafe { transmute_mut::<Vector<4, T, A>, Vec4Fields<T>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "({:?}, {:?})", self[0], self[1]),
            3 => write!(f, "({:?}, {:?}, {:?})", self[0], self[1], self[2]),
            4 => write!(
                f,
                "({:?}, {:?}, {:?}, {:?})",
                self[0], self[1], self[2], self[3]
            ),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> Display for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Display,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "({}, {})", self[0], self[1]),
            3 => write!(f, "({}, {}, {})", self[0], self[1], self[2]),
            4 => write!(f, "({}, {}, {}, {})", self[0], self[1], self[2], self[3]),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N, A>>::vec_eq(self, other))
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N, A>>::vec_ne(self, other))
    }
}

impl<const N: usize, T, A: Alignment> Eq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_array_ref().hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Default,
{
    #[inline]
    fn default() -> Self {
        Self::splat(T::default())
    }
}

macro_rules! impl_unary_op {
    ($Op:ident $op:ident $vec_op:ident) => {
        impl<const N: usize, T, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            #[inline]
            fn $op(self) -> Self::Output {
                specialize!(<T as ScalarBackend<N, A>>::$vec_op(self))
            }
        }
    };
}
impl_unary_op!(Neg neg vec_neg);
impl_unary_op!(Not not vec_not);

macro_rules! impl_binary_op {
    ($Op:ident $op:ident $vec_op:ident) => {
        impl<const N: usize, T, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: Self) -> Self::Output {
                specialize!(<T as ScalarBackend<N, A>>::$vec_op(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: T) -> Self::Output {
                self.$op(Self::splat(rhs))
            }
        }
    };
}
impl_binary_op!(Add add vec_add);
impl_binary_op!(Sub sub vec_sub);
impl_binary_op!(Mul mul vec_mul);
impl_binary_op!(Div div vec_div);
impl_binary_op!(Rem rem vec_rem);
impl_binary_op!(Shl shl vec_shl);
impl_binary_op!(Shr shr vec_shr);
impl_binary_op!(BitAnd bitand vec_bitand);
impl_binary_op!(BitOr bitor vec_bitor);
impl_binary_op!(BitXor bitxor vec_bitxor);

macro_rules! impl_assign_op {
    ($Op:ident $OpAssign:ident $op_assign:ident $op:ident) => {
        impl<const N: usize, T, A: Alignment> $OpAssign for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: T) {
                *self = self.$op(rhs);
            }
        }
    };
}
impl_assign_op!(Add AddAssign add_assign add);
impl_assign_op!(Sub SubAssign sub_assign sub);
impl_assign_op!(Mul MulAssign mul_assign mul);
impl_assign_op!(Div DivAssign div_assign div);
impl_assign_op!(Rem RemAssign rem_assign rem);
impl_assign_op!(Shl ShlAssign shl_assign shl);
impl_assign_op!(Shr ShrAssign shr_assign shr);
impl_assign_op!(BitAnd BitAndAssign bitand_assign bitand);
impl_assign_op!(BitOr BitOrAssign bitor_assign bitor);
impl_assign_op!(BitXor BitXorAssign bitxor_assign bitxor);

// SAFETY: Vectors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Send` the list also is, and the padding is `Send` too.
unsafe impl<const N: usize, T, A: Alignment> Send for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Send,
{
}

// SAFETY: Vectors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Sync` the list also is, and the padding is `Sync` too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + RefUnwindSafe,
{
}
