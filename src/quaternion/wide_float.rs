use wide::{CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{Alignment, EulerRot, Matrix, Quaternion, Vector};

macro_rules! impl_wide_float {
    ($Wide:ident, $T:ident) => {
        impl<A: Alignment> Quaternion<$Wide, A> {
            /// Creates a quaternion from an `angle` (in radians) around the x
            /// axis.
            ///
            /// This rotates `+Y` to `+Z`.
            #[inline]
            #[must_use]
            pub fn from_rotation_x(angle: $Wide) -> Self {
                let (sin, cos) = (angle * $Wide::HALF).sin_cos();
                Self::from_xyzw(sin, $Wide::ZERO, $Wide::ZERO, cos)
            }

            /// Creates a quaternion from an `angle` (in radians) around the y
            /// axis.
            ///
            /// This rotates `+Z` to `+X`.
            #[inline]
            #[must_use]
            pub fn from_rotation_y(angle: $Wide) -> Self {
                let (sin, cos) = (angle * $Wide::HALF).sin_cos();
                Self::from_xyzw($Wide::ZERO, sin, $Wide::ZERO, cos)
            }

            /// Creates a quaternion from an `angle` (in radians) around the z
            /// axis.
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_rotation_z(angle: $Wide) -> Self {
                let (sin, cos) = (angle * $Wide::HALF).sin_cos();
                Self::from_xyzw($Wide::ZERO, $Wide::ZERO, sin, cos)
            }

            /// Creates a quaternion from a rotation `axis` and `angle` (in
            /// radians).
            ///
            /// `axis` must be normalized.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
                let (sin, cos) = (angle * $Wide::HALF).sin_cos();
                let xyz = axis * sin;
                Self::from_xyzw(xyz.x, xyz.y, xyz.z, cos)
            }

            /// Creates a quaternion that rotates `scaled_axis.length()` radians
            /// around `scaled_axis.normalize()`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_scaled_axis(scaled_axis: Vector<3, $Wide, A>) -> Self {
                let angle = scaled_axis.length();
                let (sin, cos) = (angle * $Wide::HALF).sin_cos();
                let xyz = scaled_axis / angle * sin;

                Self(
                    Vector::<4, $Wide, A>::splat(angle.simd_eq($Wide::ZERO)).blend(
                        Self::IDENTITY.0,
                        Vector::<4, $Wide, A>::new(xyz.x, xyz.y, xyz.z, cos),
                    ),
                )
            }

            /// For each lane, returns the minimal rotation transforming `from`
            /// to `to`.
            ///
            /// The rotation is in the plane spanned by `from` and `to`. Rotates
            /// up to 180 degrees.
            ///
            /// When `from≈to` this is only accurate to about `0.001` (for
            /// `f32`).
            ///
            /// `from` and `to` must be normalized. Otherwise the result is
            /// unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_rotation_arc(from: Vector<3, $Wide, A>, to: Vector<3, $Wide, A>) -> Self {
                // Ported from `https://github.com/bitshifter/glam-rs`.

                let almost_one = $Wide::ONE - $Wide::splat(2.0) * $Wide::splat($T::EPSILON);

                let dot = from.dot(to);
                Self::from_vector(Vector::<4, $Wide, A>::splat(dot.simd_gt(almost_one)).blend(
                    {
                        // 0° singularity: from ≈ to.
                        Vector::W
                    },
                    Vector::<4, $Wide, A>::splat(dot.simd_lt(-almost_one)).blend(
                        {
                            // 180° singularity: from ≈ -to.
                            // Half a turn = 𝛕/2 = 180°.
                            Self::from_axis_angle(from.any_orthonormal_vector(), $Wide::PI).0
                        },
                        {
                            let cross = from.cross(to);
                            Self::from_xyzw(cross.x, cross.y, cross.z, $Wide::ONE + dot)
                                .normalize()
                                .0
                        },
                    ),
                ))
            }

            /// Returns the minimal rotation transforming `from` to either `to`
            /// or `-to`. This rotates `from` so that it is colinear with `to`.
            ///
            /// The rotation is in the plane spanned by `from` and `to`. Rotates
            /// up to 90 degrees.
            ///
            /// When `from≈to` or `from≈-to` this is only accurate to about
            /// `0.001` (for `f32`).
            ///
            /// `from` and `to` must be normalized.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_rotation_arc_colinear(
                from: Vector<3, $Wide, A>,
                to: Vector<3, $Wide, A>,
            ) -> Self {
                // Ported from `https://github.com/bitshifter/glam-rs`.

                let almost_one = $Wide::splat(const { 1.0 - 2.0 * $T::EPSILON });

                let dot = from.dot(to);
                let dot_sign = dot & -$Wide::ZERO;
                let to = to ^ dot_sign;
                let dot = dot ^ dot_sign;
                let cross = from.cross(to);

                Quaternion::from_vector(
                    Vector::<4, $Wide, A>::splat(dot.simd_gt(almost_one)).blend(
                        // 0° singularity: from ≈ to.
                        Self::IDENTITY.0,
                        Self::from_xyzw(cross.x, cross.y, cross.z, $Wide::ONE + dot)
                            .normalize()
                            .0,
                    ),
                )
            }

            /// Creates a quaternion from an Euler rotation order/sequence and
            /// angles (in radians).
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

                if order.parity_even {
                    angles.y = -angles.y;
                }

                let ti = angles.x * $Wide::HALF;
                let tj = angles.y * $Wide::HALF;
                let th = angles.z * $Wide::HALF;
                let (si, ci) = ti.sin_cos();
                let (sj, cj) = tj.sin_cos();
                let (sh, ch) = th.sin_cos();
                let cc = ci * ch;
                let cs = ci * sh;
                let sc = si * ch;
                let ss = si * sh;

                let parity = if !order.parity_even {
                    $Wide::ONE
                } else {
                    -$Wide::ONE
                };

                let mut result = Vector::ZERO;

                if order.initial_repeated {
                    result[i] = cj * (cs + sc);
                    result[j] = sj * (cc + ss) * parity;
                    result[k] = sj * (cs - sc);
                    result[3] = cj * (cc - ss);
                } else {
                    result[i] = cj * sc - sj * cs;
                    result[j] = (cj * ss + sj * cc) * parity;
                    result[k] = cj * cs - sj * sc;
                    result[3] = cj * cc + sj * ss;
                }

                Self::from_vector(result)
            }

            /// Creates a quaternion from a 3D rotation matrix.
            #[inline]
            #[must_use]
            pub fn from_matrix(matrix: &Matrix<3, $Wide, A>) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs `Quat::from_rotation_axes`
                // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`

                let [m00, m01, m02] = matrix.x_axis.to_array();
                let [m10, m11, m12] = matrix.y_axis.to_array();
                let [m20, m21, m22] = matrix.z_axis.to_array();

                // x^2 + y^2 >= z^2 + w^2
                let dif10 = m11 - m00;
                let omm22 = $Wide::ONE - m22;
                // z^2 + w^2 >= x^2 + y^2
                let sum10 = m11 + m00;
                let opm22 = $Wide::ONE + m22;
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = $Wide::HALF / four_xsq.sqrt();
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = $Wide::HALF / four_ysq.sqrt();
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = $Wide::HALF / four_zsq.sqrt();
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = $Wide::HALF / four_wsq.sqrt();

                Self::from_vector(
                    Vector::<4, $Wide, A>::splat(m22.simd_le($Wide::ZERO)).blend(
                        Vector::<4, $Wide, A>::splat(dif10.simd_le($Wide::ZERO)).blend(
                            Vector::<4, $Wide, A>::new(
                                four_xsq * inv4x,
                                (m01 + m10) * inv4x,
                                (m02 + m20) * inv4x,
                                (m12 - m21) * inv4x,
                            ),
                            Vector::<4, $Wide, A>::new(
                                (m01 + m10) * inv4y,
                                four_ysq * inv4y,
                                (m12 + m21) * inv4y,
                                (m20 - m02) * inv4y,
                            ),
                        ),
                        Vector::<4, $Wide, A>::splat(sum10.simd_le($Wide::ZERO)).blend(
                            Vector::<4, $Wide, A>::new(
                                (m02 + m20) * inv4z,
                                (m12 + m21) * inv4z,
                                four_zsq * inv4z,
                                (m01 - m10) * inv4z,
                            ),
                            Vector::<4, $Wide, A>::new(
                                (m12 - m21) * inv4w,
                                (m20 - m02) * inv4w,
                                (m01 - m10) * inv4w,
                                four_wsq * inv4w,
                            ),
                        ),
                    ),
                )
            }

            /// Creates a quaternion from a facing direction and an up
            /// direction.
            ///
            /// For a left-handed view coordinate system with `+X=right`,
            /// `+Y=up` and `+Z=forward`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_to_lh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
                Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_lh(dir, up))
            }

            /// Creates a quaternion from a facing direction and an up
            /// direction.
            ///
            /// For a right-handed view coordinate system with `+X=right`,
            /// `+Y=up` and `+Z=back`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_to_rh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
                Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_rh(dir, up))
            }

            /// Creates a quaternion from a camera position, a focal point and
            /// an up direction.
            ///
            /// For a left-handed view coordinate system with `+X=right`,
            /// `+Y=up` and `+Z=forward`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_at_lh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::from_matrix(&Matrix::<3, $Wide, A>::look_at_lh(eye, center, up))
            }

            /// Creates a quaternion from a camera position, a focal point and
            /// an up direction.
            ///
            /// For a right-handed view coordinate system with `+X=right`,
            /// `+Y=up` and `+Z=back`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_at_rh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::from_matrix(&Matrix::<3, $Wide, A>::look_at_rh(eye, center, up))
            }

            /// Converts the quaternion `self` to a normalized rotation axis and
            /// an angle (in radians).
            #[inline]
            #[must_use]
            pub fn to_axis_angle(self) -> (Vector<3, $Wide, A>, $Wide) {
                let xyz = Vector::<3, $Wide, A>::new(self.x, self.y, self.z);
                let length = xyz.length();
                let axis = xyz / length;
                let angle = length.atan2(self.w) * $Wide::splat(2.0);

                let non_zero_mask = length.simd_ge($Wide::splat(1e-8));
                (
                    Vector::<3, $Wide, A>::splat(non_zero_mask)
                        .blend(axis, Vector::<3, $Wide, A>::X),
                    non_zero_mask.blend(angle, $Wide::ZERO),
                )
            }

            /// Converts the quaternion `self` to a rotation axis scaled by an
            /// angle (in radians).
            #[inline]
            #[must_use]
            pub fn to_scaled_axis(self) -> Vector<3, $Wide, A> {
                let (axis, angle) = self.to_axis_angle();
                axis * angle
            }

            /// Returns the Euler angles forming `self` for the given Euler
            /// rotation order/sequence.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn to_euler(self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
                Matrix::<3, $Wide, A>::from_quat(self).to_euler(order)
            }

            /// Returns `true` if any element is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(self) -> $Wide {
                self.0.is_nan()
            }

            /// Returns `true` if all elements are neither infinite nor NaN.
            #[inline]
            #[must_use]
            pub fn is_finite(self) -> $Wide {
                self.0.is_finite()
            }

            /// Returns the inverse of the quaternion `self`.
            ///
            /// `self` must be normalized, otherwise the result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn inverse(self) -> Self {
                self.conjugate()
            }

            /// Returns the angle (in radians) for the minimal rotation for
            /// transforming `self` into `other`.
            ///
            /// `self` and `other` must be normalized.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn angle_between(self, other: Self) -> $Wide {
                self.dot(other).abs().acos() * $Wide::splat(2.0)
            }

            /// Computes the linear interpolation between `self` and `other`
            /// based on the value `t`.
            ///
            /// When `t` is `0`, the result is `self`.  When `t` is `1`, the
            /// result is `rhs`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn lerp(self, other: Self, t: $Wide) -> Self {
                let other = Self(other.0 ^ (self.dot(other) & $Wide::splat(-0.0)));

                (self * ($Wide::ONE - t) + other * t).normalize()
            }

            /// Computes the spherical linear interpolation between `self` and
            /// `other` based on the value `t`.
            ///
            /// When `t` is `0`, the result is `self`.  When `t` is `1`, the
            /// result is `other`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn slerp(self, other: Self, t: $Wide) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs
                // See http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/

                // Note that a rotation can be represented by two quaternions:
                // `q` and `-q`. The slerp path between `q` and `other` will be
                // different from the path between `-q` and `other`. One path
                // will take the long way around and one will take the short
                // way. In order to correct for this, the `dot` product between
                // `self` and `other` should be positive. If the `dot` product
                // is negative, slerp between `self` and `-other`.
                let dot = self.dot(other);
                let dot_sign = dot & $Wide::splat(-0.0);
                let other = Self(other.0 ^ dot_sign);
                let dot = dot.abs();

                Self(
                    Vector::<4, $Wide, A>::splat(
                        dot.simd_gt($Wide::ONE - $Wide::splat($T::EPSILON)),
                    )
                    .blend(
                        // If above threshold, perform linear interpolation to avoid divide by zero.
                        (self * ($Wide::ONE - t) + other * t).normalize().0,
                        {
                            let theta = dot.acos();

                            let x = $Wide::ONE - t;
                            let y = t;
                            let z = $Wide::ONE;

                            let tmp = Vector::<4, $Wide, A>::new(x, y, z, $Wide::ZERO) * theta;
                            let tmp = tmp.sin();

                            (self.0 * tmp.x + other.0 * tmp.y) / tmp.z
                        },
                    ),
                )
            }

            /// For each lane, rotates `self` towards `target` by at most
            /// `max_angle` (in radians).
            ///
            /// When `max_angle` is `0`, the result is `self`. When `max_angle`
            /// is equal to or greater than `self.angle_between(target)`, the
            /// result is `target`. When `max_angle` is negative, rotates
            /// towards the opposite of `target`.
            ///
            /// `self` and `target` must be normalized. Otherwise the result is
            /// unspecified.
            #[inline]
            #[must_use]
            pub fn rotate_towards(self, target: Self, max_angle: $Wide) -> Self {
                let angle = self.angle_between(target);
                let t = (max_angle / angle).clamp(-$Wide::ONE, $Wide::ONE);
                Self(
                    Vector::<4, $Wide, A>::splat(angle.simd_le($Wide::splat(1e-4)))
                        .blend(target.0, self.slerp(target, t).0),
                )
            }

            /// Returns the length/magnitude of `self`.
            #[inline]
            #[must_use]
            pub fn length(self) -> $Wide {
                self.0.length()
            }

            /// Returns `self` normalized to length `1`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn normalize(self) -> Self {
                self / self.length()
            }

            /// Returns [`normalize`], or `None` if `self` is zero or if the
            /// result is non finite or zero.
            ///
            /// [`normalize`]: Self::normalize
            #[inline]
            #[must_use]
            pub fn try_normalize(self) -> Option<Self> {
                self.0.try_normalize().map(Self::from_vector)
            }

            /// Returns [`normalize`], or `fallback` if `self` is zero or if the
            /// result is non finite or zero.
            ///
            /// [`normalize`]: Self::normalize
            #[inline]
            #[must_use]
            pub fn normalize_or(self, fallback: Self) -> Self {
                Self::from_vector(self.0.normalize_or(fallback.0))
            }

            /// Simultaneously computes [`normalize`] and [`length`].
            ///
            /// [`normalize`]: Self::normalize
            /// [`length`]: Self::length
            #[inline]
            #[must_use]
            pub fn normalize_and_length(self) -> (Self, $Wide) {
                let (normalize, length) = self.0.normalize_and_length();

                (Self::from_vector(normalize), length)
            }

            /// For each lane, returns whether the quaternion has the length `1`
            /// or not.
            ///
            /// This uses a precision threshold of approximately `1e-4`.
            #[inline]
            #[must_use]
            pub fn is_normalized(self) -> $Wide {
                self.0.is_normalized()
            }

            /// Returns `true` if the absolute difference of all elements
            /// between `self` and `other` is less than or equal to
            /// `max_abs_diff` for all lanes.
            ///
            /// This can be used to compare two quaternions that should be
            /// equal, but may have a slight difference due to operations having
            /// rounding errors.
            #[inline]
            #[must_use]
            pub fn abs_diff_eq(self, other: Self, max_abs_diff: $Wide) -> bool {
                self.0.abs_diff_eq(other.0, max_abs_diff)
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
    extern crate std;

    use crate::{
        FloatExt, Matrix, Quaternion, Vector,
        utils::{assert_float_eq, assert_float_eq_or_panic, for_parameters},
    };

    #[test]
    fn test_from_rotation_x() {
        for_parameters!(|Wide: WideFloat, A, angle| {
            let _: Wide = angle;

            assert_float_eq!(
                Quaternion::<Wide, A>::from_rotation_x(angle),
                Quaternion::from_lane_fn(|i| Quaternion::<T, A>::from_rotation_x(
                    angle.to_array()[i]
                )),
                r2nd <= Quaternion::from_array([Wide::splat(1e-5) * angle.abs(); 4])
            );
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_parameters!(|Wide: WideFloat, A, angle| {
            let _: Wide = angle;

            assert_float_eq!(
                Quaternion::<Wide, A>::from_rotation_y(angle),
                Quaternion::from_lane_fn(|i| Quaternion::<T, A>::from_rotation_y(
                    angle.to_array()[i]
                )),
                r2nd <= Quaternion::from_array([Wide::splat(1e-5) * angle.abs(); 4])
            );
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_parameters!(|Wide: WideFloat, A, angle| {
            let _: Wide = angle;

            assert_float_eq!(
                Quaternion::<Wide, A>::from_rotation_z(angle),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::from_rotation_z(
                    angle.to_array()[lane]
                )),
                r2nd <= Quaternion::from_array([Wide::splat(1e-5) * angle.abs(); 4])
            );
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_parameters!(|Wide: WideFloat, A, x, y, angle| {
            let _: [Wide; 3] = [x, y, angle];
            let axis = Vector::<3, Wide, A>::new(x, y, x * 0.12 + y * 0.21 - 0.3)
                .normalize_or(Vector::<3, Wide, A>::X);

            assert_float_eq!(
                Quaternion::<Wide, A>::from_axis_angle(axis, angle),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::from_axis_angle(
                    axis.lane(lane),
                    angle.to_array()[lane]
                )),
                r2nd <= Quaternion::from_array([Wide::splat(1e-5) * angle.abs(); 4]),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_scaled_axis() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let scaled_axis = Vector::<3, Wide, A>::new(x, y, z);

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::from_scaled_axis(scaled_axis),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::from_scaled_axis(
                    scaled_axis.lane(lane)
                )),
                r2nd <= Quaternion::from_array([Wide::splat(1e-5) * scaled_axis.length(); 4]),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_arc() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let [w, a, b] = [z + 1.3, x + 2.3, y + 3.3];
            let from = Vector::<3, Wide, A>::new(x, y, z).normalize_or(Vector::<3, Wide, A>::X);
            let to = Vector::<3, Wide, A>::new(w, a, b).normalize_or(Vector::<3, Wide, A>::Y);

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::from_rotation_arc(from, to),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::from_rotation_arc(
                    from.lane(lane),
                    to.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let [w, a, b] = [z + 1.3, x + 2.3, y + 3.3];
            let from = Vector::<3, Wide, A>::new(x, y, z).normalize_or(Vector::<3, Wide, A>::X);
            let to = Vector::<3, Wide, A>::new(w, a, b).normalize_or(Vector::<3, Wide, A>::Y);

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::from_rotation_arc_colinear(from, to),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::from_rotation_arc_colinear(
                    from.lane(lane),
                    to.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_from_euler() {
        for_parameters!(|Wide: WideFloat, A, order, a, b| {
            let _: [Wide; 2] = [a, b];
            let c = a * 0.3 + b * 0.1 + 0.6;
            let [a, b, c] = [
                a.is_finite().blend(a, Wide::ONE),
                b.is_finite().blend(b, Wide::ONE),
                c.is_finite().blend(c, Wide::ONE),
            ];

            assert_float_eq!(
                Quaternion::<Wide, A>::from_euler(order, a, b, c),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::from_euler(
                    order,
                    a.to_array()[lane],
                    b.to_array()[lane],
                    c.to_array()[lane]
                )),
                abs <= Quaternion::from_array(
                    [Wide::splat(1e-5) * a.abs().max(b.abs()).max(c.abs()); 4]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_matrix() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let matrix = Matrix::<3, Wide, A>::from_rotation_x(x)
                * Matrix::<3, Wide, A>::from_rotation_y(y)
                * Matrix::<3, Wide, A>::from_rotation_z(z);

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::from_matrix(&matrix),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::from_matrix(
                    &matrix.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let dir = Vector::<3, Wide, A>::new(x, y, z).normalize_or(Vector::<3, Wide, A>::Z);
            let up = (dir * Wide::splat(0.4) + dir.zxy().with_z(Wide::splat(0.3)))
                .normalize_or(Vector::<3, Wide, A>::Y);

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::look_to_lh(dir, up),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::look_to_lh(
                    dir.lane(lane),
                    up.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let dir = Vector::<3, Wide, A>::new(x, y, z).normalize_or(Vector::<3, Wide, A>::Z);
            let up = (dir * Wide::splat(0.4) + dir.zxy().with_z(Wide::splat(0.3)))
                .normalize_or(Vector::<3, Wide, A>::Y);

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::look_to_rh(dir, up),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::look_to_rh(
                    dir.lane(lane),
                    up.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let center = Vector::<3, Wide, A>::new(x, y, z);
            let eye = Vector::<3, Wide, A>::new(y + 0.3, z * 0.7 + 1.5, x * 1.1);
            let up = (center * Wide::splat(0.4) + center.zxy().with_z(Wide::splat(0.3)))
                .normalize_or(Vector::<3, Wide, A>::Y);

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::look_at_lh(eye, center, up),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::look_at_lh(
                    eye.lane(lane),
                    center.lane(lane),
                    up.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_look_at_rh() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let center = Vector::<3, Wide, A>::new(x, y, z);
            let eye = Vector::<3, Wide, A>::new(y + 0.3, z * 0.7 + 1.5, x * 1.1);
            let up = (center * Wide::splat(0.4) + center.zxy().with_z(Wide::splat(0.3)))
                .normalize_or(Vector::<3, Wide, A>::Y);

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::look_at_rh(eye, center, up),
                Quaternion::from_lane_fn(|lane| Quaternion::<T, A>::look_at_rh(
                    eye.lane(lane),
                    center.lane(lane),
                    up.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_to_axis_angle() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);

            assert_float_eq!(
                quat.to_axis_angle(),
                (
                    Vector::from_lane_fn(|lane| quat.lane(lane).to_axis_angle().0),
                    Wide::new(std::array::from_fn(|lane| quat
                        .lane(lane)
                        .to_axis_angle()
                        .1))
                ),
                r2nd <= (Vector::splat(Wide::splat(1e-5)), Wide::splat(1e-5))
            );
        });
    }

    #[test]
    fn test_to_scaled_axis() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);

            assert_float_eq!(
                quat.to_scaled_axis(),
                Vector::from_lane_fn(|lane| quat.lane(lane).to_scaled_axis()),
                r2nd <= Vector::splat(Wide::splat(1e-5))
            );
        });
    }

    #[test]
    fn test_to_euler() {
        for_parameters!(|Wide: WideFloat, A, order, x, y| {
            let _: [Wide; 2] = [x, y];
            let z = x * 0.1 + y * 0.3 + 0.8;
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);

            assert_float_eq!(
                quat.to_euler(order),
                (
                    Wide::new(std::array::from_fn(|lane| quat
                        .lane(lane)
                        .to_euler(order)
                        .0)),
                    Wide::new(std::array::from_fn(|lane| quat
                        .lane(lane)
                        .to_euler(order)
                        .1)),
                    Wide::new(std::array::from_fn(|lane| quat
                        .lane(lane)
                        .to_euler(order)
                        .2))
                ),
                r2nd <= (Wide::splat(1e-5), Wide::splat(1e-5), Wide::splat(1e-5))
            );
        });
    }

    #[test]
    fn test_is_nan() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan()
            );
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq_or_panic!(
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
            );
        });
    }

    #[test]
    fn test_inverse() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);

            assert_float_eq_or_panic!(
                quat.inverse(),
                Quaternion::from_lane_fn(|lane| quat.lane(lane).inverse())
            );
        });
    }

    #[test]
    fn test_angle_between() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);
            let other =
                Quaternion::<Wide, A>::from_xyzw(w, z, y, x).normalize_or(Quaternion::IDENTITY);

            assert_float_eq_or_panic!(
                quat.angle_between(other),
                Wide::new(std::array::from_fn(|lane| quat
                    .lane(lane)
                    .angle_between(other.lane(lane)))),
                r2nd <= Wide::splat(1e-5)
            );
        });
    }

    #[test]
    fn test_lerp() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);
            let other =
                Quaternion::<Wide, A>::from_xyzw(w, z, y, x).normalize_or(Quaternion::IDENTITY);
            let t = w * 0.3;

            assert_float_eq_or_panic!(
                quat.lerp(other, t),
                Quaternion::from_lane_fn(|lane| quat
                    .lane(lane)
                    .lerp(other.lane(lane), t.to_array()[lane]))
            );
        });
    }

    #[test]
    fn test_slerp() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + z * 0.2 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);
            let other =
                Quaternion::<Wide, A>::from_xyzw(w, z, y, x).normalize_or(Quaternion::IDENTITY);
            let t = Wide::new(std::array::from_fn(|lane| (w.to_array()[lane] * 0.3) % 3.0));

            assert_float_eq_or_panic!(
                quat.slerp(other, t),
                Quaternion::from_lane_fn(|lane| quat
                    .lane(lane)
                    .slerp(other.lane(lane), t.to_array()[lane])),
                abs <= Quaternion::from_array([Wide::splat(1e-5); 4])
            );
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + z * 0.2 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);
            let target =
                Quaternion::<Wide, A>::from_xyzw(w, z, y, x).normalize_or(Quaternion::IDENTITY);
            let max_angle = Wide::new(std::array::from_fn(|lane| (w.to_array()[lane] * 0.3) % 3.0));

            assert_float_eq_or_panic!(
                quat.rotate_towards(target, max_angle),
                Quaternion::from_lane_fn(|lane| quat
                    .lane(lane)
                    .rotate_towards(target.lane(lane), max_angle.to_array()[lane])),
                abs <= Quaternion::from_array([Wide::splat(1e-5); 4])
            );
        });
    }

    #[test]
    fn test_length() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let quat = Quaternion::<Wide, A>::from_xyzw(x, y, z, w);

            assert_float_eq!(
                quat.length(),
                Wide::new(std::array::from_fn(|lane| quat.lane(lane).length()))
            );
        });
    }

    #[test]
    fn test_normalize() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let quat = Quaternion::<Wide, A>::from_xyzw(x, y, z, w);

            assert_float_eq_or_panic!(
                quat.normalize(),
                Quaternion::from_lane_fn(|lane| quat.lane(lane).normalize())
            );
        });
    }

    #[test]
    fn test_try_normalize() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);

            assert_float_eq_or_panic!(
                quat.try_normalize().unwrap(),
                Quaternion::from_lane_fn(|lane| quat.lane(lane).try_normalize().unwrap())
            );
        });
    }

    #[test]
    fn test_normalize_or() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);

            assert_float_eq!(
                quat.normalize_or(Quaternion::from_vector(Vector::NEG_ONE)),
                Quaternion::from_lane_fn(|lane| quat
                    .lane(lane)
                    .normalize_or(Quaternion::from_vector(Vector::NEG_ONE)))
            );
        });
    }

    #[test]
    fn test_normalize_and_length() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat =
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY);

            assert_float_eq!(
                quat.normalize_and_length(),
                (
                    Quaternion::from_lane_fn(|lane| quat.lane(lane).normalize_and_length().0),
                    Wide::new(std::array::from_fn(|lane| quat
                        .lane(lane)
                        .normalize_and_length()
                        .1))
                )
            );
        });
    }

    #[test]
    fn test_is_normalized() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1 + 0.6;
            let quat = Quaternion::<Wide, A>::from_xyzw(x, y, z, w)
                .normalize_or(Quaternion::from_vector(Vector::NEG_ONE));

            assert_float_eq!(
                quat.is_normalized(),
                Wide::new(std::array::from_fn(|lane| T::from_bits(
                    if quat.lane(lane).is_normalized() {
                        !0
                    } else {
                        0
                    }
                )))
            );
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let quat = Quaternion::<Wide, A>::from_xyzw(x, y, z, w);
            let other = Quaternion::<Wide, A>::from_xyzw(a, b, x, z);
            assert_eq!(
                quat.abs_diff_eq(other, Wide::ONE),
                x.abs_diff_eq(a, Wide::ONE)
                    && y.abs_diff_eq(b, Wide::ONE)
                    && z.abs_diff_eq(x, Wide::ONE)
                    && w.abs_diff_eq(z, Wide::ONE)
            );
        });
    }
}
