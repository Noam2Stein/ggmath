use core::convert::identity;

use crate::{
    Alignment, EulerRot, Length, Matrix, PrimitiveFloat, Quaternion, SupportedLength, Vector,
    utils::{PrimitiveFloatUtils, transmute_generic, transmute_ref},
};

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveFloat,
{
    /// A matrix with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::from_rows(&[Vector::<N, T, A>::NAN; N]);

    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let normal = Mat3::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let nan = Mat3::from_rows(&[
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
            3 => self[0].is_nan() || self[1].is_nan() || self[2].is_nan(),
            4 => self[0].is_nan() || self[1].is_nan() || self[2].is_nan() || self[3].is_nan(),
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
    /// let finite = Mat3::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let infinite = Mat3::from_rows(&[
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
            3 => self[0].is_finite() && self[1].is_finite() && self[2].is_finite(),
            4 => {
                self[0].is_finite()
                    && self[1].is_finite()
                    && self[2].is_finite()
                    && self[3].is_finite()
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
                let matrix = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<2, T, A>>(self) };

                let determinant = matrix.determinant();
                if let Err(error) = check_determinant(determinant) {
                    return error;
                }

                let determinant_recip = determinant.recip();
                let result = Matrix::<2, T, A>::from_row_array(&[
                    matrix.y_axis.y * determinant_recip,
                    matrix.x_axis.y * -determinant_recip,
                    matrix.y_axis.x * -determinant_recip,
                    matrix.x_axis.x * determinant_recip,
                ]);

                // SAFETY: Because `N == 2`, `Matrix<2, T, A>` is `Matrix<N, T, A>`.
                wrap_result(unsafe {
                    transmute_generic::<Matrix<2, T, A>, Matrix<N, T, A>>(result)
                })
            }
            3 => {
                // SAFETY: Because `N == 3`, `Matrix<N, T, A>` is `Matrix<3, T, A>`.
                let matrix = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self) };

                let y_cross_z = matrix.y_axis.cross(matrix.z_axis);
                let z_cross_x = matrix.z_axis.cross(matrix.x_axis);
                let x_cross_y = matrix.x_axis.cross(matrix.y_axis);

                let determinant = matrix.z_axis.dot(x_cross_y);
                if let Err(error) = check_determinant(determinant) {
                    return error;
                }

                let determinant_recip = Vector::<3, T, A>::splat(determinant.recip());
                let result = Matrix::<3, T, A>::from_rows(&[
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
                let matrix = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<4, T, A>>(self) };

                let [m00, m01, m02, m03] = matrix.x_axis.to_array();
                let [m10, m11, m12, m13] = matrix.y_axis.to_array();
                let [m20, m21, m22, m23] = matrix.z_axis.to_array();
                let [m30, m31, m32, m33] = matrix.w_axis.to_array();

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

                let inverse = Matrix::<4, T, A>::from_rows(&[
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

                let dot0 = matrix.x_axis * col0;
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
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant is `0`.
    #[must_use]
    #[track_caller]
    pub fn inverse(&self) -> Self {
        #[cfg(debug_assertions)]
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
        #[cfg(not(debug_assertions))]
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
    /// let matrix = Mat3::from_rows(&[
    ///     Vec3::new(2.0, 4.0, 1.0),
    ///     Vec3::new(1.0, 2.0, 4.0),
    ///     Vec3::new(4.0, 1.0, 2.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     matrix.recip(),
    ///     Mat3::from_rows(&[
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
                let matrix = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Matrix<N, T, A>>(matrix.recip())
            },
            _ => Self::from_row_fn(|i| self[i].recip()),
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
    /// let matrix = Mat3::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, -1.0, 0.0),
    ///     Vec3::new(0.0, 0.0, -1.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     matrix.abs(),
    ///     Mat3::from_rows(&[
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
                let matrix = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Matrix<N, T, A>>(matrix.abs())
            },
            _ => Self::from_row_fn(|i| self[i].abs()),
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
                let matrix = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                let other = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(other);

                matrix.abs_diff_eq(*other, max_abs_diff)
            },
            3 => {
                self[0].abs_diff_eq(other[0], max_abs_diff)
                    && self[1].abs_diff_eq(other[1], max_abs_diff)
                    && self[2].abs_diff_eq(other[2], max_abs_diff)
            }
            4 => {
                self[0].abs_diff_eq(other[0], max_abs_diff)
                    && self[1].abs_diff_eq(other[1], max_abs_diff)
                    && self[2].abs_diff_eq(other[2], max_abs_diff)
                    && self[3].abs_diff_eq(other[3], max_abs_diff)
            }
            _ => unreachable!(),
        }
    }
}

impl<T, A: Alignment> Matrix<2, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a matrix containing a rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<2, T, A>::new(cos, sin),
            Vector::<2, T, A>::new(-sin, cos),
        ])
    }

    /// Creates a matrix containing the non-uniform `scale` and a rotation of
    /// `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<2, T, A>::new(cos * scale.x, sin * scale.x),
            Vector::<2, T, A>::new(-sin * scale.y, cos * scale.y),
        ])
    }

    /// Returns the `scale` and `angle` of `self`.
    ///
    /// `self` must not contain shearing. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle(&self) -> (Vector<2, T, A>, T) {
        let determinant = self.determinant();

        debug_assert!(determinant != T::ZERO);

        let scale = Vector::<2, T, A>::new(
            self.x_axis.length() * determinant.signum(),
            self.y_axis.length(),
        );

        let angle = PrimitiveFloatUtils::atan2(-self.y_axis.x, self.y_axis.y);

        (scale, angle)
    }
}

impl<T, A: Alignment> Matrix<3, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates an affine transformation matrix containing a rotation of `angle`
    /// (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos, sin, T::as_from(0.0)),
            Vector::<3, T, A>::new(-sin, cos, T::as_from(0.0)),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Creates an affine transformation matrix containing the non-uniform
    /// `scale` and a rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos * scale.x, sin * scale.x, T::as_from(0.0)),
            Vector::<3, T, A>::new(-sin * scale.y, cos * scale.y, T::as_from(0.0)),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Creates an affine transformation matrix containing a rotation of `angle`
    /// (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub fn from_angle_translation(angle: T, translation: Vector<2, T, A>) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos, sin, T::ZERO),
            Vector::<3, T, A>::new(-sin, cos, T::ZERO),
            Vector::<3, T, A>::new(translation.x, translation.y, T::ONE),
        ])
    }

    /// Creates an affine transformation matrix containing the non-uniform
    /// `scale`, a rotation of `angle` (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub fn from_scale_angle_translation(
        scale: Vector<2, T, A>,
        angle: T,
        translation: Vector<2, T, A>,
    ) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos * scale.x, sin * scale.x, T::as_from(0.0)),
            Vector::<3, T, A>::new(-sin * scale.y, cos * scale.y, T::as_from(0.0)),
            Vector::<3, T, A>::new(translation.x, translation.y, T::as_from(1.0)),
        ])
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the x
    /// axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::X,
            Vector::<3, T, A>::new(T::as_from(0.0), cos, sin),
            Vector::<3, T, A>::new(T::as_from(0.0), -sin, cos),
        ])
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the y
    /// axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos, T::as_from(0.0), -sin),
            Vector::<3, T, A>::Y,
            Vector::<3, T, A>::new(sin, T::as_from(0.0), cos),
        ])
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the z
    /// axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos, sin, T::as_from(0.0)),
            Vector::<3, T, A>::new(-sin, cos, T::as_from(0.0)),
            Vector::<3, T, A>::Z,
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn quat_to_axes(quat: Quaternion<T, A>) -> [Vector<3, T, A>; 3] {
        debug_assert!(quat.to_vector().is_normalized());

        let x2 = quat.x + quat.x;
        let y2 = quat.y + quat.y;
        let z2 = quat.z + quat.z;
        let xx2 = quat.x * x2;
        let xy2 = quat.x * y2;
        let xz2 = quat.x * z2;
        let yy2 = quat.y * y2;
        let yz2 = quat.y * z2;
        let zz2 = quat.z * z2;
        let wx2 = quat.w * x2;
        let wy2 = quat.w * y2;
        let wz2 = quat.w * z2;

        [
            Vector::<3, T, A>::new(T::ONE - (yy2 + zz2), xy2 + wz2, xz2 - wy2),
            Vector::<3, T, A>::new(xy2 - wz2, T::ONE - (xx2 + zz2), yz2 + wx2),
            Vector::<3, T, A>::new(xz2 + wy2, yz2 - wx2, T::ONE - (xx2 + yy2)),
        ]
    }

    /// Creates a 3D rotation matrix from a quaternion.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the quaternion is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_quat(quat: Quaternion<T, A>) -> Self {
        let [x_axis, y_axis, z_axis] = Self::quat_to_axes(quat);
        Self::from_rows(&[x_axis, y_axis, z_axis])
    }

    /// Creates a 3D rotation matrix from a rotation `axis` and `angle` (in
    /// radians).
    ///
    /// `axis` must be normalized. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `axis` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(axis: Vector<3, T, A>, angle: T) -> Self {
        debug_assert!(axis.is_normalized());

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        let [xsin, ysin, zsin] = (axis * sin).to_array();
        let [x, y, z] = axis.to_array();
        let [x2, y2, z2] = (axis * axis).to_array();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;

        Self::from_rows(&[
            Vector::<3, T, A>::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            Vector::<3, T, A>::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            Vector::<3, T, A>::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
        ])
    }

    /// Creates a 3D rotation matrix from an Euler rotation order/sequence and
    /// angles (in radians).
    #[inline]
    #[must_use]
    pub fn from_euler(order: EulerRot, a: T, b: T, c: T) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs.

        // Based on Ken Shoemake. 1994. Euler angle conversion. Graphics gems IV.
        // Academic Press Professional, Inc., USA, 222–229.

        let order = order.properties();
        let (i, j, k) = order.axes_indices();

        let mut angles = if order.frame_static {
            Vector::<3, T, A>::new(a, b, c)
        } else {
            Vector::<3, T, A>::new(c, b, a)
        };

        // Rotation direction is reverse from original paper.
        if order.parity_even {
            angles = -angles;
        }

        let (si, ci) = PrimitiveFloatUtils::sin_cos(angles.x);
        let (sj, cj) = PrimitiveFloatUtils::sin_cos(angles.y);
        let (sh, ch) = PrimitiveFloatUtils::sin_cos(angles.z);

        let cc = ci * ch;
        let cs = ci * sh;
        let sc = si * ch;
        let ss = si * sh;

        let mut result = Self::ZERO;

        if order.initial_repeated {
            result[i][i] = cj;
            result[i][j] = sj * si;
            result[i][k] = sj * ci;
            result[j][i] = sj * sh;
            result[j][j] = -cj * ss + cc;
            result[j][k] = -cj * cs - sc;
            result[k][i] = -sj * ch;
            result[k][j] = cj * sc + cs;
            result[k][k] = cj * cc - ss;
        } else {
            result[i][i] = cj * ch;
            result[i][j] = sj * sc - cs;
            result[i][k] = sj * cc + ss;
            result[j][i] = cj * sh;
            result[j][j] = sj * ss + cc;
            result[j][k] = sj * cs - sc;
            result[k][i] = -sj;
            result[k][j] = cj * si;
            result[k][k] = cj * ci;
        }

        result
    }

    /// Creates a matrix containing a non-uniform `scale` and a 3D `rotation`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_scale_rotation(scale: Vector<3, T, A>, rotation: Quaternion<T, A>) -> Self {
        let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
        Self::from_rows(&[
            rotation_x * scale.x,
            rotation_y * scale.y,
            rotation_z * scale.z,
        ])
    }

    /// Creates a left-handed view matrix from a facing direction and an up
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `dir` or `up` are not normalized
    /// - `dir` and `up` are parallel
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_lh(dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(dir.is_normalized());
        debug_assert!(up.is_normalized());

        let forward = dir;
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, forward.x),
            Vector::<3, T, A>::new(right.y, up.y, forward.y),
            Vector::<3, T, A>::new(right.z, up.z, forward.z),
        ])
    }

    /// Creates a right-handed view matrix from a facing direction and an up
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `dir` or `up` are not normalized
    /// - `dir` and `up` are parallel
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_rh(dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(dir.is_normalized());
        debug_assert!(up.is_normalized());

        let forward = dir;
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, -forward.x),
            Vector::<3, T, A>::new(right.y, up.y, -forward.y),
            Vector::<3, T, A>::new(right.z, up.z, -forward.z),
        ])
    }

    /// Creates a left-handed view matrix from a camera position, a focal point
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `up` is not normalized
    /// - `center` is equal to `eye`
    /// - The resulting forward direction is parallel to `up`
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_lh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(up.is_normalized());

        let forward = (center - eye).normalize();
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, forward.x),
            Vector::<3, T, A>::new(right.y, up.y, forward.y),
            Vector::<3, T, A>::new(right.z, up.z, forward.z),
        ])
    }

    /// Creates a right-handed view matrix from a camera position, a focal point
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `up` is not normalized
    /// - `center` is equal to `eye`
    /// - The resulting forward direction is parallel to `up`
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_rh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(up.is_normalized());

        let forward = (center - eye).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, -forward.x),
            Vector::<3, T, A>::new(right.y, up.y, -forward.y),
            Vector::<3, T, A>::new(right.z, up.z, -forward.z),
        ])
    }

    /// Returns the `scale` and `angle` of `self`.
    ///
    /// `self` must contain a valid affine transformation without shearing.
    /// Otherwise the result is unspecified.
    ///
    /// `self` can contain translation which is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle(&self) -> (Vector<2, T, A>, T) {
        self.submatrix().to_scale_angle()
    }

    /// Returns the `scale`, `angle` and `translation` of `self`.
    ///
    /// `self` must contain a valid affine transformation without shearing.
    /// Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle_translation(&self) -> (Vector<2, T, A>, T, Vector<2, T, A>) {
        let (scale, angle) = self.to_scale_angle();
        (scale, angle, self.translation())
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// `self` must not contain any non-rotation transformations. Otherwise the
    /// result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if any column of `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(&self, order: EulerRot) -> (T, T, T) {
        // Ported from https://github.com/bitshifter/glam-rs.

        // Based on Ken Shoemake. 1994. Euler angle conversion. Graphics gems IV.
        // Academic Press Professional, Inc., USA, 222–229.

        debug_assert!(
            self.x_axis.is_normalized()
                && self.y_axis.is_normalized()
                && self.z_axis.is_normalized()
        );

        let order = order.properties();
        let (i, j, k) = order.axes_indices();

        let mut ea = Vector::<3, T, A>::ZERO;
        if order.initial_repeated {
            let sy = PrimitiveFloatUtils::sqrt(self[i][j] * self[i][j] + self[i][k] * self[i][k]);

            if sy > T::as_from(16.0) * T::EPSILON {
                ea.x = PrimitiveFloatUtils::atan2(self[i][j], self[i][k]);
                ea.y = PrimitiveFloatUtils::atan2(sy, self[i][i]);
                ea.z = PrimitiveFloatUtils::atan2(self[j][i], -self[k][i]);
            } else {
                ea.x = PrimitiveFloatUtils::atan2(-self[j][k], self[j][j]);
                ea.y = PrimitiveFloatUtils::atan2(sy, self[i][i]);
            }
        } else {
            let cy = PrimitiveFloatUtils::sqrt(self[i][i] * self[i][i] + self[j][i] * self[j][i]);

            if cy > T::as_from(16.0) * T::EPSILON {
                ea.x = PrimitiveFloatUtils::atan2(self[k][j], self[k][k]);
                ea.y = PrimitiveFloatUtils::atan2(-self[k][i], cy);
                ea.z = PrimitiveFloatUtils::atan2(self[j][i], self[i][i]);
            } else {
                ea.x = PrimitiveFloatUtils::atan2(-self[j][k], self[j][j]);
                ea.y = PrimitiveFloatUtils::atan2(-self[k][i], cy);
            }
        }

        // Reverse rotation angle of original code.
        if order.parity_even {
            ea = -ea;
        }

        if !order.frame_static {
            ea = ea.zyx();
        }

        (ea.x, ea.y, ea.z)
    }

    /// Returns the `scale` and `rotation` of `self`.
    ///
    /// `self` must not contain shearing. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation(&self) -> (Vector<3, T, A>, Quaternion<T, A>) {
        let determinant = self.determinant();

        debug_assert!(determinant != T::ZERO);

        let scale = Vector::<3, T, A>::new(
            self.x_axis.length() * determinant.signum(),
            self.y_axis.length(),
            self.z_axis.length(),
        );

        let scale_recip = scale.recip();

        let rotation = Quaternion::<T, A>::from_matrix(&Self::from_rows(&[
            self.x_axis * scale_recip.x,
            self.y_axis * scale_recip.y,
            self.z_axis * scale_recip.z,
        ]));

        (scale, rotation)
    }

    /// Transforms the given 2D vector as a point.
    ///
    /// Equivalent to `(point, 1) * self` but is faster.
    ///
    /// `self` must contain a valid affine transform, meaning the third column
    /// must be `(0, 0, 1)`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the third column of `self` is not `(0, 0, 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_point(&self, point: Vector<2, T, A>) -> Vector<2, T, A> {
        debug_assert!(self.column(2) == Vector::<3, T, A>::Z);

        self.x_axis.xy() * point.x + self.y_axis.xy() * point.y + self.z_axis.xy()
    }

    /// Transforms the given 2D vector without applying translation.
    ///
    /// Equivalent to `(vector, 0) * self` but is faster.
    ///
    /// `self` must contain a valid affine transform, meaning the third column
    /// must be `(0, 0, 1)`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the third column of `self` is not `(0, 0, 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_vector(&self, vector: Vector<2, T, A>) -> Vector<2, T, A> {
        debug_assert!(self.column(2) == Vector::<3, T, A>::Z);

        self.x_axis.xy() * vector.x + self.y_axis.xy() * vector.y
    }
}

impl<T, A: Alignment> Matrix<4, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates an affine transformation matrix containing a 3D rotation from
    /// `angle` (in radians) around the x axis.
    ///
    /// This rotates `+Y` to `+Z`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
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
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
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
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<4, T, A>::new(cos, sin, T::as_from(0.0), T::as_from(0.0)),
            Vector::<4, T, A>::new(-sin, cos, T::as_from(0.0), T::as_from(0.0)),
            Vector::<4, T, A>::Z,
            Vector::<4, T, A>::W,
        ])
    }

    #[inline(always)]
    #[track_caller]
    fn quat_to_axes(quat: Quaternion<T, A>) -> [Vector<4, T, A>; 3] {
        debug_assert!(quat.to_vector().is_normalized());

        let x2 = quat.x + quat.x;
        let y2 = quat.y + quat.y;
        let z2 = quat.z + quat.z;
        let xx2 = quat.x * x2;
        let xy2 = quat.x * y2;
        let xz2 = quat.x * z2;
        let yy2 = quat.y * y2;
        let yz2 = quat.y * z2;
        let zz2 = quat.z * z2;
        let wx2 = quat.w * x2;
        let wy2 = quat.w * y2;
        let wz2 = quat.w * z2;

        [
            Vector::<4, T, A>::new(T::ONE - (yy2 + zz2), xy2 + wz2, xz2 - wy2, T::ZERO),
            Vector::<4, T, A>::new(xy2 - wz2, T::ONE - (xx2 + zz2), yz2 + wx2, T::ZERO),
            Vector::<4, T, A>::new(xz2 + wy2, yz2 - wx2, T::ONE - (xx2 + yy2), T::ZERO),
        ]
    }

    /// Creates an affine transformation matrix containing a 3D rotation from a
    /// quaternion.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the quaternion is not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_quat(quat: Quaternion<T, A>) -> Self {
        let [x_axis, y_axis, z_axis] = Self::quat_to_axes(quat);
        Self::from_rows(&[x_axis, y_axis, z_axis, Vector::W])
    }

    /// Creates an affine transformation matrix containing a rotation from a
    /// rotation `axis` and `angle` (in radians).
    ///
    /// `axis` must be normalized. Otherwise the result is unspecified.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `axis` is not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(axis: Vector<3, T, A>, angle: T) -> Self {
        debug_assert!(axis.is_normalized());

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        let [xsin, ysin, zsin] = (axis * sin).to_array();
        let [x, y, z] = axis.to_array();
        let [x2, y2, z2] = (axis * axis).to_array();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;

        Self::from_rows(&[
            Vector::<4, T, A>::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin, T::ZERO),
            Vector::<4, T, A>::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin, T::ZERO),
            Vector::<4, T, A>::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos, T::ZERO),
            Vector::W,
        ])
    }

    /// Creates an affine transformation matrix containing a rotation from an
    /// Euler rotation order/sequence and angles (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub fn from_euler(order: EulerRot, a: T, b: T, c: T) -> Self {
        Self::from_submatrix(&Matrix::<3, T, A>::from_euler(order, a, b, c))
    }

    /// Creates an affine transformation matrix containing a non-uniform `scale`
    /// and a 3D `rotation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_scale_rotation(scale: Vector<3, T, A>, rotation: Quaternion<T, A>) -> Self {
        let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
        Self::from_rows(&[
            rotation_x * scale.x,
            rotation_y * scale.y,
            rotation_z * scale.z,
            Vector::W,
        ])
    }

    /// Creates an affine transformation matrix containing a 3D `rotation` and
    /// `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_translation(
        rotation: Quaternion<T, A>,
        translation: Vector<3, T, A>,
    ) -> Self {
        let [x_axis, y_axis, z_axis] = Self::quat_to_axes(rotation);
        Self::from_rows(&[
            x_axis,
            y_axis,
            z_axis,
            Vector::<4, T, A>::new(translation.x, translation.y, translation.z, T::ONE),
        ])
    }

    /// Creates an affine transformation matrix containing the non-uniform
    /// `scale`, a 3D `rotation` and `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_scale_rotation_translation(
        scale: Vector<3, T, A>,
        rotation: Quaternion<T, A>,
        translation: Vector<3, T, A>,
    ) -> Self {
        let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
        Self::from_rows(&[
            rotation_x * scale.x,
            rotation_y * scale.y,
            rotation_z * scale.z,
            Vector::<4, T, A>::new(translation.x, translation.y, translation.z, T::ONE),
        ])
    }

    /// Creates a left-handed view matrix from a camera position, a facing
    /// direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `dir` or `up` are not normalized
    /// - `dir` and `up` are parallel
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_lh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(dir.is_normalized());
        debug_assert!(up.is_normalized());

        let forward = dir;
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<4, T, A>::new(right.x, up.x, forward.x, T::ZERO),
            Vector::<4, T, A>::new(right.y, up.y, forward.y, T::ZERO),
            Vector::<4, T, A>::new(right.z, up.z, forward.z, T::ZERO),
            Vector::<4, T, A>::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward), T::ONE),
        ])
    }

    /// Creates a right-handed view matrix from a camera position, a facing
    /// direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `dir` or `up` are not normalized
    /// - `dir` and `up` are parallel
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_rh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(dir.is_normalized());
        debug_assert!(up.is_normalized());

        let forward = dir;
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<4, T, A>::new(right.x, up.x, -forward.x, T::ZERO),
            Vector::<4, T, A>::new(right.y, up.y, -forward.y, T::ZERO),
            Vector::<4, T, A>::new(right.z, up.z, -forward.z, T::ZERO),
            Vector::<4, T, A>::new(-eye.dot(right), -eye.dot(up), eye.dot(forward), T::ONE),
        ])
    }

    /// Creates a left-handed view matrix from a camera position, a focal point
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `up` is not normalized
    /// - `center` is equal to `eye`
    /// - The resulting forward direction is parallel to `up`
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_lh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(up.is_normalized());

        let forward = (center - eye).normalize();
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<4, T, A>::new(right.x, up.x, forward.x, T::ZERO),
            Vector::<4, T, A>::new(right.y, up.y, forward.y, T::ZERO),
            Vector::<4, T, A>::new(right.z, up.z, forward.z, T::ZERO),
            Vector::<4, T, A>::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward), T::ONE),
        ])
    }

    /// Creates a right-handed view matrix from a camera position, a focal point
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `up` is not normalized
    /// - `center` is equal to `eye`
    /// - The resulting forward direction is parallel to `up`
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_rh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(up.is_normalized());

        let forward = (center - eye).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<4, T, A>::new(right.x, up.x, -forward.x, T::ZERO),
            Vector::<4, T, A>::new(right.y, up.y, -forward.y, T::ZERO),
            Vector::<4, T, A>::new(right.z, up.z, -forward.z, T::ZERO),
            Vector::<4, T, A>::new(-eye.dot(right), -eye.dot(up), eye.dot(forward), T::ONE),
        ])
    }

    /// Creates a left-handed perspective projection matrix with `0..1` depth
    /// range.
    ///
    /// Useful to map the standard left-handed coordinate system into what
    /// WebGPU/Metal/Direct3D expect.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_lh(vertical_fov: T, aspect_ratio: T, near_plane: T, far_plane: T) -> Self {
        debug_assert!(near_plane > T::ZERO && far_plane > near_plane);

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(vertical_fov * T::as_from(0.5));
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let depth_scale = far_plane / (far_plane - near_plane);

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, depth_scale, T::ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, -depth_scale * near_plane, T::ZERO),
        ])
    }

    /// Creates a right-handed perspective projection matrix with `0..1` depth
    /// range.
    ///
    /// Useful to map the standard right-handed coordinate system into what
    /// WebGPU/Metal/Direct3D expect.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_rh(vertical_fov: T, aspect_ratio: T, near_plane: T, far_plane: T) -> Self {
        debug_assert!(near_plane > T::ZERO && far_plane > near_plane);

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(vertical_fov * T::as_from(0.5));
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let neg_depth_scale = far_plane / (near_plane - far_plane);

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, neg_depth_scale, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, neg_depth_scale * near_plane, T::ZERO),
        ])
    }

    /// Creates a right-handed perspective projection matrix with `-1..1` depth
    /// range.
    ///
    /// Equivalent to the OpenGL [`gluPerspective`] function.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`gluPerspective`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_rh_gl(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
        far_plane: T,
    ) -> Self {
        debug_assert!(near_plane > T::ZERO && far_plane > near_plane);

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(vertical_fov * T::as_from(0.5));
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let depth_recip = (near_plane - far_plane).recip();

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(
                T::ZERO,
                T::ZERO,
                (near_plane + far_plane) * depth_recip,
                T::NEG_ONE,
            ),
            Vector::<4, T, A>::new(
                T::ZERO,
                T::ZERO,
                T::as_from(2.0) * near_plane * far_plane * depth_recip,
                T::ZERO,
            ),
        ])
    }

    /// Creates an infinite left-handed perspective projection matrix with
    /// `0..1` depth range.
    ///
    /// Equivalent to `perspective_lh`, but with an infinite value for
    /// `far_plane`. The result is that points near `near_plane` have depth `0`,
    /// and as they move towards infinity the depth approaches `1`.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_lh(vertical_fov: T, aspect_ratio: T, near_plane: T) -> Self {
        debug_assert!(near_plane > T::ZERO);

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(vertical_fov * T::as_from(0.5));
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ONE, T::ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, -near_plane, T::ZERO),
        ])
    }

    /// Creates an infinite right-handed perspective projection matrix with
    /// `0..1` depth range.
    ///
    /// Equivalent to `perspective_rh`, but with an infinite value for
    /// `far_plane`. The result is that points near `near_plane` have depth `0`,
    /// and as they move towards infinity the depth approaches `1`.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_rh(vertical_fov: T, aspect_ratio: T, near_plane: T) -> Self {
        debug_assert!(near_plane > T::ZERO);

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(vertical_fov * T::as_from(0.5));
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::NEG_ONE, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, -near_plane, T::ZERO),
        ])
    }

    /// Creates an infinite left-handed perspective projection matrix with
    /// reversed `0..1` depth range.
    ///
    /// Equivalent to `perspective_infinite_lh`, but maps points at `near_plane`
    /// to depth `1` and points at infinity to depth `0`.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_reverse_lh(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
    ) -> Self {
        debug_assert!(near_plane > T::ZERO);

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(vertical_fov * T::as_from(0.5));
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, near_plane, T::ZERO),
        ])
    }

    /// Creates an infinite right-handed perspective projection matrix with
    /// reversed `0..1` depth range.
    ///
    /// Equivalent to `perspective_infinite_rh`, but maps points at `near_plane`
    /// to depth `1` and points at infinity to depth `0`.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_reverse_rh(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
    ) -> Self {
        debug_assert!(near_plane > T::ZERO);

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(vertical_fov * T::as_from(0.5));
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, near_plane, T::ZERO),
        ])
    }

    /// Creates a left-handed perspective projection matrix with `0..1` depth
    /// range.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn frustum_lh(left: T, right: T, bottom: T, top: T, near_plane: T, far_plane: T) -> Self {
        debug_assert!(near_plane > T::ZERO && far_plane > near_plane);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = far_plane * depth_recip;
        let d = -near_plane * c;

        Self::from_rows(&[
            Vector::<4, T, A>::new(two_near_plane * width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, two_near_plane * height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(a, b, c, T::ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, d, T::ZERO),
        ])
    }

    /// Creates a right-handed perspective projection matrix with `0..1` depth
    /// range.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn frustum_rh(left: T, right: T, bottom: T, top: T, near_plane: T, far_plane: T) -> Self {
        debug_assert!(near_plane > T::ZERO && far_plane > near_plane);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = -far_plane * depth_recip;
        let d = near_plane * c;

        Self::from_rows(&[
            Vector::<4, T, A>::new(two_near_plane * width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, two_near_plane * height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(a, b, c, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, d, T::ZERO),
        ])
    }

    /// Creates a right-handed perspective projection matrix with `-1..1` depth
    /// range.
    ///
    /// Equivalent to the OpenGL [`glFrustum`] function.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`glFrustum`]: https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn frustum_rh_gl(
        left: T,
        right: T,
        bottom: T,
        top: T,
        near_plane: T,
        far_plane: T,
    ) -> Self {
        debug_assert!(near_plane > T::ZERO && far_plane > near_plane);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = -(far_plane + near_plane) * depth_recip;
        let d = -two_near_plane * far_plane * depth_recip;

        Self::from_rows(&[
            Vector::<4, T, A>::new(two_near_plane * width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, two_near_plane * height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(a, b, c, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, d, T::ZERO),
        ])
    }

    /// Creates a left-handed orthographic projection matrix with `0..1` depth
    /// range.
    ///
    /// Useful to map a left-handed coordinate system into what
    /// WebGPU/Metal/Direct3D expect.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `far` is less than or equal to `near`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_lh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        debug_assert!(far > near);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far - near).recip();

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip + width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip + height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, depth_recip, T::ZERO),
            Vector::<4, T, A>::new(
                -(left + right) * width_recip,
                -(top + bottom) * height_recip,
                -depth_recip * near,
                T::ONE,
            ),
        ])
    }

    /// Creates a right-handed orthographic projection matrix with `0..1` depth
    /// range.
    ///
    /// Useful to map a right-handed coordinate system into what
    /// WebGPU/Metal/Direct3D expect.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `far` is less than or equal to `near`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_rh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        debug_assert!(far > near);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let neg_depth_recip = (near - far).recip();

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip + width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip + height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, neg_depth_recip, T::ZERO),
            Vector::<4, T, A>::new(
                -(left + right) * width_recip,
                -(top + bottom) * height_recip,
                neg_depth_recip * near,
                T::ONE,
            ),
        ])
    }

    /// Creates a right-handed orthographic projection matrix with `-1..1` depth
    /// range.
    ///
    /// Equivalent to the OpenGL [`glOrtho`] function.
    ///
    /// The resulting matrix can be used to transform 3D points using [`project_point`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `far` is less than or equal to `near`.
    ///
    /// [`glOrtho`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_rh_gl(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        debug_assert!(far > near);

        let scale_x = T::as_from(2.0) / (right - left);
        let scale_y = T::as_from(2.0) / (top - bottom);
        let scale_z = T::as_from(2.0) / (near - far);
        let translation_x = -(right + left) / (right - left);
        let translation_y = -(top + bottom) / (top - bottom);
        let translation_z = -(far + near) / (far - near);

        Self::from_rows(&[
            Vector::<4, T, A>::new(scale_x, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, scale_y, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, scale_z, T::ZERO),
            Vector::<4, T, A>::new(translation_x, translation_y, translation_z, T::ONE),
        ])
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// The upper 3x3 matrix must not contain any non-rotation transformations.
    /// Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if any column of the upper 3x3 matrix is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(&self, order: EulerRot) -> (T, T, T) {
        self.submatrix().to_euler(order)
    }

    /// Returns the `scale` and `rotation` of `self`.
    ///
    /// `self` must contain a valid affine transformation. Otherwise the result
    /// is unspecified.
    ///
    /// `self` can contain translation which is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation(&self) -> (Vector<3, T, A>, Quaternion<T, A>) {
        self.submatrix().to_scale_rotation()
    }

    /// Returns the `scale`, `rotation` and `translation` of `self`.
    ///
    /// `self` must contain a valid affine transformation. Otherwise the result
    /// is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation_translation(
        &self,
    ) -> (Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>) {
        let (scale, rotation) = self.to_scale_rotation();
        (scale, rotation, self.translation())
    }

    /// Transforms the given 3D vector as a point.
    ///
    /// Equivalent to `(point, 1) * self` but is faster. This does not perform a
    /// perspective divide.
    ///
    /// `self` must contain a valid affine transform, meaning the fourth column
    /// must be `(0, 0, 0, 1)`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the fourth column of `self` is not `(0, 0, 0, 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_point(&self, point: Vector<3, T, A>) -> Vector<3, T, A> {
        debug_assert!(self.column(3) == Vector::<4, T, A>::W);

        self.x_axis.xyz() * point.x
            + self.y_axis.xyz() * point.y
            + self.z_axis.xyz() * point.z
            + self.w_axis.xyz()
    }

    /// Transforms the given 3D vector without applying translation.
    ///
    /// Equivalent to `(vector, 0) * self` but is faster.
    ///
    /// `self` must contain a valid affine transform, meaning the fourth column
    /// must be `(0, 0, 0, 1)`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the fourth column of `self` is not `(0, 0, 0, 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_vector(&self, vector: Vector<3, T, A>) -> Vector<3, T, A> {
        debug_assert!(self.column(3) == Vector::<4, T, A>::W);

        self.x_axis.xyz() * vector.x + self.y_axis.xyz() * vector.y + self.z_axis.xyz() * vector.z
    }

    /// Transforms the given 3D vector as a point, applying perspective
    /// projection.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// let result = matrix * (point, 1);
    /// result.xyz / result.w
    /// ```
    #[inline]
    #[must_use]
    pub fn project_point(&self, point: Vector<3, T, A>) -> Vector<3, T, A> {
        let result =
            self.x_axis * point.x + self.y_axis * point.y + self.z_axis * point.z + self.w_axis;

        (result / result.w).xyz()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        EulerRot, Mat3A, Mat4A, Matrix, Quaternion, Vec2A, Vec3A, Vec4A, Vector,
        utils::{assert_debug_panic, assert_panic_test_eq, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, T: PrimitiveFloat, A| {
            assert_test_eq!(
                Matrix::<N, T, A>::NAN,
                Matrix::from_rows(&[Vector::<N, T, A>::NAN; N])
            );
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let nan = Vector::<2, T, A>::NAN;
            assert!(!Matrix::<2, T, A>::from_rows(&[one; 2]).is_nan());
            assert!(Matrix::<2, T, A>::from_rows(&[nan, one]).is_nan());
            assert!(Matrix::<2, T, A>::from_rows(&[one, nan]).is_nan());
            assert!(Matrix::<2, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::<3, T, A>::NAN;
            assert!(!Matrix::<3, T, A>::from_rows(&[one; 3]).is_nan());
            assert!(Matrix::<3, T, A>::from_rows(&[nan, one, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_rows(&[one, nan, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_rows(&[one, one, nan]).is_nan());
            assert!(Matrix::<3, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::<4, T, A>::NAN;
            assert!(!Matrix::<4, T, A>::from_rows(&[one; 4]).is_nan());
            assert!(Matrix::<4, T, A>::from_rows(&[nan, one, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_rows(&[one, nan, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_rows(&[one, one, nan, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_rows(&[one, one, one, nan]).is_nan());
            assert!(Matrix::<4, T, A>::NAN.is_nan());
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let inf = Vector::<2, T, A>::INFINITY;
            assert!(Matrix::<2, T, A>::from_rows(&[one, one]).is_finite());
            assert!(!Matrix::<2, T, A>::from_rows(&[inf, one]).is_finite());
            assert!(!Matrix::<2, T, A>::from_rows(&[one, inf]).is_finite());
            assert!(!Matrix::<2, T, A>::from_rows(&[inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::<3, T, A>::INFINITY;
            assert!(Matrix::<3, T, A>::from_rows(&[one, one, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_rows(&[inf, one, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_rows(&[one, inf, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_rows(&[one, one, inf]).is_finite());
            assert!(!Matrix::<3, T, A>::from_rows(&[inf, inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::<4, T, A>::INFINITY;
            assert!(Matrix::<4, T, A>::from_rows(&[one, one, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[inf, one, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[one, inf, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[one, one, inf, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[one, one, one, inf]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[inf, inf, inf, inf]).is_finite());
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                if matrix.determinant() == 0.0 {
                    assert_debug_panic!(matrix.inverse());
                }

                if !matrix.is_finite()
                    || matrix.as_rows().iter().flatten().any(|x| x.abs() > 1e6)
                    || !(1e-2..=1e2).contains(
                        &(matrix.determinant()
                            / matrix
                                .as_rows()
                                .iter()
                                .flatten()
                                .reduce(T::max)
                                .unwrap()
                                .powi(N as i32)),
                    )
                {
                    continue;
                }

                assert_test_eq!(
                    matrix.inverse() * matrix,
                    Matrix::IDENTITY,
                    abs <= matrix
                        .determinant()
                        .abs()
                        .max(matrix.determinant().recip().abs())
                        * 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_try_inverse() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                let Some(try_inverse) = matrix.try_inverse() else {
                    assert_debug_panic!(matrix.inverse());
                    continue;
                };

                assert_test_eq!(try_inverse, matrix.inverse())
            }
        });
    }

    #[test]
    fn test_inverse_or() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [matrix, fallback] in random_iter::<[Matrix<N, T, A>; 2]>() {
                let Some(inverse) = matrix.try_inverse() else {
                    assert_test_eq!(matrix.inverse_or(&fallback), fallback);
                    continue;
                };

                assert_test_eq!(matrix.inverse_or(&fallback), inverse);
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                assert_test_eq!(matrix.inverse_or_zero(), matrix.inverse_or(&Matrix::ZERO));
            }
        });
    }

    #[test]
    fn test_recip() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                assert_test_eq!(matrix.recip(), Matrix::from_row_fn(|r| matrix[r].recip()));
            }
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                assert_test_eq!(matrix.abs(), Matrix::from_row_fn(|r| matrix[r].abs()));
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|T: PrimitiveFloat, A| {
            let x_axis = Vector::<2, T, A>::new(0.0, 1.0);
            let y_axis = Vector::<2, T, A>::new(2.0, 3.0);
            assert!(
                Matrix::<2, T, A>::from_rows(&[x_axis, y_axis])
                    .abs_diff_eq(&Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1]), 0.125)
            );
            assert!(
                !Matrix::<2, T, A>::from_rows(&[x_axis, y_axis])
                    .abs_diff_eq(&Matrix::from_rows(&[x_axis + 0.5, y_axis - 0.1]), 0.125)
            );
            assert!(
                !Matrix::<2, T, A>::from_rows(&[x_axis, y_axis])
                    .abs_diff_eq(&Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.5]), 0.125)
            );

            let x_axis = Vector::<3, T, A>::new(0.0, 1.0, 2.0);
            let y_axis = Vector::<3, T, A>::new(3.0, 4.0, 5.0);
            let z_axis = Vector::<3, T, A>::new(6.0, 7.0, 8.0);
            assert!(
                Matrix::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.5, y_axis - 0.1, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.5, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.5]),
                    0.125
                )
            );

            let x_axis = Vector::<4, T, A>::new(0.0, 1.0, 2.0, 3.0);
            let y_axis = Vector::<4, T, A>::new(4.0, 5.0, 6.0, 7.0);
            let z_axis = Vector::<4, T, A>::new(8.0, 9.0, 10.0, 11.0);
            let w_axis = Vector::<4, T, A>::new(12.0, 13.0, 14.0, 15.0);
            assert!(
                Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.05, w_axis - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.5, y_axis - 0.1, z_axis + 0.05, w_axis - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.5, z_axis + 0.05, w_axis - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.5, w_axis - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.05, w_axis - 0.5]),
                    0.125
                )
            );
        });
    }

    #[test]
    fn test_from_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                assert_test_eq!(
                    vector * Matrix::<2, T, A>::from_angle(angle),
                    vector.rotate(angle)
                );
                assert_test_eq!(
                    Matrix::<3, T, A>::from_angle(angle).transform_point(vector),
                    vector.rotate(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                if !scale.is_finite() || !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<2, T, A>::from_scale_angle(scale, angle),
                    Matrix::<2, T, A>::from_diagonal(scale) * Matrix::<2, T, A>::from_angle(angle),
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<3, T, A>::from_scale_angle(scale, angle),
                    Matrix::<3, T, A>::from_scale(scale) * Matrix::<3, T, A>::from_angle(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_debug_panic!(Matrix::<2, T, A>::ZERO.to_scale_angle());
            assert_debug_panic!(Matrix::<3, T, A>::ZERO.to_scale_angle());

            for (scale, angle, translation) in
                random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
            {
                let matrix = Matrix::<2, T, A>::from_scale_angle(scale, angle);

                assert_panic_test_eq!(
                    Matrix::<3, T, A>::from_submatrix_translation(&matrix, translation)
                        .to_scale_angle(),
                    matrix.to_scale_angle()
                );

                if scale.iter().any(|x| x > 1e10)
                    || !matrix.determinant().is_finite()
                    || matrix.determinant() == 0.0
                {
                    continue;
                }

                let (result_scale, result_angle) = matrix.to_scale_angle();
                assert_test_eq!(
                    Matrix::<2, T, A>::from_scale_angle(result_scale, result_angle),
                    matrix,
                    abs <= scale.max_element() * 1e-5 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (angle, translation) in random_iter::<(T, Vector<2, T, A>)>() {
                if !angle.is_finite() || !translation.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<3, T, A>::from_angle_translation(angle, translation),
                    Matrix::<3, T, A>::from_angle(angle)
                        * Matrix::<3, T, A>::from_translation(translation),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, angle, translation) in
                random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
            {
                if !scale.is_finite() || !angle.is_finite() || !translation.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<3, T, A>::from_scale_angle_translation(scale, angle, translation),
                    Matrix::<3, T, A>::from_scale(scale)
                        * Matrix::<3, T, A>::from_angle(angle)
                        * Matrix::<3, T, A>::from_translation(translation),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                if !vector.is_finite() || !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    vector * Matrix::<3, T, A>::from_rotation_x(angle),
                    vector.rotate_x(angle),
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::from_rotation_x(angle).transform_point(vector),
                    vector.rotate_x(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                if !vector.is_finite() || !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    vector * Matrix::<3, T, A>::from_rotation_y(angle),
                    vector.rotate_y(angle),
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::from_rotation_y(angle).transform_point(vector),
                    vector.rotate_y(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                if !vector.is_finite() || !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    vector * Matrix::<3, T, A>::from_rotation_z(angle),
                    vector.rotate_z(angle),
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::from_rotation_z(angle).transform_point(vector),
                    vector.rotate_z(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_quat() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Matrix::<3, T, A>::from_quat(Quaternion::IDENTITY),
                Matrix::IDENTITY
            );
            assert_test_eq!(
                Matrix::<4, T, A>::from_quat(Quaternion::IDENTITY),
                Matrix::IDENTITY
            );

            for quat in random_iter::<Quaternion<T, A>>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::from_quat(quat));
                    assert_debug_panic!(Matrix::<4, T, A>::from_quat(quat));
                }

                let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();

                assert_test_eq!(
                    Matrix::<3, T, A>::from_quat(quat).determinant(),
                    1.0,
                    abs <= 1e-5
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::from_quat(quat).determinant(),
                    1.0,
                    abs <= 1e-5
                );

                let (axis, angle) = quat.to_axis_angle();
                assert_test_eq!(
                    Matrix::<3, T, A>::from_quat(quat),
                    Matrix::<3, T, A>::from_axis_angle(axis, angle),
                    abs <= 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::from_quat(quat),
                    Matrix::<4, T, A>::from_axis_angle(axis, angle),
                    abs <= 1e-5,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>() {
                if !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::X, angle),
                    Matrix::<3, T, A>::from_rotation_x(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::Y, angle),
                    Matrix::<3, T, A>::from_rotation_y(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::Z, angle),
                    Matrix::<3, T, A>::from_rotation_z(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );

                assert_test_eq!(
                    Matrix::<4, T, A>::from_axis_angle(Vector::<3, T, A>::X, angle),
                    Matrix::<4, T, A>::from_rotation_x(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::from_axis_angle(Vector::<3, T, A>::Y, angle),
                    Matrix::<4, T, A>::from_rotation_y(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::from_axis_angle(Vector::<3, T, A>::Z, angle),
                    Matrix::<4, T, A>::from_rotation_z(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for [x, y, z] in random_iter::<[T; 3]>() {
                let rot_x = Matrix::<3, T, A>::from_rotation_x(x);
                let rot_y = Matrix::<3, T, A>::from_rotation_y(y);
                let rot_z = Matrix::<3, T, A>::from_rotation_z(z);
                let rot_x_by_y = Matrix::<3, T, A>::from_rotation_x(y);
                let rot_x_by_z = Matrix::<3, T, A>::from_rotation_x(z);
                let rot_y_by_x = Matrix::<3, T, A>::from_rotation_y(x);
                let rot_y_by_z = Matrix::<3, T, A>::from_rotation_y(z);
                let rot_z_by_x = Matrix::<3, T, A>::from_rotation_z(x);
                let rot_z_by_y = Matrix::<3, T, A>::from_rotation_z(y);

                for (order, a, b, c, result) in [
                    (EulerRot::Xyz, x, y, z, rot_z * rot_y * rot_x),
                    (EulerRot::Xzy, x, z, y, rot_y * rot_z * rot_x),
                    (EulerRot::Yxz, y, x, z, rot_z * rot_x * rot_y),
                    (EulerRot::Yzx, y, z, x, rot_x * rot_z * rot_y),
                    (EulerRot::Zxy, z, x, y, rot_y * rot_x * rot_z),
                    (EulerRot::Zyx, z, y, x, rot_x * rot_y * rot_z),
                    (EulerRot::Xyx, x, y, z, rot_x_by_z * rot_y * rot_x),
                    (EulerRot::Xzx, x, z, y, rot_x_by_y * rot_z * rot_x),
                    (EulerRot::Yxy, y, x, z, rot_y_by_z * rot_x * rot_y),
                    (EulerRot::Yzy, y, z, x, rot_y_by_x * rot_z * rot_y),
                    (EulerRot::Zxz, z, x, y, rot_z_by_y * rot_x * rot_z),
                    (EulerRot::Zyz, z, y, x, rot_z_by_x * rot_y * rot_z),
                    (EulerRot::XyzEx, x, y, z, rot_x * rot_y * rot_z),
                    (EulerRot::XzyEx, x, z, y, rot_x * rot_z * rot_y),
                    (EulerRot::YxzEx, y, x, z, rot_y * rot_x * rot_z),
                    (EulerRot::YzxEx, y, z, x, rot_y * rot_z * rot_x),
                    (EulerRot::ZxyEx, z, x, y, rot_z * rot_x * rot_y),
                    (EulerRot::ZyxEx, z, y, x, rot_z * rot_y * rot_x),
                    (EulerRot::XyxEx, x, y, z, rot_x * rot_y * rot_x_by_z),
                    (EulerRot::XzxEx, x, z, y, rot_x * rot_z * rot_x_by_y),
                    (EulerRot::YxyEx, y, x, z, rot_y * rot_x * rot_y_by_z),
                    (EulerRot::YzyEx, y, z, x, rot_y * rot_z * rot_y_by_x),
                    (EulerRot::ZxzEx, z, x, y, rot_z * rot_x * rot_z_by_y),
                    (EulerRot::ZyzEx, z, y, x, rot_z * rot_y * rot_z_by_x),
                ] {
                    assert_test_eq!(
                        Matrix::<3, T, A>::from_euler(order, a, b, c),
                        result,
                        abs <= 1e-5,
                        0.0 = -0.0
                    );
                    assert_test_eq!(
                        Matrix::<4, T, A>::from_euler(order, a, b, c),
                        Matrix::<4, T, A>::from_submatrix(&result),
                        abs <= 1e-5,
                        0.0 = -0.0
                    );
                }
            }
        });
    }

    #[test]
    fn test_from_scale_rotation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation) in random_iter::<(Vector<3, T, A>, Quaternion<T, A>)>() {
                if !rotation.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::from_scale_rotation(scale, rotation));
                    assert_debug_panic!(Matrix::<4, T, A>::from_scale_rotation(scale, rotation));
                }

                let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();

                assert_test_eq!(
                    Matrix::<3, T, A>::from_scale_rotation(scale, rotation),
                    Matrix::<3, T, A>::from_diagonal(scale)
                        * Matrix::<3, T, A>::from_quat(rotation),
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::from_scale_rotation(scale, rotation),
                    Matrix::<4, T, A>::from_scale(scale) * Matrix::<4, T, A>::from_quat(rotation),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, dir, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                if !dir.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_lh(dir, up.normalize()));
                    assert_debug_panic!(Matrix::<4, T, A>::look_to_lh(eye, dir, up.normalize()));
                }
                if !up.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_lh(dir.normalize(), up));
                    assert_debug_panic!(Matrix::<4, T, A>::look_to_lh(eye, dir.normalize(), up));
                }

                let dir = dir.normalize_or(Vector::<3, T, A>::Z).normalize();
                let up = up.normalize_or(Vector::<3, T, A>::Y).normalize();
                if dir.cross(up).try_normalize().is_none() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_lh(dir, up));
                    assert_debug_panic!(Matrix::<4, T, A>::look_to_lh(eye, dir, up));
                    continue;
                }

                let result = Matrix::<3, T, A>::look_to_lh(dir, up);
                assert_test_eq!(result.determinant(), 1.0, abs <= 1e-2);
                assert_test_eq!(dir * result, Vector::<3, T, A>::Z, abs <= 1e-5, 0.0 = -0.0);
                assert_test_eq!((up * result).x, 0.0, abs <= 1e-6, 0.0 = -0.0);
                assert!((0.0..=1.00001).contains(&(up * result).y));

                if !eye.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<4, T, A>::look_to_lh(eye, dir, up),
                    Matrix::<4, T, A>::from_translation(-eye)
                        * Matrix::<4, T, A>::from_submatrix(&result),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, dir, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                if !dir.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_rh(dir, up.normalize()));
                    assert_debug_panic!(Matrix::<4, T, A>::look_to_rh(eye, dir, up.normalize()));
                }
                if !up.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_rh(dir.normalize(), up));
                    assert_debug_panic!(Matrix::<4, T, A>::look_to_rh(eye, dir.normalize(), up));
                }

                let dir = dir.normalize_or(Vector::<3, T, A>::Z).normalize();
                let up = up.normalize_or(Vector::<3, T, A>::Y).normalize();
                if dir.cross(up).try_normalize().is_none() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_rh(dir, up));
                    assert_debug_panic!(Matrix::<4, T, A>::look_to_rh(eye, dir, up));
                    continue;
                }

                let result = Matrix::<3, T, A>::look_to_rh(dir, up);
                assert_test_eq!(result.determinant(), 1.0, abs <= 1e-2);
                assert_test_eq!(
                    dir * result,
                    Vector::<3, T, A>::NEG_Z,
                    abs <= 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!((up * result).x, 0.0, abs <= 1e-6, 0.0 = -0.0);
                assert!((0.0..=1.00001).contains(&(up * result).y));

                if !eye.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<4, T, A>::look_to_rh(eye, dir, up),
                    Matrix::<4, T, A>::from_translation(-eye)
                        * Matrix::<4, T, A>::from_submatrix(&result),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                if !up.is_normalized() || center == eye {
                    assert_debug_panic!(Matrix::<3, T, A>::look_at_lh(eye, center, up));
                    assert_debug_panic!(Matrix::<4, T, A>::look_at_lh(eye, center, up));
                }

                let up = up.normalize_or(Vector::<3, T, A>::Y);
                let Some(dir) = (center - eye).try_normalize() else {
                    continue;
                };
                if up.cross(dir).try_normalize().is_none() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_at_lh(eye, center, up));
                    assert_debug_panic!(Matrix::<4, T, A>::look_at_lh(eye, center, up));
                    continue;
                }

                let result = Matrix::<3, T, A>::look_at_lh(eye, center, up);
                assert_test_eq!(result.determinant(), 1.0, abs <= 1e-5);
                assert_test_eq!(dir * result, Vector::<3, T, A>::Z, abs <= 1e-5, 0.0 = -0.0);
                assert_test_eq!((up * result).x, 0.0, abs <= 1e-6, 0.0 = -0.0);
                assert!((0.0..=1.00001).contains(&(up * result).y));

                assert_test_eq!(
                    Matrix::<4, T, A>::look_at_lh(eye, center, up),
                    Matrix::<4, T, A>::from_translation(-eye)
                        * Matrix::<4, T, A>::from_submatrix(&result),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_look_at_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                if !up.is_normalized() || center == eye {
                    assert_debug_panic!(Matrix::<3, T, A>::look_at_rh(eye, center, up));
                    assert_debug_panic!(Matrix::<4, T, A>::look_at_rh(eye, center, up));
                }

                let up = up.normalize_or(Vector::<3, T, A>::Y);
                let Some(dir) = (center - eye).try_normalize() else {
                    continue;
                };
                if up.cross(dir).try_normalize().is_none() {
                    continue;
                }

                let result = Matrix::<3, T, A>::look_at_rh(eye, center, up);
                assert_test_eq!(result.determinant(), 1.0, abs <= 1e-5);
                assert_test_eq!(
                    dir * result,
                    Vector::<3, T, A>::NEG_Z,
                    abs <= 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!((up * result).x, 0.0, abs <= 1e-6, 0.0 = -0.0);
                assert!((0.0..=1.00001).contains(&(up * result).y));

                assert_test_eq!(
                    Matrix::<4, T, A>::look_at_rh(eye, center, up),
                    Matrix::<4, T, A>::from_translation(-eye)
                        * Matrix::<4, T, A>::from_submatrix(&result),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_debug_panic!(Matrix::<3, T, A>::ZERO.to_scale_angle_translation());

            for (scale, angle, translation) in
                random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
            {
                let matrix =
                    Matrix::<3, T, A>::from_scale_angle_translation(scale, angle, translation);

                assert_panic_test_eq!(
                    matrix.to_scale_angle_translation(),
                    (
                        matrix.to_scale_angle().0,
                        matrix.to_scale_angle().1,
                        matrix.translation()
                    )
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for matrix in random_iter::<Matrix<3, T, A>>().take(20) {
                    if matrix.as_rows().iter().any(|row| !row.is_normalized()) {
                        assert_debug_panic!(matrix.to_euler(order));
                        assert_debug_panic!(
                            Matrix::<4, T, A>::from_submatrix(&matrix).to_euler(order)
                        );
                    }
                }
            }
        });
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for quat in random_iter::<Quaternion<T, A>>() {
                    let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();
                    let matrix = Matrix::<3, T, A>::from_quat(quat);

                    let result = matrix.to_euler(order);
                    assert_test_eq!(
                        Quaternion::<T, A>::from_euler(order, result.0, result.1, result.2),
                        quat,
                        abs <= quat.to_vector().abs() * 1e-3 + 1e-2,
                        0.0 = -0.0,
                        quat = -quat
                    );

                    let matrix = Matrix::<4, T, A>::from_quat(quat);
                    assert_test_eq!(matrix.to_euler(order), matrix.submatrix().to_euler(order));
                }
            }
        });
    }

    #[test]
    fn test_to_scale_rotation() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_debug_panic!(Matrix::<3, T, A>::ZERO.to_scale_rotation());
            assert_debug_panic!(Matrix::<4, T, A>::ZERO.to_scale_rotation());

            for (scale, rotation, translation) in
                random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
            {
                let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();

                let matrix = Matrix::<3, T, A>::from_scale_rotation(scale, rotation);

                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_submatrix_translation(&matrix, translation)
                        .to_scale_rotation(),
                    matrix.to_scale_rotation()
                );

                if scale.iter().any(|x| x > 1e10)
                    || !matrix.is_finite()
                    || !(1e-5..1e8).contains(&matrix.determinant().abs())
                {
                    continue;
                }

                let (result_scale, result_rotation) = matrix.to_scale_rotation();
                assert_test_eq!(
                    Matrix::<3, T, A>::from_scale_rotation(result_scale, result_rotation),
                    matrix,
                    abs <= matrix.abs() * 1e-4 + Matrix::<3, T, A>::from_row_array(&[1e-3; 9]),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_transform_point() {
        assert_eq!(
            Mat3A::from_rows(&[
                Vec3A::new(2.0, 3.0, 0.0),
                Vec3A::new(4.0, 5.0, 0.0),
                Vec3A::new(6.0, 7.0, 1.0)
            ])
            .transform_point(Vec2A::new(-1.0, -2.0)),
            Vec2A::new(-4.0, -6.0)
        );
        assert_eq!(
            Mat4A::from_rows(&[
                Vec4A::new(2.0, 3.0, 4.0, 0.0),
                Vec4A::new(5.0, 6.0, 7.0, 0.0),
                Vec4A::new(8.0, 9.0, 10.0, 0.0),
                Vec4A::new(11.0, 12.0, 13.0, 1.0)
            ])
            .transform_point(Vec3A::new(-1.0, -2.0, -3.0)),
            Vec3A::new(-25.0, -30.0, -35.0)
        );

        assert_debug_panic!(
            Mat3A::from_rows(&[
                Vec3A::new(2.0, 3.0, 0.0),
                Vec3A::new(4.0, 5.0, 1.0),
                Vec3A::new(6.0, 7.0, 1.0)
            ])
            .transform_point(Vec2A::new(-1.0, -2.0))
        );
        assert_debug_panic!(
            Mat4A::from_rows(&[
                Vec4A::new(2.0, 3.0, 4.0, 0.0),
                Vec4A::new(5.0, 6.0, 7.0, 0.0),
                Vec4A::new(8.0, 9.0, 10.0, 1.0),
                Vec4A::new(11.0, 12.0, 13.0, 1.0)
            ])
            .transform_point(Vec3A::new(-1.0, -2.0, -3.0))
        );
    }

    #[test]
    fn test_transform_vector() {
        assert_eq!(
            Mat3A::from_rows(&[
                Vec3A::new(2.0, 3.0, 0.0),
                Vec3A::new(4.0, 5.0, 0.0),
                Vec3A::new(6.0, 7.0, 1.0)
            ])
            .transform_vector(Vec2A::new(-1.0, -2.0)),
            Vec2A::new(-10.0, -13.0)
        );
        assert_eq!(
            Mat4A::from_rows(&[
                Vec4A::new(2.0, 3.0, 4.0, 0.0),
                Vec4A::new(5.0, 6.0, 7.0, 0.0),
                Vec4A::new(8.0, 9.0, 10.0, 0.0),
                Vec4A::new(11.0, 12.0, 13.0, 1.0)
            ])
            .transform_vector(Vec3A::new(-1.0, -2.0, -3.0)),
            Vec3A::new(-36.0, -42.0, -48.0)
        );

        assert_debug_panic!(
            Mat3A::from_rows(&[
                Vec3A::new(2.0, 3.0, 0.0),
                Vec3A::new(4.0, 5.0, 1.0),
                Vec3A::new(6.0, 7.0, 1.0)
            ])
            .transform_vector(Vec2A::new(-1.0, -2.0))
        );
        assert_debug_panic!(
            Mat4A::from_rows(&[
                Vec4A::new(2.0, 3.0, 4.0, 0.0),
                Vec4A::new(5.0, 6.0, 7.0, 0.0),
                Vec4A::new(8.0, 9.0, 10.0, 1.0),
                Vec4A::new(11.0, 12.0, 13.0, 1.0)
            ])
            .transform_vector(Vec3A::new(-1.0, -2.0, -3.0))
        );
    }

    #[test]
    fn test_from_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (rotation, translation) in random_iter::<(Quaternion<T, A>, Vector<3, T, A>)>() {
                if !rotation.is_normalized() {
                    assert_debug_panic!(Matrix::<4, T, A>::from_rotation_translation(
                        rotation,
                        translation
                    ));
                }

                let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();
                if !translation.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<4, T, A>::from_rotation_translation(rotation, translation),
                    Matrix::<4, T, A>::from_quat(rotation)
                        * Matrix::<4, T, A>::from_translation(translation),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation, translation) in
                random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
            {
                if !rotation.is_normalized() {
                    assert_debug_panic!(Matrix::<4, T, A>::from_rotation_translation(
                        rotation,
                        translation
                    ));
                }

                let rotation = rotation.normalize_or(Quaternion::IDENTITY);
                if !scale.is_finite() || !translation.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<4, T, A>::from_scale_rotation_translation(
                        scale,
                        rotation,
                        translation
                    ),
                    Matrix::<4, T, A>::from_scale(scale)
                        * Matrix::<4, T, A>::from_quat(rotation)
                        * Matrix::<4, T, A>::from_translation(translation),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_perspective_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                assert_debug_panic!(Matrix::<4, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Matrix::<4, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));

                let matrix = Matrix::<4, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                );

                let half_size = Vector::<2, T, A>::new(
                    (vertical_fov / 2.0).tan() * aspect_ratio,
                    (vertical_fov / 2.0).tan(),
                );

                for point in random_iter::<Vector<2, T, A>>() {
                    let point = point.map(|x| if x.abs() < 1e7 { x } else { 0.0 });

                    for (z, projection_z) in [(near_plane, 0.0), (far_plane, 1.0)] {
                        let projection = point / z / half_size;

                        assert_test_eq!(
                            matrix.project_point(point.extend(z)),
                            projection.extend(projection_z),
                            abs <= point.abs().max_element().max(1.0) * 1e-3,
                            0.0 = -0.0
                        );
                    }
                }
            }
        });
    }

    #[test]
    fn test_perspective_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                assert_debug_panic!(Matrix::<4, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Matrix::<4, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));

                assert_test_eq!(
                    Matrix::<4, T, A>::perspective_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane,
                    ),
                    Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                        * Matrix::<4, T, A>::perspective_lh(
                            vertical_fov,
                            aspect_ratio,
                            near_plane,
                            far_plane,
                        )
                );
            }
        });
    }

    #[test]
    fn test_perspective_rh_gl() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                assert_debug_panic!(Matrix::<4, T, A>::perspective_rh_gl(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Matrix::<4, T, A>::perspective_rh_gl(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));

                let expected = Matrix::<4, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                ) * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(
                    1.0, 1.0, 2.0,
                )) * Matrix::<4, T, A>::from_translation(Vector::<3, T, A>::NEG_Z);
                assert_test_eq!(
                    Matrix::<4, T, A>::perspective_rh_gl(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane,
                    ),
                    expected,
                    abs <= expected.abs() * 1e-4
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5),
                ((120.0 as T).to_radians(), 20.0, 1e-3),
            ] {
                assert_debug_panic!(Matrix::<4, T, A>::perspective_infinite_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));

                let matrix = Matrix::<4, T, A>::perspective_infinite_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                );

                let half_size = Vector::<2, T, A>::new(
                    (vertical_fov / 2.0).tan() * aspect_ratio,
                    (vertical_fov / 2.0).tan(),
                );

                for point in random_iter::<Vector<2, T, A>>() {
                    let point = point.map(|x| if x.abs() < 1e7 { x } else { 0.0 });

                    for (z, projection_z) in [(near_plane, 0.0), (1000.0, 1.0 - 1.0 / 1000.0)] {
                        let projection = point / z / half_size;

                        assert_test_eq!(
                            matrix.project_point(point.extend(z)),
                            projection.extend(projection_z),
                            abs <= point.abs().max_element().max(1.0) * 1e-3,
                            0.0 = -0.0
                        );
                    }
                }
            }
        });
    }

    #[test]
    fn test_perspective_infinite_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5),
                ((120.0 as T).to_radians(), 20.0, 1e-3),
            ] {
                assert_debug_panic!(Matrix::<4, T, A>::perspective_infinite_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));

                assert_test_eq!(
                    Matrix::<4, T, A>::perspective_infinite_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                        * Matrix::<4, T, A>::perspective_infinite_lh(
                            vertical_fov,
                            aspect_ratio,
                            near_plane
                        )
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5),
                ((120.0 as T).to_radians(), 20.0, 1e-3),
            ] {
                assert_debug_panic!(Matrix::<4, T, A>::perspective_infinite_reverse_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));

                assert_test_eq!(
                    Matrix::<4, T, A>::perspective_infinite_reverse_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Matrix::<4, T, A>::perspective_infinite_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ) * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                        * Matrix::<4, T, A>::from_translation(Vector::<3, T, A>::Z)
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5),
                ((120.0 as T).to_radians(), 20.0, 1e-3),
            ] {
                assert_debug_panic!(Matrix::<4, T, A>::perspective_infinite_reverse_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));

                assert_test_eq!(
                    Matrix::<4, T, A>::perspective_infinite_reverse_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                        * Matrix::<4, T, A>::perspective_infinite_reverse_lh(
                            vertical_fov,
                            aspect_ratio,
                            near_plane
                        )
                );
            }
        });
    }

    #[test]
    fn test_frustum_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                let half_height = (vertical_fov / 2.0).tan() * near_plane;
                let half_width = half_height * aspect_ratio;

                assert_debug_panic!(Matrix::<4, T, A>::frustum_lh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Matrix::<4, T, A>::frustum_lh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    6.0,
                    4.0
                ));

                let expected = Matrix::<4, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::frustum_lh(
                        -half_width,
                        half_width,
                        -half_height,
                        half_height,
                        near_plane,
                        far_plane
                    ),
                    expected,
                    abs <= expected.abs() * 1e-4
                );
            }
        });
    }

    #[test]
    fn test_frustum_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                let half_height = (vertical_fov / 2.0).tan() * near_plane;
                let half_width = half_height * aspect_ratio;

                assert_debug_panic!(Matrix::<4, T, A>::frustum_rh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Matrix::<4, T, A>::frustum_rh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    6.0,
                    4.0
                ));

                let expected = Matrix::<4, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                );
                assert_test_eq!(
                    Matrix::<4, T, A>::frustum_rh(
                        -half_width,
                        half_width,
                        -half_height,
                        half_height,
                        near_plane,
                        far_plane
                    ),
                    expected,
                    abs <= expected.abs() * 1e-4
                );
            }
        });
    }

    #[test]
    fn test_frustum_rh_gl() {
        for_types!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near_plane = 0.34;
            let far_plane = 420.0;

            assert_debug_panic!(Matrix::<4, T, A>::frustum_rh_gl(
                left, right, bottom, top, -1.0, 4.0
            ));
            assert_debug_panic!(Matrix::<4, T, A>::frustum_rh_gl(
                left, right, bottom, top, 6.0, 4.0
            ));

            let expected =
                Matrix::<4, T, A>::frustum_rh(left, right, bottom, top, near_plane, far_plane)
                    * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, 2.0))
                    * Matrix::<4, T, A>::from_translation(Vector::<3, T, A>::NEG_Z);
            assert_test_eq!(
                Matrix::<4, T, A>::frustum_rh_gl(left, right, bottom, top, near_plane, far_plane),
                expected,
                abs <= expected.abs() * 1e-4
            );
        });
    }

    #[test]
    fn test_orthographic_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            assert_debug_panic!(Matrix::<4, T, A>::orthographic_lh(
                left, right, bottom, top, 6.0, 4.0
            ));

            let matrix = Matrix::<4, T, A>::orthographic_lh(left, right, bottom, top, near, far);

            for (x, projection_x) in [(left, -1.0), (right, 1.0), (left.midpoint(right), 0.0)] {
                for (y, projection_y) in [(bottom, -1.0), (top, 1.0), (bottom.midpoint(top), 0.0)] {
                    for (z, projection_z) in [(near, 0.0), (far, 1.0), (near.midpoint(far), 0.5)] {
                        let point = Vector::<3, T, A>::new(x, y, z);
                        let projection =
                            Vector::<3, T, A>::new(projection_x, projection_y, projection_z);

                        assert_test_eq!(matrix.project_point(point), projection, abs <= 1e-5);
                    }
                }
            }
        });
    }

    #[test]
    fn test_orthographic_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            assert_debug_panic!(Matrix::<4, T, A>::orthographic_rh(
                left, right, bottom, top, 6.0, 4.0
            ));

            assert_test_eq!(
                Matrix::<4, T, A>::orthographic_rh(left, right, bottom, top, near, far),
                Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                    * Matrix::<4, T, A>::orthographic_lh(left, right, bottom, top, near, far)
            );
        });
    }

    #[test]
    fn test_orthographic_rh_gl() {
        for_types!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            assert_debug_panic!(Matrix::<4, T, A>::orthographic_rh_gl(
                left, right, bottom, top, 6.0, 4.0
            ));

            let expected = Matrix::<4, T, A>::orthographic_rh(left, right, bottom, top, near, far)
                * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, 2.0))
                * Matrix::<4, T, A>::from_translation(Vector::<3, T, A>::NEG_Z);
            assert_test_eq!(
                Matrix::<4, T, A>::orthographic_rh_gl(left, right, bottom, top, near, far),
                expected,
                abs <= expected.abs() * 1e-4
            );
        });
    }

    #[test]
    fn test_to_scale_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_debug_panic!(Matrix::<4, T, A>::ZERO.to_scale_rotation_translation());

            for (scale, rotation, translation) in
                random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
            {
                let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();

                let matrix = Matrix::<4, T, A>::from_scale_rotation_translation(
                    scale,
                    rotation,
                    translation,
                );

                assert_panic_test_eq!(
                    matrix.to_scale_rotation_translation(),
                    (
                        matrix.to_scale_rotation().0,
                        matrix.to_scale_rotation().1,
                        matrix.translation()
                    )
                );
            }
        });
    }

    #[test]
    fn test_project_point() {
        for_types!(|T: PrimitiveFloat, A| {
            for (matrix, point) in random_iter::<(Matrix<4, T, A>, Vector<3, T, A>)>() {
                assert_test_eq!(
                    matrix.project_point(point),
                    Vector::<3, T, A>::from_homogeneous(point.to_homogeneous() * matrix)
                );
            }
        });
    }
}
