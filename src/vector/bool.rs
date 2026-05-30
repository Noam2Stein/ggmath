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
    extern crate std;

    use std::convert::identity;

    use crate::{
        Vector,
        utils::{for_types, random_iter},
    };

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
