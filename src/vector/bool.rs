use crate::{Alignment, Length, Scalar, SupportedLength, Vector, utils::specialize};

impl<const N: usize, A: Alignment> Vector<N, bool, A>
where
    Length<N>: SupportedLength,
{
    /// A vector with all elements set to `false`.
    pub const FALSE: Self = Self::splat(false);

    /// A vector with all elements set to `true`.
    pub const TRUE: Self = Self::splat(true);

    /// Returns `true` if all elements of `self` are `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(true, true, false).all();
    /// assert_eq!(a, false);
    ///
    /// let a = Vec3::new(true, true, true).all();
    /// assert_eq!(a, true);
    /// ```
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        specialize!(Vector::<N, bool, A>::all_backend(self))
    }

    /// Returns `true` if any element of `self` is `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(true, true, false).any();
    /// assert_eq!(a, true);
    ///
    /// let a = Vec3::new(false, false, false).any();
    /// assert_eq!(a, false);
    /// ```
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        specialize!(Vector::<N, bool, A>::any_backend(self))
    }

    /// Selects between the elements of `if_true` and `if_false` based on the
    /// boolean elements of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec4;
    /// #
    /// let a = Vec4::new(true, false, false, true);
    /// let b = Vec4::new(1, 2, 3, 4);
    /// let c = Vec4::new(-1, -2, -3, -4);
    /// let d = a.select(b, c);
    /// assert_eq!(d, Vec4::new(1, -2, -3, 4));
    /// ```
    #[inline]
    #[must_use]
    pub fn select<T: Scalar>(
        self,
        if_true: Vector<N, T, A>,
        if_false: Vector<N, T, A>,
    ) -> Vector<N, T, A> {
        specialize!(Vector::<N, bool, A>::select_backend(
            self, if_true, if_false
        ))
    }
}

impl<A: Alignment> Vector<2, bool, A> {
    #[inline(always)]
    fn all_backend(self) -> bool {
        self.x && self.y
    }

    #[inline(always)]
    fn any_backend(self) -> bool {
        self.x || self.y
    }

    #[inline(always)]
    fn select_backend<T: Scalar>(
        self,
        if_true: Vector<2, T, A>,
        if_false: Vector<2, T, A>,
    ) -> Vector<2, T, A> {
        Vector::<2, T, A>::new(
            if self.x { if_true.x } else { if_false.x },
            if self.y { if_true.y } else { if_false.y },
        )
    }
}

impl<A: Alignment> Vector<3, bool, A> {
    #[inline(always)]
    fn all_backend(self) -> bool {
        self.x && self.y && self.z
    }

    #[inline(always)]
    fn any_backend(self) -> bool {
        self.x || self.y || self.z
    }

    #[inline(always)]
    fn select_backend<T: Scalar>(
        self,
        if_true: Vector<3, T, A>,
        if_false: Vector<3, T, A>,
    ) -> Vector<3, T, A> {
        Vector::<3, T, A>::new(
            if self.x { if_true.x } else { if_false.x },
            if self.y { if_true.y } else { if_false.y },
            if self.z { if_true.z } else { if_false.z },
        )
    }
}

impl<A: Alignment> Vector<4, bool, A> {
    #[inline(always)]
    fn all_backend(self) -> bool {
        self.x && self.y && self.z && self.w
    }

    #[inline(always)]
    fn any_backend(self) -> bool {
        self.x || self.y || self.z || self.w
    }

    #[inline(always)]
    fn select_backend<T: Scalar>(
        self,
        if_true: Vector<4, T, A>,
        if_false: Vector<4, T, A>,
    ) -> Vector<4, T, A> {
        Vector::<4, T, A>::new(
            if self.x { if_true.x } else { if_false.x },
            if self.y { if_true.y } else { if_false.y },
            if self.z { if_true.z } else { if_false.z },
            if self.w { if_true.w } else { if_false.w },
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::convert::identity;

    use crate::{
        Vector,
        test_utils::{for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, A| {
            assert_eq!(Vector::<N, bool, A>::FALSE, Vector::splat(false));
            assert_eq!(Vector::<N, bool, A>::TRUE, Vector::splat(true));
        });
    }

    #[test]
    fn test_all() {
        for_types!(|N, A| {
            for vector in [Vector::splat(false), Vector::splat(true)]
                .into_iter()
                .chain(random_iter::<Vector<N, bool, A>>())
            {
                assert_eq!(vector.all(), vector.iter().all(identity));
            }
        });
    }

    #[test]
    fn test_any() {
        for_types!(|N, A| {
            for vector in [Vector::splat(false), Vector::splat(true)]
                .into_iter()
                .chain(random_iter::<Vector<N, bool, A>>())
            {
                assert_eq!(vector.any(), vector.iter().any(identity));
            }
        });
    }

    #[test]
    fn test_select() {
        for_types!(|N, A| {
            let if_true = Vector::<N, usize, A>::from_fn(identity);
            let if_false = Vector::<N, usize, A>::from_fn(|i| i + N);

            for vector in random_iter::<Vector<N, bool, A>>() {
                assert_eq!(
                    vector.select(if_true, if_false),
                    Vector::from_fn(|i| if vector[i] { if_true[i] } else { if_false[i] })
                );
            }
        });
    }
}
