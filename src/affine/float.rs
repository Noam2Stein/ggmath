use crate::{Affine, Alignment, Length, Scalar, SupportedLength, utils::PrimitiveFloat};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PrimitiveFloat,
{
    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two affines that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool {
        self.matrix.abs_diff_eq(&other.matrix, max_abs_diff)
            && self
                .translation
                .abs_diff_eq(other.translation, max_abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Affine2, Vec2, utils::for_parameters};

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|T: PrimitiveFloat| {
            assert!(
                Affine2::<T>::from_columns(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_columns(&[
                        Vec2::new(0.1, 0.9),
                        Vec2::new(1.95, 3.05),
                        Vec2::new(4.1, 4.9)
                    ]),
                    0.125
                )
            );
            assert!(
                !Affine2::<T>::from_columns(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_columns(&[
                        Vec2::new(0.1, 0.9),
                        Vec2::new(1.95, 3.5),
                        Vec2::new(4.1, 4.9)
                    ]),
                    0.125
                )
            );
            assert!(
                !Affine2::<T>::from_columns(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_columns(&[
                        Vec2::new(0.1, 0.9),
                        Vec2::new(1.95, 3.05),
                        Vec2::new(4.6, 4.9)
                    ]),
                    0.125
                )
            );
        });
    }
}
