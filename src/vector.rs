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
    Aligned, Alignment, Backend, Length, Mask, Scalar, SupportedLength, Unaligned,
    constants::{False, Infinity, Max, Min, Nan, NegInfinity, NegOne, One, True, Zero},
    utils::{Repr2, Repr3, Repr4, specialize, transmute_generic, transmute_mut, transmute_ref},
};

mod bool;
mod float;
mod signed;
mod swizzle;
mod unsigned;
#[cfg(feature = "wide")]
mod wide;
#[cfg(feature = "wide")]
mod wide_float;
#[cfg(feature = "wide")]
mod wide_signed;
#[cfg(feature = "wide")]
mod wide_unsigned;

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
/// [`Vec2<T>`]: crate::Vec2
/// [`Vec3<T>`]: crate::Vec3
/// [`Vec4<T>`]: crate::Vec4
/// [`Vec2U<T>`]: crate::Vec2U
/// [`Vec3U<T>`]: crate::Vec3U
/// [`Vec4U<T>`]: crate::Vec4U
#[repr(transparent)]
pub struct Vector<const N: usize, T, A: Alignment>(
    /// The internal representation of the vector.
    ///
    /// This field's insane type corresponds to [`<T as Backend<N, A>>::Vector`]
    /// which cannot be used directly because of type system limitations. In
    /// generic contexts this field will not work, in which case you should use
    /// [`from_inner`], [`inner`] and [`inner_mut`].
    ///
    /// This field should only be accessed from the crate defining `T`, else its
    /// type may change silently as it is considered an implementation detail.
    ///
    /// [`<T as Backend<N, A>>::Vector`]: Backend
    /// [`from_inner`]: Self::from_inner
    /// [`inner`]: Self::inner
    /// [`inner_mut`]: Self::inner_mut
    #[expect(clippy::type_complexity)]
    pub  <A as Alignment>::Select<
        <Length<N> as SupportedLength>::Select<
            <T as Backend<2, Aligned>>::Vector,
            <T as Backend<3, Aligned>>::Vector,
            <T as Backend<4, Aligned>>::Vector,
        >,
        <Length<N> as SupportedLength>::Select<
            <T as Backend<2, Unaligned>>::Vector,
            <T as Backend<3, Unaligned>>::Vector,
            <T as Backend<4, Unaligned>>::Vector,
        >,
    >,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

/// A 2-dimensional vector.
///
/// # SIMD alignment
///
/// `Vec2<T>` has SIMD alignment for appropriate scalar types. See [`Vec2U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec2<T> = Vector<2, T, Aligned>;

/// A 3-dimensional vector.
///
/// # SIMD alignment
///
/// `Vec3<T>` has SIMD alignment for appropriate scalar types. See [`Vec3U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// `z: T`
///
/// The third element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec3<T> = Vector<3, T, Aligned>;

/// A 4-dimensional vector.
///
/// # SIMD alignment
///
/// `Vec4<T>` has SIMD alignment for appropriate scalar types. See [`Vec4U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// `z: T`
///
/// The third element of the vector.
///
/// `w: T`
///
/// The fourth element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec4<T> = Vector<4, T, Aligned>;

/// A 2-dimensional vector.
///
/// # No SIMD alignment
///
/// `Vec2U<T>` does not have SIMD alignment. See [`Vec2<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec2U<T> = Vector<2, T, Unaligned>;

/// A 3-dimensional vector.
///
/// # No SIMD alignment
///
/// `Vec3U<T>` does not have SIMD alignment. See [`Vec3<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// `z: T`
///
/// The third element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec3U<T> = Vector<3, T, Unaligned>;

/// A 4-dimensional vector.
///
/// # No SIMD alignment
///
/// `Vec4U<T>` does not have SIMD alignment. See [`Vec4<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// `z: T`
///
/// The third element of the vector.
///
/// `w: T`
///
/// The fourth element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec4U<T> = Vector<4, T, Unaligned>;

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
    /// let vector = Vec3::splat(5);
    /// assert_eq!(vector, Vec3::new(5, 5, 5));
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
    /// let vector = Vec3::from_fn(|i| i % 2);
    /// assert_eq!(vector, Vec3::new(0, 1, 0));
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
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(1, 2, 3).reverse();
    /// assert_eq!(vector, Vec3::new(3, 2, 1));
    /// ```
    #[inline]
    #[must_use]
    pub fn reverse(self) -> Self {
        Self::from_fn(|i| self[N - 1 - i])
    }

    /// Computes the sum of the elements of `self`.
    ///
    /// Equivalent to `self.x + self.y + ...`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation) or overflow
    /// checks are enabled:
    ///
    /// For integers this panics if any addition overflows (order unspecified).
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross platform deterministic.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn element_sum(self) -> T
    where
        T: Add<Output = T>,
    {
        specialize!(<T as Backend<N, A>>::vector_element_sum(self))
    }

    /// Computes the product of the elements of `self`.
    ///
    /// Equivalent to `self.x * self.y * ...`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation) or overflow
    /// checks are enabled:
    ///
    /// For integers this panics if any multiplication overflows (order unspecified).
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross platform deterministic.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn element_product(self) -> T
    where
        T: Mul<Output = T>,
    {
        specialize!(<T as Backend<N, A>>::vector_element_product(self))
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
    /// let vector = Vec3::new(1, 2, 3);
    /// let mask = vector.eq_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(false, true, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn eq_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialEq,
    {
        specialize!(<T as Backend<N, A>>::vector_eq_mask(self, other))
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
    /// let vector = Vec3::new(1, 2, 3);
    /// let mask = vector.ne_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(true, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn ne_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialEq,
    {
        specialize!(<T as Backend<N, A>>::vector_ne_mask(self, other))
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
    /// let vector = Vec3::new(1, 2, 3);
    /// let mask = vector.lt_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(false, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn lt_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as Backend<N, A>>::vector_lt_mask(self, other))
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
    /// let vector = Vec3::new(1, 2, 3);
    /// let mask = vector.gt_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(true, false, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn gt_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as Backend<N, A>>::vector_gt_mask(self, other))
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
    /// let vector = Vec3::new(1, 2, 3);
    /// let mask = vector.le_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(false, true, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn le_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as Backend<N, A>>::vector_le_mask(self, other))
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
    /// let vector = Vec3::new(1, 2, 3);
    /// let mask = vector.ge_mask(Vec3::new(0, 2, 5));
    /// assert_eq!(mask, Mask3::new(true, true, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn ge_mask(self, other: Self) -> Mask<N, T, A>
    where
        T: PartialOrd,
    {
        specialize!(<T as Backend<N, A>>::vector_ge_mask(self, other))
    }

    /// Computes the dot product of `self` and `rhs`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation) or
    /// overflow checks are enabled:
    ///
    /// For integers this panics if an overflow occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let x = Vec3::new(2, 0, 0);
    /// let y = Vec3::new(0, 3, 0);
    ///
    /// assert_eq!(x.dot(y), 0);
    /// assert_eq!(x.dot(x), 4);
    /// assert_eq!(y.dot(y), 9);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        (self * rhs).element_sum()
    }

    /// Computes the squared length/magnitude of `self`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation) or
    /// overflow checks are enabled:
    ///
    /// For integers this panics if an overflow occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec2;
    /// #
    /// let vector = Vec2::new(1, 2);
    /// assert_eq!(vector.length_squared(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        (self * self).element_sum()
    }

    /// Computes the squared Euclidean distance between `self` and `other`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation) or
    /// overflow checks are enabled:
    ///
    /// For integers this panics if an overflow occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let x = Vec3::<i32>::new(2, 0, 0);
    /// let y = Vec3::<i32>::new(0, 3, 0);
    ///
    /// assert_eq!(x.distance_squared(y), 13);
    /// assert_eq!(x.distance_squared(x), 0);
    /// assert_eq!(y.distance_squared(y), 0);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn distance_squared(self, other: Self) -> T
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        (self - other).length_squared()
    }

    /// Creates a vector from its internal representation.
    ///
    /// Equivalent to `Vector(inner)` but works in generic contexts.
    ///
    /// The input type is specified by [`<T as Backend<N, A>>`]. This should
    /// only be called from the crate defining `T`, else the input type may
    /// change silently as it is considered an implementation detail.
    ///
    /// [`<T as Backend<N, A>>`]: Backend
    #[inline]
    #[must_use]
    pub const fn from_inner(inner: <T as Backend<N, A>>::Vector) -> Self
    where
        T: Backend<N, A>,
    {
        // SAFETY: These always correspond to the same type.
        Self(unsafe {
            transmute_generic::<
                <T as Backend<N, A>>::Vector,
                <A as Alignment>::Select<
                    <Length<N> as SupportedLength>::Select<
                        <T as Backend<2, Aligned>>::Vector,
                        <T as Backend<3, Aligned>>::Vector,
                        <T as Backend<4, Aligned>>::Vector,
                    >,
                    <Length<N> as SupportedLength>::Select<
                        <T as Backend<2, Unaligned>>::Vector,
                        <T as Backend<3, Unaligned>>::Vector,
                        <T as Backend<4, Unaligned>>::Vector,
                    >,
                >,
            >(inner)
        })
    }

    /// Returns the internal representation of `self`.
    ///
    /// Equivalent to `self.0` but works in generic contexts.
    ///
    /// The resulting type is specified by [`<T as Backend<N, A>>`]. This should
    /// only be called from the crate defining `T`, else the resulting type may
    /// change silently as it is considered an implementation detail.
    ///
    /// [`<T as Backend<N, A>>`]: Backend
    #[inline]
    #[must_use]
    pub const fn inner(self) -> <T as Backend<N, A>>::Vector
    where
        T: Backend<N, A>,
    {
        // SAFETY: `Vector<N, T, A>` is a transparent wrapper over
        // `<T as Backend<N, A>>::Vector`.
        unsafe { transmute_generic::<Vector<N, T, A>, <T as Backend<N, A>>::Vector>(self) }
    }

    /// Returns a mutable reference to the internal representation of `self`.
    ///
    /// Equivalent to `&mut self.0` but works in generic contexts.
    ///
    /// The resulting type is specified by [`<T as Backend<N, A>>`]. This should
    /// only be called from the crate defining `T`, else the resulting type may
    /// change silently as it is considered an implementation detail.
    ///
    /// [`<T as Backend<N, A>>`]: Backend
    #[inline]
    #[must_use]
    pub const fn inner_mut(&mut self) -> &mut <T as Backend<N, A>>::Vector
    where
        T: Backend<N, A>,
    {
        // SAFETY: `Vector<N, T, A>` is a transparent wrapper over
        // `<T as Backend<N, A>>::Vector`.
        unsafe { transmute_mut::<Vector<N, T, A>, <T as Backend<N, A>>::Vector>(self) }
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

    /// Returns a 3-dimensional vector containing the elements of `self` then
    /// the scalar `value`.
    ///
    /// Equivalent to `(self, value)`.
    #[inline]
    #[must_use]
    pub fn extend(self, value: T) -> Vector<3, T, A> {
        Vector::<3, T, A>::new(self.x, self.y, value)
    }

    /// Returns `self` rotated by 90 degrees.
    ///
    /// This rotates `+X` to `+Y`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec2;
    /// #
    /// let x = Vec2::new(1, 0);
    /// let y = Vec2::new(0, 1);
    ///
    /// assert_eq!(x.perp(), y);
    /// assert_eq!(y.perp(), -x);
    /// assert_eq!((-x).perp(), -y);
    /// assert_eq!((-y).perp(), x);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perp(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::new(-self.y, self.x)
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

    /// Returns a 4-dimensional vector containing the elements of `self` then
    /// the scalar `value`.
    ///
    /// Equivalent to `(self, value)`.
    #[inline]
    #[must_use]
    pub fn extend(self, value: T) -> Vector<4, T, A> {
        Vector::<4, T, A>::new(self.x, self.y, self.z, value)
    }

    /// Returns a 2-dimensional vector containing the first 2 elements of
    /// `self`, discarding the last element.
    ///
    /// Equivalent to `self.xy`.
    #[inline]
    #[must_use]
    pub fn truncate(self) -> Vector<2, T, A> {
        self.xy()
    }

    /// Converts `self` to homogeneous coordinates.
    ///
    /// Equivalent to `self.extend(1)`.
    #[inline]
    #[must_use]
    pub fn to_homogeneous(self) -> Vector<4, T, A>
    where
        T: One,
    {
        self.extend(T::ONE)
    }

    /// Computes the cross product of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let x = Vec3::new(1, 0, 0);
    /// let y = Vec3::new(0, 1, 0);
    ///
    /// assert_eq!(x.cross(y), Vec3::new(0, 0, 1));
    /// assert_eq!(y.cross(x), Vec3::new(0, 0, -1));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn cross(self, rhs: Self) -> Self
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        (self.zxy() * rhs - self * rhs.zxy()).zxy()
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

    /// Returns a 3-dimensional vector containing the first 3 elements of
    /// `self`, discarding the last element.
    ///
    /// Equivalent to `self.xyz`.
    #[inline]
    #[must_use]
    pub fn truncate(self) -> Vector<3, T, A> {
        self.xyz()
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

impl<T, A: Alignment> From<(T, T)> for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T, A: Alignment> From<(T, T, T)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T, A: Alignment> From<(T, Vector<2, T, A>)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<2, T, A>)) -> Self {
        Self::new(value.0, value.1[0], value.1[1])
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, T)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.1)
    }
}

impl<T, A: Alignment> From<(T, T, T, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl<T, A: Alignment> From<(T, T, Vector<2, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, Vector<2, T, A>)) -> Self {
        Self::new(value.0, value.1, value.2[0], value.2[1])
    }
}

impl<T, A: Alignment> From<(T, Vector<2, T, A>, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<2, T, A>, T)) -> Self {
        Self::new(value.0, value.1[0], value.1[1], value.2)
    }
}

impl<T, A: Alignment> From<(T, Vector<3, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<3, T, A>)) -> Self {
        Self::new(value.0, value.1[0], value.1[1], value.1[2])
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, T, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, T, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.1, value.2)
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, Vector<2, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, Vector<2, T, A>)) -> Self {
        Self::new(value.0[0], value.0[1], value.1[0], value.1[1])
    }
}

impl<T, A: Alignment> From<(Vector<3, T, A>, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<3, T, A>, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.0[2], value.1)
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
        specialize!(<T as Backend<N, A>>::vector_eq(self, other))
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as Backend<N, A>>::vector_ne(self, other))
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
    ($Op:ident, $op:ident, $vector_op:ident, $(#[$doc:meta])*) => {
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
                specialize!(<T as Backend<N, A>>::$vector_op(self))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self) -> Self::Output {
                Vector::$op(*self)
            }
        }
    };
}
impl_unary_operator!(
    Neg,
    neg,
    vector_neg,
    /// Performs the unary `-` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = -Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(-1, -2, -3));
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
    vector_not,
    /// Performs the unary `!` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = !Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(!1, !2, !3));
    /// ```
);

macro_rules! impl_binary_operator {
    ($Op:ident, $op:ident, $vector_op:ident, $(#[$doc:meta])*, $(#[$doc_scalar:meta])*) => {
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
                specialize!(<T as Backend<N, A>>::$vector_op(self, rhs))
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

        impl<const N: usize, T, A: Alignment> $Op<&Vector<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: &Vector<N, T, A>) -> Self::Output {
                self.$op(*rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: &T) -> Self::Output {
                self.$op(Self::splat(*rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<Vector<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: Vector<N, T, A>) -> Self::Output {
                Vector::$op(*self, rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<T> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: T) -> Self::Output {
                Vector::$op(*self, Vector::splat(rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&Vector<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: &Vector<N, T, A>) -> Self::Output {
                Vector::$op(*self, *rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&T> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: &T) -> Self::Output {
                Vector::$op(*self, Vector::splat(*rhs))
            }
        }
    };
}
impl_binary_operator!(
    Add,
    add,
    vector_add,
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
    /// This operation is fully consistent with `vector + splat(scalar)`.
);
impl_binary_operator!(
    Sub,
    sub,
    vector_sub,
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
    /// This operation is fully consistent with `vector - splat(scalar)`.
);
impl_binary_operator!(
    Mul,
    mul,
    vector_mul,
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
    /// This operation is fully consistent with `vector * splat(scalar)`.
);
impl_binary_operator!(
    Div,
    div,
    vector_div,
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
    /// This operation is fully consistent with `vector / splat(scalar)`.
);
impl_binary_operator!(
    Rem,
    rem,
    vector_rem,
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
    /// This operation is fully consistent with `vector % splat(scalar)`.
);
impl_binary_operator!(
    Shl,
    shl,
    vector_shl,
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
    /// This operation is fully consistent with `vector << splat(scalar)`.
);
impl_binary_operator!(
    Shr,
    shr,
    vector_shr,
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
    /// This operation is fully consistent with `vector >> splat(scalar)`.
);
impl_binary_operator!(
    BitAnd,
    bitand,
    vector_bitand,
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
    /// This operation is fully consistent with `vector & splat(scalar)`.
);
impl_binary_operator!(
    BitOr,
    bitor,
    vector_bitor,
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
    /// This operation is fully consistent with `vector | splat(scalar)`.
);
impl_binary_operator!(
    BitXor,
    bitxor,
    vector_bitxor,
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
    /// This operation is fully consistent with `vector ^ splat(scalar)`.
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

        impl<const N: usize, T, A: Alignment> $OpAssign<&Vector<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op_assign(&mut self, rhs: &Vector<N, T, A>) {
                *self = self.$op(*rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<&T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op_assign(&mut self, rhs: &T) {
                *self = self.$op(*rhs);
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
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector += Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 + 4, 2 + 5, 3 + 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector + vector`.
    ,
    /// Performs the `+=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector += 4;
    /// assert_eq!(vector, Vec3::new(1 + 4, 2 + 4, 3 + 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector + vector`.
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
    /// let mut vector = Vec3::new(5, 7, 9);
    /// vector -= Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(5 - 1, 7 - 2, 9 - 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector - vector`.
    ,
    /// Performs the `-=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(5, 7, 9);
    /// vector -= 2;
    /// assert_eq!(vector, Vec3::new(5 - 2, 7 - 2, 9 - 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector - vector`.
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
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector *= Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 * 4, 2 * 5, 3 * 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector * vector`.
    ,
    /// Performs the `*=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector *= 4;
    /// assert_eq!(vector, Vec3::new(1 * 4, 2 * 4, 3 * 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector * vector`.
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
    /// let mut vector = Vec3::new(8, 10, 12);
    /// vector /= Vec3::new(2, 5, 3);
    /// assert_eq!(vector, Vec3::new(8 / 2, 10 / 5, 12 / 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector / vector`.
    ,
    /// Performs the `/=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(8, 10, 12);
    /// vector /= 2;
    /// assert_eq!(vector, Vec3::new(8 / 2, 10 / 2, 12 / 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector / vector`.
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
    /// let mut vector = Vec3::new(5, 7, 9);
    /// vector %= Vec3::new(2, 3, 4);
    /// assert_eq!(vector, Vec3::new(5 % 2, 7 % 3, 9 % 4));
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
    /// This operation is fully consistent with `vector % vector`.
    ,
    /// Performs the `%=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(5, 7, 9);
    /// vector %= 2;
    /// assert_eq!(vector, Vec3::new(5 % 2, 7 % 2, 9 % 2));
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
    /// This operation is fully consistent with `vector % vector`.
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
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector <<= Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(1 << 1, 2 << 2, 3 << 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector << vector`.
    ,
    /// Performs the `<<=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector <<= 1;
    /// assert_eq!(vector, Vec3::new(1 << 1, 2 << 1, 3 << 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector << vector`.
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
    /// let mut vector = Vec3::new(8, 16, 32);
    /// vector >>= Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(8 >> 1, 16 >> 2, 32 >> 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector >> vector`.
    ,
    /// Performs the `>>=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(8, 16, 32);
    /// vector >>= 1;
    /// assert_eq!(vector, Vec3::new(8 >> 1, 16 >> 1, 32 >> 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector >> vector`.
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
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector &= Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 & 4, 2 & 5, 3 & 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector & vector`.
    ,
    /// Performs the `&=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector &= 4;
    /// assert_eq!(vector, Vec3::new(1 & 4, 2 & 4, 3 & 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector & vector`.
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
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector |= Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 | 4, 2 | 5, 3 | 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector | vector`.
    ,
    /// Performs the `|=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector |= 4;
    /// assert_eq!(vector, Vec3::new(1 | 4, 2 | 4, 3 | 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector | vector`.
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
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector ^= Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 ^ 4, 2 ^ 5, 3 ^ 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector ^ vector`.
    ,
    /// Performs the `^=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector ^= 4;
    /// assert_eq!(vector, Vec3::new(1 ^ 4, 2 ^ 4, 3 ^ 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector ^ vector`.
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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::{format, vec};

    use itertools::Itertools;

    use crate::{
        Aligned, Mask, Unaligned, Vec2, Vec2U, Vec3, Vec3U, Vec4, Vec4U, Vector,
        utils::{
            Repr2, Repr3, Repr4, assert_float_eq, assert_panic, assert_panic_eq, for_parameters,
        },
    };

    #[test]
    fn test_layout() {
        for_parameters!(|T: PrimitiveNumber| {
            assert_eq!(size_of::<Vec2<T>>(), size_of::<T>() * 2);
            assert!(
                align_of::<Vec2<T>>() == align_of::<T>()
                    || align_of::<Vec2<T>>() == size_of::<T>() * 2
            );

            assert!(
                size_of::<Vec3<T>>() == size_of::<T>() * 3
                    && align_of::<Vec3<T>>() == align_of::<T>()
                    || size_of::<Vec3<T>>() == size_of::<T>() * 4
                        && align_of::<Vec3<T>>() == size_of::<T>() * 4
            );

            assert_eq!(size_of::<Vec4<T>>(), size_of::<T>() * 4);
            assert!(
                align_of::<Vec4<T>>() == align_of::<T>()
                    || align_of::<Vec4<T>>() == size_of::<T>() * 4
            );

            assert_eq!(size_of::<Vec2U<T>>(), size_of::<T>() * 2);
            assert_eq!(align_of::<Vec2U<T>>(), align_of::<T>());

            assert_eq!(size_of::<Vec3U<T>>(), size_of::<T>() * 3);
            assert_eq!(align_of::<Vec3U<T>>(), align_of::<T>());

            assert_eq!(size_of::<Vec4U<T>>(), size_of::<T>() * 4);
            assert_eq!(align_of::<Vec4U<T>>(), align_of::<T>());
        });
    }

    #[test]
    fn test_zero() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(Vector::<2, T, A>::ZERO, Vector::splat(T::as_from(0)));
            assert_eq!(Vector::<3, T, A>::ZERO, Vector::splat(T::as_from(0)));
            assert_eq!(Vector::<4, T, A>::ZERO, Vector::splat(T::as_from(0)));
        });
    }

    #[test]
    fn test_one() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(Vector::<2, T, A>::ONE, Vector::splat(T::as_from(1)));
            assert_eq!(Vector::<3, T, A>::ONE, Vector::splat(T::as_from(1)));
            assert_eq!(Vector::<4, T, A>::ONE, Vector::splat(T::as_from(1)));
        });
    }

    #[test]
    fn test_neg_one() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(Vector::<2, T, A>::NEG_ONE, Vector::splat(-1.0));
            assert_eq!(Vector::<3, T, A>::NEG_ONE, Vector::splat(-1.0));
            assert_eq!(Vector::<4, T, A>::NEG_ONE, Vector::splat(-1.0));
        });
        for_parameters!(|T: PrimitiveSigned, A| {
            assert_eq!(Vector::<2, T, A>::NEG_ONE, Vector::splat(-1));
            assert_eq!(Vector::<3, T, A>::NEG_ONE, Vector::splat(-1));
            assert_eq!(Vector::<4, T, A>::NEG_ONE, Vector::splat(-1));
        });
    }

    #[test]
    fn test_min() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(Vector::<2, T, A>::MIN, Vector::splat(T::MIN));
            assert_eq!(Vector::<3, T, A>::MIN, Vector::splat(T::MIN));
            assert_eq!(Vector::<4, T, A>::MIN, Vector::splat(T::MIN));
        });
        for_parameters!(|T: PrimitiveSigned, A| {
            assert_eq!(Vector::<2, T, A>::MIN, Vector::splat(T::MIN));
            assert_eq!(Vector::<3, T, A>::MIN, Vector::splat(T::MIN));
            assert_eq!(Vector::<4, T, A>::MIN, Vector::splat(T::MIN));
        });
        for_parameters!(|T: PrimitiveUnsigned, A| {
            assert_eq!(Vector::<2, T, A>::MIN, Vector::splat(T::MIN));
            assert_eq!(Vector::<3, T, A>::MIN, Vector::splat(T::MIN));
            assert_eq!(Vector::<4, T, A>::MIN, Vector::splat(T::MIN));
        });
    }

    #[test]
    fn test_max() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(Vector::<2, T, A>::MAX, Vector::splat(T::MAX));
            assert_eq!(Vector::<3, T, A>::MAX, Vector::splat(T::MAX));
            assert_eq!(Vector::<4, T, A>::MAX, Vector::splat(T::MAX));
        });
        for_parameters!(|T: PrimitiveSigned, A| {
            assert_eq!(Vector::<2, T, A>::MAX, Vector::splat(T::MAX));
            assert_eq!(Vector::<3, T, A>::MAX, Vector::splat(T::MAX));
            assert_eq!(Vector::<4, T, A>::MAX, Vector::splat(T::MAX));
        });
        for_parameters!(|T: PrimitiveUnsigned, A| {
            assert_eq!(Vector::<2, T, A>::MAX, Vector::splat(T::MAX));
            assert_eq!(Vector::<3, T, A>::MAX, Vector::splat(T::MAX));
            assert_eq!(Vector::<4, T, A>::MAX, Vector::splat(T::MAX));
        });
    }

    #[test]
    fn test_nan() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(Vector::<2, T, A>::NAN, Vector::splat(T::NAN));
            assert_float_eq!(Vector::<3, T, A>::NAN, Vector::splat(T::NAN));
            assert_float_eq!(Vector::<4, T, A>::NAN, Vector::splat(T::NAN));
        });
    }

    #[test]
    fn test_infinity() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(Vector::<2, T, A>::INFINITY, Vector::splat(T::INFINITY));
            assert_eq!(Vector::<3, T, A>::INFINITY, Vector::splat(T::INFINITY));
            assert_eq!(Vector::<4, T, A>::INFINITY, Vector::splat(T::INFINITY));
        });
    }

    #[test]
    fn test_neg_infinity() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Vector::<2, T, A>::NEG_INFINITY,
                Vector::splat(T::NEG_INFINITY)
            );
            assert_eq!(
                Vector::<3, T, A>::NEG_INFINITY,
                Vector::splat(T::NEG_INFINITY)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_INFINITY,
                Vector::splat(T::NEG_INFINITY)
            );
        });
    }

    #[test]
    fn test_true() {
        for_parameters!(|A| {
            assert_eq!(Vector::<2, bool, A>::TRUE, Vector::splat(true));
            assert_eq!(Vector::<3, bool, A>::TRUE, Vector::splat(true));
            assert_eq!(Vector::<4, bool, A>::TRUE, Vector::splat(true));
        });
    }

    #[test]
    fn test_false() {
        for_parameters!(|A| {
            assert_eq!(Vector::<2, bool, A>::FALSE, Vector::splat(false));
            assert_eq!(Vector::<3, bool, A>::FALSE, Vector::splat(false));
            assert_eq!(Vector::<4, bool, A>::FALSE, Vector::splat(false));
        });
    }

    #[test]
    fn test_from_array() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::from_array([x, y]),
                Vector::<2, T, A>::new(x, y)
            );
            assert_eq!(
                Vector::<3, T, A>::from_array([x, y, z]),
                Vector::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Vector::<4, T, A>::from_array([x, y, z, w]),
                Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_splat() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x] = std::array::from_fn(T::as_from);

            assert_eq!(Vector::<2, T, A>::splat(x), Vector::<2, T, A>::new(x, x));
            assert_eq!(Vector::<3, T, A>::splat(x), Vector::<3, T, A>::new(x, x, x));
            assert_eq!(
                Vector::<4, T, A>::splat(x),
                Vector::<4, T, A>::new(x, x, x, x)
            );
        });
    }

    #[test]
    fn test_from_fn() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::from_fn(|i| [x, y][i]),
                Vector::<2, T, A>::new(x, y)
            );
            assert_eq!(
                Vector::<3, T, A>::from_fn(|i| [x, y, z][i]),
                Vector::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Vector::<4, T, A>::from_fn(|i| [x, y, z, w][i]),
                Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).to_alignment(),
                Vector::<2, T, Aligned>::new(x, y)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).to_alignment(),
                Vector::<3, T, Aligned>::new(x, y, z)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).to_alignment(),
                Vector::<4, T, Aligned>::new(x, y, z, w)
            );

            assert_eq!(
                Vector::<2, T, A>::new(x, y).to_alignment(),
                Vector::<2, T, Unaligned>::new(x, y)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).to_alignment(),
                Vector::<3, T, Unaligned>::new(x, y, z)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).to_alignment(),
                Vector::<4, T, Unaligned>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_align() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).align(),
                Vector::<2, T, Aligned>::new(x, y)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).align(),
                Vector::<3, T, Aligned>::new(x, y, z)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).align(),
                Vector::<4, T, Aligned>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).unalign(),
                Vector::<2, T, Unaligned>::new(x, y)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).unalign(),
                Vector::<3, T, Unaligned>::new(x, y, z)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).unalign(),
                Vector::<4, T, Unaligned>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_array() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(Vector::<2, T, A>::new(x, y).to_array(), [x, y]);
            assert_eq!(Vector::<3, T, A>::new(x, y, z).to_array(), [x, y, z]);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).to_array(), [x, y, z, w]);
        });
    }

    #[test]
    fn test_as_array_ref() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(Vector::<2, T, A>::new(x, y).as_array_ref(), &[x, y]);
            assert_eq!(Vector::<3, T, A>::new(x, y, z).as_array_ref(), &[x, y, z]);
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).as_array_ref(),
                &[x, y, z, w]
            );
        });
    }

    #[test]
    fn test_as_array_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(Vector::<2, T, A>::new(x, y).as_array_mut(), &mut [x, y]);
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).as_array_mut(),
                &mut [x, y, z]
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).as_array_mut(),
                &mut [x, y, z, w]
            );
        });
    }

    #[test]
    fn test_iter() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).iter().collect_vec(),
                vec![x, y]
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).iter().collect_vec(),
                vec![x, y, z]
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).iter().collect_vec(),
                vec![x, y, z, w]
            );
        });
    }

    #[test]
    fn test_iter_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).iter_mut().collect_vec(),
                vec![&mut x, &mut y]
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).iter_mut().collect_vec(),
                vec![&mut x, &mut y, &mut z]
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).iter_mut().collect_vec(),
                vec![&mut x, &mut y, &mut z, &mut w]
            );
        });
    }

    #[test]
    fn test_map() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).map(T::as_to),
                Vector::<2, usize, A>::new(0, 1)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).map(T::as_to),
                Vector::<3, usize, A>::new(0, 1, 2)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).map(T::as_to),
                Vector::<4, usize, A>::new(0, 1, 2, 3)
            );
        });
    }

    #[test]
    fn test_reverse() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).reverse(),
                Vector::<2, T, A>::new(y, x)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).reverse(),
                Vector::<3, T, A>::new(z, y, x)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).reverse(),
                Vector::<4, T, A>::new(w, z, y, x)
            );
        });
    }

    #[test]
    fn test_element_sum() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(Vector::<2, T, A>::new(x, y).element_sum(), x + y);
            assert_float_eq!(Vector::<3, T, A>::new(x, y, z).element_sum(), x + y + z);
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).element_sum(),
                x + y + (z + w)
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(Vector::<2, T, A>::new(x, y).element_sum(), x + y);
            assert_panic_eq!(Vector::<3, T, A>::new(x, y, z).element_sum(), x + y + z);
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w).element_sum(),
                x + y + (z + w)
            );
        });
    }

    #[test]
    fn test_element_product() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(Vector::<2, T, A>::new(x, y).element_product(), x * y);
            assert_float_eq!(Vector::<3, T, A>::new(x, y, z).element_product(), x * y * z);
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).element_product(),
                x * y * (z * w)
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(Vector::<2, T, A>::new(x, y).element_product(), x * y);
            assert_panic_eq!(Vector::<3, T, A>::new(x, y, z).element_product(), x * y * z);
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w).element_product(),
                x * y * (z * w)
            );
        });
    }

    #[test]
    fn test_eq_mask() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Vector::<2, T, A>::new(x, y).eq_mask(Vector::<2, T, A>::new(z, w)),
                Mask::<2, T, A>::new(x == z, y == w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).eq_mask(Vector::<3, T, A>::new(w, x, y)),
                Mask::<3, T, A>::new(x == w, y == x, z == y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).eq_mask(Vector::<4, T, A>::new(y, z, w, x)),
                Mask::<4, T, A>::new(x == y, y == z, z == w, w == x)
            );
        });
    }

    #[test]
    fn test_ne_mask() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Vector::<2, T, A>::new(x, y).ne_mask(Vector::<2, T, A>::new(z, w)),
                Mask::<2, T, A>::new(x != z, y != w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).ne_mask(Vector::<3, T, A>::new(w, x, y)),
                Mask::<3, T, A>::new(x != w, y != x, z != y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).ne_mask(Vector::<4, T, A>::new(y, z, w, x)),
                Mask::<4, T, A>::new(x != y, y != z, z != w, w != x)
            );
        });
    }

    #[test]
    fn test_lt_mask() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Vector::<2, T, A>::new(x, y).lt_mask(Vector::<2, T, A>::new(z, w)),
                Mask::<2, T, A>::new(x < z, y < w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).lt_mask(Vector::<3, T, A>::new(w, x, y)),
                Mask::<3, T, A>::new(x < w, y < x, z < y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).lt_mask(Vector::<4, T, A>::new(y, z, w, x)),
                Mask::<4, T, A>::new(x < y, y < z, z < w, w < x)
            );
        });
    }

    #[test]
    fn test_gt_mask() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Vector::<2, T, A>::new(x, y).gt_mask(Vector::<2, T, A>::new(z, w)),
                Mask::<2, T, A>::new(x > z, y > w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).gt_mask(Vector::<3, T, A>::new(w, x, y)),
                Mask::<3, T, A>::new(x > w, y > x, z > y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).gt_mask(Vector::<4, T, A>::new(y, z, w, x)),
                Mask::<4, T, A>::new(x > y, y > z, z > w, w > x)
            );
        });
    }

    #[test]
    fn test_le_mask() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Vector::<2, T, A>::new(x, y).le_mask(Vector::<2, T, A>::new(z, w)),
                Mask::<2, T, A>::new(x <= z, y <= w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).le_mask(Vector::<3, T, A>::new(w, x, y)),
                Mask::<3, T, A>::new(x <= w, y <= x, z <= y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).le_mask(Vector::<4, T, A>::new(y, z, w, x)),
                Mask::<4, T, A>::new(x <= y, y <= z, z <= w, w <= x)
            );
        });
    }

    #[test]
    fn test_ge_mask() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Vector::<2, T, A>::new(x, y).ge_mask(Vector::<2, T, A>::new(z, w)),
                Mask::<2, T, A>::new(x >= z, y >= w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).ge_mask(Vector::<3, T, A>::new(w, x, y)),
                Mask::<3, T, A>::new(x >= w, y >= x, z >= y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).ge_mask(Vector::<4, T, A>::new(y, z, w, x)),
                Mask::<4, T, A>::new(x >= y, y >= z, z >= w, w >= x)
            );
        });
    }

    #[test]
    fn test_dot() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).dot(Vector::<2, T, A>::new(z, w)),
                x * z + y * w
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).dot(Vector::<3, T, A>::new(z, w, y)),
                x * z + y * w + z * y
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).dot(Vector::<4, T, A>::new(z, w, y, w)),
                x * z + y * w + (z * y + w * w)
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y).dot(Vector::<2, T, A>::new(z, w)),
                x * z + y * w
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z).dot(Vector::<3, T, A>::new(z, w, y)),
                x * z + y * w + z * y
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w).dot(Vector::<4, T, A>::new(z, w, y, x)),
                x * z + y * w + (z * y + w * x)
            );
        });
    }

    #[test]
    fn test_length_squared() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(Vector::<2, T, A>::new(x, y).length_squared(), x * x + y * y);
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).length_squared(),
                x * x + y * y + z * z
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).length_squared(),
                x * x + y * y + (z * z + w * w)
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(Vector::<2, T, A>::new(x, y).length_squared(), x * x + y * y);
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z).length_squared(),
                x * x + y * y + z * z
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w).length_squared(),
                x * x + y * y + z * z + w * w
            );
        });
    }

    #[test]
    fn test_distance_squared() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).distance_squared(Vector::<2, T, A>::new(z, w)),
                (x - z) * (x - z) + (y - w) * (y - w)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).distance_squared(Vector::<3, T, A>::new(z, w, y)),
                (x - z) * (x - z) + (y - w) * (y - w) + (z - y) * (z - y)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w)
                    .distance_squared(Vector::<4, T, A>::new(z, w, y, z)),
                (x - z) * (x - z) + (y - w) * (y - w) + ((z - y) * (z - y) + (w - z) * (w - z))
            );
        });
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y).distance_squared(Vector::<2, T, A>::new(z, w)),
                (x - z) * (x - z) + (y - w) * (y - w)
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z).distance_squared(Vector::<3, T, A>::new(z, w, y)),
                (x - z) * (x - z) + (y - w) * (y - w) + (z - y) * (z - y)
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w)
                    .distance_squared(Vector::<4, T, A>::new(z, w, y, z)),
                (x - z) * (x - z) + (y - w) * (y - w) + ((z - y) * (z - y) + (w - z) * (w - z))
            );
        });
    }

    #[test]
    fn test_from_inner() {
        assert_eq!(Vec2U::<u32>::from_inner(Repr2(0, 1)), Vec2U::new(0, 1));
        assert_eq!(
            Vec3U::<u32>::from_inner(Repr3(0, 1, 2)),
            Vec3U::new(0, 1, 2)
        );
        assert_eq!(
            Vec4U::<u32>::from_inner(Repr4(0, 1, 2, 3)),
            Vec4U::new(0, 1, 2, 3)
        );
    }

    #[test]
    fn test_inner() {
        assert_eq!(Vec2U::<u32>::new(0, 1).inner(), Repr2(0, 1));
        assert_eq!(Vec3U::<u32>::new(0, 1, 2).inner(), Repr3(0, 1, 2));
        assert_eq!(Vec4U::<u32>::new(0, 1, 2, 3).inner(), Repr4(0, 1, 2, 3));
    }

    #[test]
    fn test_inner_mut() {
        assert_eq!(Vec2U::<u32>::new(0, 1).inner_mut(), &mut Repr2(0, 1));
        assert_eq!(Vec3U::<u32>::new(0, 1, 2).inner_mut(), &mut Repr3(0, 1, 2));
        assert_eq!(
            Vec4U::<u32>::new(0, 1, 2, 3).inner_mut(),
            &mut Repr4(0, 1, 2, 3)
        );
    }

    #[test]
    fn test_axes() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Vector::<2, T, A>::X,
                Vector::<2, T, A>::new(T::as_from(1), T::as_from(0))
            );
            assert_eq!(
                Vector::<2, T, A>::Y,
                Vector::<2, T, A>::new(T::as_from(0), T::as_from(1))
            );

            assert_eq!(
                Vector::<3, T, A>::X,
                Vector::<3, T, A>::new(T::as_from(1), T::as_from(0), T::as_from(0))
            );
            assert_eq!(
                Vector::<3, T, A>::Y,
                Vector::<3, T, A>::new(T::as_from(0), T::as_from(1), T::as_from(0))
            );
            assert_eq!(
                Vector::<3, T, A>::Z,
                Vector::<3, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(1))
            );

            assert_eq!(
                Vector::<4, T, A>::X,
                Vector::<4, T, A>::new(T::as_from(1), T::as_from(0), T::as_from(0), T::as_from(0))
            );
            assert_eq!(
                Vector::<4, T, A>::Y,
                Vector::<4, T, A>::new(T::as_from(0), T::as_from(1), T::as_from(0), T::as_from(0))
            );
            assert_eq!(
                Vector::<4, T, A>::Z,
                Vector::<4, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(1), T::as_from(0))
            );
            assert_eq!(
                Vector::<4, T, A>::W,
                Vector::<4, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(0), T::as_from(1))
            );
        });
    }

    #[test]
    fn test_neg_axes() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(Vector::<2, T, A>::NEG_X, Vector::<2, T, A>::new(-1.0, 0.0));
            assert_eq!(Vector::<2, T, A>::NEG_Y, Vector::<2, T, A>::new(0.0, -1.0));

            assert_eq!(
                Vector::<3, T, A>::NEG_X,
                Vector::<3, T, A>::new(-1.0, 0.0, 0.0)
            );
            assert_eq!(
                Vector::<3, T, A>::NEG_Y,
                Vector::<3, T, A>::new(0.0, -1.0, 0.0)
            );
            assert_eq!(
                Vector::<3, T, A>::NEG_Z,
                Vector::<3, T, A>::new(0.0, 0.0, -1.0)
            );

            assert_eq!(
                Vector::<4, T, A>::NEG_X,
                Vector::<4, T, A>::new(-1.0, 0.0, 0.0, 0.0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_Y,
                Vector::<4, T, A>::new(0.0, -1.0, 0.0, 0.0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_Z,
                Vector::<4, T, A>::new(0.0, 0.0, -1.0, 0.0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_W,
                Vector::<4, T, A>::new(0.0, 0.0, 0.0, -1.0)
            );
        });
        for_parameters!(|T: PrimitiveSigned, A| {
            assert_eq!(Vector::<2, T, A>::NEG_X, Vector::<2, T, A>::new(-1, 0));
            assert_eq!(Vector::<2, T, A>::NEG_Y, Vector::<2, T, A>::new(0, -1));

            assert_eq!(Vector::<3, T, A>::NEG_X, Vector::<3, T, A>::new(-1, 0, 0));
            assert_eq!(Vector::<3, T, A>::NEG_Y, Vector::<3, T, A>::new(0, -1, 0));
            assert_eq!(Vector::<3, T, A>::NEG_Z, Vector::<3, T, A>::new(0, 0, -1));

            assert_eq!(
                Vector::<4, T, A>::NEG_X,
                Vector::<4, T, A>::new(-1, 0, 0, 0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_Y,
                Vector::<4, T, A>::new(0, -1, 0, 0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_Z,
                Vector::<4, T, A>::new(0, 0, -1, 0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_W,
                Vector::<4, T, A>::new(0, 0, 0, -1)
            );
        });
    }

    #[test]
    fn test_extend() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).extend(z),
                Vector::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).extend(w),
                Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_perp() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(Vector::<2, T, A>::X.perp(), Vector::<2, T, A>::Y);
            assert_eq!(Vector::<2, T, A>::Y.perp(), Vector::<2, T, A>::NEG_X);
            assert_eq!(Vector::<2, T, A>::NEG_X.perp(), Vector::<2, T, A>::NEG_Y);
            assert_eq!(Vector::<2, T, A>::NEG_Y.perp(), Vector::<2, T, A>::X);
        });
        for_parameters!(|T: PrimitiveSigned, A| {
            assert_eq!(Vector::<2, T, A>::X.perp(), Vector::<2, T, A>::Y);
            assert_eq!(Vector::<2, T, A>::Y.perp(), Vector::<2, T, A>::NEG_X);
            assert_eq!(Vector::<2, T, A>::NEG_X.perp(), Vector::<2, T, A>::NEG_Y);
            assert_eq!(Vector::<2, T, A>::NEG_Y.perp(), Vector::<2, T, A>::X);
        });
    }

    #[test]
    fn test_truncate() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).truncate(),
                Vector::<2, T, A>::new(x, y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).truncate(),
                Vector::<3, T, A>::new(x, y, z)
            );
        });
    }

    #[test]
    fn test_to_homogeneous() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).to_homogeneous(),
                Vector::<4, T, A>::new(x, y, z, T::ONE)
            );
        });
    }

    #[test]
    fn test_cross() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Vector::<3, T, A>::X.cross(Vector::<3, T, A>::Y),
                Vector::<3, T, A>::Z
            );
            assert_eq!(
                Vector::<3, T, A>::Y.cross(Vector::<3, T, A>::Z),
                Vector::<3, T, A>::X
            );
            assert_eq!(
                Vector::<3, T, A>::Z.cross(Vector::<3, T, A>::X),
                Vector::<3, T, A>::Y
            );

            for a in [
                Vector::<3, T, A>::X,
                Vector::<3, T, A>::Y,
                Vector::<3, T, A>::Z,
            ] {
                assert_eq!(a.cross(a), Vector::ZERO);

                for b in [
                    Vector::<3, T, A>::X,
                    Vector::<3, T, A>::Y,
                    Vector::<3, T, A>::Z,
                ] {
                    assert_eq!(b.cross(a), -a.cross(b));
                    assert_eq!((-a).cross(b), -a.cross(b));
                    assert_eq!(a.cross(-b), -a.cross(b));
                }
            }
        });
        for_parameters!(|T: PrimitiveSigned, A| {
            assert_eq!(
                Vector::<3, T, A>::X.cross(Vector::<3, T, A>::Y),
                Vector::<3, T, A>::Z
            );
            assert_eq!(
                Vector::<3, T, A>::Y.cross(Vector::<3, T, A>::Z),
                Vector::<3, T, A>::X
            );
            assert_eq!(
                Vector::<3, T, A>::Z.cross(Vector::<3, T, A>::X),
                Vector::<3, T, A>::Y
            );

            for a in [
                Vector::<3, T, A>::X,
                Vector::<3, T, A>::Y,
                Vector::<3, T, A>::Z,
            ] {
                assert_eq!(a.cross(a), Vector::ZERO);

                for b in [
                    Vector::<3, T, A>::X,
                    Vector::<3, T, A>::Y,
                    Vector::<3, T, A>::Z,
                ] {
                    assert_eq!(b.cross(a), -a.cross(b));
                    assert_eq!((-a).cross(b), -a.cross(b));
                    assert_eq!(a.cross(-b), -a.cross(b));
                }
            }
        });
    }

    #[test]
    fn test_index() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(Vector::<2, T, A>::new(x, y)[0], x);
            assert_eq!(Vector::<2, T, A>::new(x, y)[1], y);
            assert_panic!(Vector::<2, T, A>::new(x, y)[2]);

            assert_eq!(Vector::<3, T, A>::new(x, y, z)[0], x);
            assert_eq!(Vector::<3, T, A>::new(x, y, z)[1], y);
            assert_eq!(Vector::<3, T, A>::new(x, y, z)[2], z);
            assert_panic!(Vector::<3, T, A>::new(x, y, z)[3]);

            assert_eq!(Vector::<4, T, A>::new(x, y, z, w)[0], x);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w)[1], y);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w)[2], z);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w)[3], w);
            assert_panic!(Vector::<4, T, A>::new(x, y, z, w)[4]);
        });
    }

    #[test]
    fn test_index_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(T::as_from);

            assert_eq!(&mut Vector::<2, T, A>::new(x, y)[0], &mut x);
            assert_eq!(&mut Vector::<2, T, A>::new(x, y)[1], &mut y);
            assert_panic!(&mut Vector::<2, T, A>::new(x, y)[2]);

            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z)[0], &mut x);
            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z)[1], &mut y);
            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z)[2], &mut z);
            assert_panic!(&mut Vector::<3, T, A>::new(x, y, z)[3]);

            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w)[0], &mut x);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w)[1], &mut y);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w)[2], &mut z);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w)[3], &mut w);
            assert_panic!(&mut Vector::<4, T, A>::new(x, y, z, w)[4]);
        });
    }

    #[test]
    fn test_into_iter() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).into_iter().collect_vec(),
                vec![x, y]
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).into_iter().collect_vec(),
                vec![x, y, z]
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).into_iter().collect_vec(),
                vec![x, y, z, w]
            );
        });
    }

    #[test]
    fn test_mut_into_iter() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(T::as_from);

            assert_eq!(
                (&mut Vector::<2, T, A>::new(x, y))
                    .into_iter()
                    .collect_vec(),
                vec![&mut x, &mut y]
            );
            assert_eq!(
                (&mut Vector::<3, T, A>::new(x, y, z))
                    .into_iter()
                    .collect_vec(),
                vec![&mut x, &mut y, &mut z]
            );
            assert_eq!(
                (&mut Vector::<4, T, A>::new(x, y, z, w))
                    .into_iter()
                    .collect_vec(),
                vec![&mut x, &mut y, &mut z, &mut w]
            );
        });
    }

    #[test]
    fn test_deref() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(Vector::<2, T, A>::new(x, y).x, x);
            assert_eq!(Vector::<2, T, A>::new(x, y).y, y);

            assert_eq!(Vector::<3, T, A>::new(x, y, z).x, x);
            assert_eq!(Vector::<3, T, A>::new(x, y, z).y, y);
            assert_eq!(Vector::<3, T, A>::new(x, y, z).z, z);

            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).x, x);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).y, y);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).z, z);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).w, w);
        });
    }

    #[test]
    fn test_deref_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(T::as_from);

            assert_eq!(&mut Vector::<2, T, A>::new(x, y).x, &mut x);
            assert_eq!(&mut Vector::<2, T, A>::new(x, y).y, &mut y);

            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z).x, &mut x);
            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z).y, &mut y);
            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z).z, &mut z);

            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w).x, &mut x);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w).y, &mut y);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w).z, &mut z);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w).w, &mut w);
        });
    }

    #[test]
    fn test_from_tuples() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::from((x, y)),
                Vector::<2, T, A>::new(x, y)
            );

            assert_eq!(
                Vector::<3, T, A>::from((x, y, z)),
                Vector::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Vector::<3, T, A>::from((x, Vector::<2, T, A>::new(y, z))),
                Vector::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Vector::<3, T, A>::from((Vector::<2, T, A>::new(x, y), z)),
                Vector::<3, T, A>::new(x, y, z)
            );

            assert_eq!(
                Vector::<4, T, A>::from((x, y, z, w)),
                Vector::<4, T, A>::new(x, y, z, w)
            );
            assert_eq!(
                Vector::<4, T, A>::from((x, y, Vector::<2, T, A>::new(z, w))),
                Vector::<4, T, A>::new(x, y, z, w)
            );
            assert_eq!(
                Vector::<4, T, A>::from((x, Vector::<2, T, A>::new(y, z), w)),
                Vector::<4, T, A>::new(x, y, z, w)
            );
            assert_eq!(
                Vector::<4, T, A>::from((x, Vector::<3, T, A>::new(y, z, w))),
                Vector::<4, T, A>::new(x, y, z, w)
            );
            assert_eq!(
                Vector::<4, T, A>::from((Vector::<2, T, A>::new(x, y), z, w)),
                Vector::<4, T, A>::new(x, y, z, w)
            );
            assert_eq!(
                Vector::<4, T, A>::from((
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                )),
                Vector::<4, T, A>::new(x, y, z, w)
            );
            assert_eq!(
                Vector::<4, T, A>::from((Vector::<3, T, A>::new(x, y, z), w)),
                Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_debug() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!("{:?}", Vector::<2, T, A>::new(x, y)),
                format!("({x:?}, {y:?})")
            );
            assert_eq!(
                format!("{:?}", Vector::<3, T, A>::new(x, y, z)),
                format!("({x:?}, {y:?}, {z:?})")
            );
            assert_eq!(
                format!("{:?}", Vector::<4, T, A>::new(x, y, z, w)),
                format!("({x:?}, {y:?}, {z:?}, {w:?})")
            );
        });
    }

    #[test]
    fn test_display() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!("{}", Vector::<2, T, A>::new(x, y)),
                format!("({x}, {y})")
            );
            assert_eq!(
                format!("{}", Vector::<3, T, A>::new(x, y, z)),
                format!("({x}, {y}, {z})")
            );
            assert_eq!(
                format!("{}", Vector::<4, T, A>::new(x, y, z, w)),
                format!("({x}, {y}, {z}, {w})")
            );
        });
    }

    #[test]
    fn test_eq() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Vector::<2, T, A>::new(x, y) == Vector::<2, T, A>::new(z, w),
                x == z && y == w
            );

            assert_eq!(
                Vector::<3, T, A>::new(x, y, z) == Vector::<3, T, A>::new(x, y, w),
                x == x && y == y && z == w
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z) == Vector::<3, T, A>::new(z, w, y),
                x == z && y == w && z == y
            );

            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w) == Vector::<4, T, A>::new(w, y, z, w),
                x == w && y == y && z == z && w == w
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w) == Vector::<4, T, A>::new(z, w, y, x),
                x == z && y == w && z == y && w == x
            );
        });
    }

    #[test]
    fn test_ne() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Vector::<2, T, A>::new(x, y) != Vector::<2, T, A>::new(z, w),
                x != z || y != w
            );

            assert_eq!(
                Vector::<3, T, A>::new(x, y, z) != Vector::<3, T, A>::new(x, y, w),
                x != x || y != y || z != w
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z) != Vector::<3, T, A>::new(z, w, y),
                x != z || y != w || z != y
            );

            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w) != Vector::<4, T, A>::new(w, y, z, w),
                x != w || y != y || z != z || w != w
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w) != Vector::<4, T, A>::new(z, w, y, x),
                x != z || y != w || z != y || w != x
            );
        });
    }

    #[test]
    fn test_default() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(Vector::<2, T, A>::default(), Vector::splat(T::default()));
            assert_eq!(Vector::<3, T, A>::default(), Vector::splat(T::default()));
            assert_eq!(Vector::<4, T, A>::default(), Vector::splat(T::default()));
        });
    }

    #[test]
    fn test_neg() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                -Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(-x, -y)
            );
            assert_float_eq!(
                -Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(-x, -y, -z)
            );
            assert_float_eq!(
                -Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(-x, -y, -z, -w)
            );
        });
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                -Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(-x, -y)
            );
            assert_panic_eq!(
                -Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(-x, -y, -z)
            );
            assert_panic_eq!(
                -Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(-x, -y, -z, -w)
            );
        });
    }

    #[test]
    fn test_not() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                !Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(!x, !y)
            );
            assert_eq!(
                !Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(!x, !y, !z)
            );
            assert_eq!(
                !Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(!x, !y, !z, !w)
            );
        });
    }

    #[test]
    fn test_add() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y) + Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x + z, y + w)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z) + Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x + z, y + w, z + y)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w) + Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x + z, y + w, z + y, w + x)
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y) + Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x + z, y + w)
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z) + Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x + z, y + w, z + y)
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w) + Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x + z, y + w, z + y, w + x)
            );
        });
    }

    #[test]
    fn test_sub() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y) - Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x - z, y - w)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z) - Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x - z, y - w, z - y)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w) - Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x - z, y - w, z - y, w - x)
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y) - Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x - z, y - w)
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z) - Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x - z, y - w, z - y)
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w) - Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x - z, y - w, z - y, w - x)
            );
        });
    }

    #[test]
    fn test_mul() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y) * Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x * z, y * w)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z) * Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x * z, y * w, z * y)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w) * Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x * z, y * w, z * y, w * x)
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y) * Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x * z, y * w)
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z) * Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x * z, y * w, z * y)
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w) * Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x * z, y * w, z * y, w * x)
            );
        });
    }

    #[test]
    fn test_div() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y) / Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x / z, y / w)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z) / Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x / z, y / w, z / y)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w) / Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x / z, y / w, z / y, w / x)
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y) / Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x / z, y / w)
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z) / Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x / z, y / w, z / y)
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w) / Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x / z, y / w, z / y, w / x)
            );
        });
    }

    #[test]
    fn test_rem() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if T::is_infinite(x * 2.0) || T::is_infinite(y * 2.0) || T::is_infinite(z * 2.0) {
                return;
            }

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y) % Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x % z, y % w),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 0.00001
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z) % Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x % z, y % w, z % y),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 0.00001
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w) % Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x % z, y % w, z % y, w % x),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 0.00001
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y) % Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x % z, y % w)
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z) % Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x % z, y % w, z % y)
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w) % Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x % z, y % w, z % y, w % x)
            );
        });
    }

    #[test]
    fn test_shl() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y) << Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x << z, y << w)
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z) << Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x << z, y << w, z << y)
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w) << Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x << z, y << w, z << y, w << x)
            );
        });
    }

    #[test]
    fn test_shr() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y) >> Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x >> z, y >> w)
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z) >> Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x >> z, y >> w, z >> y)
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w) >> Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x >> z, y >> w, z >> y, w >> x)
            );
        });
    }

    #[test]
    fn test_bitand() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y) & Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x & z, y & w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z) & Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x & z, y & w, z & y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w) & Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x & z, y & w, z & y, w & x)
            );
        });
    }

    #[test]
    fn test_bitor() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y) | Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x | z, y | w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z) | Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x | z, y | w, z | y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w) | Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x | z, y | w, z | y, w | x)
            );
        });
    }

    #[test]
    fn test_bitxor() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y) ^ Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x ^ z, y ^ w)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z) ^ Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x ^ z, y ^ w, z ^ y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w) ^ Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x ^ z, y ^ w, z ^ y, w ^ x)
            );
        });
    }

    #[test]
    fn test_add_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut vector = Vector::<2, T, A>::new(x, y);
            vector += Vector::<2, T, A>::new(z, w);
            assert_float_eq!(vector, Vector::<2, T, A>::new(x + z, y + w));

            let mut vector = Vector::<3, T, A>::new(x, y, z);
            vector += Vector::<3, T, A>::new(z, w, y);
            assert_float_eq!(vector, Vector::<3, T, A>::new(x + z, y + w, z + y));

            let mut vector = Vector::<4, T, A>::new(x, y, z, w);
            vector += Vector::<4, T, A>::new(z, w, y, x);
            assert_float_eq!(vector, Vector::<4, T, A>::new(x + z, y + w, z + y, w + x));
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                {
                    let mut vector = Vector::<2, T, A>::new(x, y);
                    vector += Vector::<2, T, A>::new(z, w);
                    vector
                },
                Vector::<2, T, A>::new(x + z, y + w)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<3, T, A>::new(x, y, z);
                    vector += Vector::<3, T, A>::new(z, w, y);
                    vector
                },
                Vector::<3, T, A>::new(x + z, y + w, z + y)
            );

            assert_panic_eq!(
                {
                    let mut vector = Vector::<4, T, A>::new(x, y, z, w);
                    vector += Vector::<4, T, A>::new(z, w, y, x);
                    vector
                },
                Vector::<4, T, A>::new(x + z, y + w, z + y, w + x)
            );
        });
    }

    #[test]
    fn test_sub_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut vector = Vector::<2, T, A>::new(x, y);
            vector -= Vector::<2, T, A>::new(z, w);
            assert_float_eq!(vector, Vector::<2, T, A>::new(x - z, y - w));

            let mut vector = Vector::<3, T, A>::new(x, y, z);
            vector -= Vector::<3, T, A>::new(z, w, y);
            assert_float_eq!(vector, Vector::<3, T, A>::new(x - z, y - w, z - y));

            let mut vector = Vector::<4, T, A>::new(x, y, z, w);
            vector -= Vector::<4, T, A>::new(z, w, y, x);
            assert_float_eq!(vector, Vector::<4, T, A>::new(x - z, y - w, z - y, w - x));
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                {
                    let mut vector = Vector::<2, T, A>::new(x, y);
                    vector -= Vector::<2, T, A>::new(z, w);
                    vector
                },
                Vector::<2, T, A>::new(x - z, y - w)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<3, T, A>::new(x, y, z);
                    vector -= Vector::<3, T, A>::new(z, w, y);
                    vector
                },
                Vector::<3, T, A>::new(x - z, y - w, z - y)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<4, T, A>::new(x, y, z, w);
                    vector -= Vector::<4, T, A>::new(z, w, y, x);
                    vector
                },
                Vector::<4, T, A>::new(x - z, y - w, z - y, w - x)
            );
        });
    }

    #[test]
    fn test_mul_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut vector = Vector::<2, T, A>::new(x, y);
            vector *= Vector::<2, T, A>::new(z, w);
            assert_float_eq!(vector, Vector::<2, T, A>::new(x * z, y * w));

            let mut vector = Vector::<3, T, A>::new(x, y, z);
            vector *= Vector::<3, T, A>::new(z, w, y);
            assert_float_eq!(vector, Vector::<3, T, A>::new(x * z, y * w, z * y));

            let mut vector = Vector::<4, T, A>::new(x, y, z, w);
            vector *= Vector::<4, T, A>::new(z, w, y, x);
            assert_float_eq!(vector, Vector::<4, T, A>::new(x * z, y * w, z * y, w * x));
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                {
                    let mut vector = Vector::<2, T, A>::new(x, y);
                    vector *= Vector::<2, T, A>::new(z, w);
                    vector
                },
                Vector::<2, T, A>::new(x * z, y * w)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<3, T, A>::new(x, y, z);
                    vector *= Vector::<3, T, A>::new(z, w, y);
                    vector
                },
                Vector::<3, T, A>::new(x * z, y * w, z * y)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<4, T, A>::new(x, y, z, w);
                    vector *= Vector::<4, T, A>::new(z, w, y, x);
                    vector
                },
                Vector::<4, T, A>::new(x * z, y * w, z * y, w * x)
            );
        });
    }

    #[test]
    fn test_div_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut vector = Vector::<2, T, A>::new(x, y);
            vector /= Vector::<2, T, A>::new(z, w);
            assert_float_eq!(vector, Vector::<2, T, A>::new(x / z, y / w));

            let mut vector = Vector::<3, T, A>::new(x, y, z);
            vector /= Vector::<3, T, A>::new(z, w, y);
            assert_float_eq!(vector, Vector::<3, T, A>::new(x / z, y / w, z / y));

            let mut vector = Vector::<4, T, A>::new(x, y, z, w);
            vector /= Vector::<4, T, A>::new(z, w, y, x);
            assert_float_eq!(vector, Vector::<4, T, A>::new(x / z, y / w, z / y, w / x));
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                {
                    let mut vector = Vector::<2, T, A>::new(x, y);
                    vector /= Vector::<2, T, A>::new(z, w);
                    vector
                },
                Vector::<2, T, A>::new(x / z, y / w)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<3, T, A>::new(x, y, z);
                    vector /= Vector::<3, T, A>::new(z, w, y);
                    vector
                },
                Vector::<3, T, A>::new(x / z, y / w, z / y)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<4, T, A>::new(x, y, z, w);
                    vector /= Vector::<4, T, A>::new(z, w, y, x);
                    vector
                },
                Vector::<4, T, A>::new(x / z, y / w, z / y, w / x)
            );
        });
    }

    #[test]
    fn test_rem_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if T::is_infinite(x * 2.0) || T::is_infinite(y * 2.0) || T::is_infinite(z * 2.0) {
                return;
            }

            let mut vector = Vector::<2, T, A>::new(x, y);
            vector %= Vector::<2, T, A>::new(z, w);
            assert_float_eq!(
                vector,
                Vector::<2, T, A>::new(x % z, y % w),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 0.00001
            );

            let mut vector = Vector::<3, T, A>::new(x, y, z);
            vector %= Vector::<3, T, A>::new(z, w, y);
            assert_float_eq!(
                vector,
                Vector::<3, T, A>::new(x % z, y % w, z % y),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 0.00001
            );

            let mut vector = Vector::<4, T, A>::new(x, y, z, w);
            vector %= Vector::<4, T, A>::new(z, w, y, x);
            assert_float_eq!(
                vector,
                Vector::<4, T, A>::new(x % z, y % w, z % y, w % x),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 0.00001
            );
        });
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                {
                    let mut vector = Vector::<2, T, A>::new(x, y);
                    vector %= Vector::<2, T, A>::new(z, w);
                    vector
                },
                Vector::<2, T, A>::new(x % z, y % w)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<3, T, A>::new(x, y, z);
                    vector %= Vector::<3, T, A>::new(z, w, y);
                    vector
                },
                Vector::<3, T, A>::new(x % z, y % w, z % y)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<4, T, A>::new(x, y, z, w);
                    vector %= Vector::<4, T, A>::new(z, w, y, x);
                    vector
                },
                Vector::<4, T, A>::new(x % z, y % w, z % y, w % x)
            );
        });
    }

    #[test]
    fn test_shl_assign() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                {
                    let mut vector = Vector::<2, T, A>::new(x, y);
                    vector <<= Vector::<2, T, A>::new(z, w);
                    vector
                },
                Vector::<2, T, A>::new(x << z, y << w)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<3, T, A>::new(x, y, z);
                    vector <<= Vector::<3, T, A>::new(z, w, y);
                    vector
                },
                Vector::<3, T, A>::new(x << z, y << w, z << y)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<4, T, A>::new(x, y, z, w);
                    vector <<= Vector::<4, T, A>::new(z, w, y, x);
                    vector
                },
                Vector::<4, T, A>::new(x << z, y << w, z << y, w << x)
            );
        });
    }

    #[test]
    fn test_shr_assign() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                {
                    let mut vector = Vector::<2, T, A>::new(x, y);
                    vector >>= Vector::<2, T, A>::new(z, w);
                    vector
                },
                Vector::<2, T, A>::new(x >> z, y >> w)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<3, T, A>::new(x, y, z);
                    vector >>= Vector::<3, T, A>::new(z, w, y);
                    vector
                },
                Vector::<3, T, A>::new(x >> z, y >> w, z >> y)
            );
            assert_panic_eq!(
                {
                    let mut vector = Vector::<4, T, A>::new(x, y, z, w);
                    vector >>= Vector::<4, T, A>::new(z, w, y, x);
                    vector
                },
                Vector::<4, T, A>::new(x >> z, y >> w, z >> y, w >> x)
            );
        });
    }

    #[test]
    fn test_bitand_assign() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            let mut vector = Vector::<2, T, A>::new(x, y);
            vector &= Vector::<2, T, A>::new(z, w);
            assert_eq!(vector, Vector::<2, T, A>::new(x & z, y & w));

            let mut vector = Vector::<3, T, A>::new(x, y, z);
            vector &= Vector::<3, T, A>::new(z, w, y);
            assert_eq!(vector, Vector::<3, T, A>::new(x & z, y & w, z & y));

            let mut vector = Vector::<4, T, A>::new(x, y, z, w);
            vector &= Vector::<4, T, A>::new(z, w, y, x);
            assert_eq!(vector, Vector::<4, T, A>::new(x & z, y & w, z & y, w & x));
        });
    }

    #[test]
    fn test_bitor_assign() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            let mut vector = Vector::<2, T, A>::new(x, y);
            vector |= Vector::<2, T, A>::new(z, w);
            assert_eq!(vector, Vector::<2, T, A>::new(x | z, y | w));

            let mut vector = Vector::<3, T, A>::new(x, y, z);
            vector |= Vector::<3, T, A>::new(z, w, y);
            assert_eq!(vector, Vector::<3, T, A>::new(x | z, y | w, z | y));

            let mut vector = Vector::<4, T, A>::new(x, y, z, w);
            vector |= Vector::<4, T, A>::new(z, w, y, x);
            assert_eq!(vector, Vector::<4, T, A>::new(x | z, y | w, z | y, w | x));
        });
    }

    #[test]
    fn test_bitxor_assign() {
        for_parameters!(|T: PrimitiveInteger, A, x, y, z| {
            let w = T::max(x, y);

            let mut vector = Vector::<2, T, A>::new(x, y);
            vector ^= Vector::<2, T, A>::new(z, w);
            assert_eq!(vector, Vector::<2, T, A>::new(x ^ z, y ^ w));

            let mut vector = Vector::<3, T, A>::new(x, y, z);
            vector ^= Vector::<3, T, A>::new(z, w, y);
            assert_eq!(vector, Vector::<3, T, A>::new(x ^ z, y ^ w, z ^ y));

            let mut vector = Vector::<4, T, A>::new(x, y, z, w);
            vector ^= Vector::<4, T, A>::new(z, w, y, x);
            assert_eq!(vector, Vector::<4, T, A>::new(x ^ z, y ^ w, z ^ y, w ^ x));
        });
    }
}
