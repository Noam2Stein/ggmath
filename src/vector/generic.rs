use core::ops::{Add, Mul, Neg, Sub};

use crate::{
    Aligned, Alignment, Length, Mask, NegOne, One, Scalar, SupportedLength, Unaligned, Vector,
    Zero,
    backend::VectorBackend,
    utils::{Repr2, Repr3, Repr4, specialize, transmute_generic, transmute_mut, transmute_ref},
};

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
    /// # use ggmath::{Aligned, Unaligned, Vec3, Vec3A};
    /// #
    /// let unaligned = Vec3::new(1, 2, 3);
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Vec3A::new(1, 2, 3));
    ///
    /// let aligned = Vec3A::new(1, 2, 3);
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Vec3::new(1, 2, 3));
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

            3 => {
                if const { size_of::<Vector<N, T, A2>>() > size_of::<Vector<N, T, A>>() } {
                    // SAFETY: Because `N == 3`, `Vector<N, T, A2>` and
                    // `Vector<3, T, A2>` are the same type.
                    unsafe {
                        transmute_generic::<Vector<3, T, A2>, Vector<N, T, A2>>(
                            Vector::<3, T, A2>::new(
                                self.as_array()[0],
                                self.as_array()[1],
                                self.as_array()[2],
                            ),
                        )
                    }
                } else {
                    // SAFETY: The output type contains `[T; 3]` then `Pod`
                    // padding. The input type also begins with exactly this.
                    unsafe { *transmute_ref::<Vector<N, T, A>, Vector<N, T, A2>>(&self) }
                }
            }

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
    /// # use ggmath::{Vec3, Vec3A};
    /// #
    /// let unaligned = Vec3::new(1, 2, 3);
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Vec3A::new(1, 2, 3));
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
    /// # use ggmath::{Vec3, Vec3A};
    /// #
    /// let aligned = Vec3A::new(1, 2, 3);
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Vec3::new(1, 2, 3));
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
        *self.as_array()
    }

    /// Returns a reference to the vector's elements.
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[T; N] {
        // SAFETY: `Vector<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `T`.
        unsafe { transmute_ref::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns a mutable reference to the vector's elements.
    #[inline]
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [T; N] {
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
        self.as_mut_array().iter_mut()
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
        specialize!(Vector::<N, T, A>::reverse_backend(self))
    }

    /// Computes the sum of the elements of `self`.
    ///
    /// Equivalent to `self.x + self.y + ...`.
    ///
    /// # Panics
    ///
    /// When debug assertions or overflow checks are enabled:
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
        specialize!(<T as VectorBackend<N, A>>::vector_element_sum(self))
    }

    /// Computes the product of the elements of `self`.
    ///
    /// Equivalent to `self.x * self.y * ...`.
    ///
    /// # Panics
    ///
    /// When debug assertions or overflow checks are enabled:
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
        specialize!(<T as VectorBackend<N, A>>::vector_element_product(self))
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
        specialize!(<T as VectorBackend<N, A>>::vector_eq_mask(self, other))
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
        specialize!(<T as VectorBackend<N, A>>::vector_ne_mask(self, other))
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
        specialize!(<T as VectorBackend<N, A>>::vector_lt_mask(self, other))
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
        specialize!(<T as VectorBackend<N, A>>::vector_gt_mask(self, other))
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
        specialize!(<T as VectorBackend<N, A>>::vector_le_mask(self, other))
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
        specialize!(<T as VectorBackend<N, A>>::vector_ge_mask(self, other))
    }

    /// Computes the dot product of `self` and `rhs`.
    ///
    /// # Panics
    ///
    /// When debug assertions or overflow checks are enabled:
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
    /// When debug assertions or overflow checks are enabled:
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
    /// When debug assertions or overflow checks are enabled:
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

    #[inline]
    #[must_use]
    #[allow(
        dead_code,
        reason = "this will likely be used for fixed-point numbers (TODO)"
    )]
    pub(crate) const fn from_inner(inner: <T as VectorBackend<N, A>>::Inner) -> Self
    where
        T: VectorBackend<N, A>,
    {
        // SAFETY: These always correspond to the same type.
        Self(unsafe {
            transmute_generic::<
                <T as VectorBackend<N, A>>::Inner,
                <A as Alignment>::Select<
                    <Length<N> as SupportedLength>::Select<
                        <T as VectorBackend<2, Aligned>>::Inner,
                        <T as VectorBackend<3, Aligned>>::Inner,
                        <T as VectorBackend<4, Aligned>>::Inner,
                    >,
                    <Length<N> as SupportedLength>::Select<
                        <T as VectorBackend<2, Unaligned>>::Inner,
                        <T as VectorBackend<3, Unaligned>>::Inner,
                        <T as VectorBackend<4, Unaligned>>::Inner,
                    >,
                >,
            >(inner)
        })
    }

    #[inline]
    #[must_use]
    #[allow(
        dead_code,
        reason = "this will likely be used for fixed-point numbers (TODO)"
    )]
    pub(crate) const fn inner(self) -> <T as VectorBackend<N, A>>::Inner
    where
        T: VectorBackend<N, A>,
    {
        // SAFETY: `Vector<N, T, A>` is a transparent wrapper over
        // `<T as VectorBackend<N, A>>::Inner`.
        unsafe { transmute_generic::<Vector<N, T, A>, <T as VectorBackend<N, A>>::Inner>(self) }
    }

    #[inline]
    #[must_use]
    #[allow(
        dead_code,
        reason = "this will likely be used for fixed-point numbers (TODO)"
    )]
    pub(crate) const fn inner_mut(&mut self) -> &mut <T as VectorBackend<N, A>>::Inner
    where
        T: VectorBackend<N, A>,
    {
        // SAFETY: `Vector<N, T, A>` is a transparent wrapper over
        // `<T as VectorBackend<N, A>>::Inner`.
        unsafe { transmute_mut::<Vector<N, T, A>, <T as VectorBackend<N, A>>::Inner>(self) }
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

    /// Converts `self` to homogeneous coordinates.
    ///
    /// Equivalent to `self.extend(1)`.
    #[inline]
    #[must_use]
    pub fn to_homogeneous(self) -> Vector<3, T, A>
    where
        T: One,
    {
        self.extend(T::ONE)
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

    /// Computes the wedge product of `self` and `rhs`.
    ///
    /// Also reffered to as the 2D cross product, the determinant and the
    /// signed area.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec2;
    /// #
    /// let x = Vec2::new(1, 0);
    /// let y = Vec2::new(0, 1);
    ///
    /// assert_eq!(x.wedge(y), 1);
    /// assert_eq!(y.wedge(x), -1);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn wedge(self, rhs: Self) -> T
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        self.x * rhs.y - self.y * rhs.x
    }

    #[inline(always)]
    fn reverse_backend(self) -> Self {
        self.yx()
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

    #[inline(always)]
    fn reverse_backend(self) -> Self {
        self.zyx()
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

    #[inline(always)]
    fn reverse_backend(self) -> Self {
        self.wzyx()
    }
}

// Tests are located at `src/vector.rs`. This module's contents are separated
// into this `generic` module as a workaround for a rustdoc bug, so no reason to
// also move the tests.
