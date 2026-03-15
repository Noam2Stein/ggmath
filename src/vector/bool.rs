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

#[cfg(test)]
mod tests {
    use crate::{Vector, test_utils::for_parameters};

    #[test]
    fn test_all() {
        for_parameters!(|A, x, y, z, w| {
            assert_eq!(Vector::<2, bool, A>::new(x, y).all(), x && y);
            assert_eq!(Vector::<3, bool, A>::new(x, y, z).all(), x && y && z);
            assert_eq!(
                Vector::<4, bool, A>::new(x, y, z, w).all(),
                x && y && z && w
            );
        });
    }

    #[test]
    fn test_any() {
        for_parameters!(|A, x, y, z, w| {
            assert_eq!(Vector::<2, bool, A>::new(x, y).any(), x || y);
            assert_eq!(Vector::<3, bool, A>::new(x, y, z).any(), x || y || z);
            assert_eq!(
                Vector::<4, bool, A>::new(x, y, z, w).any(),
                x || y || z || w
            );
        });
    }

    #[test]
    fn test_select() {
        for_parameters!(|A, x, y, z, w| {
            assert_eq!(
                Vector::<2, bool, A>::new(x, y).select(
                    Vector::<2, i32, A>::new(1, 2),
                    Vector::<2, i32, A>::new(3, 4)
                ),
                Vector::<2, i32, A>::new(if x { 1 } else { 3 }, if y { 2 } else { 4 })
            );
            assert_eq!(
                Vector::<3, bool, A>::new(x, y, z).select(
                    Vector::<3, i32, A>::new(1, 2, 3),
                    Vector::<3, i32, A>::new(4, 5, 6)
                ),
                Vector::<3, i32, A>::new(
                    if x { 1 } else { 4 },
                    if y { 2 } else { 5 },
                    if z { 3 } else { 6 }
                )
            );
            assert_eq!(
                Vector::<4, bool, A>::new(x, y, z, w).select(
                    Vector::<4, i32, A>::new(1, 2, 3, 4),
                    Vector::<4, i32, A>::new(5, 6, 7, 8)
                ),
                Vector::<4, i32, A>::new(
                    if x { 1 } else { 5 },
                    if y { 2 } else { 6 },
                    if z { 3 } else { 7 },
                    if w { 4 } else { 8 }
                )
            );
        });
    }
}
