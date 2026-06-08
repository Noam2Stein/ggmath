use wide::{CmpEq, CmpGt, CmpNe, f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Alignment, EulerRot, Length, Matrix, Quaternion, SupportedLength, Vector,
    utils::{transmute_generic, transmute_ref},
};

macro_rules! impl_wide_float {
    ($Wide:ident, $T:ident) => {
        impl<const N: usize, A: Alignment> Matrix<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            /// For each lane, returns `true` if any element is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(&self) -> $Wide {
                match N {
                    2 => self[0].is_nan() | self[1].is_nan(),
                    3 => self[0].is_nan() | self[1].is_nan() | self[2].is_nan(),
                    4 => self[0].is_nan() | self[1].is_nan() | self[2].is_nan() | self[3].is_nan(),
                    _ => unreachable!(),
                }
            }

            /// For each lane, returns `true` if all elements are neither
            /// infinite nor NaN.
            #[inline]
            #[must_use]
            pub fn is_finite(&self) -> $Wide {
                match N {
                    2 => self[0].is_finite() & self[1].is_finite(),
                    3 => self[0].is_finite() & self[1].is_finite() & self[2].is_finite(),
                    4 => {
                        self[0].is_finite()
                            & self[1].is_finite()
                            & self[2].is_finite()
                            & self[3].is_finite()
                    }
                    _ => unreachable!(),
                }
            }

            #[inline(always)]
            #[track_caller]
            pub(crate) fn generic_inverse<Output, W, C>(
                &self,
                wrap_result: W,
                check_determinant: C,
            ) -> Output
            where
                W: FnOnce($Wide, Self) -> Output,
                C: FnOnce($Wide) -> Result<(), Output>,
            {
                match N {
                    2 => {
                        // SAFETY: Because `N == 2`, `Matrix<N, $Wide, A>` is `Matrix<2, $Wide, A>`.
                        let matrix = unsafe {
                            transmute_ref::<Matrix<N, $Wide, A>, Matrix<2, $Wide, A>>(self)
                        };

                        let determinant = matrix.determinant();
                        if let Err(error) = check_determinant(determinant) {
                            return error;
                        }

                        let determinant_recip = $Wide::ONE / determinant;
                        let result = Matrix::<2, $Wide, A>::from_row_array(&[
                            matrix.y_axis.y * determinant_recip,
                            matrix.x_axis.y * -determinant_recip,
                            matrix.y_axis.x * -determinant_recip,
                            matrix.x_axis.x * determinant_recip,
                        ]);

                        // SAFETY: Because `N == 2`, `Matrix<2, $Wide, A>` is `Matrix<N, $Wide, A>`.
                        wrap_result(determinant, unsafe {
                            transmute_generic::<Matrix<2, $Wide, A>, Matrix<N, $Wide, A>>(result)
                        })
                    }
                    3 => {
                        // SAFETY: Because `N == 3`, `Matrix<N, $Wide, A>` is `Matrix<3, $Wide, A>`.
                        let matrix = unsafe {
                            transmute_ref::<Matrix<N, $Wide, A>, Matrix<3, $Wide, A>>(self)
                        };

                        let y_cross_z = matrix.y_axis.cross(matrix.z_axis);
                        let z_cross_x = matrix.z_axis.cross(matrix.x_axis);
                        let x_cross_y = matrix.x_axis.cross(matrix.y_axis);

                        let determinant = matrix.z_axis.dot(x_cross_y);
                        if let Err(error) = check_determinant(determinant) {
                            return error;
                        }

                        let determinant_recip =
                            Vector::<3, $Wide, A>::splat($Wide::ONE / determinant);
                        let result = Matrix::<3, $Wide, A>::from_rows(&[
                            y_cross_z * determinant_recip,
                            z_cross_x * determinant_recip,
                            x_cross_y * determinant_recip,
                        ])
                        .transpose();

                        // SAFETY: Because `N == 3`, `Matrix<3, $Wide, A>` is
                        // `Matrix<N, $Wide, A>`.
                        wrap_result(determinant, unsafe {
                            transmute_generic::<Matrix<3, $Wide, A>, Matrix<N, $Wide, A>>(result)
                        })
                    }
                    4 => {
                        // SAFETY: Because `N == 4`, `Matrix<N, $Wide, A>` is `Matrix<4, $Wide, A>`.
                        let matrix = unsafe {
                            transmute_ref::<Matrix<N, $Wide, A>, Matrix<4, $Wide, A>>(self)
                        };

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

                        let fac0 = Vector::<4, $Wide, A>::new(coef00, coef00, coef02, coef03);
                        let fac1 = Vector::<4, $Wide, A>::new(coef04, coef04, coef06, coef07);
                        let fac2 = Vector::<4, $Wide, A>::new(coef08, coef08, coef10, coef11);
                        let fac3 = Vector::<4, $Wide, A>::new(coef12, coef12, coef14, coef15);
                        let fac4 = Vector::<4, $Wide, A>::new(coef16, coef16, coef18, coef19);
                        let fac5 = Vector::<4, $Wide, A>::new(coef20, coef20, coef22, coef23);

                        let vec0 = Vector::<4, $Wide, A>::new(m10, m00, m00, m00);
                        let vec1 = Vector::<4, $Wide, A>::new(m11, m01, m01, m01);
                        let vec2 = Vector::<4, $Wide, A>::new(m12, m02, m02, m02);
                        let vec3 = Vector::<4, $Wide, A>::new(m13, m03, m03, m03);

                        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
                        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
                        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
                        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

                        let sign_a = Vector::<4, $Wide, A>::new(
                            $Wide::ONE,
                            -$Wide::ONE,
                            $Wide::ONE,
                            -$Wide::ONE,
                        );
                        let sign_b = Vector::<4, $Wide, A>::new(
                            -$Wide::ONE,
                            $Wide::ONE,
                            -$Wide::ONE,
                            $Wide::ONE,
                        );

                        let inverse = Matrix::<4, $Wide, A>::from_rows(&[
                            inv0 * sign_a,
                            inv1 * sign_b,
                            inv2 * sign_a,
                            inv3 * sign_b,
                        ]);

                        let col0 = Vector::<4, $Wide, A>::new(
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

                        let determinant_recip = $Wide::ONE / dot1;
                        let result = inverse * determinant_recip;

                        // SAFETY: Because `N == 4`, `Matrix<4, $Wide, A>` is
                        // `Matrix<N, $Wide, A>`.
                        wrap_result(dot1, unsafe {
                            transmute_generic::<Matrix<4, $Wide, A>, Matrix<N, $Wide, A>>(result)
                        })
                    }
                    _ => unreachable!(),
                }
            }

            /// Returns the inverse of `self`.
            ///
            /// If `self` is not invertable the result is unspecified.
            #[must_use]
            #[track_caller]
            pub fn inverse(&self) -> Self {
                self.generic_inverse(|_, result| result, |_| Ok(()))
            }

            /// Returns the inverse of `self` or `None` if `self` is not
            /// invertable for any lane.
            #[must_use]
            pub fn try_inverse(&self) -> Option<Self> {
                self.generic_inverse(
                    |_, result| Some(result),
                    |determinant| {
                        if determinant.simd_eq($Wide::ZERO).any() {
                            Err(None)
                        } else {
                            Ok(())
                        }
                    },
                )
            }

            /// Returns the inverse of `self` or `fallback` if `self` is not
            /// invertable.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            #[must_use]
            pub fn inverse_or(&self, fallback: &Self) -> Self {
                self.generic_inverse(
                    |determinant, result| {
                        let fallback_mask = determinant.simd_eq($Wide::ZERO);
                        Self::from_row_fn(|r| {
                            Vector::from_fn(|c| fallback_mask.blend(fallback[r][c], result[r][c]))
                        })
                    },
                    |_| Ok(()),
                )
            }

            /// Returns the inverse of `self` or the zero matrix if `self` is
            /// not invertable.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            #[must_use]
            pub fn inverse_or_zero(&self) -> Self {
                self.generic_inverse(
                    |determinant, result| {
                        let non_fallback_mask = determinant.simd_ne($Wide::ZERO);
                        Self::from_row_fn(|r| Vector::from_fn(|c| result[r][c] & non_fallback_mask))
                    },
                    |_| Ok(()),
                )
            }

            /// Returns the element-wise reciprocal (inverse) of a matrix,
            /// `1 / self`.
            #[inline]
            #[must_use]
            pub fn recip(&self) -> Self {
                Self::from_row_fn(|i| self[i].recip())
            }

            /// Returns the absolute values of the elements of `self`.
            ///
            /// Equivalent to `(self.x_axis.abs(), self.y_axis.abs(), ...)`.
            #[inline]
            #[must_use]
            pub fn abs(&self) -> Self {
                Self::from_row_fn(|i| self[i].abs())
            }

            /// Returns `true` if the absolute difference of all elements
            /// between `self` and `other` is less than or equal to
            /// `max_abs_diff` for all lanes.
            ///
            /// This can be used to compare two matrices that should be equal,
            /// but may have a slight difference due to operations having
            /// rounding errors.
            #[inline]
            #[must_use]
            pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: $Wide) -> bool {
                match N {
                    2 => {
                        self[0].abs_diff_eq(other[0], max_abs_diff)
                            && self[1].abs_diff_eq(other[1], max_abs_diff)
                    }
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

        impl<A: Alignment> Matrix<2, $Wide, A> {
            /// Creates a matrix containing a rotation of `angle` (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_angle(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<2, $Wide, A>::new(cos, sin),
                    Vector::<2, $Wide, A>::new(-sin, cos),
                ])
            }

            /// Creates a matrix containing the non-uniform `scale` and a
            /// rotation of `angle` (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_scale_angle(scale: Vector<2, $Wide, A>, angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<2, $Wide, A>::new(cos * scale.x, sin * scale.x),
                    Vector::<2, $Wide, A>::new(-sin * scale.y, cos * scale.y),
                ])
            }

            /// Returns the `scale` and `angle` of `self`.
            ///
            /// `self` must not contain shearing. Otherwise the result is
            /// unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn to_scale_angle(&self) -> (Vector<2, $Wide, A>, $Wide) {
                let determinant = self.determinant();

                let scale = Vector::<2, $Wide, A>::new(
                    self.x_axis.length() * determinant.signum(),
                    self.y_axis.length(),
                );

                let angle = (-self.y_axis.x).atan2(self.y_axis.y);

                (scale, angle)
            }
        }

        impl<A: Alignment> Matrix<3, $Wide, A> {
            /// Creates an affine transformation matrix containing a rotation of
            /// `angle` (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 2D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_angle(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos, sin, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin, cos, $Wide::ZERO),
                    Vector::<3, $Wide, A>::Z,
                ])
            }

            /// Creates an affine transformation matrix containing the
            /// non-uniform `scale` and a rotation of `angle` (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 2D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_scale_angle(scale: Vector<2, $Wide, A>, angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos * scale.x, sin * scale.x, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin * scale.y, cos * scale.y, $Wide::ZERO),
                    Vector::<3, $Wide, A>::Z,
                ])
            }

            /// Creates an affine transformation matrix containing a rotation of
            /// `angle` (in radians) and `translation`.
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 2D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_angle_translation(angle: $Wide, translation: Vector<2, $Wide, A>) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos, sin, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin, cos, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(translation.x, translation.y, $Wide::ONE),
                ])
            }

            /// Creates an affine transformation matrix containing the
            /// non-uniform `scale`, a rotation of `angle` (in radians) and
            /// `translation`.
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 2D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_scale_angle_translation(
                scale: Vector<2, $Wide, A>,
                angle: $Wide,
                translation: Vector<2, $Wide, A>,
            ) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos * scale.x, sin * scale.x, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin * scale.y, cos * scale.y, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(translation.x, translation.y, $Wide::ONE),
                ])
            }

            /// Creates a 3D rotation matrix from `angle` (in radians) around
            /// the x axis.
            ///
            /// This rotates `+Y` to `+Z`.
            #[inline]
            #[must_use]
            pub fn from_rotation_x(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::X,
                    Vector::<3, $Wide, A>::new($Wide::ZERO, cos, sin),
                    Vector::<3, $Wide, A>::new($Wide::ZERO, -sin, cos),
                ])
            }

            /// Creates a 3D rotation matrix from `angle` (in radians) around
            /// the y axis.
            ///
            /// This rotates `+Z` to `+X`.
            #[inline]
            #[must_use]
            pub fn from_rotation_y(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos, $Wide::ZERO, -sin),
                    Vector::<3, $Wide, A>::Y,
                    Vector::<3, $Wide, A>::new(sin, $Wide::ZERO, cos),
                ])
            }

            /// Creates a 3D rotation matrix from `angle` (in radians) around
            /// the z axis.
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_rotation_z(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos, sin, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin, cos, $Wide::ZERO),
                    Vector::<3, $Wide, A>::Z,
                ])
            }

            #[track_caller]
            #[inline]
            fn quat_to_axes(quat: Quaternion<$Wide, A>) -> [Vector<3, $Wide, A>; 3] {
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
                    Vector::<3, $Wide, A>::new($Wide::ONE - (yy2 + zz2), xy2 + wz2, xz2 - wy2),
                    Vector::<3, $Wide, A>::new(xy2 - wz2, $Wide::ONE - (xx2 + zz2), yz2 + wx2),
                    Vector::<3, $Wide, A>::new(xz2 + wy2, yz2 - wx2, $Wide::ONE - (xx2 + yy2)),
                ]
            }

            /// Creates a 3D rotation matrix from a quaternion.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_quat(quat: Quaternion<$Wide, A>) -> Self {
                let [x_axis, y_axis, z_axis] = Self::quat_to_axes(quat);
                Self::from_rows(&[x_axis, y_axis, z_axis])
            }

            /// Creates a 3D rotation matrix from a rotation `axis` and `angle`
            /// (in radians).
            ///
            /// `axis` must be normalized. Otherwise the result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                let [xsin, ysin, zsin] = (axis * sin).to_array();
                let [x, y, z] = axis.to_array();
                let [x2, y2, z2] = (axis * axis).to_array();
                let omc = $Wide::ONE - cos;
                let xyomc = x * y * omc;
                let xzomc = x * z * omc;
                let yzomc = y * z * omc;

                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
                    Vector::<3, $Wide, A>::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
                    Vector::<3, $Wide, A>::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
                ])
            }

            /// Creates a 3D rotation matrix from an Euler rotation
            /// order/sequence and angles (in radians).
            #[inline]
            #[must_use]
            pub fn from_euler(order: EulerRot, a: $Wide, b: $Wide, c: $Wide) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs.

                // Based on Ken Shoemake. 1994. Euler angle conversion. Graphics gems IV.
                // Academic Press Professional, Inc., USA, 222–229.

                let order = order.properties();
                let (i, j, k) = order.axes_indices();

                let mut angles = if order.frame_static {
                    Vector::<3, $Wide, A>::new(a, b, c)
                } else {
                    Vector::<3, $Wide, A>::new(c, b, a)
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

            /// Creates a matrix containing a non-uniform `scale` and a 3D
            /// `rotation`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_scale_rotation(
                scale: Vector<3, $Wide, A>,
                rotation: Quaternion<$Wide, A>,
            ) -> Self {
                let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
                Self::from_rows(&[
                    rotation_x * scale.x,
                    rotation_y * scale.y,
                    rotation_z * scale.z,
                ])
            }

            /// Creates a left-handed view matrix from a facing direction and an
            /// up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=forward`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_to_lh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
                let forward = dir;
                let right = up.cross(forward).normalize();
                let up = forward.cross(right);

                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(right.x, up.x, forward.x),
                    Vector::<3, $Wide, A>::new(right.y, up.y, forward.y),
                    Vector::<3, $Wide, A>::new(right.z, up.z, forward.z),
                ])
            }

            /// Creates a right-handed view matrix from a facing direction and
            /// an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_to_rh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
                let forward = dir;
                let right = forward.cross(up).normalize();
                let up = right.cross(forward);

                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(right.x, up.x, -forward.x),
                    Vector::<3, $Wide, A>::new(right.y, up.y, -forward.y),
                    Vector::<3, $Wide, A>::new(right.z, up.z, -forward.z),
                ])
            }

            /// Creates a left-handed view matrix from a camera position, a
            /// focal point and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=forward`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_at_lh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::look_to_lh((center - eye).normalize(), up)
            }

            /// Creates a right-handed view matrix from a camera position, a
            /// focal point and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_at_rh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::look_to_rh((center - eye).normalize(), up)
            }

            /// For each lane, returns the `scale`, `angle` and `translation` of
            /// `self`.
            ///
            /// `self` must contain a valid affine transformation without
            /// shearing. Otherwise the result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn to_scale_angle_translation(
                &self,
            ) -> (Vector<2, $Wide, A>, $Wide, Vector<2, $Wide, A>) {
                let determinant = self.determinant();

                let scale = Vector::<2, $Wide, A>::new(
                    self.x_axis.length() * determinant.signum(),
                    self.y_axis.length(),
                );

                let angle = (-self.y_axis.x).atan2(self.y_axis.y);

                let translation = self.z_axis.xy();

                (scale, angle, translation)
            }

            /// Returns the Euler angles forming `self` for the given Euler
            /// rotation order/sequence.
            ///
            /// `self` must not contain any non-rotation transformations.
            /// Otherwise the result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn to_euler(&self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
                // Ported from https://github.com/bitshifter/glam-rs.

                // Based on Ken Shoemake. 1994. Euler angle conversion. Graphics
                // gems IV. Academic Press Professional, Inc., USA, 222–229.

                let order = order.properties();
                let (i, j, k) = order.axes_indices();

                let mut ea = Vector::<3, $Wide, A>::ZERO;
                if order.initial_repeated {
                    let sy = (self[i][j] * self[i][j] + self[i][k] * self[i][k]).sqrt();

                    let mask = sy.simd_gt($Wide::splat(16.0 * $T::EPSILON));
                    ea.x = mask.blend(
                        self[i][j].atan2(self[i][k]),
                        (-self[j][k]).atan2(self[j][j]),
                    );
                    ea.y = sy.atan2(self[i][i]);
                    ea.z = mask & self[j][i].atan2(-self[k][i]);
                } else {
                    let cy = (self[i][i] * self[i][i] + self[j][i] * self[j][i]).sqrt();

                    let mask = cy.simd_gt($Wide::splat(16.0 * $T::EPSILON));
                    ea.x = mask.blend(
                        self[k][j].atan2(self[k][k]),
                        (-self[j][k]).atan2(self[j][j]),
                    );
                    ea.y = (-self[k][i]).atan2(cy);
                    ea.z = mask & self[j][i].atan2(self[i][i]);
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

            /// For each lane, returns the `scale` and `rotation` of `self`.
            ///
            /// `self` must not contain shearing. Otherwise the result is
            /// unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn to_scale_rotation(&self) -> (Vector<3, $Wide, A>, Quaternion<$Wide, A>) {
                let determinant = self.determinant();

                let scale = Vector::<3, $Wide, A>::new(
                    self.x_axis.length() * determinant.signum(),
                    self.y_axis.length(),
                    self.z_axis.length(),
                );

                let scale_recip = scale.recip();

                let rotation = Quaternion::<$Wide, A>::from_matrix(&Self::from_rows(&[
                    self.x_axis * scale_recip.x,
                    self.y_axis * scale_recip.y,
                    self.z_axis * scale_recip.z,
                ]));

                (scale, rotation)
            }
        }

        impl<A: Alignment> Matrix<4, $Wide, A> {
            /// Creates an affine transformation matrix containing a 3D rotation
            /// from `angle` (in radians) around the x axis.
            ///
            /// This rotates `+Y` to `+Z`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_rotation_x(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<4, $Wide, A>::X,
                    Vector::<4, $Wide, A>::new($Wide::ZERO, cos, sin, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, -sin, cos, $Wide::ZERO),
                    Vector::<4, $Wide, A>::W,
                ])
            }

            /// Creates an affine transformation matrix containing a 3D rotation
            /// from `angle` (in radians) around the y axis.
            ///
            /// This rotates `+Z` to `+X`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_rotation_y(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(cos, $Wide::ZERO, -sin, $Wide::ZERO),
                    Vector::<4, $Wide, A>::Y,
                    Vector::<4, $Wide, A>::new(sin, $Wide::ZERO, cos, $Wide::ZERO),
                    Vector::<4, $Wide, A>::W,
                ])
            }

            /// Creates an affine transformation matrix containing a 3D rotation
            /// from `angle` (in radians) around the z axis.
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_rotation_z(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(cos, sin, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(-sin, cos, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::Z,
                    Vector::<4, $Wide, A>::W,
                ])
            }

            #[inline]
            #[track_caller]
            fn quat_to_axes(quat: Quaternion<$Wide, A>) -> [Vector<4, $Wide, A>; 3] {
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
                    Vector::<4, $Wide, A>::new(
                        $Wide::ONE - (yy2 + zz2),
                        xy2 + wz2,
                        xz2 - wy2,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        xy2 - wz2,
                        $Wide::ONE - (xx2 + zz2),
                        yz2 + wx2,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        xz2 + wy2,
                        yz2 - wx2,
                        $Wide::ONE - (xx2 + yy2),
                        $Wide::ZERO,
                    ),
                ]
            }

            /// Creates an affine transformation matrix containing a 3D rotation
            /// from a quaternion.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_quat(quat: Quaternion<$Wide, A>) -> Self {
                let [x_axis, y_axis, z_axis] = Self::quat_to_axes(quat);
                Self::from_rows(&[x_axis, y_axis, z_axis, Vector::W])
            }

            /// Creates an affine transformation matrix containing a rotation
            /// from a rotation `axis` and `angle` (in radians).
            ///
            /// `axis` must be normalized. Otherwise the result is unspecified.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                let [xsin, ysin, zsin] = (axis * sin).to_array();
                let [x, y, z] = axis.to_array();
                let [x2, y2, z2] = (axis * axis).to_array();
                let omc = $Wide::ONE - cos;
                let xyomc = x * y * omc;
                let xzomc = x * z * omc;
                let yzomc = y * z * omc;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        x2 * omc + cos,
                        xyomc + zsin,
                        xzomc - ysin,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        xyomc - zsin,
                        y2 * omc + cos,
                        yzomc + xsin,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        xzomc + ysin,
                        yzomc - xsin,
                        z2 * omc + cos,
                        $Wide::ZERO,
                    ),
                    Vector::W,
                ])
            }

            /// Creates an affine transformation matrix containing a rotation
            /// from an Euler rotation order/sequence and angles (in radians).
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_euler(order: EulerRot, a: $Wide, b: $Wide, c: $Wide) -> Self {
                Self::from_submatrix(&Matrix::<3, $Wide, A>::from_euler(order, a, b, c))
            }

            /// Creates an affine transformation matrix containing a non-uniform
            /// `scale` and a 3D `rotation`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_scale_rotation(
                scale: Vector<3, $Wide, A>,
                rotation: Quaternion<$Wide, A>,
            ) -> Self {
                let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
                Self::from_rows(&[
                    rotation_x * scale.x,
                    rotation_y * scale.y,
                    rotation_z * scale.z,
                    Vector::W,
                ])
            }

            /// Creates an affine transformation matrix containing a 3D
            /// `rotation` and `translation`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_rotation_translation(
                rotation: Quaternion<$Wide, A>,
                translation: Vector<3, $Wide, A>,
            ) -> Self {
                let [x_axis, y_axis, z_axis] = Self::quat_to_axes(rotation);
                Self::from_rows(&[
                    x_axis,
                    y_axis,
                    z_axis,
                    Vector::<4, $Wide, A>::new(
                        translation.x,
                        translation.y,
                        translation.z,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates an affine transformation matrix containing the
            /// non-uniform `scale`, a 3D `rotation` and `translation`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_scale_rotation_translation(
                scale: Vector<3, $Wide, A>,
                rotation: Quaternion<$Wide, A>,
                translation: Vector<3, $Wide, A>,
            ) -> Self {
                let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
                Self::from_rows(&[
                    rotation_x * scale.x,
                    rotation_y * scale.y,
                    rotation_z * scale.z,
                    Vector::<4, $Wide, A>::new(
                        translation.x,
                        translation.y,
                        translation.z,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a left-handed view matrix from a camera position, a
            /// facing direction and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=forward`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_to_lh(
                eye: Vector<3, $Wide, A>,
                dir: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                let forward = dir;
                let right = up.cross(forward).normalize();
                let up = forward.cross(right);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(right.x, up.x, forward.x, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(right.y, up.y, forward.y, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(right.z, up.z, forward.z, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        -eye.dot(right),
                        -eye.dot(up),
                        -eye.dot(forward),
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a right-handed view matrix from a camera position, a
            /// facing direction and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_to_rh(
                eye: Vector<3, $Wide, A>,
                dir: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                let forward = dir;
                let right = forward.cross(up).normalize();
                let up = right.cross(forward);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(right.x, up.x, -forward.x, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(right.y, up.y, -forward.y, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(right.z, up.z, -forward.z, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        -eye.dot(right),
                        -eye.dot(up),
                        eye.dot(forward),
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a left-handed view matrix from a camera position, a
            /// focal point and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=forward`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_at_lh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::look_to_lh(eye, (center - eye).normalize(), up)
            }

            /// Creates a right-handed view matrix from a camera position, a
            /// focal point and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_at_rh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::look_to_rh(eye, (center - eye).normalize(), up)
            }

            /// Creates a left-handed perspective projection matrix with `0..1`
            /// depth range.
            ///
            /// Useful to map the standard left-handed coordinate system into
            /// what WebGPU/Metal/Direct3D expect.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn perspective_lh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::splat(0.5)).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;
                let depth_scale = far_plane / (far_plane - near_plane);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, depth_scale, $Wide::ONE),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        -depth_scale * near_plane,
                        $Wide::ZERO,
                    ),
                ])
            }

            /// Creates a right-handed perspective projection matrix with `0..1`
            /// depth range.
            ///
            /// Useful to map the standard right-handed coordinate system into
            /// what WebGPU/Metal/Direct3D expect.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn perspective_rh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;
                let neg_depth_scale = far_plane / (near_plane - far_plane);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        neg_depth_scale,
                        -$Wide::ONE,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        neg_depth_scale * near_plane,
                        $Wide::ZERO,
                    ),
                ])
            }

            /// Creates a right-handed perspective projection matrix with
            /// `-1..1` depth range.
            ///
            /// Equivalent to the OpenGL [`gluPerspective`] function.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`gluPerspective`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn perspective_rh_gl(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;
                let depth_recip = $Wide::ONE / (near_plane - far_plane);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        (near_plane + far_plane) * depth_recip,
                        -$Wide::ONE,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::splat(2.0) * near_plane * far_plane * depth_recip,
                        $Wide::ZERO,
                    ),
                ])
            }

            /// Creates an infinite left-handed perspective projection matrix
            /// with `0..1` depth range.
            ///
            /// Equivalent to `perspective_lh`, but with an infinite value for
            /// `far_plane`. The result is that points near `near_plane` have
            /// depth `0`, and as they move towards infinity the depth
            /// approaches `1`.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn perspective_infinite_lh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, $Wide::ONE, $Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, -near_plane, $Wide::ZERO),
                ])
            }

            /// Creates an infinite right-handed perspective projection matrix
            /// with `0..1` depth range.
            ///
            /// Equivalent to `perspective_rh`, but with an infinite value for
            /// `far_plane`. The result is that points near `near_plane` have
            /// depth `0`, and as they move towards infinity the depth
            /// approaches `1`.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn perspective_infinite_rh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, -$Wide::ONE, -$Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, -near_plane, $Wide::ZERO),
                ])
            }

            /// Creates an infinite left-handed perspective projection matrix
            /// with reversed `0..1` depth range.
            ///
            /// Equivalent to `perspective_infinite_lh`, but maps points at
            /// `near_plane` to depth `1` and points at infinity to depth `0`.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn perspective_infinite_reverse_lh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, $Wide::ZERO, $Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, near_plane, $Wide::ZERO),
                ])
            }

            /// Creates an infinite right-handed perspective projection matrix
            /// with reversed `0..1` depth range.
            ///
            /// Equivalent to `perspective_infinite_rh`, but maps points at
            /// `near_plane` to depth `1` and points at infinity to depth `0`.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn perspective_infinite_reverse_rh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, $Wide::ZERO, -$Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, near_plane, $Wide::ZERO),
                ])
            }

            /// Creates a left-handed perspective projection matrix with `0..1`
            /// depth range.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn frustum_lh(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let depth_recip = $Wide::ONE / (far_plane - near_plane);
                let two_near_plane = $Wide::splat(2.0) * near_plane;
                let a = (right + left) * width_recip;
                let b = (top + bottom) * height_recip;
                let c = far_plane * depth_recip;
                let d = -near_plane * c;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        two_near_plane * width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        two_near_plane * height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(a, b, c, $Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, d, $Wide::ZERO),
                ])
            }

            /// Creates a right-handed perspective projection matrix with `0..1`
            /// depth range.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn frustum_rh(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let depth_recip = $Wide::ONE / (far_plane - near_plane);
                let two_near_plane = $Wide::splat(2.0) * near_plane;
                let a = (right + left) * width_recip;
                let b = (top + bottom) * height_recip;
                let c = -far_plane * depth_recip;
                let d = near_plane * c;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        two_near_plane * width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        two_near_plane * height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(a, b, c, -$Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, d, $Wide::ZERO),
                ])
            }

            /// Creates a right-handed perspective projection matrix with
            /// `-1..1` depth range.
            ///
            /// Equivalent to the OpenGL [`glFrustum`] function.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`glFrustum`]: https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn frustum_rh_gl(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let depth_recip = $Wide::ONE / (far_plane - near_plane);
                let two_near_plane = $Wide::splat(2.0) * near_plane;
                let a = (right + left) * width_recip;
                let b = (top + bottom) * height_recip;
                let c = -(far_plane + near_plane) * depth_recip;
                let d = -two_near_plane * far_plane * depth_recip;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        two_near_plane * width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        two_near_plane * height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(a, b, c, -$Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, d, $Wide::ZERO),
                ])
            }

            /// Creates a left-handed orthographic projection matrix with `0..1`
            /// depth range.
            ///
            /// Useful to map a left-handed coordinate system into what
            /// WebGPU/Metal/Direct3D expect.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn orthographic_lh(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near: $Wide,
                far: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let depth_recip = $Wide::ONE / (far - near);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        width_recip + width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        height_recip + height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, depth_recip, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        -(left + right) * width_recip,
                        -(top + bottom) * height_recip,
                        -depth_recip * near,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a right-handed orthographic projection matrix with
            /// `0..1` depth range.
            ///
            /// Useful to map a right-handed coordinate system into what
            /// WebGPU/Metal/Direct3D expect.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn orthographic_rh(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near: $Wide,
                far: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let neg_depth_recip = $Wide::ONE / (near - far);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        width_recip + width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        height_recip + height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        neg_depth_recip,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        -(left + right) * width_recip,
                        -(top + bottom) * height_recip,
                        neg_depth_recip * near,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a right-handed orthographic projection matrix with
            /// `-1..1` depth range.
            ///
            /// Equivalent to the OpenGL [`glOrtho`] function.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`glOrtho`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn orthographic_rh_gl(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near: $Wide,
                far: $Wide,
            ) -> Self {
                let scale_x = $Wide::splat(2.0) / (right - left);
                let scale_y = $Wide::splat(2.0) / (top - bottom);
                let scale_z = $Wide::splat(2.0) / (near - far);
                let translation_x = -(right + left) / (right - left);
                let translation_y = -(top + bottom) / (top - bottom);
                let translation_z = -(far + near) / (far - near);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(scale_x, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, scale_y, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, scale_z, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        translation_x,
                        translation_y,
                        translation_z,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Returns the Euler angles forming `self` for the given Euler
            /// rotation order/sequence.
            ///
            /// The upper 3x3 matrix must not contain any non-rotation
            /// transformations. Otherwise the result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn to_euler(&self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
                self.submatrix().to_euler(order)
            }

            /// For each lane, returns the `scale`, `rotation` and `translation`
            /// of `self`.
            ///
            /// `self` must contain a valid affine transformation. Otherwise the
            /// result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn to_scale_rotation_translation(
                &self,
            ) -> (
                Vector<3, $Wide, A>,
                Quaternion<$Wide, A>,
                Vector<3, $Wide, A>,
            ) {
                let determinant = self.determinant();

                let scale = Vector::<3, $Wide, A>::new(
                    self.x_axis.length() * determinant.signum(),
                    self.y_axis.length(),
                    self.z_axis.length(),
                );

                let scale_recip = scale.recip();

                let rotation = Quaternion::<$Wide, A>::from_matrix(&Matrix::from_rows(&[
                    self.x_axis.xyz() * scale_recip.x,
                    self.y_axis.xyz() * scale_recip.y,
                    self.z_axis.xyz() * scale_recip.z,
                ]));

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
            pub fn project_point(&self, point: Vector<3, $Wide, A>) -> Vector<3, $Wide, A> {
                let result = self.x_axis * point.x
                    + self.y_axis * point.y
                    + self.z_axis * point.z
                    + self.w_axis;

                (result / result.w).xyz()
            }
        }
    };
}
impl_wide_float!(f32x4, f32);
impl_wide_float!(f32x8, f32);
impl_wide_float!(f32x16, f32);
impl_wide_float!(f64x2, f64);
impl_wide_float!(f64x4, f64);
impl_wide_float!(f64x8, f64);

#[cfg(test)]
mod tests {
    use wide::CmpLt;

    use crate::{
        EulerRot, Mat2, Mat3, Mat4, Matrix, Quat, Unaligned, Vec2, Vec3, Vector,
        utils::{
            assert_panic_test_eq, assert_test_eq, assert_test_eq_or_panic, for_types, random_iter,
        },
    };

    #[test]
    fn test_is_nan() {
        for_types!(|Wide: WideFloat| {
            for [x, y, z, w] in random_iter::<[Wide; 4]>() {
                assert_test_eq!(
                    Mat2::from_rows(&[x, y].map(Vector::splat)).is_nan(),
                    x.is_nan() | y.is_nan()
                );
                assert_test_eq!(
                    Mat3::from_rows(&[x, y, z].map(Vector::splat)).is_nan(),
                    x.is_nan() | y.is_nan() | z.is_nan()
                );
                assert_test_eq!(
                    Mat4::from_rows(&[x, y, z, w].map(Vector::splat)).is_nan(),
                    x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan()
                );
            }
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|Wide: WideFloat| {
            for [x, y, z, w] in random_iter::<[Wide; 4]>() {
                assert_test_eq!(
                    Mat2::from_rows(&[x, y].map(Vector::splat)).is_finite(),
                    x.is_finite() & y.is_finite()
                );
                assert_test_eq!(
                    Mat3::from_rows(&[x, y, z].map(Vector::splat)).is_finite(),
                    x.is_finite() & y.is_finite() & z.is_finite()
                );
                assert_test_eq!(
                    Mat4::from_rows(&[x, y, z, w].map(Vector::splat)).is_finite(),
                    x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
                );
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_test_eq_or_panic!(
                    matrix.inverse(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).inverse())
                );
            }
        });
    }

    #[test]
    fn test_try_inverse() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_panic_test_eq!(
                    matrix.try_inverse().unwrap(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).try_inverse().unwrap())
                );
            }
        });
    }

    #[test]
    fn test_inverse_or() {
        for_types!(|N, Wide: WideFloat| {
            for [matrix, fallback] in random_iter::<[Matrix<N, Wide, Unaligned>; 2]>() {
                assert_test_eq!(
                    matrix.inverse_or(&fallback),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).inverse_or(&fallback.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    matrix.inverse_or_zero(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).inverse_or_zero())
                );
            }
        });
    }

    #[test]
    fn test_recip() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    matrix.recip(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).recip())
                );
            }
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    matrix.abs(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).abs())
                );
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|N, Wide: WideFloat| {
            for ([a, b], max_abs_diff) in random_iter::<([Matrix<N, Wide, Unaligned>; 2], Wide)>() {
                assert_test_eq!(
                    a.abs_diff_eq(&b, max_abs_diff),
                    (0..LANES).all(|lane| a
                        .lane(lane)
                        .abs_diff_eq(&b.lane(lane), max_abs_diff.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_from_angle() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Mat2::<Wide>::from_angle(angle),
                    Mat2::from_lane_fn(|lane| Mat2::<T>::from_angle(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Mat3::<Wide>::from_angle(angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_angle(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle() {
        for_types!(|Wide: WideFloat| {
            for (scale, angle) in random_iter::<(Vec2<Wide>, Wide)>() {
                let scale = Vec2::splat(scale.length().is_finite()).blend(scale, Vec2::ONE);

                assert_test_eq!(
                    Mat2::<Wide>::from_scale_angle(scale, angle),
                    Mat2::from_lane_fn(|lane| Mat2::<T>::from_scale_angle(
                        scale.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= (scale.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0,
                    INFINITY = NAN
                );
                assert_test_eq!(
                    Mat3::<Wide>::from_scale_angle(scale, angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_scale_angle(
                        scale.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= (scale.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle() {
        for_types!(|Wide: WideFloat| {
            for matrix in random_iter::<Mat2<Wide>>().chain(
                random_iter::<(Vec2<Wide>, Wide)>()
                    .map(|(scale, angle)| Mat2::<Wide>::from_scale_angle(scale, angle)),
            ) {
                assert_test_eq_or_panic!(
                    matrix.to_scale_angle(),
                    (
                        Vec2::from_lane_fn(|lane| matrix.lane(lane).to_scale_angle().0),
                        Wide::new(std::array::from_fn(|lane| matrix
                            .lane(lane)
                            .to_scale_angle()
                            .1))
                    ),
                    abs <= (
                        matrix.to_scale_angle().0.abs() * Wide::splat(1e-4) + Wide::splat(1e-3),
                        matrix.to_scale_angle().1.abs() * 1e-4 + 1e-3
                    )
                );
            }
        });
    }

    #[test]
    fn test_from_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for (angle, translation) in random_iter::<(Wide, Vec2<Wide>)>() {
                assert_test_eq!(
                    Mat3::<Wide>::from_angle_translation(angle, translation),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_angle_translation(
                        angle.to_array()[lane],
                        translation.lane(lane)
                    )),
                    abs <= angle.abs() * 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for (scale, angle, translation) in random_iter::<(Vec2<Wide>, Wide, Vec2<Wide>)>() {
                let scale = Vec2::splat(scale.length().is_finite()).blend(scale, Vec2::ONE);

                assert_test_eq!(
                    Mat3::<Wide>::from_scale_angle_translation(scale, angle, translation),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_scale_angle_translation(
                        scale.lane(lane),
                        angle.to_array()[lane],
                        translation.lane(lane)
                    )),
                    abs <= (scale.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Mat3::<Wide>::from_rotation_x(angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_rotation_x(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Mat4::<Wide>::from_rotation_x(angle),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::from_rotation_x(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Mat3::<Wide>::from_rotation_y(angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_rotation_y(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Mat4::<Wide>::from_rotation_y(angle),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::from_rotation_y(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Mat3::<Wide>::from_rotation_z(angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_rotation_z(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Mat4::<Wide>::from_rotation_z(angle),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::from_rotation_z(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_quat() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>().flat_map(|quat| [quat, quat.normalize()]) {
                assert_test_eq_or_panic!(
                    Mat3::<Wide>::from_quat(quat),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_quat(quat.lane(lane)))
                );
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::from_quat(quat),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::from_quat(quat.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_types!(|Wide: WideFloat| {
            for (axis, angle) in random_iter::<(Vec3<Wide>, Wide)>()
                .flat_map(|(axis, angle)| [(axis, angle), (axis.normalize(), angle)])
            {
                let condition =
                    axis.length().is_finite() & angle.is_finite() & angle.abs().simd_lt(1e3);
                let axis = Vec3::splat(condition).blend(axis, Vec3::X);
                let angle = condition.blend(angle, Wide::ONE);

                assert_test_eq_or_panic!(
                    Mat3::<Wide>::from_axis_angle(axis, angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_axis_angle(
                        axis.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= Mat3::<Wide>::from_axis_angle(axis, angle).abs()
                        * axis.length().max(Wide::ONE)
                        * angle.abs().max(Wide::ONE)
                        * Wide::splat(1e-4)
                        + Mat3::from_row_array(&[Wide::splat(1e-3); 9]),
                    0.0 = -0.0
                );
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::from_axis_angle(axis, angle),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::from_axis_angle(
                        axis.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= Mat4::<Wide>::from_axis_angle(axis, angle).abs()
                        * axis.length().max(Wide::ONE)
                        * angle.abs().max(Wide::ONE)
                        * Wide::splat(1e-4)
                        + Mat4::from_row_array(&[Wide::splat(1e-3); 16]),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_euler() {
        for_types!(|Wide: WideFloat| {
            for order in EulerRot::values() {
                for [a, b, c] in random_iter::<[Wide; 3]>() {
                    assert_test_eq!(
                        Mat3::<Wide>::from_euler(order, a, b, c),
                        Mat3::from_lane_fn(|lane| Mat3::<T>::from_euler(
                            order,
                            a.to_array()[lane],
                            b.to_array()[lane],
                            c.to_array()[lane]
                        )),
                        abs <= a.abs().max(b.abs()).max(c.abs()) * 1e-4,
                        0.0 = -0.0
                    );
                    assert_test_eq!(
                        Mat4::<Wide>::from_euler(order, a, b, c),
                        Mat4::from_lane_fn(|lane| Mat4::<T>::from_euler(
                            order,
                            a.to_array()[lane],
                            b.to_array()[lane],
                            c.to_array()[lane]
                        )),
                        abs <= a.abs().max(b.abs()).max(c.abs()) * 1e-4,
                        0.0 = -0.0
                    );
                }
            }
        });
    }

    #[test]
    fn test_from_scale_rotation() {
        for_types!(|Wide: WideFloat| {
            for (scale, rotation) in random_iter::<(Vec3<Wide>, Quat<Wide>)>()
                .flat_map(|(scale, quat)| [(scale, quat), (scale, quat.normalize())])
            {
                assert_test_eq_or_panic!(
                    Mat3::<Wide>::from_scale_rotation(scale, rotation),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_scale_rotation(
                        scale.lane(lane),
                        rotation.lane(lane)
                    ))
                );
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::from_scale_rotation(scale, rotation),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::from_scale_rotation(
                        scale.lane(lane),
                        rotation.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|Wide: WideFloat| {
            for [eye, dir, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|[eye, dir, up]| [[eye, dir, up], [eye, dir.normalize(), up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Mat3::<Wide>::look_to_lh(dir, up),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::look_to_lh(dir.lane(lane), up.lane(lane)))
                );
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::look_to_lh(eye, dir, up),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::look_to_lh(
                        eye.lane(lane),
                        dir.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|Wide: WideFloat| {
            for [eye, dir, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|[eye, dir, up]| [[eye, dir, up], [eye, dir.normalize(), up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Mat3::<Wide>::look_to_rh(dir, up),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::look_to_rh(dir.lane(lane), up.lane(lane)))
                );
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::look_to_rh(eye, dir, up),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::look_to_rh(
                        eye.lane(lane),
                        dir.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|Wide: WideFloat| {
            for [eye, center, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|[eye, center, up]| [[eye, center, up], [eye, center, up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Mat3::<Wide>::look_at_lh(eye, center, up),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::look_at_lh(
                        eye.lane(lane),
                        center.lane(lane),
                        up.lane(lane)
                    ))
                );
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::look_at_lh(eye, center, up),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::look_at_lh(
                        eye.lane(lane),
                        center.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_look_at_rh() {
        for_types!(|Wide: WideFloat| {
            for [eye, center, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|[eye, center, up]| [[eye, center, up], [eye, center, up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Mat3::<Wide>::look_at_rh(eye, center, up),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::look_at_rh(
                        eye.lane(lane),
                        center.lane(lane),
                        up.lane(lane)
                    ))
                );
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::look_at_rh(eye, center, up),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::look_at_rh(
                        eye.lane(lane),
                        center.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for matrix in random_iter::<Mat3<Wide>>().chain(
                random_iter::<(Vec2<Wide>, Wide, Vec2<Wide>)>().map(
                    |(scale, angle, translation)| {
                        Mat3::<Wide>::from_scale_angle_translation(scale, angle, translation)
                    },
                ),
            ) {
                assert_test_eq_or_panic!(
                    matrix.to_scale_angle_translation(),
                    (
                        Vec2::from_lane_fn(|lane| matrix.lane(lane).to_scale_angle_translation().0),
                        Wide::new(std::array::from_fn(|lane| matrix
                            .lane(lane)
                            .to_scale_angle_translation()
                            .1)),
                        Vec2::from_lane_fn(|lane| matrix.lane(lane).to_scale_angle_translation().2)
                    ),
                    abs <= (
                        matrix.to_scale_angle_translation().0.abs() * Wide::splat(1e-4)
                            + Wide::splat(1e-3),
                        matrix.to_scale_angle_translation().1.abs() * 1e-4 + 1e-3,
                        Vector::ZERO
                    )
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|Wide: WideFloat| {
            for order in EulerRot::values() {
                for matrix in random_iter::<Mat4<Wide>>().chain(
                    random_iter::<[Wide; 3]>()
                        .map(|[a, b, c]| Mat4::<Wide>::from_euler(order, a, b, c)),
                ) {
                    assert_test_eq_or_panic!(
                        matrix.to_euler(order),
                        (
                            Wide::new(std::array::from_fn(|lane| matrix
                                .lane(lane)
                                .to_euler(order)
                                .0)),
                            Wide::new(std::array::from_fn(|lane| matrix
                                .lane(lane)
                                .to_euler(order)
                                .1)),
                            Wide::new(std::array::from_fn(|lane| matrix
                                .lane(lane)
                                .to_euler(order)
                                .2))
                        ),
                        abs <= (Wide::splat(1e-4), Wide::splat(1e-4), Wide::splat(1e-4)),
                        0.0 = -0.0
                    );

                    let matrix = matrix.submatrix();
                    assert_test_eq_or_panic!(
                        matrix.to_euler(order),
                        (
                            Wide::new(std::array::from_fn(|lane| matrix
                                .lane(lane)
                                .to_euler(order)
                                .0)),
                            Wide::new(std::array::from_fn(|lane| matrix
                                .lane(lane)
                                .to_euler(order)
                                .1)),
                            Wide::new(std::array::from_fn(|lane| matrix
                                .lane(lane)
                                .to_euler(order)
                                .2))
                        ),
                        abs <= (Wide::splat(1e-4), Wide::splat(1e-4), Wide::splat(1e-4)),
                        0.0 = -0.0
                    );
                }
            }
        });
    }

    #[test]
    fn test_to_scale_rotation() {
        for_types!(|Wide: WideFloat| {
            for matrix in random_iter::<Mat3<Wide>>().chain(
                random_iter::<(Vec3<Wide>, Quat<Wide>)>().map(|(scale, rotation)| {
                    Mat3::<Wide>::from_scale_rotation(scale, rotation.normalize())
                }),
            ) {
                assert_test_eq_or_panic!(
                    matrix.to_scale_rotation(),
                    (
                        Vec3::from_lane_fn(|lane| matrix.lane(lane).to_scale_rotation().0),
                        Quat::from_lane_fn(|lane| matrix.lane(lane).to_scale_rotation().1)
                    )
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_translation() {
        for_types!(|Wide: WideFloat| {
            for (rotation, translation) in
                random_iter::<(Quat<Wide>, Vec3<Wide>)>().flat_map(|(rotation, translation)| {
                    [(rotation, translation), (rotation.normalize(), translation)]
                })
            {
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::from_rotation_translation(rotation, translation),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::from_rotation_translation(
                        rotation.lane(lane),
                        translation.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        for_types!(|Wide: WideFloat| {
            for (scale, rotation, translation) in
                random_iter::<(Vec3<Wide>, Quat<Wide>, Vec3<Wide>)>().flat_map(
                    |(scale, rotation, translation)| {
                        [
                            (scale, rotation, translation),
                            (scale, rotation.normalize(), translation),
                        ]
                    },
                )
            {
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::from_scale_rotation_translation(scale, rotation, translation),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::from_scale_rotation_translation(
                        scale.lane(lane),
                        rotation.lane(lane),
                        translation.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_perspective_lh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, far_plane, aspect_ratio] in random_iter::<[Wide; 4]>() {
                let [vertical_fov, near_plane, far_plane, aspect_ratio] =
                    [vertical_fov, near_plane, far_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).blend(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Mat4::<Wide>::perspective_lh(vertical_fov, aspect_ratio, near_plane, far_plane),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::perspective_lh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    )),
                    abs <= Mat4::<Wide>::perspective_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Mat4::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_rh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, far_plane, aspect_ratio] in random_iter::<[Wide; 4]>() {
                let [vertical_fov, near_plane, far_plane, aspect_ratio] =
                    [vertical_fov, near_plane, far_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).blend(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Mat4::<Wide>::perspective_rh(vertical_fov, aspect_ratio, near_plane, far_plane),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::perspective_rh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    )),
                    abs <= Mat4::<Wide>::perspective_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Mat4::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_rh_gl() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, far_plane, aspect_ratio] in random_iter::<[Wide; 4]>() {
                let [vertical_fov, near_plane, far_plane, aspect_ratio] =
                    [vertical_fov, near_plane, far_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).blend(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Mat4::<Wide>::perspective_rh_gl(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    ),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::perspective_rh_gl(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    )),
                    abs <= Mat4::<Wide>::perspective_rh_gl(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Mat4::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_lh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, aspect_ratio] in random_iter::<[Wide; 3]>() {
                let [vertical_fov, near_plane, aspect_ratio] =
                    [vertical_fov, near_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).blend(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Mat4::<Wide>::perspective_infinite_lh(vertical_fov, aspect_ratio, near_plane),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::perspective_infinite_lh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane]
                    )),
                    abs <= Mat4::<Wide>::perspective_infinite_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Mat4::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_rh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, aspect_ratio] in random_iter::<[Wide; 3]>() {
                let [vertical_fov, near_plane, aspect_ratio] =
                    [vertical_fov, near_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).blend(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Mat4::<Wide>::perspective_infinite_rh(vertical_fov, aspect_ratio, near_plane),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::perspective_infinite_rh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane]
                    )),
                    abs <= Mat4::<Wide>::perspective_infinite_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Mat4::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_lh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, aspect_ratio] in random_iter::<[Wide; 3]>() {
                let [vertical_fov, near_plane, aspect_ratio] =
                    [vertical_fov, near_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).blend(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Mat4::<Wide>::perspective_infinite_reverse_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::perspective_infinite_reverse_lh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane]
                    )),
                    abs <= Mat4::<Wide>::perspective_infinite_reverse_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Mat4::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_rh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, aspect_ratio] in random_iter::<[Wide; 3]>() {
                let [vertical_fov, near_plane, aspect_ratio] =
                    [vertical_fov, near_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).blend(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Mat4::<Wide>::perspective_infinite_reverse_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::perspective_infinite_reverse_rh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane]
                    )),
                    abs <= Mat4::<Wide>::perspective_infinite_reverse_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Mat4::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_frustum_lh() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near_plane, far_plane] in random_iter::<[Wide; 6]>()
                .flat_map(|[left, right, bottom, top, near_plane, far_plane]| {
                    [
                        [left, right, bottom, top, near_plane, far_plane],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near_plane.min(far_plane),
                            near_plane.max(far_plane),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::frustum_lh(left, right, bottom, top, near_plane, far_plane),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::frustum_lh(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_frustum_rh() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near_plane, far_plane] in random_iter::<[Wide; 6]>()
                .flat_map(|[left, right, bottom, top, near_plane, far_plane]| {
                    [
                        [left, right, bottom, top, near_plane, far_plane],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near_plane.min(far_plane),
                            near_plane.max(far_plane),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::frustum_rh(left, right, bottom, top, near_plane, far_plane),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::frustum_rh(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_frustum_rh_gl() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near_plane, far_plane] in random_iter::<[Wide; 6]>()
                .flat_map(|[left, right, bottom, top, near_plane, far_plane]| {
                    [
                        [left, right, bottom, top, near_plane, far_plane],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near_plane.min(far_plane),
                            near_plane.max(far_plane),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::frustum_rh_gl(left, right, bottom, top, near_plane, far_plane),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::frustum_rh_gl(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_orthographic_lh() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near, far] in
                random_iter::<[Wide; 6]>().flat_map(|[left, right, bottom, top, near, far]| {
                    [
                        [left, right, bottom, top, near, far],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near.min(far),
                            near.max(far),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::orthographic_lh(left, right, bottom, top, near, far),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::orthographic_lh(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near.to_array()[lane],
                        far.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_orthographic_rh() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near, far] in
                random_iter::<[Wide; 6]>().flat_map(|[left, right, bottom, top, near, far]| {
                    [
                        [left, right, bottom, top, near, far],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near.min(far),
                            near.max(far),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::orthographic_rh(left, right, bottom, top, near, far),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::orthographic_rh(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near.to_array()[lane],
                        far.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_orthographic_rh_gl() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near, far] in
                random_iter::<[Wide; 6]>().flat_map(|[left, right, bottom, top, near, far]| {
                    [
                        [left, right, bottom, top, near, far],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near.min(far),
                            near.max(far),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Mat4::<Wide>::orthographic_rh_gl(left, right, bottom, top, near, far),
                    Mat4::from_lane_fn(|lane| Mat4::<T>::orthographic_rh_gl(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near.to_array()[lane],
                        far.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_to_scale_rotation_translation() {
        for_types!(|Wide: WideFloat| {
            for matrix in random_iter::<Mat4<Wide>>().chain(
                random_iter::<(Vec3<Wide>, Quat<Wide>, Vec3<Wide>)>().map(
                    |(scale, rotation, translation)| {
                        Mat4::<Wide>::from_scale_rotation_translation(
                            scale,
                            rotation.normalize(),
                            translation,
                        )
                    },
                ),
            ) {
                assert_test_eq_or_panic!(
                    matrix.to_scale_rotation_translation(),
                    (
                        Vec3::from_lane_fn(|lane| matrix
                            .lane(lane)
                            .to_scale_rotation_translation()
                            .0),
                        Quat::from_lane_fn(|lane| matrix
                            .lane(lane)
                            .to_scale_rotation_translation()
                            .1),
                        Vec3::from_lane_fn(|lane| matrix
                            .lane(lane)
                            .to_scale_rotation_translation()
                            .2)
                    )
                );
            }
        });
    }

    #[test]
    fn test_project_point() {
        for_types!(|Wide: WideFloat| {
            for (matrix, point) in random_iter::<(Mat4<Wide>, Vec3<Wide>)>() {
                assert_test_eq!(
                    matrix.project_point(point),
                    Vec3::from_lane_fn(|lane| matrix.lane(lane).project_point(point.lane(lane)))
                );
            }
        });
    }
}
