use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{Alignment, EulerRot, Matrix, Quaternion, Vector};

macro_rules! items {
    ($Wide:ident, $T:ident) => {
        /// A quaternion with all elements set to NaN (Not a Number).
        pub const NAN: Self = Self::from_vector(Vector::<4, $Wide, A>::NAN);

        /// Creates a quaternion from an `angle` (in radians) around the x axis.
        ///
        /// This rotates `+Y` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_x(angle: $Wide) -> Self {
            let (sin, cos) = (angle * $Wide::HALF).sin_cos();
            Self::from_xyzw(sin, $Wide::ZERO, $Wide::ZERO, cos)
        }

        /// Creates a quaternion from an `angle` (in radians) around the y axis.
        ///
        /// This rotates `+Z` to `+X`.
        #[inline]
        #[must_use]
        pub fn from_rotation_y(angle: $Wide) -> Self {
            let (sin, cos) = (angle * $Wide::HALF).sin_cos();
            Self::from_xyzw($Wide::ZERO, sin, $Wide::ZERO, cos)
        }

        /// Creates a quaternion from an `angle` (in radians) around the z axis.
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
        pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
            let (sin, cos) = (angle * $Wide::HALF).sin_cos();
            let xyz = axis * sin;
            Self::from_xyzw(xyz.x, xyz.y, xyz.z, cos)
        }

        /// Creates a quaternion that rotates `scaled_axis.length()` radians
        /// around `scaled_axis.normalize()`.
        #[inline]
        #[must_use]
        pub fn from_scaled_axis(scaled_axis: Vector<3, $Wide, A>) -> Self {
            let angle = scaled_axis.length();
            let (sin, cos) = (angle * $Wide::HALF).sin_cos();
            let xyz = scaled_axis / angle * sin;

            angle
                .simd_eq($Wide::ZERO)
                .select(Self::IDENTITY, Self::from_xyzw(xyz.x, xyz.y, xyz.z, cos))
        }

        /// For each lane, returns the minimal rotation transforming `from` to
        /// `to`.
        ///
        /// The rotation is in the plane spanned by `from` and `to`. Rotates up
        /// to 180 degrees.
        ///
        /// When `from≈to` this is only accurate to about `0.001` (for `f32`).
        ///
        /// `from` and `to` must be normalized. Otherwise the result is
        /// unspecified.
        #[inline]
        #[must_use]
        pub fn from_rotation_arc(from: Vector<3, $Wide, A>, to: Vector<3, $Wide, A>) -> Self {
            // Ported from `https://github.com/bitshifter/glam-rs`.

            let almost_one = $Wide::ONE - $Wide::splat(2.0) * $Wide::splat($T::EPSILON);

            let dot = from.dot(to);
            dot.simd_gt(almost_one).select(
                // 0° singularity: from ≈ to.
                Self::IDENTITY,
                dot.simd_lt(-almost_one).select(
                    // 180° singularity: from ≈ -to.
                    // Half a turn = 𝛕/2 = 180°.
                    Self::from_axis_angle(from.any_orthonormal_vector(), $Wide::PI),
                    {
                        let cross = from.cross(to);
                        Self::from_xyzw(cross.x, cross.y, cross.z, $Wide::ONE + dot).normalize()
                    },
                ),
            )
        }

        /// Returns the minimal rotation transforming `from` to either `to` or
        /// `-to`. This rotates `from` so that it is colinear with `to`.
        ///
        /// The rotation is in the plane spanned by `from` and `to`. Rotates up
        /// to 90 degrees.
        ///
        /// When `from≈to` or `from≈-to` this is only accurate to about `0.001`
        /// (for `f32`).
        ///
        /// `from` and `to` must be normalized.
        #[inline]
        #[must_use]
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

            dot.simd_gt(almost_one).select(
                // 0° singularity: from ≈ to.
                Self::IDENTITY,
                Self::from_xyzw(cross.x, cross.y, cross.z, $Wide::ONE + dot).normalize(),
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

            Self(result)
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

            m22.simd_le($Wide::ZERO).select(
                dif10.simd_le($Wide::ZERO).select(
                    Self::from_xyzw(
                        four_xsq * inv4x,
                        (m01 + m10) * inv4x,
                        (m02 + m20) * inv4x,
                        (m12 - m21) * inv4x,
                    ),
                    Self::from_xyzw(
                        (m01 + m10) * inv4y,
                        four_ysq * inv4y,
                        (m12 + m21) * inv4y,
                        (m20 - m02) * inv4y,
                    ),
                ),
                sum10.simd_le($Wide::ZERO).select(
                    Self::from_xyzw(
                        (m02 + m20) * inv4z,
                        (m12 + m21) * inv4z,
                        four_zsq * inv4z,
                        (m01 - m10) * inv4z,
                    ),
                    Self::from_xyzw(
                        (m12 - m21) * inv4w,
                        (m20 - m02) * inv4w,
                        (m01 - m10) * inv4w,
                        four_wsq * inv4w,
                    ),
                ),
            )
        }

        /// Creates a quaternion from a facing direction and an up direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_to_lh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_lh(dir, up))
        }

        /// Creates a quaternion from a facing direction and an up direction.
        ///
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_to_rh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_rh(dir, up))
        }

        /// Creates a quaternion from a camera position, a focal point and an up
        /// direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_at_lh(
            eye: Vector<3, $Wide, A>,
            center: Vector<3, $Wide, A>,
            up: Vector<3, $Wide, A>,
        ) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_at_lh(eye, center, up))
        }

        /// Creates a quaternion from a camera position, a focal point and an up
        /// direction.
        ///
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_at_rh(
            eye: Vector<3, $Wide, A>,
            center: Vector<3, $Wide, A>,
            up: Vector<3, $Wide, A>,
        ) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_at_rh(eye, center, up))
        }

        /// Converts the quaternion `self` to a normalized rotation axis and an
        /// angle (in radians).
        #[inline]
        #[must_use]
        pub fn to_axis_angle(self) -> (Vector<3, $Wide, A>, $Wide) {
            let xyz = Vector::<3, $Wide, A>::new(self.x, self.y, self.z);
            let length = xyz.length();
            let axis = xyz / length;
            let angle = length.atan2(self.w) * $Wide::splat(2.0);

            let non_zero_mask = length.simd_ge($Wide::splat(1e-8));
            (
                non_zero_mask.select(axis, Vector::<3, $Wide, A>::X),
                non_zero_mask.blend(angle, $Wide::ZERO),
            )
        }

        /// Converts the quaternion `self` to a rotation axis scaled by an angle
        /// (in radians).
        #[inline]
        #[must_use]
        pub fn to_scaled_axis(self) -> Vector<3, $Wide, A> {
            let (axis, angle) = self.to_axis_angle();
            axis * angle
        }

        /// Returns the Euler angles forming `self` for the given Euler rotation
        /// order/sequence.
        #[inline]
        #[must_use]
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
        pub fn inverse(self) -> Self {
            self.conjugate()
        }

        /// Returns the angle (in radians) for the minimal rotation for
        /// transforming `self` into `other`.
        ///
        /// `self` and `other` must be normalized.
        #[inline]
        #[must_use]
        pub fn angle_between(self, other: Self) -> $Wide {
            self.dot(other).abs().min($Wide::ONE).acos() * $Wide::splat(2.0)
        }

        /// Computes the linear interpolation between `self` and `other` based
        /// on the value `t`.
        ///
        /// When `t` is `0`, the result is `self`.  When `t` is `1`, the result
        /// is `rhs`.
        #[inline]
        #[must_use]
        pub fn lerp(self, other: Self, t: $Wide) -> Self {
            let other = Self(other.0 ^ (self.dot(other) & $Wide::splat(-0.0)));

            (self * ($Wide::ONE - t) + other * t).normalize()
        }

        /// Computes the spherical linear interpolation between `self` and
        /// `other` based on the value `t`.
        ///
        /// When `t` is `0`, the result is `self`.  When `t` is `1`, the result
        /// is `other`.
        #[inline]
        #[must_use]
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

            dot.simd_gt($Wide::ONE - $Wide::splat($T::EPSILON)).select(
                // If above threshold, perform linear interpolation to avoid divide by zero.
                (self * ($Wide::ONE - t) + other * t).normalize(),
                {
                    let theta = dot.acos();

                    let x = $Wide::ONE - t;
                    let y = t;
                    let z = $Wide::ONE;

                    let tmp = Vector::<4, $Wide, A>::new(x, y, z, $Wide::ZERO) * theta;
                    let tmp = tmp.sin();

                    Self((self.0 * tmp.x + other.0 * tmp.y) / tmp.z)
                },
            )
        }

        /// For each lane, rotates `self` towards `target` by at most
        /// `max_angle` (in radians).
        ///
        /// When `max_angle` is `0`, the result is `self`. When `max_angle` is
        /// equal to or greater than `self.angle_between(target)`, the result is
        /// `target`. When `max_angle` is negative, rotates towards the opposite
        /// of `target`.
        ///
        /// `self` and `target` must be normalized. Otherwise the result is
        /// unspecified.
        #[inline]
        #[must_use]
        pub fn rotate_towards(self, target: Self, max_angle: $Wide) -> Self {
            let angle = self.angle_between(target);
            let t = (max_angle / angle).clamp(-$Wide::ONE, $Wide::ONE);
            angle
                .simd_le($Wide::splat(1e-4))
                .select(target, self.slerp(target, t))
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
        pub fn normalize(self) -> Self {
            self / self.length()
        }

        // `try_normalize` is exluded on purpose. It would not be useful because
        // it would only return `Some` if all lanes succeed.

        /// Returns [`normalize`], or `fallback` if `self` is zero or if the
        /// result is non finite or zero.
        ///
        /// [`normalize`]: Self::normalize
        #[inline]
        #[must_use]
        pub fn normalize_or(self, fallback: Self) -> Self {
            Self(self.0.normalize_or(fallback.0))
        }

        /// Simultaneously computes [`normalize`] and [`length`].
        ///
        /// [`normalize`]: Self::normalize
        /// [`length`]: Self::length
        #[inline]
        #[must_use]
        pub fn normalize_and_length(self) -> (Self, $Wide) {
            let (normalize, length) = self.0.normalize_and_length();

            (Self(normalize), length)
        }

        /// For each lane, returns whether the quaternion has the length `1` or
        /// not.
        ///
        /// This uses a precision threshold of approximately `1e-4`.
        #[inline]
        #[must_use]
        pub fn is_normalized(self) -> $Wide {
            self.0.is_normalized()
        }

        /// Returns `true` if the absolute difference of all elements between
        /// `self` and `other` is less than or equal to `max_abs_diff` for all
        /// lanes.
        ///
        /// This can be used to compare two quaternions that should be equal,
        /// but may have a slight difference due to operations having rounding
        /// errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(self, other: Self, max_abs_diff: $Wide) -> bool {
            self.0.abs_diff_eq(other.0, max_abs_diff)
        }
    };
}

// Since all wide-float functions have names that conflict with normal float
// functions, We cannot implement this API using generics. Duplicating the API
// for each supported wide-float type works, but then documentation shows the
// duplicated API, making it hard to read.
//
// When generating documentation, Rust does not care that these items are
// conflicting. This allows us to cheat by showing these items in a generic
// context in documentation, but making them separate in all other cases.

#[cfg(doc)]
#[doc(hidden)]
pub trait WideFloat: crate::Scalar {}

/// Functionality for [SoA] (Structure of Arrays) float quaternions.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Quaternion<Wide, A>
where
    Wide: WideFloat,
{
    items!(Wide, f32);
}

macro_rules! impl_items {
    ($Wide:ident, $T:ident) => {
        #[cfg(not(doc))]
        impl<A: Alignment> Quaternion<$Wide, A> {
            items!($Wide, $T);
        }
    };
}
impl_items!(f32x4, f32);
impl_items!(f32x8, f32);
impl_items!(f32x16, f32);
impl_items!(f64x2, f64);
impl_items!(f64x4, f64);
impl_items!(f64x8, f64);

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::{
        EulerRot, Mat3, Quat, Vec3,
        test_utils::{assert_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|Wide: WideFloat| {
            assert_test_eq!(
                Quat::<Wide>::NAN,
                Quat::from_xyzw(Wide::NAN, Wide::NAN, Wide::NAN, Wide::NAN)
            );
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Quat::<Wide>::from_rotation_x(angle),
                    Quat::from_lane_fn(|lane| Quat::<T>::from_rotation_x(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Quat::<Wide>::from_rotation_y(angle),
                    Quat::from_lane_fn(|lane| Quat::<T>::from_rotation_y(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Quat::<Wide>::from_rotation_z(angle),
                    Quat::from_lane_fn(|lane| Quat::<T>::from_rotation_z(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3
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
                let axis = condition.select(axis, Vec3::X);
                let angle = condition.blend(angle, Wide::ONE);

                assert_test_eq_or_panic!(
                    Quat::<Wide>::from_axis_angle(axis, angle),
                    Quat::from_lane_fn(|lane| Quat::<T>::from_axis_angle(
                        axis.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= axis.length() * angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scaled_axis() {
        for_types!(|Wide: WideFloat| {
            for scaled_axis in random_iter::<Vec3<Wide>>() {
                assert_test_eq!(
                    Quat::<Wide>::from_scaled_axis(scaled_axis),
                    Quat::from_lane_fn(|lane| Quat::<T>::from_scaled_axis(scaled_axis.lane(lane))),
                    abs <= scaled_axis.length() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_arc() {
        for_types!(|Wide: WideFloat| {
            for [start, end] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|start_end| [start_end, start_end.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Quat::<Wide>::from_rotation_arc(start, end),
                    Quat::from_lane_fn(|lane| Quat::<T>::from_rotation_arc(
                        start.lane(lane),
                        end.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        for_types!(|Wide: WideFloat| {
            for [start, end] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|start_end| [start_end, start_end.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Quat::<Wide>::from_rotation_arc_colinear(start, end),
                    Quat::from_lane_fn(|lane| Quat::<T>::from_rotation_arc_colinear(
                        start.lane(lane),
                        end.lane(lane)
                    ))
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
                        Quat::<Wide>::from_euler(order, a, b, c),
                        Quat::from_lane_fn(|lane| Quat::<T>::from_euler(
                            order,
                            a.to_array()[lane],
                            b.to_array()[lane],
                            c.to_array()[lane]
                        )),
                        abs <= a.abs().max(b.abs()).max(c.abs()) * 1e-5,
                        0.0 = -0.0
                    );
                }
            }
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|Wide: WideFloat| {
            for [x, y, z] in random_iter::<[Wide; 3]>() {
                let matrix = Mat3::<Wide>::from_rotation_x(x)
                    * Mat3::<Wide>::from_rotation_y(y)
                    * Mat3::<Wide>::from_rotation_z(z);

                assert_test_eq_or_panic!(
                    Quat::<Wide>::from_matrix(&matrix),
                    Quat::from_lane_fn(|lane| Quat::<T>::from_matrix(&matrix.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|Wide: WideFloat| {
            for [dir, up] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|dir_up| [dir_up, dir_up.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Quat::<Wide>::look_to_lh(dir, up),
                    Quat::from_lane_fn(|lane| Quat::<T>::look_to_lh(dir.lane(lane), up.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|Wide: WideFloat| {
            for [dir, up] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|dir_up| [dir_up, dir_up.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Quat::<Wide>::look_to_rh(dir, up),
                    Quat::from_lane_fn(|lane| Quat::<T>::look_to_rh(dir.lane(lane), up.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|Wide: WideFloat| {
            for [eye, center, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|eye_center_up| [eye_center_up, eye_center_up.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Quat::<Wide>::look_at_lh(eye, center, up),
                    Quat::from_lane_fn(|lane| Quat::<T>::look_at_lh(
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
                .flat_map(|eye_center_up| [eye_center_up, eye_center_up.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Quat::<Wide>::look_at_rh(eye, center, up),
                    Quat::from_lane_fn(|lane| Quat::<T>::look_at_rh(
                        eye.lane(lane),
                        center.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_to_axis_angle() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>().flat_map(|quat| [quat, quat.normalize()]) {
                assert_test_eq_or_panic!(
                    quat.to_axis_angle(),
                    (
                        Vec3::from_lane_fn(|lane| quat.lane(lane).to_axis_angle().0),
                        Wide::new(std::array::from_fn(|lane| quat
                            .lane(lane)
                            .to_axis_angle()
                            .1))
                    ),
                    abs <= (Wide::splat(1e-5), Wide::splat(1e-5))
                );
            }
        });
    }

    #[test]
    fn test_to_scaled_axis() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>().flat_map(|quat| [quat, quat.normalize()]) {
                assert_test_eq_or_panic!(
                    quat.to_scaled_axis(),
                    Vec3::from_lane_fn(|lane| quat.lane(lane).to_scaled_axis()),
                    abs <= Wide::splat(1e-5)
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|Wide: WideFloat| {
            for order in EulerRot::values() {
                for quat in random_iter::<Quat<Wide>>().flat_map(|quat| [quat, quat.normalize()]) {
                    assert_test_eq_or_panic!(
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
                        abs <= (Wide::splat(1e-5), Wide::splat(1e-5), Wide::splat(1e-5))
                    );
                }
            }
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>() {
                assert_test_eq_or_panic!(quat.is_nan(), quat.to_vector().is_nan());
            }
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>() {
                assert_test_eq_or_panic!(quat.is_finite(), quat.to_vector().is_finite());
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>().flat_map(|quat| [quat, quat.normalize()]) {
                assert_test_eq_or_panic!(
                    quat.inverse(),
                    Quat::from_lane_fn(|lane| quat.lane(lane).inverse())
                );
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_types!(|Wide: WideFloat| {
            for [a, b] in
                random_iter::<[Quat<Wide>; 2]>().flat_map(|ab| [ab, ab.map(|q| q.normalize())])
            {
                assert_test_eq_or_panic!(
                    a.angle_between(b),
                    Wide::new(std::array::from_fn(|lane| a
                        .lane(lane)
                        .angle_between(b.lane(lane)))),
                    abs <= Wide::splat(1e-5)
                );
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Quat<Wide>; 2], Wide)>()
                .flat_map(|(ab, t)| [(ab, t), (ab.map(|q| q.normalize()), t)])
            {
                assert_test_eq_or_panic!(
                    a.lerp(b, t),
                    Quat::from_lane_fn(|lane| a.lane(lane).lerp(b.lane(lane), t.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Quat<Wide>; 2], Wide)>()
                .flat_map(|(ab, t)| [(ab, t), (ab.map(|q| q.normalize()), t)])
            {
                let t = (t / 10.0).clamp(Wide::splat(-100.0), Wide::splat(100.0));

                assert_test_eq_or_panic!(
                    a.slerp(b, t),
                    Quat::from_lane_fn(|lane| a.lane(lane).slerp(b.lane(lane), t.to_array()[lane])),
                    abs <= a.length().max(b.length()) * t.abs().max(Wide::ONE) * 1e-3 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_types!(|Wide: WideFloat| {
            for ([quat, target], max_angle) in
                random_iter::<([Quat<Wide>; 2], Wide)>().flat_map(|(quat_target, max_angle)| {
                    [
                        (quat_target, max_angle),
                        (quat_target.map(|q| q.normalize()), max_angle),
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    quat.rotate_towards(target, max_angle),
                    Quat::from_lane_fn(|lane| quat
                        .lane(lane)
                        .rotate_towards(target.lane(lane), max_angle.to_array()[lane])),
                    abs <= quat.length().max(target.length()) * 1e-3 + 1e-3
                );
            }
        });
    }

    #[test]
    fn test_length() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>() {
                assert_test_eq!(
                    quat.length(),
                    Wide::new(std::array::from_fn(|lane| quat.lane(lane).length()))
                );
            }
        });
    }

    #[test]
    fn test_normalize() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>() {
                assert_test_eq_or_panic!(
                    quat.normalize(),
                    Quat::from_lane_fn(|lane| quat.lane(lane).normalize())
                );
            }
        });
    }

    // `try_normalize` is excluded on purpose.

    #[test]
    fn test_normalize_or() {
        for_types!(|Wide: WideFloat| {
            for [quat, fallback] in random_iter::<[Quat<Wide>; 2]>() {
                assert_test_eq_or_panic!(
                    quat.normalize_or(fallback),
                    Quat::from_lane_fn(|lane| quat.lane(lane).normalize_or(fallback.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_normalize_and_length() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>() {
                assert_test_eq_or_panic!(
                    quat.normalize_and_length(),
                    (
                        Quat::from_lane_fn(|lane| quat.lane(lane).normalize_and_length().0),
                        Wide::new(std::array::from_fn(|lane| quat
                            .lane(lane)
                            .normalize_and_length()
                            .1))
                    )
                );
            }
        });
    }

    #[test]
    fn test_is_normalized() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>() {
                assert_test_eq!(
                    quat.is_normalized(),
                    Wide::new(std::array::from_fn(|lane| {
                        if quat.lane(lane).is_normalized() {
                            T::from_bits(!0)
                        } else {
                            0.0
                        }
                    }))
                );
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], max_abs_diff) in random_iter::<([Quat<Wide>; 2], Wide)>() {
                assert_test_eq!(
                    a.abs_diff_eq(b, max_abs_diff),
                    (0..LANES).all(|lane| a
                        .lane(lane)
                        .abs_diff_eq(b.lane(lane), max_abs_diff.to_array()[lane]))
                );
            }
        });
    }
}
