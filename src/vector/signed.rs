use crate::{Alignment, Length, Scalar, SupportedLength, Vector, specialize::specialize};

macro_rules! impl_int {
    ($T:ident, $Backend:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            /// Computes the sum of the elements of `self`.
            ///
            /// Equivalent to `self.x + self.y + ...`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any addition overflows (performed in order).
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vec = Vec3::<i32>::new(1, 2, 3);
            /// assert_eq!(vec.element_sum(), 1 + 2 + 3);
            /// ```
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn element_sum(self) -> $T {
                if cfg!(assertions) {
                    specialize!(<$T as $Backend<N, A>>::vec_strict_element_sum(self))
                } else {
                    specialize!(<$T as $Backend<N, A>>::vec_wrapping_element_sum(self))
                }
            }

            /// Computes the product of the elements of `self`.
            ///
            /// Equivalent to `self.x * self.y * ...`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any multiplication overflows (performed in order).
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vec = Vec3::<i32>::new(1, 2, 3);
            /// assert_eq!(vec.element_product(), 1 * 2 * 3);
            /// ```
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn element_product(self) -> $T {
                if cfg!(assertions) {
                    specialize!(<$T as $Backend<N, A>>::vec_strict_element_product(self))
                } else {
                    specialize!(<$T as $Backend<N, A>>::vec_wrapping_element_product(self))
                }
            }

            /// Returns the maximum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec4;
            /// #
            /// let a = Vec4::<i32>::new(1, 5, 3, 0);
            /// let b = Vec4::<i32>::new(3, 2, 7, -1);
            /// let max = a.max(b);
            ///
            /// assert_eq!(max, Vec4::new(3, 5, 7, 0));
            /// ```
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_max(self, other))
            }

            /// Returns the minimum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec4;
            /// #
            /// let a = Vec4::<i32>::new(1, 5, 3, 0);
            /// let b = Vec4::<i32>::new(3, 2, 7, -1);
            /// let min = a.min(b);
            ///
            /// assert_eq!(min, Vec4::new(1, 2, 3, -1));
            /// ```
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_min(self, other))
            }

            /// Clamps the elements of `self` between the elements of `min` and
            /// `max`.
            ///
            /// Equivalent to
            /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
            ///
            /// # Panics
            ///
            /// Panics if any element of `min` is greater than the corresponding
            /// element of `max`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec4;
            /// #
            /// let vec = Vec4::<i32>::new(1, 2, 3, 0);
            /// let min = Vec4::new(0, 5, 1, -2);
            /// let max = Vec4::new(3, 6, 2, -1);
            /// let clamp = vec.clamp(min, max);
            ///
            /// assert_eq!(clamp, Vec4::new(1, 5, 2, -1));
            /// ```
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                assert!((0..N).all(|i| min[i] <= max[i]), "min <= max");

                self.max(min).min(max)
            }

            /// Returns the maximum between the elements of `self`.
            ///
            /// Equivalent to `self.x.max(self.y).max(self.z)...`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vec = Vec3::<i32>::new(-1, 7, 3);
            /// assert_eq!(vec.max_element(), 7);
            /// ```
            #[inline]
            #[must_use]
            pub fn max_element(self) -> $T {
                specialize!(<$T as $Backend<N, A>>::vec_max_element(self))
            }

            /// Returns the minimum between the elements of `self`.
            ///
            /// Equivalent to `self.x.min(self.y).min(self.z)...`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vec = Vec3::<i32>::new(7, -1, 3);
            /// assert_eq!(vec.min_element(), -1);
            /// ```
            #[inline]
            #[must_use]
            pub fn min_element(self) -> $T {
                specialize!(<$T as $Backend<N, A>>::vec_min_element(self))
            }

            /// Returns the absolute values of the elements of `self`.
            ///
            /// Equivalent to `(self.x.abs(), self.y.abs(), ...)`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation) or
            /// overflow checks are enabled:
            ///
            /// Panics if any component is [`T::MIN`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vec = Vec3::<i32>::new(7, -1, -3);
            /// assert_eq!(vec.abs(), Vec3::new(7, 1, 3));
            /// ```
            ///
            /// [`T::MIN`]: crate::constants::Min::MIN
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn abs(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_abs(self))
            }

            /// Returns the signum of the elements of `self`.
            ///
            /// Equivalent to `(self.x.signum(), self.y.signum(), ...)`.
            ///
            /// For each element:
            ///
            /// - `0` if the element is zero
            /// - `1` if the element is positive
            /// - `-1` if the element is negative
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec4;
            /// #
            /// let vec = Vec4::<i32>::new(7, -1, -3, 0);
            /// assert_eq!(vec.signum(), Vec4::new(1, -1, -1, 0));
            /// ```
            #[inline]
            #[must_use]
            pub fn signum(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_signum(self))
            }

            /// Computes the dot product of `self` and `rhs`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation) or
            /// overflow checks are enabled:
            ///
            /// Panics if an overflow occurs.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let x = Vec3::<i32>::new(2, 0, 0);
            /// let y = Vec3::<i32>::new(0, 3, 0);
            ///
            /// assert_eq!(x.dot(y), 0);
            /// assert_eq!(x.dot(x), 4);
            /// assert_eq!(y.dot(y), 9);
            /// ```
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn dot(self, rhs: Self) -> $T {
                (self * rhs).element_sum()
            }

            /// Computes the squared length/magnitude of `self`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation) or
            /// overflow checks are enabled:
            ///
            /// Panics if an overflow occurs.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vec = Vec3::<i32>::new(1, 1, 1);
            /// assert_eq!(vec.length_squared(), 3);
            /// ```
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn length_squared(self) -> $T {
                (self * self).element_sum()
            }

            /// Computes the squared Euclidean distance between `self` and
            /// `other`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation) or
            /// overflow checks are enabled:
            ///
            /// Panics if an overflow occurs.
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
            pub fn distance_squared(self, other: Self) -> $T {
                (self - other).length_squared()
            }

            /// Computes `self + rhs`, returning `None` if overflow occured.
            #[inline]
            #[must_use]
            pub fn checked_add(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as $Backend<N, A>>::vec_checked_add(self, rhs))
            }

            /// Computes `self - rhs`, returning `None` if overflow occured.
            #[inline]
            #[must_use]
            pub fn checked_sub(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as $Backend<N, A>>::vec_checked_sub(self, rhs))
            }

            /// Computes `self * rhs`, returning `None` if overflow occured.
            #[inline]
            #[must_use]
            pub fn checked_mul(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as $Backend<N, A>>::vec_checked_mul(self, rhs))
            }

            /// Computes `self / rhs`, returning `None` if overflow or division
            /// by zero occured.
            #[inline]
            #[must_use]
            pub fn checked_div(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as $Backend<N, A>>::vec_checked_div(self, rhs))
            }

            /// Computes `self % rhs`, returning `None` if overflow or division
            /// by zero occurred.
            #[inline]
            #[must_use]
            pub fn checked_rem(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as $Backend<N, A>>::vec_checked_rem(self, rhs))
            }

            /// Computes `self + rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_add(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_saturating_add(self, rhs))
            }

            /// Computes `self - rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_sub(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_saturating_sub(self, rhs))
            }

            /// Computes `self * rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_mul(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_saturating_mul(self, rhs))
            }

            /// Computes `self / rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// # Panics
            ///
            /// Panics if any component of `rhs` is `0`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn saturating_div(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_saturating_div(self, rhs))
            }

            /// Computes `self + rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub fn wrapping_add(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_wrapping_add(self, rhs))
            }

            /// Computes `self - rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub fn wrapping_sub(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_wrapping_sub(self, rhs))
            }

            /// Computes `self * rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub fn wrapping_mul(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_wrapping_mul(self, rhs))
            }

            /// Computes `self / rhs`, wrapping around at the boundary of the type.
            ///
            /// # Panics
            ///
            /// Panics if any component of `rhs` is `0`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn wrapping_div(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_wrapping_div(self, rhs))
            }

            /// Computes `self % rhs`, wrapping around at the boundary of the type.
            ///
            /// # Panics
            ///
            /// Panics if any component of `rhs` is `0`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn wrapping_rem(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_wrapping_rem(self, rhs))
            }
        }

        impl<A: Alignment> Vector<2, $T, A> {
            /// Returns `self` rotated by 90 degrees.
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec2;
            /// #
            /// let x = Vec2::<i32>::X;
            /// let y = Vec2::<i32>::Y;
            ///
            /// assert_eq!(x.perp(), y);
            /// assert_eq!(y.perp(), -x);
            /// assert_eq!((-x).perp(), -y);
            /// assert_eq!((-y).perp(), x);
            /// ```
            #[inline]
            #[must_use]
            pub fn perp(self) -> Self {
                Self::new(-self.y, self.x)
            }
        }

        impl<A: Alignment> Vector<3, $T, A> {
            /// Computes the cross product of `self` and `rhs`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let x = Vec3::<i32>::new(1, 0, 0);
            /// let y = Vec3::<i32>::new(0, 1, 0);
            ///
            /// assert_eq!(x.cross(y), Vec3::new(0, 0, 1));
            /// assert_eq!(y.cross(x), Vec3::new(0, 0, -1));
            /// ```
            #[inline]
            #[must_use]
            pub fn cross(self, rhs: Self) -> Self {
                (self.zxy() * rhs - self * rhs.zxy()).zxy()
            }
        }

        pub(crate) trait $Backend<const N: usize, A: Alignment>: Scalar
        where
            Length<N>: SupportedLength,
        {
            #[inline]
            fn vec_max(vec: Vector<N, $T, A>, other: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].max(other[i]))
            }

            #[inline]
            fn vec_min(vec: Vector<N, $T, A>, other: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].min(other[i]))
            }

            #[inline]
            fn vec_max_element(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce($T::max).unwrap()
            }

            #[inline]
            fn vec_min_element(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce($T::min).unwrap()
            }

            #[inline]
            #[track_caller]
            fn vec_abs(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::abs)
            }

            #[inline]
            fn vec_signum(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::signum)
            }

            #[inline]
            fn vec_checked_add(
                mut vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Option<Vector<N, $T, A>> {
                for i in 0..N {
                    vec[i] = vec[i].checked_add(rhs[i])?;
                }

                Some(vec)
            }

            #[inline]
            fn vec_checked_sub(
                mut vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Option<Vector<N, $T, A>> {
                for i in 0..N {
                    vec[i] = vec[i].checked_sub(rhs[i])?;
                }

                Some(vec)
            }

            #[inline]
            fn vec_checked_mul(
                mut vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Option<Vector<N, $T, A>> {
                for i in 0..N {
                    vec[i] = vec[i].checked_mul(rhs[i])?;
                }

                Some(vec)
            }

            #[inline]
            fn vec_checked_div(
                mut vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Option<Vector<N, $T, A>> {
                for i in 0..N {
                    vec[i] = vec[i].checked_div(rhs[i])?;
                }

                Some(vec)
            }

            #[inline]
            fn vec_checked_rem(
                mut vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Option<Vector<N, $T, A>> {
                for i in 0..N {
                    vec[i] = vec[i].checked_rem(rhs[i])?;
                }
                Some(vec)
            }

            #[inline]
            #[track_caller]
            fn vec_strict_element_sum(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce(|a, b| a.checked_add(b).unwrap()).unwrap()
            }

            #[inline]
            #[track_caller]
            fn vec_strict_element_product(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce(|a, b| a.checked_mul(b).unwrap()).unwrap()
            }

            #[inline]
            fn vec_saturating_add(
                vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].saturating_add(rhs[i]))
            }

            #[inline]
            fn vec_saturating_sub(
                vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].saturating_sub(rhs[i]))
            }

            #[inline]
            fn vec_saturating_mul(
                vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].saturating_mul(rhs[i]))
            }

            #[inline]
            #[track_caller]
            fn vec_saturating_div(
                vec: Vector<N, $T, A>,
                rhs: Vector<N, $T, A>,
            ) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].saturating_div(rhs[i]))
            }

            #[inline]
            fn vec_wrapping_add(vec: Vector<N, $T, A>, rhs: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].wrapping_add(rhs[i]))
            }

            #[inline]
            fn vec_wrapping_sub(vec: Vector<N, $T, A>, rhs: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].wrapping_sub(rhs[i]))
            }

            #[inline]
            fn vec_wrapping_mul(vec: Vector<N, $T, A>, rhs: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].wrapping_mul(rhs[i]))
            }

            #[inline]
            #[track_caller]
            fn vec_wrapping_div(vec: Vector<N, $T, A>, rhs: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].wrapping_div(rhs[i]))
            }

            #[inline]
            #[track_caller]
            fn vec_wrapping_rem(vec: Vector<N, $T, A>, rhs: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].wrapping_rem(rhs[i]))
            }

            #[inline]
            fn vec_wrapping_element_sum(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce($T::wrapping_add).unwrap()
            }

            #[inline]
            fn vec_wrapping_element_product(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce($T::wrapping_mul).unwrap()
            }
        }
    };
}
impl_int!(i8, I8VectorBackend);
impl_int!(i16, I16VectorBackend);
impl_int!(i32, I32VectorBackend);
impl_int!(i64, I64VectorBackend);
impl_int!(i128, I128VectorBackend);
impl_int!(isize, IsizeVectorBackend);
