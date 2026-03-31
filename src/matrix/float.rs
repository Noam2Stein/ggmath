use core::convert::identity;

#[cfg(backend)]
use crate::EulerRot;
use crate::{
    Alignment, Length, Matrix, Quaternion, Scalar, SupportedLength, Vector,
    utils::{PrimitiveFloat, transmute_generic, transmute_ref},
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
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
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
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[cfg(backend)]
    #[inline]
    #[must_use]
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
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[cfg(backend)]
    #[inline]
    #[must_use]
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
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[cfg(backend)]
    #[inline]
    #[must_use]
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

    /// Creates a 3D rotation matrix from a quaternion.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the quaternion is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_quat(quat: Quaternion<T, A>) -> Self {
        #[cfg(assertions)]
        assert!(quat.to_vec().is_normalized());

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

        Self::from_columns(&[
            Vector::<3, T, A>::new(T::ONE - (yy2 + zz2), xy2 + wz2, xz2 - wy2),
            Vector::<3, T, A>::new(xy2 - wz2, T::ONE - (xx2 + zz2), yz2 + wx2),
            Vector::<3, T, A>::new(xz2 + wy2, yz2 - wx2, T::ONE - (xx2 + yy2)),
        ])
    }

    /// Creates a 3D rotation matrix from a rotation `axis` and `angle` (in
    /// radians).
    ///
    /// `axis` must be normalized. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `axis` is not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(axis: Vector<3, T, A>, angle: T) -> Self {
        #[cfg(assertions)]
        assert!(axis.is_normalized());

        let (sin, cos) = angle.sin_cos();
        let [xsin, ysin, zsin] = (axis * sin).to_array();
        let [x, y, z] = axis.to_array();
        let [x2, y2, z2] = (axis * axis).to_array();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;

        Self::from_columns(&[
            Vector::<3, T, A>::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            Vector::<3, T, A>::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            Vector::<3, T, A>::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
        ])
    }

    /// Creates a 3D rotation matrix from an Euler rotation order/sequence and
    /// angles (in radians).
    #[cfg(backend)]
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

        let (si, ci) = angles.x.sin_cos();
        let (sj, cj) = angles.y.sin_cos();
        let (sh, ch) = angles.z.sin_cos();

        let cc = ci * ch;
        let cs = ci * sh;
        let sc = si * ch;
        let ss = si * sh;

        let mut result = Self::ZERO;

        if order.initial_repeated {
            result.column_mut(i)[i] = cj;
            result.column_mut(i)[j] = sj * si;
            result.column_mut(i)[k] = sj * ci;
            result.column_mut(j)[i] = sj * sh;
            result.column_mut(j)[j] = -cj * ss + cc;
            result.column_mut(j)[k] = -cj * cs - sc;
            result.column_mut(k)[i] = -sj * ch;
            result.column_mut(k)[j] = cj * sc + cs;
            result.column_mut(k)[k] = cj * cc - ss;
        } else {
            result.column_mut(i)[i] = cj * ch;
            result.column_mut(i)[j] = sj * sc - cs;
            result.column_mut(i)[k] = sj * cc + ss;
            result.column_mut(j)[i] = cj * sh;
            result.column_mut(j)[j] = sj * ss + cc;
            result.column_mut(j)[k] = sj * cs - sc;
            result.column_mut(k)[i] = -sj;
            result.column_mut(k)[j] = cj * si;
            result.column_mut(k)[k] = cj * ci;
        }

        result
    }

    /// Creates a left-handed view matrix from a facing direction and an up
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `dir` or `up` are not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_lh(dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        #[cfg(assertions)]
        assert!(dir.is_normalized());
        #[cfg(assertions)]
        assert!(up.is_normalized());

        let forward = dir;
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `dir` or `up` are not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_rh(dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        #[cfg(assertions)]
        assert!(dir.is_normalized());
        #[cfg(assertions)]
        assert!(up.is_normalized());

        let forward = dir;
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `up` is not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_lh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        Self::look_to_lh((center - eye).normalize(), up)
    }

    /// Creates a right-handed view matrix from a camera position, a focal point
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `up` is not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_rh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        Self::look_to_rh((center - eye).normalize(), up)
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// `self` must not contain any non-rotation transformations. Otherwise the
    /// result is unspecified.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if any column of `self` is not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(&self, order: EulerRot) -> (T, T, T) {
        // Ported from https://github.com/bitshifter/glam-rs.

        // Based on Ken Shoemake. 1994. Euler angle conversion. Graphics gems IV.
        // Academic Press Professional, Inc., USA, 222–229.

        #[cfg(assertions)]
        assert!(
            self.x_axis.is_normalized()
                && self.y_axis.is_normalized()
                && self.z_axis.is_normalized()
        );

        let order = order.properties();
        let (i, j, k) = order.axes_indices();

        let mut ea = Vector::<3, T, A>::ZERO;
        if order.initial_repeated {
            let sy = (self.column(i)[j] * self.column(i)[j]
                + self.column(i)[k] * self.column(i)[k])
                .sqrt();

            if sy > T::as_from(16.0) * T::EPSILON {
                ea.x = self.column(i)[j].atan2(self.column(i)[k]);
                ea.y = sy.atan2(self.column(i)[i]);
                ea.z = self.column(j)[i].atan2(-self.column(k)[i]);
            } else {
                ea.x = (-self.column(j)[k]).atan2(self.column(j)[j]);
                ea.y = sy.atan2(self.column(i)[i]);
            }
        } else {
            let cy = (self.column(i)[i] * self.column(i)[i]
                + self.column(j)[i] * self.column(j)[i])
                .sqrt();

            if cy > T::as_from(16.0) * T::EPSILON {
                ea.x = self.column(k)[j].atan2(self.column(k)[k]);
                ea.y = (-self.column(k)[i]).atan2(cy);
                ea.z = self.column(j)[i].atan2(self.column(i)[i]);
            } else {
                ea.x = (-self.column(j)[k]).atan2(self.column(j)[j]);
                ea.y = (-self.column(k)[i]).atan2(cy);
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
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
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
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
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
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
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

    #[inline(always)]
    #[track_caller]
    fn quat_to_axes(quat: Quaternion<T, A>) -> [Vector<4, T, A>; 3] {
        #[cfg(assertions)]
        assert!(quat.to_vec().is_normalized());

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
    /// When assertions are enabled (see the crate documentation):
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
        Self::from_columns(&[x_axis, y_axis, z_axis, Vector::W])
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `axis` is not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(axis: Vector<3, T, A>, angle: T) -> Self {
        #[cfg(assertions)]
        assert!(axis.is_normalized());

        let (sin, cos) = angle.sin_cos();
        let [xsin, ysin, zsin] = (axis * sin).to_array();
        let [x, y, z] = axis.to_array();
        let [x2, y2, z2] = (axis * axis).to_array();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;

        Self::from_columns(&[
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
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_euler(order: EulerRot, a: T, b: T, c: T) -> Self {
        Self::from_submatrix(&Matrix::<3, T, A>::from_euler(order, a, b, c))
    }

    /// Creates an affine transformation matrix containing a 3D `rotation` and
    /// `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
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
        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
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
        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `dir` or `up` are not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_lh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        #[cfg(assertions)]
        assert!(dir.is_normalized());
        #[cfg(assertions)]
        assert!(up.is_normalized());

        let forward = dir;
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `dir` or `up` are not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_rh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        #[cfg(assertions)]
        assert!(dir.is_normalized());
        #[cfg(assertions)]
        assert!(up.is_normalized());

        let forward = dir;
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `up` is not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_lh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        Self::look_to_lh(eye, (center - eye).normalize(), up)
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `up` is not normalized.
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_rh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        Self::look_to_rh(eye, (center - eye).normalize(), up)
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`project_point`]: Self::project_point
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_lh(vertical_fov: T, aspect_ratio: T, near_plane: T, far_plane: T) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO && far_plane > near_plane);

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let depth_scale = far_plane / (far_plane - near_plane);

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`project_point`]: Self::project_point
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_rh(vertical_fov: T, aspect_ratio: T, near_plane: T, far_plane: T) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO && far_plane > near_plane);

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let neg_depth_scale = far_plane / (near_plane - far_plane);

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`gluPerspective`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
    /// [`project_point`]: Self::project_point
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_rh_gl(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
        far_plane: T,
    ) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO && far_plane > near_plane);

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let depth_recip = (near_plane - far_plane).recip();

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    ///
    /// [`project_point`]: Self::project_point
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_lh(vertical_fov: T, aspect_ratio: T, near_plane: T) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO);

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    ///
    /// [`project_point`]: Self::project_point
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_rh(vertical_fov: T, aspect_ratio: T, near_plane: T) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO);

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    ///
    /// [`project_point`]: Self::project_point
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_reverse_lh(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
    ) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO);

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    ///
    /// [`project_point`]: Self::project_point
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_reverse_rh(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
    ) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO);

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn frustum_lh(left: T, right: T, bottom: T, top: T, near_plane: T, far_plane: T) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO && far_plane > near_plane);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = far_plane * depth_recip;
        let d = -near_plane * c;

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn frustum_rh(left: T, right: T, bottom: T, top: T, near_plane: T, far_plane: T) -> Self {
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO && far_plane > near_plane);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = -far_plane * depth_recip;
        let d = near_plane * c;

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
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
        #[cfg(assertions)]
        assert!(near_plane > T::ZERO && far_plane > near_plane);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = -(far_plane + near_plane) * depth_recip;
        let d = -two_near_plane * far_plane * depth_recip;

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `far` is less than or equal to `near`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_lh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        #[cfg(assertions)]
        assert!(far > near);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far - near).recip();

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `far` is less than or equal to `near`.
    ///
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_rh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        #[cfg(assertions)]
        assert!(far > near);

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let neg_depth_recip = (near - far).recip();

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `far` is less than or equal to `near`.
    ///
    /// [`glOrtho`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml
    /// [`project_point`]: Self::project_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_rh_gl(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        #[cfg(assertions)]
        assert!(far > near);

        let scale_x = T::as_from(2.0) / (right - left);
        let scale_y = T::as_from(2.0) / (top - bottom);
        let scale_z = T::as_from(2.0) / (near - far);
        let translation_x = -(right + left) / (right - left);
        let translation_y = -(top + bottom) / (top - bottom);
        let translation_z = -(far + near) / (far - near);

        Self::from_columns(&[
            Vector::<4, T, A>::new(scale_x, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, scale_y, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, scale_z, T::ZERO),
            Vector::<4, T, A>::new(translation_x, translation_y, translation_z, T::ONE),
        ])
    }

    #[cfg(backend)]
    #[inline(always)]
    fn quat_from_axes(
        x_axis: Vector<4, T, A>,
        y_axis: Vector<4, T, A>,
        z_axis: Vector<4, T, A>,
    ) -> Quaternion<T, A> {
        // Ported from https://github.com/bitshifter/glam-rs `Quat::from_rotation_axes`
        // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`

        let [m00, m01, m02, _] = x_axis.to_array();
        let [m10, m11, m12, _] = y_axis.to_array();
        let [m20, m21, m22, _] = z_axis.to_array();

        if m22 <= T::ZERO {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = T::ONE - m22;

            if dif10 <= T::ZERO {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = T::as_from(0.5) / four_xsq.sqrt();

                Quaternion::new(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = T::as_from(0.5) / four_ysq.sqrt();

                Quaternion::new(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = T::ONE + m22;

            if sum10 <= T::ZERO {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = T::as_from(0.5) / four_zsq.sqrt();

                Quaternion::new(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = T::as_from(0.5) / four_wsq.sqrt();

                Quaternion::new(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// The upper 3x3 matrix must not contain any non-rotation transformations.
    /// Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if any column of the upper 3x3 matrix is not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(&self, order: EulerRot) -> (T, T, T) {
        self.submatrix().to_euler(order)
    }

    /// Returns the `scale`, `rotation` and `translation` of `self`.
    ///
    /// `self` must contain a valid affine transformation. Otherwise the result
    /// is unspecified.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the determinant of `self` is zero.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation_translation(
        &self,
    ) -> (Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>) {
        let determinant = self.determinant();

        #[cfg(assertions)]
        assert!(determinant != T::ZERO);

        let scale = Vector::<3, T, A>::new(
            self.x_axis.length() * determinant.signum(),
            self.y_axis.length(),
            self.z_axis.length(),
        );

        let scale_recip = scale.recip();

        let rotation = Self::quat_from_axes(
            self.x_axis * scale_recip.x,
            self.y_axis * scale_recip.y,
            self.z_axis * scale_recip.z,
        );

        let translation = self.w_axis.xyz();

        (scale, rotation, translation)
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
    use itertools::iproduct;

    use crate::{
        EulerRot, Matrix, Quaternion, Vector,
        utils::{assert_float_eq, assert_panic, for_parameters},
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

    #[test]
    fn test_from_quat() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Matrix::<3, T, A>::from_quat(Quaternion::IDENTITY),
                Matrix::IDENTITY
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_quat(Quaternion::IDENTITY),
                Matrix::IDENTITY
            );
        });
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x * 0.3 + 0.5;

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let quat = Quaternion::from_vec(Vector::<4, T, A>::new(x, y, z, w).normalize());
            let invalid = Quaternion::from_vec(Vector::<4, T, A>::new(x, y, z, w));

            assert_float_eq!(
                Matrix::<3, T, A>::from_quat(quat).determinant(),
                1.0,
                abs <= 1e-5
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_quat(quat).determinant(),
                1.0,
                abs <= 1e-5
            );

            if cfg!(assertions) && !invalid.to_vec().is_normalized() {
                assert_panic!(Matrix::<3, T, A>::from_quat(invalid));
                assert_panic!(Matrix::<4, T, A>::from_quat(invalid));
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_parameters!(|T: PrimitiveFloat, A, x| {
            let _: T = x;

            if !x.is_finite() || x.abs() > 100000.0 {
                return;
            }

            assert_float_eq!(
                Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::X, x),
                Matrix::<3, T, A>::from_rotation_x(x),
                0.0 = -0.0
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::Y, x),
                Matrix::<3, T, A>::from_rotation_y(x),
                0.0 = -0.0
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::Z, x),
                Matrix::<3, T, A>::from_rotation_z(x),
                0.0 = -0.0
            );

            assert_float_eq!(
                Matrix::<4, T, A>::from_axis_angle(Vector::<3, T, A>::X, x),
                Matrix::<4, T, A>::from_rotation_x(x),
                0.0 = -0.0
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_axis_angle(Vector::<3, T, A>::Y, x),
                Matrix::<4, T, A>::from_rotation_y(x),
                0.0 = -0.0
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_axis_angle(Vector::<3, T, A>::Z, x),
                Matrix::<4, T, A>::from_rotation_z(x),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_euler() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x > 1000000.0
                || y > 1000000.0
                || z > 1000000.0
            {
                return;
            }

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
                (EulerRot::Xyz, x, y, z, rot_x * rot_y * rot_z),
                (EulerRot::Xzy, x, z, y, rot_x * rot_z * rot_y),
                (EulerRot::Yxz, y, x, z, rot_y * rot_x * rot_z),
                (EulerRot::Yzx, y, z, x, rot_y * rot_z * rot_x),
                (EulerRot::Zxy, z, x, y, rot_z * rot_x * rot_y),
                (EulerRot::Zyx, z, y, x, rot_z * rot_y * rot_x),
                (EulerRot::Xyx, x, y, z, rot_x * rot_y * rot_x_by_z),
                (EulerRot::Xzx, x, z, y, rot_x * rot_z * rot_x_by_y),
                (EulerRot::Yxy, y, x, z, rot_y * rot_x * rot_y_by_z),
                (EulerRot::Yzy, y, z, x, rot_y * rot_z * rot_y_by_x),
                (EulerRot::Zxz, z, x, y, rot_z * rot_x * rot_z_by_y),
                (EulerRot::Zyz, z, y, x, rot_z * rot_y * rot_z_by_x),
                (EulerRot::XyzEx, x, y, z, rot_z * rot_y * rot_x),
                (EulerRot::XzyEx, x, z, y, rot_y * rot_z * rot_x),
                (EulerRot::YxzEx, y, x, z, rot_z * rot_x * rot_y),
                (EulerRot::YzxEx, y, z, x, rot_x * rot_z * rot_y),
                (EulerRot::ZxyEx, z, x, y, rot_y * rot_x * rot_z),
                (EulerRot::ZyxEx, z, y, x, rot_x * rot_y * rot_z),
                (EulerRot::XyxEx, x, y, z, rot_x_by_z * rot_y * rot_x),
                (EulerRot::XzxEx, x, z, y, rot_x_by_y * rot_z * rot_x),
                (EulerRot::YxyEx, y, x, z, rot_y_by_z * rot_x * rot_y),
                (EulerRot::YzyEx, y, z, x, rot_y_by_x * rot_z * rot_y),
                (EulerRot::ZxzEx, z, x, y, rot_z_by_y * rot_x * rot_z),
                (EulerRot::ZyzEx, z, y, x, rot_z_by_x * rot_y * rot_z),
            ] {
                assert_float_eq!(
                    Matrix::<3, T, A>::from_euler(order, a, b, c),
                    result,
                    abs <= Matrix::<3, T, A>::from_column_array(&[1e-5; 9]),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Matrix::<4, T, A>::from_euler(order, a, b, c),
                    Matrix::<4, T, A>::from_submatrix(&result),
                    abs <= Matrix::<4, T, A>::from_column_array(&[1e-5; 16]),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let Some(forward) = Vector::<3, T, A>::new(x, y, z).try_normalize() else {
                return;
            };

            let up = (forward * 0.4 + forward.zxy().with_z(0.3)).normalize();
            let mat = Matrix::<3, T, A>::look_to_lh(forward, up);
            let mat_mul_up = mat * up;

            assert_float_eq!(mat.determinant(), 1.0, abs <= 1e-5);
            assert_float_eq!(
                mat * forward,
                Vector::<3, T, A>::Z,
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
            assert_float_eq!(mat_mul_up.x, 0.0, abs <= 1e-7, 0.0 = -0.0);
            assert!((0.0..=1.00001).contains(&mat_mul_up.y));

            let eye = forward * 0.3 + forward.yzx().with_z(0.6);
            let mat4 = Matrix::<4, T, A>::look_to_lh(eye, forward, up);
            assert_float_eq!(
                mat4,
                Matrix::<4, T, A>::from_submatrix(&mat) * Matrix::<4, T, A>::from_translation(-eye),
                0.0 = -0.0
            );

            if cfg!(assertions) && !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_panic!(Matrix::<3, T, A>::look_to_lh(
                    Vector::<3, T, A>::new(x, y, z),
                    up
                ));
                assert_panic!(Matrix::<3, T, A>::look_to_lh(
                    forward,
                    Vector::<3, T, A>::new(x, y, z)
                ));
            }
        })
    }

    #[test]
    fn test_look_to_rh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let Some(forward) = Vector::<3, T, A>::new(x, y, z).try_normalize() else {
                return;
            };

            let up = (forward * 0.4 + forward.zxy().with_z(0.3)).normalize();
            let mat = Matrix::<3, T, A>::look_to_rh(forward, up);
            let mat_mul_up = mat * up;

            assert_float_eq!(mat.determinant(), 1.0, abs <= 1e-5);
            assert_float_eq!(
                mat * forward,
                Vector::<3, T, A>::NEG_Z,
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
            assert_float_eq!(mat_mul_up.x, 0.0, abs <= 1e-7, 0.0 = -0.0);
            assert!((0.0..=1.00001).contains(&mat_mul_up.y));

            let eye = forward * 0.3 + forward.yzx().with_z(0.6);
            let mat4 = Matrix::<4, T, A>::look_to_rh(eye, forward, up);
            assert_float_eq!(
                mat4,
                Matrix::<4, T, A>::from_submatrix(&mat) * Matrix::<4, T, A>::from_translation(-eye),
                0.0 = -0.0
            );

            if cfg!(assertions) && !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_panic!(Matrix::<3, T, A>::look_to_rh(
                    Vector::<3, T, A>::new(x, y, z),
                    up
                ));
                assert_panic!(Matrix::<3, T, A>::look_to_rh(
                    forward,
                    Vector::<3, T, A>::new(x, y, z)
                ));
            }
        })
    }

    #[test]
    fn test_look_at_lh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let eye = Vector::<3, T, A>::new(x, y, z);
            let center = eye * 0.6 + eye.yzx();
            let Some(up) = (eye * 0.4 + center.zxy().with_z(0.6)).try_normalize() else {
                return;
            };

            let Some(forward) = (center - eye).try_normalize() else {
                return;
            };

            assert_float_eq!(
                Matrix::<3, T, A>::look_at_lh(eye, center, up),
                Matrix::<3, T, A>::look_to_lh(forward, up),
                abs <= Matrix::<3, T, A>::from_column_array(&[1e-6; 9])
            );
            assert_float_eq!(
                Matrix::<4, T, A>::look_at_lh(eye, center, up),
                Matrix::<4, T, A>::from_submatrix(&Matrix::<3, T, A>::look_at_lh(eye, center, up))
                    * Matrix::<4, T, A>::from_translation(-eye),
                0.0 = -0.0
            );

            if cfg!(assertions) && !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_panic!(Matrix::<3, T, A>::look_at_lh(
                    eye,
                    center,
                    Vector::<3, T, A>::new(x, y, z),
                ));
            }
        })
    }

    #[test]
    fn test_look_at_rh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let eye = Vector::<3, T, A>::new(x, y, z);
            let center = eye * 0.6 + eye.yzx();
            let Some(up) = (eye * 0.4 + center.zxy().with_z(0.6)).try_normalize() else {
                return;
            };

            let Some(forward) = (center - eye).try_normalize() else {
                return;
            };

            assert_float_eq!(
                Matrix::<3, T, A>::look_at_rh(eye, center, up),
                Matrix::<3, T, A>::look_to_rh(forward, up),
                abs <= Matrix::<3, T, A>::from_column_array(&[1e-6; 9])
            );
            assert_float_eq!(
                Matrix::<4, T, A>::look_at_rh(eye, center, up),
                Matrix::<4, T, A>::from_submatrix(&Matrix::<3, T, A>::look_at_rh(eye, center, up))
                    * Matrix::<4, T, A>::from_translation(-eye),
                0.0 = -0.0
            );

            if cfg!(assertions) && !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_panic!(Matrix::<3, T, A>::look_at_rh(
                    eye,
                    center,
                    Vector::<3, T, A>::new(x, y, z),
                ));
            }
        })
    }

    #[test]
    fn test_to_euler() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z, order| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x > 1000000.0
                || y > 1000000.0
                || z > 1000000.0
            {
                return;
            }

            let mat = Matrix::<3, T, A>::from_euler(order, x, y, z);
            let (x2, y2, z2) = mat.to_euler(order);
            assert_float_eq!(
                Matrix::<3, T, A>::from_euler(order, x2, y2, z2),
                mat,
                abs <= Matrix::<3, T, A>::from_column_array(&[1e-5; 9]),
                0.0 = -0.0
            );

            let mat = Matrix::<4, T, A>::from_euler(order, x, y, z)
                * Matrix::<4, T, A>::from_translation(Vector::NAN);
            let (x2, y2, z2) = mat.to_euler(order);
            assert_float_eq!(
                Matrix::<3, T, A>::from_euler(order, x2, y2, z2),
                mat.submatrix(),
                abs <= Matrix::<3, T, A>::from_column_array(&[1e-5; 9]),
                0.0 = -0.0
            );

            let mat = Matrix::<3, T, A>::from_column_array(&[x, y, z, z, x, x, y, y, z]);
            if cfg!(assertions)
                && !(mat.x_axis.is_normalized()
                    && mat.y_axis.is_normalized()
                    && mat.z_axis.is_normalized())
            {
                assert_panic!(mat.to_euler(order));
            }

            let mat = Matrix::<4, T, A>::from_column_array(&[
                x, y, z, z, x, x, y, y, z, z, x, y, y, z, z, x,
            ]);
            if cfg!(assertions)
                && !(mat.x_axis.xyz().is_normalized()
                    && mat.y_axis.xyz().is_normalized()
                    && mat.z_axis.xyz().is_normalized())
            {
                assert_panic!(mat.to_euler(order));
            }
        });
    }

    #[test]
    fn test_from_rotation_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x * 0.3 + 0.5;

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let rotation = Quaternion::from_vec(Vector::<4, T, A>::new(x, y, z, w).normalize());
            let translation = Vector::<3, T, A>::new(x, y, z);
            let invalid_rotation = Quaternion::from_vec(Vector::<4, T, A>::new(x, y, z, w));

            assert_float_eq!(
                Matrix::<4, T, A>::from_rotation_translation(rotation, translation),
                Matrix::<4, T, A>::from_translation(translation)
                    * Matrix::<4, T, A>::from_quat(rotation),
                0.0 = -0.0
            );

            if cfg!(assertions) && !invalid_rotation.to_vec().is_normalized() {
                assert_panic!(Matrix::<4, T, A>::from_rotation_translation(
                    invalid_rotation,
                    translation
                ));
            }
        });
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b, c] = [x * 0.3 + 0.5, x + 1.0, y + 2.0, z + 3.0];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<3, T, A>::new(x, y, z);
            let rotation = Quaternion::from_vec(Vector::<4, T, A>::new(x, y, z, w).normalize());
            let translation = Vector::<3, T, A>::new(a, b, c);
            let invalid_rotation = Quaternion::from_vec(Vector::<4, T, A>::new(x, y, z, w));

            assert_float_eq!(
                Matrix::<4, T, A>::from_scale_rotation_translation(scale, rotation, translation),
                Matrix::<4, T, A>::from_translation(translation)
                    * Matrix::<4, T, A>::from_quat(rotation)
                    * Matrix::<4, T, A>::from_scale(scale),
                0.0 = -0.0
            );

            if cfg!(assertions) && !invalid_rotation.to_vec().is_normalized() {
                assert_panic!(Matrix::<4, T, A>::from_scale_rotation_translation(
                    scale,
                    invalid_rotation,
                    translation
                ));
            }
        });
    }

    #[test]
    fn test_perspective_lh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y| {
            let _: [T; 2] = [x, y];

            if !x.is_finite() || !y.is_finite() || x.abs() >= 10000000.0 || y.abs() >= 10000000.0 {
                return;
            }

            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;
            let far_plane = 420.0;

            let mat = Matrix::<4, T, A>::perspective_lh(
                vertical_fov,
                aspect_ratio,
                near_plane,
                far_plane,
            );

            let half_height = (vertical_fov / 2.0).tan();
            let half_width = half_height * aspect_ratio;

            for (z, depth) in [(near_plane, 0.0), (far_plane, 1.0)] {
                let x2 = x / z / half_width;
                let y2 = y / z / half_height;

                assert_float_eq!(
                    mat.project_point(Vector::<3, T, A>::new(x, y, z)),
                    Vector::<3, T, A>::new(x2, y2, depth),
                    abs <= Vector::<3, T, A>::new(x2.abs(), y2.abs(), 1.0) * 1e-3,
                    0.0 = -0.0
                );
            }

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_panic!(Matrix::<4, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));
            }
        });
    }

    #[test]
    fn test_perspective_rh() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;
            let far_plane = 420.0;

            assert_float_eq!(
                Matrix::<4, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                ),
                Matrix::<4, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                ) * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_panic!(Matrix::<4, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));
            }
        });
    }

    #[test]
    fn test_perspective_rh_gl() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;
            let far_plane = 420.0;

            assert_float_eq!(
                Matrix::<4, T, A>::perspective_rh_gl(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                ),
                Matrix::<4, T, A>::from_translation(Vector::<3, T, A>::NEG_Z)
                    * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, 2.0))
                    * Matrix::<4, T, A>::perspective_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane,
                    ),
                r2nd <= Matrix::<4, T, A>::from_column_array(&[1e-4; 16])
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::perspective_rh_gl(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_panic!(Matrix::<4, T, A>::perspective_rh_gl(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));
            }
        });
    }

    #[test]
    fn test_perspective_infinite_lh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y| {
            let _: [T; 2] = [x, y];

            if !x.is_finite() || !y.is_finite() || x.abs() >= 10000000.0 || y.abs() >= 10000000.0 {
                return;
            }

            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;

            let mat =
                Matrix::<4, T, A>::perspective_infinite_lh(vertical_fov, aspect_ratio, near_plane);

            let half_height = (vertical_fov / 2.0).tan();
            let half_width = half_height * aspect_ratio;

            for (z, depth) in [(near_plane, 0.0), (1000.0, 1.0 - 1.0 / 1000.0)] {
                let x2 = x / z / half_width;
                let y2 = y / z / half_height;

                assert_float_eq!(
                    mat.project_point(Vector::<3, T, A>::new(x, y, z)),
                    Vector::<3, T, A>::new(x2, y2, depth),
                    abs <= Vector::<3, T, A>::new(x2.abs(), y2.abs(), 1.0) * 1e-3,
                    0.0 = -0.0
                );
            }

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::perspective_infinite_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));
            }
        });
    }

    #[test]
    fn test_perspective_infinite_rh() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;

            assert_float_eq!(
                Matrix::<4, T, A>::perspective_infinite_rh(vertical_fov, aspect_ratio, near_plane),
                Matrix::<4, T, A>::perspective_infinite_lh(vertical_fov, aspect_ratio, near_plane)
                    * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::perspective_infinite_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_lh() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;

            assert_float_eq!(
                Matrix::<4, T, A>::perspective_infinite_reverse_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane
                ),
                Matrix::<4, T, A>::from_translation(Vector::<3, T, A>::Z)
                    * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                    * Matrix::<4, T, A>::perspective_infinite_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::perspective_infinite_reverse_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_rh() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;

            assert_float_eq!(
                Matrix::<4, T, A>::perspective_infinite_reverse_rh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane
                ),
                Matrix::<4, T, A>::perspective_infinite_reverse_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane
                ) * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::perspective_infinite_reverse_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));
            }
        });
    }

    #[test]
    fn test_frustum_lh() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;
            let far_plane = 420.0;

            let half_height = (vertical_fov / 2.0).tan() * near_plane;
            let half_width = half_height * aspect_ratio;

            assert_float_eq!(
                Matrix::<4, T, A>::frustum_lh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    near_plane,
                    far_plane
                ),
                Matrix::<4, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane
                ),
                r2nd <= Matrix::<4, T, A>::from_column_array(&[1e-4; 16])
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::frustum_lh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    -1.0,
                    4.0
                ));
                assert_panic!(Matrix::<4, T, A>::frustum_lh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    6.0,
                    4.0
                ));
            }
        });
    }

    #[test]
    fn test_frustum_rh() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let vertical_fov = T::to_radians(69.0);
            let aspect_ratio = 16.0 / 9.0;
            let near_plane = 0.34;
            let far_plane = 420.0;

            let half_height = (vertical_fov / 2.0).tan() * near_plane;
            let half_width = half_height * aspect_ratio;

            assert_float_eq!(
                Matrix::<4, T, A>::frustum_rh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    near_plane,
                    far_plane
                ),
                Matrix::<4, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane
                ),
                r2nd <= Matrix::<4, T, A>::from_column_array(&[1e-4; 16])
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::frustum_rh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    -1.0,
                    4.0
                ));
                assert_panic!(Matrix::<4, T, A>::frustum_rh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    6.0,
                    4.0
                ));
            }
        });
    }

    #[test]
    fn test_frustum_rh_gl() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near_plane = 0.34;
            let far_plane = 420.0;

            assert_float_eq!(
                Matrix::<4, T, A>::frustum_rh_gl(left, right, bottom, top, near_plane, far_plane),
                Matrix::<4, T, A>::from_translation(Vector::<3, T, A>::NEG_Z)
                    * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, 2.0))
                    * Matrix::<4, T, A>::frustum_rh(
                        left, right, bottom, top, near_plane, far_plane
                    ),
                r2nd <= Matrix::<4, T, A>::from_column_array(&[1e-4; 16])
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::frustum_rh_gl(
                    left, right, bottom, top, -1.0, 4.0
                ));
                assert_panic!(Matrix::<4, T, A>::frustum_rh_gl(
                    left, right, bottom, top, 6.0, 4.0
                ));
            }
        });
    }

    #[test]
    fn test_orthographic_lh() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            let mat = Matrix::<4, T, A>::orthographic_lh(left, right, bottom, top, near, far);

            for ((x, x2), (y, y2), (z, z2)) in iproduct!(
                [(left, -1.0), (right, 1.0), (left.midpoint(right), 0.0)],
                [(bottom, -1.0), (top, 1.0), (bottom.midpoint(top), 0.0)],
                [(near, 0.0), (far, 1.0), (near.midpoint(far), 0.5)]
            ) {
                assert_float_eq!(
                    mat.project_point(Vector::<3, T, A>::new(x, y, z)),
                    Vector::<3, T, A>::new(x2, y2, z2),
                    abs <= Vector::splat(1e-5)
                );
            }

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::orthographic_lh(
                    left, right, bottom, top, 6.0, 4.0
                ));
            }
        });
    }

    #[test]
    fn test_orthographic_rh() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            assert_float_eq!(
                Matrix::<4, T, A>::orthographic_rh(left, right, bottom, top, near, far),
                Matrix::<4, T, A>::orthographic_lh(left, right, bottom, top, near, far)
                    * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::orthographic_rh(
                    left, right, bottom, top, 6.0, 4.0
                ));
            }
        });
    }

    #[test]
    fn test_orthographic_rh_gl() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            assert_float_eq!(
                Matrix::<4, T, A>::orthographic_rh_gl(left, right, bottom, top, near, far),
                Matrix::<4, T, A>::from_translation(Vector::<3, T, A>::NEG_Z)
                    * Matrix::<4, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, 2.0))
                    * Matrix::<4, T, A>::orthographic_rh(left, right, bottom, top, near, far),
                r2nd <= Matrix::<4, T, A>::from_column_array(&[1e-4; 16])
            );

            if cfg!(assertions) {
                assert_panic!(Matrix::<4, T, A>::orthographic_rh_gl(
                    left, right, bottom, top, 6.0, 4.0
                ));
            }
        });
    }

    #[test]
    fn test_to_scale_rotation_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b, c] = [x * 0.3 + 0.5, x + 1.0, y + 2.0, z + 3.0];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<3, T, A>::new(x, y, z);
            let rotation = Quaternion::from_vec(Vector::<4, T, A>::new(x, y, z, w).normalize());
            let translation = Vector::<3, T, A>::new(a, b, c);

            let mat =
                Matrix::<4, T, A>::from_scale_rotation_translation(scale, rotation, translation);

            if mat.determinant() != 0.0 {
                let (scale2, rotation2, translation2) = mat.to_scale_rotation_translation();

                assert_float_eq!(
                    Matrix::<4, T, A>::from_scale_rotation_translation(
                        scale2,
                        rotation2,
                        translation2
                    ),
                    Matrix::<4, T, A>::from_scale_rotation_translation(
                        scale,
                        rotation,
                        translation
                    ),
                    abs <= mat.abs() * 1e-4 + Matrix::<4, T, A>::from_column_array(&[1e-3; 16]),
                    0.0 = -0.0
                );
            } else if cfg!(assertions) {
                assert_panic!(mat.to_scale_rotation_translation());
            }
        });
    }

    #[test]
    fn test_project_point() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let point = Vector::<3, T, A>::new(x, y, z);
            let point4 = Vector::<4, T, A>::new(x, y, z, 1.0);

            for mat in [
                Matrix::<4, T, A>::from_column_array(&[
                    1.0, 5.6, 4.2, 7.4, 3.7, 9.1, 0.1, -0.5, 0.3, 6.4, 3.8, 1.9, 4.1, 8.9, 5.3, 9.6,
                ]),
                Matrix::<4, T, A>::from_column_array(&[
                    2.3, -1.5, 7.8, 0.4, 6.1, 3.3, -2.2, 5.9, 8.7, 1.2, 4.6, -3.8, 9.0, 2.5, 6.4,
                    7.1,
                ]),
                Matrix::<4, T, A>::from_column_array(&[
                    -4.2, 3.1, 5.5, 8.8, 7.7, -6.3, 2.9, 1.0, 0.6, 4.4, -1.1, 9.3, 5.2, 8.6, 3.7,
                    -2.4,
                ]),
            ] {
                assert_float_eq!(
                    mat.project_point(point),
                    (mat * point4).xyz() / (mat * point4).w
                );
            }
        });
    }
}
