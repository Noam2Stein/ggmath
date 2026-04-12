use crate::{Alignment, Quaternion, Scalar, utils::PrimitiveFloat};

#[expect(private_bounds)]
impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two quaternions that should be equal, but
    /// may have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool {
        self.to_vector()
            .abs_diff_eq(other.to_vector(), max_abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Quat, utils::for_parameters};

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|T: PrimitiveFloat| {
            assert!(
                Quat::<T>::from_xyzw(0.0, 1.0, 2.0, 3.0)
                    .abs_diff_eq(Quat::from_xyzw(0.0, 1.1, 2.05, 2.9), 0.125)
            );
            assert!(
                !Quat::<T>::from_xyzw(0.0, 1.0, 2.0, 3.0)
                    .abs_diff_eq(Quat::from_xyzw(0.0, 1.1, 2.5, 2.9), 0.125)
            );
        });
    }
}
