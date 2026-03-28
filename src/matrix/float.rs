use core::convert::identity;

use crate::{
    Alignment, Length, Matrix, Scalar, SupportedLength, Vector,
    num_primitive::PrimitiveFloat,
    transmute::{transmute_generic, transmute_ref},
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PrimitiveFloat,
{
    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let normal = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let nan = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, f32::NAN),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    ///
    /// assert!(!normal.is_nan());
    /// assert!(nan.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe { transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self).is_nan() },
            3 => self.column(0).is_nan() || self.column(1).is_nan() || self.column(2).is_nan(),
            4 => {
                self.column(0).is_nan()
                    || self.column(1).is_nan()
                    || self.column(2).is_nan()
                    || self.column(3).is_nan()
            }
            _ => unreachable!(),
        }
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let finite = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let infinite = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, f32::INFINITY),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    ///
    /// assert!(finite.is_finite());
    /// assert!(!infinite.is_finite());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe { transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self).is_finite() },
            3 => {
                self.column(0).is_finite()
                    && self.column(1).is_finite()
                    && self.column(2).is_finite()
            }
            4 => {
                self.column(0).is_finite()
                    && self.column(1).is_finite()
                    && self.column(2).is_finite()
                    && self.column(3).is_finite()
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    #[track_caller]
    fn generic_inverse<Output, W, C>(&self, wrap_result: W, check_determinant: C) -> Output
    where
        W: FnOnce(Self) -> Output,
        C: FnOnce(T) -> Result<(), Output>,
    {
        match N {
            2 => {
                // SAFETY: Because `N == 2`, `Matrix<N, T, A>` is `Matrix<2, T, A>`.
                let mat = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<2, T, A>>(self) };

                let determinant = mat.determinant();
                if let Err(error) = check_determinant(determinant) {
                    return error;
                }

                let determinant_recip = determinant.recip();
                let result = Matrix::<2, T, A>::from_column_array(&[
                    mat.y_axis.y * determinant_recip,
                    mat.x_axis.y * -determinant_recip,
                    mat.y_axis.x * -determinant_recip,
                    mat.x_axis.x * determinant_recip,
                ]);

                // SAFETY: Because `N == 2`, `Matrix<2, T, A>` is `Matrix<N, T, A>`.
                wrap_result(unsafe {
                    transmute_generic::<Matrix<2, T, A>, Matrix<N, T, A>>(result)
                })
            }
            3 => {
                // SAFETY: Because `N == 3`, `Matrix<N, T, A>` is `Matrix<3, T, A>`.
                let mat = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self) };

                let y_cross_z = mat.y_axis.cross(mat.z_axis);
                let z_cross_x = mat.z_axis.cross(mat.x_axis);
                let x_cross_y = mat.x_axis.cross(mat.y_axis);

                let determinant = mat.z_axis.dot(x_cross_y);
                if let Err(error) = check_determinant(determinant) {
                    return error;
                }

                let determinant_recip = Vector::<3, T, A>::splat(determinant.recip());
                let result = Matrix::<3, T, A>::from_columns(&[
                    y_cross_z * determinant_recip,
                    z_cross_x * determinant_recip,
                    x_cross_y * determinant_recip,
                ])
                .transpose();

                // SAFETY: Because `N == 3`, `Matrix<3, T, A>` is `Matrix<N, T, A>`.
                wrap_result(unsafe {
                    transmute_generic::<Matrix<3, T, A>, Matrix<N, T, A>>(result)
                })
            }
            4 => {
                // SAFETY: Because `N == 4`, `Matrix<N, T, A>` is `Matrix<4, T, A>`.
                let mat = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<4, T, A>>(self) };

                let [m00, m01, m02, m03] = mat.x_axis.to_array();
                let [m10, m11, m12, m13] = mat.y_axis.to_array();
                let [m20, m21, m22, m23] = mat.z_axis.to_array();
                let [m30, m31, m32, m33] = mat.w_axis.to_array();

                let coef00 = m22 * m33 - m32 * m23;
                let coef02 = m12 * m33 - m32 * m13;
                let coef03 = m12 * m23 - m22 * m13;

                let coef04 = m21 * m33 - m31 * m23;
                let coef06 = m11 * m33 - m31 * m13;
                let coef07 = m11 * m23 - m21 * m13;

                let coef08 = m21 * m32 - m31 * m22;
                let coef10 = m11 * m32 - m31 * m12;
                let coef11 = m11 * m22 - m21 * m12;

                let coef12 = m20 * m33 - m30 * m23;
                let coef14 = m10 * m33 - m30 * m13;
                let coef15 = m10 * m23 - m20 * m13;

                let coef16 = m20 * m32 - m30 * m22;
                let coef18 = m10 * m32 - m30 * m12;
                let coef19 = m10 * m22 - m20 * m12;

                let coef20 = m20 * m31 - m30 * m21;
                let coef22 = m10 * m31 - m30 * m11;
                let coef23 = m10 * m21 - m20 * m11;

                let fac0 = Vector::<4, T, A>::new(coef00, coef00, coef02, coef03);
                let fac1 = Vector::<4, T, A>::new(coef04, coef04, coef06, coef07);
                let fac2 = Vector::<4, T, A>::new(coef08, coef08, coef10, coef11);
                let fac3 = Vector::<4, T, A>::new(coef12, coef12, coef14, coef15);
                let fac4 = Vector::<4, T, A>::new(coef16, coef16, coef18, coef19);
                let fac5 = Vector::<4, T, A>::new(coef20, coef20, coef22, coef23);

                let vec0 = Vector::<4, T, A>::new(m10, m00, m00, m00);
                let vec1 = Vector::<4, T, A>::new(m11, m01, m01, m01);
                let vec2 = Vector::<4, T, A>::new(m12, m02, m02, m02);
                let vec3 = Vector::<4, T, A>::new(m13, m03, m03, m03);

                let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
                let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
                let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
                let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

                let sign_a = Vector::<4, T, A>::new(
                    T::as_from(1.0),
                    T::as_from(-1.0),
                    T::as_from(1.0),
                    T::as_from(-1.0),
                );
                let sign_b = Vector::<4, T, A>::new(
                    T::as_from(-1.0),
                    T::as_from(1.0),
                    T::as_from(-1.0),
                    T::as_from(1.0),
                );

                let inverse = Matrix::<4, T, A>::from_columns(&[
                    inv0 * sign_a,
                    inv1 * sign_b,
                    inv2 * sign_a,
                    inv3 * sign_b,
                ]);

                let col0 = Vector::<4, T, A>::new(
                    inverse.x_axis.x,
                    inverse.y_axis.x,
                    inverse.z_axis.x,
                    inverse.w_axis.x,
                );

                let dot0 = mat.x_axis * col0;
                let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

                if let Err(error) = check_determinant(dot1) {
                    return error;
                }

                let determinant_recip = dot1.recip();
                let result = inverse * determinant_recip;

                // SAFETY: Because `N == 4`, `Matrix<4, T, A>` is `Matrix<N, T, A>`.
                wrap_result(unsafe {
                    transmute_generic::<Matrix<4, T, A>, Matrix<N, T, A>>(result)
                })
            }
            _ => unreachable!(),
        }
    }

    /// Returns the inverse of `self`.
    ///
    /// If `self` is not invertable the result is unspecified.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the determinant is `0`.
    #[must_use]
    #[track_caller]
    pub fn inverse(&self) -> Self {
        #[cfg(assertions)]
        {
            let mut determinant_is_zero = false;
            let result = self.generic_inverse(identity, |determinant| {
                determinant_is_zero = determinant == T::as_from(0.0);
                Ok(())
            });

            if determinant_is_zero {
                panic!("matrix is not invertable");
            }

            result
        }
        #[cfg(not(assertions))]
        {
            self.generic_inverse(identity, |_| Ok(()))
        }
    }

    /// Returns the inverse of `self` or `None` if `self` is not invertable.
    #[must_use]
    pub fn try_inverse(&self) -> Option<Self> {
        self.generic_inverse(Some, |determinant| {
            if determinant == T::as_from(0.0) {
                Err(None)
            } else {
                Ok(())
            }
        })
    }

    /// Returns the inverse of `self` or `fallback` if `self` is not invertable.
    #[must_use]
    pub fn inverse_or(&self, fallback: &Self) -> Self {
        self.generic_inverse(identity, |determinant| {
            if determinant == T::as_from(0.0) {
                Err(*fallback)
            } else {
                Ok(())
            }
        })
    }

    /// Returns the inverse of `self` or the zero matrix if `self` is not
    /// invertable.
    #[must_use]
    pub fn inverse_or_zero(&self) -> Self {
        self.generic_inverse(identity, |determinant| {
            if determinant == T::as_from(0.0) {
                Err(Self::ZERO)
            } else {
                Ok(())
            }
        })
    }

    /// Returns the element-wise reciprocal (inverse) of a matrix, `1 / self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let mat = Mat3::from_columns(&[
    ///     Vec3::new(2.0, 4.0, 1.0),
    ///     Vec3::new(1.0, 2.0, 4.0),
    ///     Vec3::new(4.0, 1.0, 2.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     mat.recip(),
    ///     Mat3::from_columns(&[
    ///         Vec3::new(0.5, 0.25, 1.0),
    ///         Vec3::new(1.0, 0.5, 0.25),
    ///         Vec3::new(0.25, 1.0, 0.5),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn recip(&self) -> Self {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Matrix<N, T, A>>(mat.recip())
            },
            _ => Self::from_column_fn(|i| self.column(i).recip()),
        }
    }

    /// Returns the absolute values of the elements of `self`.
    ///
    /// Equivalent to `(self.x_axis.abs(), self.y_axis.abs(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let mat = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, -1.0, 0.0),
    ///     Vec3::new(0.0, 0.0, -1.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     mat.abs(),
    ///     Mat3::from_columns(&[
    ///         Vec3::new(1.0, 0.0, 0.0),
    ///         Vec3::new(0.0, 1.0, 0.0),
    ///         Vec3::new(0.0, 0.0, 1.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Matrix<N, T, A>>(mat.abs())
            },
            _ => Self::from_column_fn(|i| self.column(i).abs()),
        }
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two matrices that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                let other = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(other);

                mat.abs_diff_eq(*other, max_abs_diff)
            },
            3 => {
                self.column(0).abs_diff_eq(other.column(0), max_abs_diff)
                    && self.column(1).abs_diff_eq(other.column(1), max_abs_diff)
                    && self.column(2).abs_diff_eq(other.column(2), max_abs_diff)
            }
            4 => {
                self.column(0).abs_diff_eq(other.column(0), max_abs_diff)
                    && self.column(1).abs_diff_eq(other.column(1), max_abs_diff)
                    && self.column(2).abs_diff_eq(other.column(2), max_abs_diff)
                    && self.column(3).abs_diff_eq(other.column(3), max_abs_diff)
            }
            _ => unreachable!(),
        }
    }
}

#[expect(private_bounds)]
impl<T, A: Alignment> Matrix<2, T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Creates a matrix containing a rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<2, T, A>::new(cos, sin),
            Vector::<2, T, A>::new(-sin, cos),
        ])
    }

    /// Creates a matrix containing the non-uniform `scale` and a rotation of
    /// `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<2, T, A>::new(cos * scale.x, sin * scale.x),
            Vector::<2, T, A>::new(-sin * scale.y, cos * scale.y),
        ])
    }
}

#[expect(private_bounds)]
impl<T, A: Alignment> Matrix<3, T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Creates an affine transformation matrix containing a rotation of `angle`
    /// (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<3, T, A>::new(cos, sin, T::as_from(0.0)),
            Vector::<3, T, A>::new(-sin, cos, T::as_from(0.0)),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Creates an affine transformation matrix containing the non-uniform
    /// `scale` and a rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<3, T, A>::new(cos * scale.x, sin * scale.x, T::as_from(0.0)),
            Vector::<3, T, A>::new(-sin * scale.y, cos * scale.y, T::as_from(0.0)),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Creates an affine transformation matrix containing the non-uniform
    /// `scale`, a rotation of `angle` (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    pub fn from_scale_angle_translation(
        scale: Vector<2, T, A>,
        angle: T,
        translation: Vector<2, T, A>,
    ) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<3, T, A>::new(cos * scale.x, sin * scale.x, T::as_from(0.0)),
            Vector::<3, T, A>::new(-sin * scale.y, cos * scale.y, T::as_from(0.0)),
            Vector::<3, T, A>::new(translation.x, translation.y, T::as_from(1.0)),
        ])
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the x
    /// axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<3, T, A>::X,
            Vector::<3, T, A>::new(T::as_from(0.0), cos, sin),
            Vector::<3, T, A>::new(T::as_from(0.0), -sin, cos),
        ])
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the y
    /// axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<3, T, A>::new(cos, T::as_from(0.0), -sin),
            Vector::<3, T, A>::Y,
            Vector::<3, T, A>::new(sin, T::as_from(0.0), cos),
        ])
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the z
    /// axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<3, T, A>::new(cos, sin, T::as_from(0.0)),
            Vector::<3, T, A>::new(-sin, cos, T::as_from(0.0)),
            Vector::<3, T, A>::Z,
        ])
    }
}

#[expect(private_bounds)]
impl<T, A: Alignment> Matrix<4, T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Creates an affine transformation matrix containing a 3D rotation from
    /// `angle` (in radians) around the x axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<4, T, A>::X,
            Vector::<4, T, A>::new(T::as_from(0.0), cos, sin, T::as_from(0.0)),
            Vector::<4, T, A>::new(T::as_from(0.0), -sin, cos, T::as_from(0.0)),
            Vector::<4, T, A>::W,
        ])
    }

    /// Creates an affine transformation matrix containing a 3D rotation from
    /// `angle` (in radians) around the y axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<4, T, A>::new(cos, T::as_from(0.0), -sin, T::as_from(0.0)),
            Vector::<4, T, A>::Y,
            Vector::<4, T, A>::new(sin, T::as_from(0.0), cos, T::as_from(0.0)),
            Vector::<4, T, A>::W,
        ])
    }

    /// Creates an affine transformation matrix containing a 3D rotation from
    /// `angle` (in radians) around the z axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_columns(&[
            Vector::<4, T, A>::new(cos, sin, T::as_from(0.0), T::as_from(0.0)),
            Vector::<4, T, A>::new(-sin, cos, T::as_from(0.0), T::as_from(0.0)),
            Vector::<4, T, A>::Z,
            Vector::<4, T, A>::W,
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Matrix, Vector,
        test_utils::{assert_float_eq, assert_panic, for_parameters},
    };

    #[test]
    fn test_is_nan() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let nan = Vector::NAN;
            assert!(!Matrix::<2, T, A>::from_columns(&[one, one]).is_nan());
            assert!(Matrix::<2, T, A>::from_columns(&[nan, one]).is_nan());
            assert!(Matrix::<2, T, A>::from_columns(&[one, nan]).is_nan());
            assert!(Matrix::<2, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::NAN;
            assert!(!Matrix::<3, T, A>::from_columns(&[one, one, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_columns(&[nan, one, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_columns(&[one, nan, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_columns(&[one, one, nan]).is_nan());
            assert!(Matrix::<3, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::NAN;
            assert!(!Matrix::<4, T, A>::from_columns(&[one, one, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_columns(&[nan, one, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_columns(&[one, nan, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_columns(&[one, one, nan, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_columns(&[one, one, one, nan]).is_nan());
            assert!(Matrix::<4, T, A>::NAN.is_nan());
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let inf = Vector::INFINITY;
            assert!(Matrix::<2, T, A>::from_columns(&[one, one]).is_finite());
            assert!(!Matrix::<2, T, A>::from_columns(&[inf, one]).is_finite());
            assert!(!Matrix::<2, T, A>::from_columns(&[one, inf]).is_finite());
            assert!(!Matrix::<2, T, A>::from_columns(&[inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::INFINITY;
            assert!(Matrix::<3, T, A>::from_columns(&[one, one, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_columns(&[inf, one, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_columns(&[one, inf, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_columns(&[one, one, inf]).is_finite());
            assert!(!Matrix::<3, T, A>::from_columns(&[inf, inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::INFINITY;
            assert!(Matrix::<4, T, A>::from_columns(&[one, one, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[inf, one, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[one, inf, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[one, one, inf, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[one, one, one, inf]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[inf, inf, inf, inf]).is_finite());
        });
    }

    #[test]
    fn test_inverse() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];
            let [i, j, k, l] = [c + d, c + e, d + f, f + g];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 10000.0
                || y.abs() > 10000.0
                || z.abs() > 10000.0
            {
                return;
            }

            let mat = Matrix::<2, T, A>::from_column_array(&[x, y, z, w]);
            if mat.determinant() != 0.0 {
                assert_float_eq!(
                    mat * mat.inverse(),
                    Matrix::IDENTITY,
                    abs <= Matrix::<2, T, A>::from_column_array(&[1e-5; 4])
                        * mat.determinant().abs().log2().abs().exp2().powi(2),
                    0.0 = -0.0
                );
            } else if cfg!(assertions) {
                assert_panic!(mat.inverse());
            }

            let mat = Matrix::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e]);
            if mat.determinant() != 0.0 {
                assert_float_eq!(
                    mat * mat.inverse(),
                    Matrix::IDENTITY,
                    abs <= Matrix::<3, T, A>::from_column_array(&[1e-5; 9])
                        * mat.determinant().abs().log2().abs().exp2().powi(2),
                    0.0 = -0.0
                );
            } else if cfg!(assertions) {
                assert_panic!(mat.inverse());
            }

            let mat = Matrix::<4, T, A>::from_column_array(&[
                x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l,
            ]);
            if mat.determinant() != 0.0 {
                assert_float_eq!(
                    mat * mat.inverse(),
                    Matrix::IDENTITY,
                    abs <= Matrix::<4, T, A>::from_column_array(&[1e-5; 16])
                        * mat.determinant().abs().log2().abs().exp2().powi(2),
                    0.0 = -0.0
                );
            } else if cfg!(assertions) {
                assert_panic!(mat.inverse());
            }
        });
    }

    #[test]
    fn test_try_inverse() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];
            let [i, j, k, l] = [c + d, c + e, d + f, f + g];

            let mat = Matrix::<2, T, A>::from_column_array(&[x, y, z, w]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse(), inverse);
            } else if cfg!(assertions) {
                assert_panic!(mat.inverse());
            }

            let mat = Matrix::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse(), inverse);
            } else if cfg!(assertions) {
                assert_panic!(mat.inverse());
            }

            let mat = Matrix::<4, T, A>::from_column_array(&[
                x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l,
            ]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse(), inverse);
            } else if cfg!(assertions) {
                assert_panic!(mat.inverse());
            }
        });
    }

    #[test]
    fn test_inverse_or() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];
            let [i, j, k, l] = [c + d, c + e, d + f, f + g];

            let mat = Matrix::<2, T, A>::from_column_array(&[x, y, z, w]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse_or(&Matrix::NAN), inverse);
            } else if cfg!(assertions) {
                assert_float_eq!(mat.inverse_or(&Matrix::NAN), Matrix::NAN);
            }

            let mat = Matrix::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse_or(&Matrix::NAN), inverse);
            } else if cfg!(assertions) {
                assert_float_eq!(mat.inverse_or(&Matrix::NAN), Matrix::NAN);
            }

            let mat = Matrix::<4, T, A>::from_column_array(&[
                x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l,
            ]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse_or(&Matrix::NAN), inverse);
            } else if cfg!(assertions) {
                assert_float_eq!(mat.inverse_or(&Matrix::NAN), Matrix::NAN);
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];
            let [i, j, k, l] = [c + d, c + e, d + f, f + g];

            let mat = Matrix::<2, T, A>::from_column_array(&[x, y, z, w]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse_or_zero(), inverse);
            } else if cfg!(assertions) {
                assert_float_eq!(mat.inverse_or_zero(), Matrix::ZERO);
            }

            let mat = Matrix::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse_or_zero(), inverse);
            } else if cfg!(assertions) {
                assert_float_eq!(mat.inverse_or_zero(), Matrix::ZERO);
            }

            let mat = Matrix::<4, T, A>::from_column_array(&[
                x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l,
            ]);
            if let Some(inverse) = mat.try_inverse() {
                assert_float_eq!(mat.inverse_or_zero(), inverse);
            } else if cfg!(assertions) {
                assert_float_eq!(mat.inverse_or_zero(), Matrix::ZERO);
            }
        });
    }

    #[test]
    fn test_recip() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(1.0, 2.0),
                    Vector::<2, T, A>::new(0.5, 4.0)
                ])
                .recip(),
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(1.0, 0.5),
                    Vector::<2, T, A>::new(2.0, 0.25)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(1.0, 2.0, 0.5),
                    Vector::<3, T, A>::new(4.0, 0.25, 8.0),
                    Vector::<3, T, A>::new(0.125, 16.0, 0.0625)
                ])
                .recip(),
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(1.0, 0.5, 2.0),
                    Vector::<3, T, A>::new(0.25, 4.0, 0.125),
                    Vector::<3, T, A>::new(8.0, 0.0625, 16.0)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(1.0, 2.0, 0.5, 4.0),
                    Vector::<4, T, A>::new(0.25, 8.0, 0.125, 16.0),
                    Vector::<4, T, A>::new(0.0625, 32.0, 0.03125, 64.0),
                    Vector::<4, T, A>::new(0.015625, 128.0, 0.0078125, 256.0)
                ])
                .recip(),
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(1.0, 0.5, 2.0, 0.25),
                    Vector::<4, T, A>::new(4.0, 0.125, 8.0, 0.0625),
                    Vector::<4, T, A>::new(16.0, 0.03125, 32.0, 0.015625),
                    Vector::<4, T, A>::new(64.0, 0.0078125, 128.0, 0.00390625)
                ])
            );
        });
    }

    #[test]
    fn test_abs() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(-1.0, 2.0),
                    Vector::<2, T, A>::new(-3.0, 4.0)
                ])
                .abs(),
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(1.0, 2.0),
                    Vector::<2, T, A>::new(3.0, 4.0)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(-1.0, 2.0, -3.0),
                    Vector::<3, T, A>::new(4.0, -5.0, 6.0),
                    Vector::<3, T, A>::new(-7.0, 8.0, -9.0)
                ])
                .abs(),
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(1.0, 2.0, 3.0),
                    Vector::<3, T, A>::new(4.0, 5.0, 6.0),
                    Vector::<3, T, A>::new(7.0, 8.0, 9.0)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(-1.0, 2.0, -3.0, 4.0),
                    Vector::<4, T, A>::new(-5.0, 6.0, -7.0, 8.0),
                    Vector::<4, T, A>::new(9.0, -10.0, 11.0, -12.0),
                    Vector::<4, T, A>::new(13.0, -14.0, 15.0, -16.0)
                ])
                .abs(),
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(1.0, 2.0, 3.0, 4.0),
                    Vector::<4, T, A>::new(5.0, 6.0, 7.0, 8.0),
                    Vector::<4, T, A>::new(9.0, 10.0, 11.0, 12.0),
                    Vector::<4, T, A>::new(13.0, 14.0, 15.0, 16.0)
                ])
            );
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let col0 = Vector::<2, T, A>::new(0.0, 1.0);
            let col1 = Vector::<2, T, A>::new(2.0, 3.0);
            assert!(
                Matrix::<2, T, A>::from_columns(&[col0, col1])
                    .abs_diff_eq(&Matrix::from_columns(&[col0 + 0.1, col1 - 0.1]), 0.125)
            );
            assert!(
                !Matrix::<2, T, A>::from_columns(&[col0, col1])
                    .abs_diff_eq(&Matrix::from_columns(&[col0 + 0.5, col1 - 0.1]), 0.125)
            );
            assert!(
                !Matrix::<2, T, A>::from_columns(&[col0, col1])
                    .abs_diff_eq(&Matrix::from_columns(&[col0 + 0.1, col1 - 0.5]), 0.125)
            );

            let col0 = Vector::<3, T, A>::new(0.0, 1.0, 2.0);
            let col1 = Vector::<3, T, A>::new(3.0, 4.0, 5.0);
            let col2 = Vector::<3, T, A>::new(6.0, 7.0, 8.0);
            assert!(
                Matrix::<3, T, A>::from_columns(&[col0, col1, col2]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_columns(&[col0, col1, col2]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.5, col1 - 0.1, col2 + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_columns(&[col0, col1, col2]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.5, col2 + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_columns(&[col0, col1, col2]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.5]),
                    0.125
                )
            );

            let col0 = Vector::<4, T, A>::new(0.0, 1.0, 2.0, 3.0);
            let col1 = Vector::<4, T, A>::new(4.0, 5.0, 6.0, 7.0);
            let col2 = Vector::<4, T, A>::new(8.0, 9.0, 10.0, 11.0);
            let col3 = Vector::<4, T, A>::new(12.0, 13.0, 14.0, 15.0);
            assert!(
                Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.05, col3 - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.5, col1 - 0.1, col2 + 0.05, col3 - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.5, col2 + 0.05, col3 - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.5, col3 - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.05, col3 - 0.5]),
                    0.125
                )
            );
        });
    }

    #[test]
    fn test_from_angle() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            assert_float_eq!(
                Matrix::<2, T, A>::from_angle(z) * Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(x, y).rotate(z)
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_angle(z).transform_point(Vector::<2, T, A>::new(x, y)),
                Vector::<2, T, A>::new(x, y).rotate(z),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_scale_angle() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<2, T, A>::new(x, y);

            assert_float_eq!(
                Matrix::<2, T, A>::from_scale_angle(scale, z),
                Matrix::<2, T, A>::from_angle(z) * Matrix::<2, T, A>::from_diagonal(scale),
                0.0 = -0.0
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_scale_angle(scale, z),
                Matrix::<3, T, A>::from_angle(z) * Matrix::<3, T, A>::from_scale(scale),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<2, T, A>::new(x, y);
            let translation = Vector::<2, T, A>::new(x + 1.0, y + 2.0);

            assert_float_eq!(
                Matrix::<3, T, A>::from_scale_angle_translation(scale, z, translation),
                Matrix::<3, T, A>::from_translation(translation)
                    * Matrix::<3, T, A>::from_angle(z)
                    * Matrix::<3, T, A>::from_scale(scale),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                return;
            }

            let vec = Vector::<3, T, A>::new(x, y, y + 1.0);

            assert_float_eq!(
                Matrix::<3, T, A>::from_rotation_x(z) * vec,
                vec.rotate_x(z),
                0.0 = -0.0
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_rotation_x(z).transform_point(vec),
                vec.rotate_x(z),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                return;
            }

            let vec = Vector::<3, T, A>::new(x, y, y + 1.0);

            assert_float_eq!(
                Matrix::<3, T, A>::from_rotation_y(z) * vec,
                vec.rotate_y(z),
                0.0 = -0.0
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_rotation_y(z).transform_point(vec),
                vec.rotate_y(z),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                return;
            }

            let vec = Vector::<3, T, A>::new(x, y, y + 1.0);

            assert_float_eq!(
                Matrix::<3, T, A>::from_rotation_z(z) * vec,
                vec.rotate_z(z),
                0.0 = -0.0
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_rotation_z(z).transform_point(vec),
                vec.rotate_z(z),
                0.0 = -0.0
            );
        });
    }
}
