use crate::{Alignment, Length, Scalar, SupportedLength, Vector};

impl<const N: usize, A: Alignment> Vector<N, bool, A>
where
    Length<N>: SupportedLength,
{
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
        match N {
            2 => self[0] && self[1],
            3 => self[0] && self[1] && self[2],
            4 => self[0] && self[1] && self[2] && self[3],
            _ => unreachable!(),
        }
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
        match N {
            2 => self[0] || self[1],
            3 => self[0] || self[1] || self[2],
            4 => self[0] || self[1] || self[2] || self[3],
            _ => unreachable!(),
        }
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
        Vector::from_fn(|i| if self[i] { if_true[i] } else { if_false[i] })
    }
}
