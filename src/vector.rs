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

/// An `N`-dimensional vector of type `T`.
///
/// `A` controls SIMD alignment and is either [`Aligned`] or [`Unaligned`]. See
/// [`Alignment`] for more details.
///
/// # Type aliases
///
/// - [`Vec2<T>`] for `Vector<2, T, Aligned>`.
/// - [`Vec3<T>`] for `Vector<3, T, Aligned>`.
/// - [`Vec4<T>`] for `Vector<4, T, Aligned>`.
/// - [`Vec2U<T>`] for `Vector<2, T, Unaligned>`.
/// - [`Vec3U<T>`] for `Vector<3, T, Unaligned>`.
/// - [`Vec4U<T>`] for `Vector<4, T, Unaligned>`.
///
/// # Fields
///
/// `x: T` (for lengths `2`, `3`, `4`)
///
/// The first element of the vector.
///
/// `y: T` (for lengths `2`, `3`, `4`)
///
/// The second element of the vector.
///
/// `z: T` (for lengths `3`, `4`)
///
/// The third element of the vector.
///
/// `w: T` (for length `4`)
///
/// The fourth element of the vector.
///
/// # Memory layout
///
/// `Vector<N, T, A>` contains `N` consecutive values of `T` followed by
/// optional padding.
///
/// `Vector<N, T, Unaligned>` has the alignment of `T` and has no padding.
/// `Vector<N, T, Aligned>` may have higher alignment than `T`. [`Vec2<T>`] and
/// [`Vec4<T>`] have no padding. [`Vec3<T>`] may have one padding element.
///
/// Padding is fully initialized and accepts all bit patterns. Unless `T`
/// accepts all bit patterns, it is not sound to assume padding contains valid
/// values of `T`.
///
/// Vectors of compatible [`Scalar::Repr`] types have the same size. This means
/// that they are transmutable, but can still have different alignments (see
/// [`to_repr`]).
///
/// Types containing compatible vectors and arrays may not have compatible
/// layouts themselves. For example, even though [`Vec2U<T>`] and `[T; 2]` have
/// compatible layouts, [`Option<Vec2U<T>>`] and `Option<[T; 2]>` may not.
///
/// [`Vec2<T>`]: crate::Vec2
/// [`Vec3<T>`]: crate::Vec3
/// [`Vec4<T>`]: crate::Vec4
/// [`Vec2U<T>`]: crate::Vec2U
/// [`Vec3U<T>`]: crate::Vec3U
/// [`Vec4U<T>`]: crate::Vec4U
/// [`to_repr`]: Self::to_repr
#[repr(transparent)]
pub struct Vector<const N: usize, T, A: Alignment>(
    pub(crate) <T::Repr as ScalarRepr>::VectorRepr<N, T, A>,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero,
{
    /// A vector with all elements set to `0`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + One,
{
    /// A vector with all elements set to `1`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + NegOne,
{
    /// A vector with all elements set to `-1`.
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Min,
{
    /// A vector with all elements set to [`T::MIN`].
    ///
    /// [`T::MIN`]: Min::MIN
    pub const MIN: Self = Self::splat(T::MIN);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Max,
{
    /// A vector with all elements set to [`T::MAX`].
    ///
    /// [`T::MAX`]: Max::MAX
    pub const MAX: Self = Self::splat(T::MAX);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Nan,
{
    /// A vector with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::splat(T::NAN);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Infinity,
{
    /// A vector with all elements set to [`T::INFINITY`].
    ///
    /// [`T::INFINITY`]: Infinity::INFINITY
    pub const INFINITY: Self = Self::splat(T::INFINITY);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + NegInfinity,
{
    /// A vector with all elements set to [`T::NEG_INFINITY`].
    ///
    /// [`T::NEG_INFINITY`]: NegInfinity::NEG_INFINITY
    pub const NEG_INFINITY: Self = Self::splat(T::NEG_INFINITY);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + True,
{
    /// A vector with all elements set to `true`.
    pub const TRUE: Self = Self::splat(T::TRUE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + False,
{
    /// A vector with all elements set to `false`.
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

    /// Creates a vector with all elements set to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vec = Vec3::splat(5);
    /// assert_eq!(vec, Vec3::new(5, 5, 5));
    /// ```
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

    /// Creates a vector by calling function `f` for each element index.
    ///
    /// Equivalent to `(f(0), f(1), f(2), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let indices = Vec3::from_fn(|i| i);
    /// assert_eq!(indices, Vec3::new(0, 1, 2));
    ///
    /// let vec = Vec3::from_fn(|i| i % 2);
    /// assert_eq!(vec, Vec3::new(0, 1, 0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
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

    /// Conversion between [`Aligned`] and [`Unaligned`] storage.
    ///
    /// See [`align`] and [`unalign`] for scenarios where the output alignment
    /// is known.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Aligned, Unaligned, Vec3, Vec3U};
    /// #
    /// let aligned = Vec3::new(1, 2, 3);
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Vec3U::new(1, 2, 3));
    ///
    /// let unaligned = Vec3U::new(1, 2, 3);
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Vec3::new(1, 2, 3));
    /// ```
    ///
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
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

    /// Conversion to [`Aligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Vec3, Vec3U};
    /// #
    /// let unaligned = Vec3U::new(1, 2, 3);
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Vec3::new(1, 2, 3));
    /// ```
    #[inline]
    #[must_use]
    pub const fn align(self) -> Vector<N, T, Aligned> {
        self.to_alignment()
    }

    /// Conversion to [`Unaligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Vec3, Vec3U};
    /// #
    /// let aligned = Vec3::new(1, 2, 3);
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Vec3U::new(1, 2, 3));
    /// ```
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

    /// Returns a reference to the vector's elements.
    #[inline]
    #[must_use]
    pub const fn as_array_ref(&self) -> &[T; N] {
        // SAFETY: `Vector<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `T`.
        unsafe { transmute_ref::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns a mutable reference to the vector's elements.
    #[inline]
    #[must_use]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        // SAFETY: `Vector<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `T`.
        unsafe { transmute_mut::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns an iterator over the vector's elements.
    #[inline]
    #[must_use]
    pub fn iter(self) -> core::array::IntoIter<T, N> {
        self.to_array().into_iter()
    }

    /// Returns an iterator over mutable references to the vector's elements.
    #[inline]
    #[must_use = "iterators are lazy and do nothing unless consumed"]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_array_mut().iter_mut()
    }

    /// Returns a vector of the same length as `self`, with function `f` applied
    /// to each element in order.
    ///
    /// Equivalent to `(f(self.x), f(self.y), f(self.z), ..)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a.map(|x| x + 1);
    /// assert_eq!(b, Vec3::new(2, 3, 4));
    ///
    /// let a = Vec3::<i32>::new(1, -2, -3);
    /// let b = a.map(|x| x.is_negative());
    /// assert_eq!(b, Vec3::new(false, true, true));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn map<U, F>(self, f: F) -> Vector<N, U, A>
    where
        U: Scalar,
        F: Fn(T) -> U,
    {
        Vector::from_fn(|i| f(self[i]))
    }

    /// Returns a vector with the elements of `self` in reverse order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Vec3, vec3};
    /// #
    /// let vec = Vec3::new(1, 2, 3).reverse();
    /// assert_eq!(vec, Vec3::new(3, 2, 1));
    /// ```
    #[inline]
    #[must_use]
    pub fn reverse(self) -> Self {
        Self::from_fn(|i| self[N - 1 - i])
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// elements of `self` and `other` are equal.
    ///
    /// Equivalent to `(self.x == other.x, self.y == other.y, ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Vec3};
    /// #
    /// let vec = Vec3::new(1, 2, 3);
    /// let mask = vec.eq_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(false, true, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn eq_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialEq,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_eq_mask(self, other))
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// elements of `self` and `other` are not equal.
    ///
    /// Equivalent to `(self.x != other.x, self.y != other.y, ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Vec3};
    /// #
    /// let vec = Vec3::new(1, 2, 3);
    /// let mask = vec.ne_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(true, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn ne_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialEq,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_ne_mask(self, other))
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` is less than the corresponding element of `other`.
    ///
    /// Equivalent to `(self.x < other.x, self.y < other.y, ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Vec3};
    /// #
    /// let vec = Vec3::new(1, 2, 3);
    /// let mask = vec.lt_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(false, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn lt_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_lt_mask(self, other))
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` is greater than the corresponding element of `other`.
    ///
    /// Equivalent to `(self.x > other.x, self.y > other.y, ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Vec3};
    /// #
    /// let vec = Vec3::new(1, 2, 3);
    /// let mask = vec.gt_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(true, false, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn gt_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_gt_mask(self, other))
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` is less than or equal to the corresponding element of
    /// `other`.
    ///
    /// Equivalent to `(self.x <= other.x, self.y <= other.y, ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Vec3};
    /// #
    /// let vec = Vec3::new(1, 2, 3);
    /// let mask = vec.le_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(false, true, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn le_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_le_mask(self, other))
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` is greater than or equal to the corresponding element
    /// of `other`.
    ///
    /// Equivalent to `(self.x >= other.x, self.y >= other.y, ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Vec3};
    /// #
    /// let vec = Vec3::new(1, 2, 3);
    /// let mask = vec.ge_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(true, true, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn ge_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as ScalarBackend<N, A>>::vec_ge_mask(self, other))
    }

    /// Raw transmutation between scalar types.
    ///
    /// This function's signature staticly guarantees that the types have
    /// compatible memory layouts.
    ///
    /// This function is used to make SIMD optimizations in implementations of
    /// [`Scalar`].
    ///
    /// # Safety
    ///
    /// The elements of `self` must contain bit patterns that are valid for the
    /// output type. For example, when converting vectors from `u8` to `bool`,
    /// the input elements must be either `0` or `1`.
    ///
    /// The padding does not need to contain valid values of the output type.
    ///
    /// # Examples
    ///
    /// Correct usage:
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let bits = Vec3::<u8>::new(0, 1, 1);
    ///
    /// // SAFETY: `bool` accepts both the `0` and `1` bit patterns.
    /// let bools = unsafe { bits.to_repr::<bool>() };
    ///
    /// assert_eq!(bools, Vec3::new(false, true, true));
    /// ```
    ///
    /// Incorrect usage:
    ///
    /// ```compile_fail
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::<i32>::new(1, 2, 3);
    ///
    /// // This does not compile since `i32` and `i64` are not compatible.
    /// let _ = unsafe { a.to_repr::<i64>() };
    /// ```
    #[inline]
    #[must_use]
    #[expect(private_bounds)]
    pub const unsafe fn to_repr<U>(self) -> Vector<N, U, A>
    where
        U: Scalar<Repr = T::Repr>,
        T::Repr: SignedInteger,
    {
        // SAFETY: Vectors of scalars with the same `Scalar::Repr` are
        // guaranteed to have compatible memory layouts if `Repr` is a signed
        // integer.
        unsafe { transmute_generic::<Vector<N, T, A>, Vector<N, U, A>>(self) }
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

macro_rules! impl_unary_operator {
    ($Op:ident, $op:ident, $vec_op:ident, $(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self) -> Self::Output {
                specialize!(<T as ScalarBackend<N, A>>::$vec_op(self))
            }
        }
    };
}
impl_unary_operator!(
    Neg,
    neg,
    vec_neg,
    /// Performs the unary `-` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vec = -Vec3::new(1, 2, 3);
    /// assert_eq!(vec, Vec3::new(-1, -2, -3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
);
impl_unary_operator!(
    Not,
    not,
    vec_not,
    /// Performs the unary `!` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vec = !Vec3::new(1, 2, 3);
    /// assert_eq!(vec, Vec3::new(!1, !2, !3));
    /// ```
);

macro_rules! impl_binary_operator {
    ($Op:ident, $op:ident, $vec_op:ident, $(#[$doc:meta])*, $(#[$doc_scalar:meta])*) => {
        impl<const N: usize, T, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
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

            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: T) -> Self::Output {
                self.$op(Self::splat(rhs))
            }
        }
    };
}
impl_binary_operator!(
    Add,
    add,
    vec_add,
    /// Performs the `+` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a + Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 + 4, 2 + 5, 3 + 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ,
    /// Performs the `+` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a + 4;
    /// assert_eq!(b, Vec3::new(1 + 4, 2 + 4, 3 + 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec + splat(scalar)`.
);
impl_binary_operator!(
    Sub,
    sub,
    vec_sub,
    /// Performs the `-` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a - Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 - 4, 2 - 5, 3 - 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ,
    /// Performs the `-` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a - 4;
    /// assert_eq!(b, Vec3::new(1 - 4, 2 - 4, 3 - 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec - splat(scalar)`.
);
impl_binary_operator!(
    Mul,
    mul,
    vec_mul,
    /// Performs the `*` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a * Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 * 4, 2 * 5, 3 * 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ,
    /// Performs the `*` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a * 4;
    /// assert_eq!(b, Vec3::new(1 * 4, 2 * 4, 3 * 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec * splat(scalar)`.
);
impl_binary_operator!(
    Div,
    div,
    vec_div,
    /// Performs the `/` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(8, 10, 12);
    /// let b = a / Vec3::new(2, 5, 3);
    /// assert_eq!(b, Vec3::new(8 / 2, 10 / 5, 12 / 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ,
    /// Performs the `/` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(8, 10, 12);
    /// let b = a / 2;
    /// assert_eq!(b, Vec3::new(8 / 2, 10 / 2, 12 / 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec / splat(scalar)`.
);
impl_binary_operator!(
    Rem,
    rem,
    vec_rem,
    /// Performs the `%` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(5, 7, 9);
    /// let b = a % Vec3::new(2, 3, 4);
    /// assert_eq!(b, Vec3::new(5 % 2, 7 % 3, 9 % 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For integers this operation is fully consistent with the scalar
    /// operation, including panics.
    ///
    /// For floats this operation may be inconsistent with the scalar operation,
    /// regarding precision and NaN propagation.
    ,
    /// Performs the `%` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(5, 7, 9);
    /// let b = a % 2;
    /// assert_eq!(b, Vec3::new(5 % 2, 7 % 2, 9 % 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For integers this operation is fully consistent with the scalar
    /// operation, including panics.
    ///
    /// For floats this operation may be inconsistent with the scalar operation,
    /// regarding precision and NaN propagation.
    ///
    /// This operation is fully consistent with `vec % splat(scalar)`.
);
impl_binary_operator!(
    Shl,
    shl,
    vec_shl,
    /// Performs the `<<` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a << Vec3::new(1, 2, 3);
    /// assert_eq!(b, Vec3::new(1 << 1, 2 << 2, 3 << 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ,
    /// Performs the `<<` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a << 1;
    /// assert_eq!(b, Vec3::new(1 << 1, 2 << 1, 3 << 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vec << splat(scalar)`.
);
impl_binary_operator!(
    Shr,
    shr,
    vec_shr,
    /// Performs the `>>` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(8, 16, 32);
    /// let b = a >> Vec3::new(1, 2, 3);
    /// assert_eq!(b, Vec3::new(8 >> 1, 16 >> 2, 32 >> 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ,
    /// Performs the `>>` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(8, 16, 32);
    /// let b = a >> 1;
    /// assert_eq!(b, Vec3::new(8 >> 1, 16 >> 1, 32 >> 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vec >> splat(scalar)`.
);
impl_binary_operator!(
    BitAnd,
    bitand,
    vec_bitand,
    /// Performs the `&` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a & Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 & 4, 2 & 5, 3 & 6));
    /// ```
    ,
    /// Performs the `&` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a & 4;
    /// assert_eq!(b, Vec3::new(1 & 4, 2 & 4, 3 & 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec & splat(scalar)`.
);
impl_binary_operator!(
    BitOr,
    bitor,
    vec_bitor,
    /// Performs the `|` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a | Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 | 4, 2 | 5, 3 | 6));
    /// ```
    ,
    /// Performs the `|` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a | 4;
    /// assert_eq!(b, Vec3::new(1 | 4, 2 | 4, 3 | 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec | splat(scalar)`.
);
impl_binary_operator!(
    BitXor,
    bitxor,
    vec_bitxor,
    /// Performs the `^` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a ^ Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 ^ 4, 2 ^ 5, 3 ^ 6));
    /// ```
    ,
    /// Performs the `^` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a ^ 4;
    /// assert_eq!(b, Vec3::new(1 ^ 4, 2 ^ 4, 3 ^ 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec ^ splat(scalar)`.
);

macro_rules! impl_assign_operator {
    ($Op:ident, $OpAssign:ident, $op_assign:ident, $op:ident, $(#[$doc:meta])*, $(#[$doc_scalar:meta])*) => {
        impl<const N: usize, T, A: Alignment> $OpAssign for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op_assign(&mut self, rhs: T) {
                *self = self.$op(rhs);
            }
        }
    };
}
impl_assign_operator!(
    Add,
    AddAssign,
    add_assign,
    add,
    /// Performs the `+=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec += Vec3::new(4, 5, 6);
    /// assert_eq!(vec, Vec3::new(1 + 4, 2 + 5, 3 + 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec + vec`.
    ,
    /// Performs the `+=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec += 4;
    /// assert_eq!(vec, Vec3::new(1 + 4, 2 + 4, 3 + 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec + vec`.
);
impl_assign_operator!(
    Sub,
    SubAssign,
    sub_assign,
    sub,
    /// Performs the `-=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(5, 7, 9);
    /// vec -= Vec3::new(1, 2, 3);
    /// assert_eq!(vec, Vec3::new(5 - 1, 7 - 2, 9 - 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec - vec`.
    ,
    /// Performs the `-=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(5, 7, 9);
    /// vec -= 2;
    /// assert_eq!(vec, Vec3::new(5 - 2, 7 - 2, 9 - 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec - vec`.
);
impl_assign_operator!(
    Mul,
    MulAssign,
    mul_assign,
    mul,
    /// Performs the `*=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec *= Vec3::new(4, 5, 6);
    /// assert_eq!(vec, Vec3::new(1 * 4, 2 * 5, 3 * 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec * vec`.
    ,
    /// Performs the `*=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec *= 4;
    /// assert_eq!(vec, Vec3::new(1 * 4, 2 * 4, 3 * 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec * vec`.
);
impl_assign_operator!(
    Div,
    DivAssign,
    div_assign,
    div,
    /// Performs the `/=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(8, 10, 12);
    /// vec /= Vec3::new(2, 5, 3);
    /// assert_eq!(vec, Vec3::new(8 / 2, 10 / 5, 12 / 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec / vec`.
    ,
    /// Performs the `/=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(8, 10, 12);
    /// vec /= 2;
    /// assert_eq!(vec, Vec3::new(8 / 2, 10 / 2, 12 / 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vec / vec`.
);
impl_assign_operator!(
    Rem,
    RemAssign,
    rem_assign,
    rem,
    /// Performs the `%=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(5, 7, 9);
    /// vec %= Vec3::new(2, 3, 4);
    /// assert_eq!(vec, Vec3::new(5 % 2, 7 % 3, 9 % 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For integers this operation is fully consistent with the scalar
    /// operation, including panics.
    ///
    /// For floats this operation may be inconsistent with the scalar operation,
    /// regarding precision and NaN propagation.
    ///
    /// This operation is fully consistent with `vec % vec`.
    ,
    /// Performs the `%=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(5, 7, 9);
    /// vec %= 2;
    /// assert_eq!(vec, Vec3::new(5 % 2, 7 % 2, 9 % 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For integers this operation is fully consistent with the scalar
    /// operation, including panics.
    ///
    /// For floats this operation may be inconsistent with the scalar operation,
    /// regarding precision and NaN propagation.
    ///
    /// This operation is fully consistent with `vec % vec`.
);
impl_assign_operator!(
    Shl,
    ShlAssign,
    shl_assign,
    shl,
    /// Performs the `<<=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec <<= Vec3::new(1, 2, 3);
    /// assert_eq!(vec, Vec3::new(1 << 1, 2 << 2, 3 << 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vec << vec`.
    ,
    /// Performs the `<<=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec <<= 1;
    /// assert_eq!(vec, Vec3::new(1 << 1, 2 << 1, 3 << 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vec << vec`.
);
impl_assign_operator!(
    Shr,
    ShrAssign,
    shr_assign,
    shr,
    /// Performs the `>>=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(8, 16, 32);
    /// vec >>= Vec3::new(1, 2, 3);
    /// assert_eq!(vec, Vec3::new(8 >> 1, 16 >> 2, 32 >> 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vec >> vec`.
    ,
    /// Performs the `>>=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(8, 16, 32);
    /// vec >>= 1;
    /// assert_eq!(vec, Vec3::new(8 >> 1, 16 >> 1, 32 >> 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vec >> vec`.
);
impl_assign_operator!(
    BitAnd,
    BitAndAssign,
    bitand_assign,
    bitand,
    /// Performs the `&=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec &= Vec3::new(4, 5, 6);
    /// assert_eq!(vec, Vec3::new(1 & 4, 2 & 5, 3 & 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec & vec`.
    ,
    /// Performs the `&=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec &= 4;
    /// assert_eq!(vec, Vec3::new(1 & 4, 2 & 4, 3 & 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec & vec`.
);
impl_assign_operator!(
    BitOr,
    BitOrAssign,
    bitor_assign,
    bitor,
    /// Performs the `|=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec |= Vec3::new(4, 5, 6);
    /// assert_eq!(vec, Vec3::new(1 | 4, 2 | 5, 3 | 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec | vec`.
    ,
    /// Performs the `|=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec |= 4;
    /// assert_eq!(vec, Vec3::new(1 | 4, 2 | 4, 3 | 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec | vec`.
);
impl_assign_operator!(
    BitXor,
    BitXorAssign,
    bitxor_assign,
    bitxor,
    /// Performs the `^=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec ^= Vec3::new(4, 5, 6);
    /// assert_eq!(vec, Vec3::new(1 ^ 4, 2 ^ 5, 3 ^ 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec ^ vec`.
    ,
    /// Performs the `^=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vec = Vec3::new(1, 2, 3);
    /// vec ^= 4;
    /// assert_eq!(vec, Vec3::new(1 ^ 4, 2 ^ 4, 3 ^ 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vec ^ vec`.
);

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
