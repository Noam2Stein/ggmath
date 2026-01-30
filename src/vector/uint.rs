use crate::{Alignment, Length, Scalar, SupportedLength, Vector, utils::specialize};

macro_rules! impl_uint {
    ($T:ident, $Backend:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            /// Computes the sum of the vector's components.
            ///
            /// Equivalent to `self.x + self.y + ...`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any addition overflows. Addition is performed in order.
            #[inline]
            #[must_use]
            pub fn element_sum(self) -> $T {
                if cfg!(assertions) {
                    specialize!(<$T as $Backend<N, A>>::vec_strict_element_sum(self))
                } else {
                    specialize!(<$T as $Backend<N, A>>::vec_wrapping_element_sum(self))
                }
            }

            /// Computes the product of the vector's components.
            ///
            /// Equivalent to `self.x * self.y * ...`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any multiplication overflows. Multiplication is performed in
            /// order.
            #[inline]
            #[must_use]
            pub fn element_product(self) -> $T {
                if cfg!(assertions) {
                    specialize!(<$T as $Backend<N, A>>::vec_strict_element_product(self))
                } else {
                    specialize!(<$T as $Backend<N, A>>::vec_wrapping_element_product(self))
                }
            }

            /// Returns the maximum between the components of `self` and `other`.
            ///
            /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_max(self, other))
            }

            /// Returns the minimum between the components of `self` and `other`.
            ///
            /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_min(self, other))
            }

            /// Clamps the vector's components between the components of `min` and
            /// `max`.
            ///
            /// Equivalent to
            /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
            ///
            /// # Panics
            ///
            /// Panics if `min > max`.
            #[inline]
            #[must_use]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                assert!((0..N).all(|i| min[i] <= max[i]), "min <= max");

                self.max(min).min(max)
            }

            /// Returns the maximum between the vector's components.
            ///
            /// Equivalent to `self.x.max(self.y).max(self.z)...`.
            #[inline]
            #[must_use]
            pub fn max_element(self) -> $T {
                specialize!(<$T as $Backend<N, A>>::vec_max_element(self))
            }

            /// Returns the minimum between the vector's components.
            ///
            /// Equivalent to `self.x.min(self.y).min(self.z)...`.
            #[inline]
            #[must_use]
            pub fn min_element(self) -> $T {
                specialize!(<$T as $Backend<N, A>>::vec_min_element(self))
            }

            /// Computes the dot product of `self` and `rhs`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation) or overflow
            /// checks are enabled:
            ///
            /// Panics if an overflow occurs.
            #[inline]
            #[must_use]
            pub fn dot(self, rhs: Self) -> $T {
                (self * rhs).element_sum()
            }

            /// Computes the squared length/magnitude of the vector.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation) or overflow
            /// checks are enabled:
            ///
            /// Panics if an overflow occurs.
            #[inline]
            #[must_use]
            pub fn length_squared(self) -> $T {
                (self * self).element_sum()
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

            /// Computes `self / rhs`, returning `None` if division by zero occured.
            #[inline]
            #[must_use]
            pub fn checked_div(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as $Backend<N, A>>::vec_checked_div(self, rhs))
            }

            /// Computes `self % rhs`, returning `None` if division by zero occurred.
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
            fn vec_strict_element_sum(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce(|a, b| a.checked_add(b).unwrap()).unwrap()
            }

            #[inline]
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
impl_uint!(u8, U8VectorBackend);
impl_uint!(u16, U16VectorBackend);
impl_uint!(u32, U32VectorBackend);
impl_uint!(u64, U64VectorBackend);
impl_uint!(u128, U128VectorBackend);
impl_uint!(usize, UsizeVectorBackend);
