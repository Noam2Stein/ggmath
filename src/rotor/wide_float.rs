use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Affine, Alignment, Dim, EulerRot, Matrix, Projective, Rotor, Vector,
    dim::Three,
    utils::{FloatUtils, specialize_3},
};

macro_rules! items {
    ($Wide:ident) => {
        /// A rotor with all elements set to NaN (Not a Number).
        pub const NAN: Self = Self::NAN_INTERNAL_IMPL;

        /// The implementation of [`Self::NAN`].
        ///
        /// We use this helper constant so that IDEs do not show the
        /// implementation of the constant.
        const NAN_INTERNAL_IMPL: Self = Self(Vector::<4, $Wide, A>::NAN);

        /// Returns the minimal rotation transforming `from` to `to`.
        ///
        /// The rotation is in the plane spanned by `from` and `to`. Rotates up
        /// to 180 degrees.
        ///
        /// When `from≈to` this is only accurate to about `0.001` (for `f32`).
        ///
        /// This assumes `from` and `to` are normalized.
        #[inline]
        #[must_use]
        pub fn from_rotation_arc(from: Vector<N, $Wide, A>, to: Vector<N, $Wide, A>) -> Self {
            specialize_3!(Rotor::<N, $Wide, A>::from_rotation_arc_backend(from, to))
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
        /// This assumes `from` and `to` are normalized.
        #[inline]
        #[must_use]
        pub fn from_rotation_arc_colinear(
            from: Vector<N, $Wide, A>,
            to: Vector<N, $Wide, A>,
        ) -> Self {
            specialize_3!(Rotor::<N, $Wide, A>::from_rotation_arc_colinear_backend(
                from, to
            ))
        }

        /// Converts a rotation matrix to a rotor.
        ///
        /// This assumes `matrix` only contains rotation.
        #[inline]
        #[must_use]
        pub fn from_matrix(matrix: &Matrix<N, $Wide, A>) -> Self {
            specialize_3!(Rotor::<N, $Wide, A>::from_matrix_backend(matrix))
        }

        /// Converts an affine transform with rotation to a rotor.
        ///
        /// This assumes `affine` only contains rotation, and translation which
        /// is ignored.
        #[inline]
        #[must_use]
        pub fn from_affine(affine: &Affine<N, $Wide, A>) -> Self {
            Self::from_matrix(&affine.matrix)
        }

        /// Converts a projective transform with rotation to a rotor.
        ///
        /// This assumes `projective` only contains rotation, and translation
        /// which is ignored.
        #[inline]
        #[must_use]
        pub fn from_projective(projective: &Projective<N, $Wide, A>) -> Self {
            specialize_3!(Rotor::<N, $Wide, A>::from_projective_backend(projective))
        }

        /// Returns the inverse of a rotor.
        ///
        /// This assumes `self` is normalized.
        ///
        /// This performs the same operation as [`conjugate`]. Use whichever
        /// function makes your intentions clearer.
        ///
        /// [`conjugate`]: Self::conjugate
        #[inline]
        #[must_use]
        pub fn inverse(self) -> Self {
            self.conjugate()
        }

        /// Returns the angle (in radians) for the minimal rotation for
        /// transforming `self` into `other`.
        ///
        /// This assumes `self` and `other` are normalized.
        #[inline]
        #[must_use]
        pub fn angle_between(self, other: Self) -> $Wide {
            let half_angle = self.dot(other).abs().acos_approx();
            half_angle + half_angle
        }

        /// Computes the linear interpolation between two rotors, then
        /// normalizes the result.
        ///
        /// When `t` is `0`, the result is `self`. When `t` is `1`, the result is
        /// `other`. This always takes the shorter path between the rotations.
        ///
        /// This assumes `self` and `other` are normalized.
        ///
        /// This does not interpolate the angle at a constant speed. For that use
        /// [`slerp`]. This function is more efficient as it avoids calling
        /// trigonometric functions.
        ///
        /// [`slerp`]: Self::slerp
        #[inline]
        #[must_use]
        pub fn lerp(self, other: Self, t: $Wide) -> Self {
            let other = other * self.dot(other).signum();

            (self * ($Wide::ONE - t) + other * t).normalize()
        }

        /// Computes the spherical linear interpolation between two rotors.
        ///
        /// When `t` is `0`, the result is `self`. When `t` is `1`, the result
        /// is `other`. This interpolates the angle at a constant speed, always
        /// taking the shorter path.
        ///
        /// This assumes `self` and `other` are normalized.
        #[inline]
        #[must_use]
        pub fn slerp(self, other: Self, t: $Wide) -> Self {
            specialize_3!(Rotor::<N, $Wide, A>::slerp_backend(self, other, t))
        }

        /// Rotates one rotor towards another by at most `max_angle` (in
        /// radians).
        ///
        /// This assumes `self` and `other` are normalized.
        ///
        /// When `max_angle` is `0`, the result is `self`. When `max_angle` is equal
        /// to or greater than `self.angle_between(target)`, the result is `target`.
        /// When `max_angle` is negative, this rotates towards the opposite of
        /// `target`.
        #[inline]
        #[must_use]
        pub fn rotate_towards(self, target: Self, max_angle: $Wide) -> Self {
            let angle = self.angle_between(target);
            let t = (max_angle / angle).clamp(-$Wide::ONE, $Wide::ONE);

            angle.simd_le(1e-4).select(target, self.slerp(target, t))
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
            Self(self.0.normalize())
        }

        // `try_normalize` is exluded on purpose.

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
        /// This assumes the rotor is not zero (so the output for that will be
        /// garbage). Consider manually checking for that case.
        ///
        /// [`normalize`]: Self::normalize
        /// [`length`]: Self::length
        #[inline]
        #[must_use]
        pub fn normalize_and_length(self) -> (Self, $Wide) {
            let (normalize, length) = self.0.normalize_and_length();
            (Self(normalize), length)
        }

        /// Returns whether the rotor has the length 1 or not.
        ///
        /// This uses a precision threshold of approximately `1e-4`.
        #[inline]
        #[must_use]
        pub fn is_normalized(self) -> $Wide {
            self.0.is_normalized()
        }

        /// Returns `true` if the absolute difference of all elements between
        /// `self` and `other` is less than or equal to `max_abs_diff`.
        ///
        /// This can be used to compare two rotors that should be equal, but may
        /// have a slight difference due to operations having rounding errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(self, other: Self, max_abs_diff: $Wide) -> bool {
            self.0.abs_diff_eq(other.0, max_abs_diff)
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
    };
}

macro_rules! items_3 {
    ($Wide:ident) => {
        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xy(angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (xy, s) = half_angle.sin_cos();
            Self::from_elements($Wide::ZERO, $Wide::ZERO, xy, s)
        }

        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xz(angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (xz, s) = half_angle.sin_cos();
            Self::from_elements($Wide::ZERO, -xz, $Wide::ZERO, s)
        }

        /// Creates a rotor from an `angle` (in radians) rotating `+Y` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_yz(angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (yz, s) = half_angle.sin_cos();
            Self::from_elements(yz, $Wide::ZERO, $Wide::ZERO, s)
        }

        /// Creates a rotor from a rotation `axis` and `angle` (in radians),
        /// using the right-hand rule.
        ///
        /// This assumes `axis` is normalized.
        #[inline]
        #[must_use]
        pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (sin, s) = half_angle.sin_cos();
            Self((axis * sin).extend(s))
        }

        /// Converts the rotor `self` to a normalized rotation axis and an angle
        /// (in radians), using the right-hand rule.
        #[inline]
        #[must_use]
        pub fn to_axis_angle(self) -> (Vector<3, $Wide, A>, $Wide) {
            let bivector = self.0.xyz();
            let (axis, sin) = bivector.normalize_and_length();

            let half_angle = sin.atan2(self.s);
            let angle = half_angle + half_angle;

            let angle_is_not_zero = sin.simd_ge(1e-8);
            (
                Vector::<3, $Wide, A>::new(
                    angle_is_not_zero.select(axis.x, $Wide::ONE),
                    axis.y & angle_is_not_zero,
                    axis.z & angle_is_not_zero,
                ),
                angle & angle_is_not_zero,
            )
        }

        /// Creates a rotor that rotates `scaled_axis.length()` radians around
        /// `scaled_axis.normalize()`, using the right-hand rule.
        #[inline]
        #[must_use]
        pub fn from_scaled_axis(scaled_axis: Vector<3, $Wide, A>) -> Self {
            let (axis, angle) = scaled_axis.normalize_and_length();
            let half_angle = angle * $Wide::HALF;
            let (sin, s) = half_angle.sin_cos();
            let [yz, zx, xy] = (axis * sin).to_array();

            let angle_is_not_zero = angle.simd_ne($Wide::ZERO);
            Self::from_elements(
                yz & angle_is_not_zero,
                zx & angle_is_not_zero,
                xy & angle_is_not_zero,
                angle_is_not_zero.select(s, $Wide::ONE),
            )
        }

        // Converts the rotor `self` to a rotation axis scaled by an angle (in
        /// radians), using the right-hand rule.
        #[inline]
        #[must_use]
        pub fn to_scaled_axis(self) -> Vector<3, $Wide, A> {
            let bivector = self.0.xyz();
            let (axis, sin) = bivector.normalize_and_length();

            let half_angle = sin.atan2(self.s);
            let angle = half_angle + half_angle;

            let angle_is_not_zero = sin.simd_ge(1e-8);
            (axis * angle) & angle_is_not_zero
        }

        /// Creates a rotor from an Euler rotation order/sequence and angles (in
        /// radians).
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

            let parity = if order.parity_even {
                -$Wide::ONE
            } else {
                $Wide::ONE
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

        /// Returns the Euler angles forming `self` for the given Euler rotation
        /// order/sequence.
        #[inline]
        #[must_use]
        pub fn to_euler(self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
            Matrix::<3, $Wide, A>::from_rotor(self).to_euler(order)
        }

        /// Creates a 3D rotor from a facing direction and an up direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_to_lh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_lh(dir, up))
        }

        /// Creates a 3D rotor from a facing direction and an up direction.
        ///
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_to_rh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_rh(dir, up))
        }

        /// Creates a 3D rotor from a camera position, a focal point and an up
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

        /// Creates a 3D rotor from a camera position, a focal point and an up
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
pub trait WideFloat: crate::Element {}

/// Functionality for [SoA] (Structure of Arrays) float rotors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
#[expect(private_bounds)]
impl<const N: usize, Wide, A: Alignment> Rotor<N, Wide, A>
where
    Dim<N>: Three,
    Wide: WideFloat,
{
    items!(Wide);
}

/// Functionality for [SoA] (Structure of Arrays) float 3D rotors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Rotor<3, Wide, A>
where
    Wide: WideFloat,
{
    items_3!(Wide);
}

macro_rules! impl_items {
    ($Wide:ident) => {
        #[cfg(not(doc))]
        #[expect(private_bounds)]
        impl<const N: usize, A: Alignment> Rotor<N, $Wide, A>
        where
            Dim<N>: Three,
        {
            items!($Wide);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Rotor<3, $Wide, A> {
            items_3!($Wide);

            #[inline(always)]
            fn from_rotation_arc_backend(
                from: Vector<3, $Wide, A>,
                to: Vector<3, $Wide, A>,
            ) -> Self {
                // Based on https://github.com/bitshifter/glam-rs

                let almost_one = $Wide::ONE - 2.0 * $Wide::EPSILON;

                let dot = from.dot(to);
                let angle_is_not_zero = dot.simd_lt(almost_one);
                let angle_is_180 = dot.simd_lt(-almost_one);

                angle_is_180.select(
                    {
                        // 180° singularity: from ≈ -to.
                        // Half a turn = 𝛕/2 = 180°.

                        // Construct any rotation plane parallel to `from`
                        let sign = from.z.signum();
                        let tmp = -$Wide::ONE / (sign + from.z);
                        let yz = from.x * from.y * tmp;
                        let zx = sign + from.y * from.y * tmp;
                        let xy = -from.y;

                        // sin(angle/2) = sin(𝛕/4) = 1
                        // cos(angle/2) = cos(𝛕/4) = 0
                        Self::from_elements(yz, zx, xy, $Wide::ZERO)
                    },
                    Self::from_elements(
                        (from.y * to.z - from.z * to.y) & angle_is_not_zero,
                        (from.z * to.x - from.x * to.z) & angle_is_not_zero,
                        (from.x * to.y - from.y * to.x) & angle_is_not_zero,
                        $Wide::ONE + (dot & angle_is_not_zero),
                    )
                    .normalize(),
                )
            }

            #[inline(always)]
            fn from_rotation_arc_colinear_backend(
                from: Vector<3, $Wide, A>,
                to: Vector<3, $Wide, A>,
            ) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs

                let almost_one = $Wide::ONE - 2.0 * $Wide::EPSILON;

                let dot = from.dot(to);
                let dot_signbit = dot & -0.0;
                let dot = dot ^ dot_signbit;
                let to = to ^ dot_signbit;

                let angle_is_not_zero = dot.simd_lt(almost_one);

                // If `not_singularity` is false, meaning there is singularity,
                // we return `IDENTITY`
                Self::from_elements(
                    (from.y * to.z - from.z * to.y) & angle_is_not_zero,
                    (from.z * to.x - from.x * to.z) & angle_is_not_zero,
                    (from.x * to.y - from.y * to.x) & angle_is_not_zero,
                    $Wide::ONE + (dot & angle_is_not_zero),
                )
                .normalize()
            }

            #[inline(always)]
            fn from_matrix_backend(matrix: &Matrix<3, $Wide, A>) -> Self {
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
                        Self::from_elements(
                            four_xsq * inv4x,
                            (m01 + m10) * inv4x,
                            (m02 + m20) * inv4x,
                            (m12 - m21) * inv4x,
                        ),
                        Self::from_elements(
                            (m01 + m10) * inv4y,
                            four_ysq * inv4y,
                            (m12 + m21) * inv4y,
                            (m20 - m02) * inv4y,
                        ),
                    ),
                    sum10.simd_le($Wide::ZERO).select(
                        Self::from_elements(
                            (m02 + m20) * inv4z,
                            (m12 + m21) * inv4z,
                            four_zsq * inv4z,
                            (m01 - m10) * inv4z,
                        ),
                        Self::from_elements(
                            (m12 - m21) * inv4w,
                            (m20 - m02) * inv4w,
                            (m01 - m10) * inv4w,
                            four_wsq * inv4w,
                        ),
                    ),
                )
            }

            #[inline(always)]
            fn from_projective_backend(projective: &Projective<3, $Wide, A>) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs `Quat::from_rotation_axes`
                // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`

                let [m00, m01, m02, _] = projective.x_axis.to_array();
                let [m10, m11, m12, _] = projective.y_axis.to_array();
                let [m20, m21, m22, _] = projective.z_axis.to_array();

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
                        Self::from_elements(
                            four_xsq * inv4x,
                            (m01 + m10) * inv4x,
                            (m02 + m20) * inv4x,
                            (m12 - m21) * inv4x,
                        ),
                        Self::from_elements(
                            (m01 + m10) * inv4y,
                            four_ysq * inv4y,
                            (m12 + m21) * inv4y,
                            (m20 - m02) * inv4y,
                        ),
                    ),
                    sum10.simd_le($Wide::ZERO).select(
                        Self::from_elements(
                            (m02 + m20) * inv4z,
                            (m12 + m21) * inv4z,
                            four_zsq * inv4z,
                            (m01 - m10) * inv4z,
                        ),
                        Self::from_elements(
                            (m12 - m21) * inv4w,
                            (m20 - m02) * inv4w,
                            (m01 - m10) * inv4w,
                            four_wsq * inv4w,
                        ),
                    ),
                )
            }

            #[inline(always)]
            fn slerp_backend(self, other: Self, t: $Wide) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs
                // See http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/

                // Note that a rotation can be represented by two rotors: `r` and `-r`.
                // The slerp path between `r` and `other` will be different from the
                // path between `-r` and `other`. One path will take the long way around
                // and one will take the short way. In order to correct for this, the
                // `dot` product between `self` and `other` should be positive. If the
                // `dot` product is negative, slerp between `self` and `-other`.
                let dot = self.dot(other);
                let dot_signbit = dot & -0.0;
                let dot = dot ^ dot_signbit;
                let other = Self(other.0 ^ dot_signbit);

                let half_angle = dot.acos_approx();
                let one_minus_t = $Wide::ONE - t;
                let angle_is_tiny = dot.simd_gt($Wide::ONE - $Wide::EPSILON);

                let self_factor =
                    angle_is_tiny.select(one_minus_t, (one_minus_t * half_angle).sin());
                let other_factor = angle_is_tiny.select(t, (t * half_angle).sin());

                (self * self_factor + other * other_factor).normalize()
            }
        }
    };
}
impl_items!(f32x4);
impl_items!(f32x8);
impl_items!(f32x16);
impl_items!(f64x2);
impl_items!(f64x4);
impl_items!(f64x8);

#[cfg(test)]
mod tests {
    use wide::f32x4;

    use crate::{
        EulerRot, Mat3, Proj3, Rotor3, Vec3,
        test_utils::{assert_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        assert_test_eq!(
            Rotor3::<f32x4>::NAN,
            Rotor3::from_elements(f32x4::NAN, f32x4::NAN, f32x4::NAN, f32x4::NAN)
        );
    }

    #[test]
    fn test_from_rotation_arc() {
        for_types!(|Wide: WideFloat| {
            for [from, to] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|from_to| [from_to, from_to.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Rotor3::<Wide>::from_rotation_arc(from, to),
                    Rotor3::from_lane_fn(|lane| Rotor3::<T>::from_rotation_arc(
                        from.lane(lane),
                        to.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        for_types!(|Wide: WideFloat| {
            for [from, to] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|from_to| [from_to, from_to.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Rotor3::<Wide>::from_rotation_arc_colinear(from, to),
                    Rotor3::from_lane_fn(|lane| {
                        Rotor3::<T>::from_rotation_arc_colinear(from.lane(lane), to.lane(lane))
                    })
                );
            }
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|Wide: WideFloat| {
            for matrix in random_iter::<[Wide; 3]>()
                .map(|[xy, xz, yz]| {
                    Mat3::<Wide>::from_rotation_xy(xy)
                        * Mat3::<Wide>::from_rotation_xz(xz)
                        * Mat3::<Wide>::from_rotation_yz(yz)
                })
                .chain(random_iter())
            {
                assert_test_eq_or_panic!(
                    Rotor3::<Wide>::from_matrix(&matrix),
                    Rotor3::from_lane_fn(|lane| Rotor3::<T>::from_matrix(&matrix.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|Wide: WideFloat| {
            for projective in random_iter::<[Wide; 3]>()
                .map(|[xy, xz, yz]| {
                    Proj3::<Wide>::from_rotation_xy(xy)
                        * Proj3::<Wide>::from_rotation_xz(xz)
                        * Proj3::<Wide>::from_rotation_yz(yz)
                })
                .chain(random_iter())
            {
                assert_test_eq_or_panic!(
                    Rotor3::<Wide>::from_projective(&projective),
                    Rotor3::from_lane_fn(|lane| Rotor3::<T>::from_projective(
                        &projective.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_types!(|Wide: WideFloat| {
            for [a, b] in random_iter::<[Rotor3<Wide>; 2]>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotor3::IDENTITY).normalize());

                assert_test_eq!(
                    a.angle_between(b),
                    (a * b.inverse()).normalize().s.abs().acos() * 2.0,
                    abs <= Wide::splat(1e-3)
                );
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Rotor3<Wide>; 2], Wide)>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotor3::IDENTITY));

                assert_test_eq_or_panic!(
                    a.lerp(b, t),
                    Rotor3::from_lane_fn(|lane| a
                        .lane(lane)
                        .lerp(b.lane(lane), t.as_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Rotor3<Wide>; 2], Wide)>()
                .flat_map(|(ab, t)| [(ab, t), (ab.map(|r| r.normalize()), t)])
            {
                let t = (t / 10.0).clamp(Wide::splat(-100.0), Wide::splat(100.0));

                assert_test_eq_or_panic!(
                    a.slerp(b, t),
                    Rotor3::from_lane_fn(|lane| a
                        .lane(lane)
                        .slerp(b.lane(lane), t.to_array()[lane])),
                    abs <= a.length().max(b.length()) * t.abs().max(Wide::ONE) * 1e-3 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_types!(|Wide: WideFloat| {
            for ([rotor, target], max_angle) in
                random_iter::<([Rotor3<Wide>; 2], Wide)>().flat_map(|(rotor_target, max_angle)| {
                    [
                        (
                            rotor_target
                                .map(|r| r.length().simd_lt(1e4).select(r, Rotor3::IDENTITY)),
                            max_angle,
                        ),
                        (rotor_target.map(|r| r.normalize()), max_angle),
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    rotor.rotate_towards(target, max_angle),
                    Rotor3::from_lane_fn(|lane| rotor
                        .lane(lane)
                        .rotate_towards(target.lane(lane), max_angle.to_array()[lane])),
                    abs <= rotor.length().max(target.length()) * 1e-3 + 1e-3
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_xy() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                let vector = vector & vector.length().simd_lt(1e6);
                let angle = (angle % 1e3) & angle.is_finite();

                assert_test_eq!(
                    vector * Rotor3::<Wide>::from_rotation_xy(angle),
                    vector.rotate_xy(angle),
                    abs <= vector.length() * 1e-5 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_xz() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                let vector = vector & vector.length().simd_lt(1e6);
                let angle = (angle % 1e3) & angle.is_finite();

                assert_test_eq!(
                    vector * Rotor3::<Wide>::from_rotation_xz(angle),
                    vector.rotate_xz(angle),
                    abs <= vector.length() * 1e-5 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_yz() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                let vector = vector & vector.length().simd_lt(1e6);
                let angle = (angle % 1e3) & angle.is_finite();

                assert_test_eq!(
                    vector * Rotor3::<Wide>::from_rotation_yz(angle),
                    vector.rotate_yz(angle),
                    abs <= vector.length() * 1e-5 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_types!(|Wide: WideFloat| {
            for (axis, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                let axis = axis.normalize_or(Vec3::ONE).normalize();
                let half_angle = angle * 0.5;

                let result = Rotor3::<Wide>::from_axis_angle(axis, angle);

                assert_test_eq!(
                    result.yz,
                    half_angle.sin() * axis.x,
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    result.zx,
                    half_angle.sin() * axis.y,
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    result.xy,
                    half_angle.sin() * axis.z,
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    result.s,
                    half_angle.cos(),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scaled_axis() {
        for_types!(|Wide: WideFloat| {
            for (axis, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                let axis = axis.normalize_or(Vec3::ONE).normalize();

                let skip = !(axis * angle).length().is_finite();
                let axis = skip.select(Vec3::X, axis);
                let angle = skip.select(Wide::ZERO, angle);

                assert_test_eq!(
                    Rotor3::<Wide>::from_scaled_axis(axis * angle),
                    Rotor3::<Wide>::from_axis_angle(axis, angle),
                    abs <= 1e-6 * axis.abs().max_element().max(angle.abs()),
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
                        Rotor3::<Wide>::from_euler(order, a, b, c),
                        Rotor3::from_lane_fn(|lane| Rotor3::<T>::from_euler(
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
    fn test_to_axis_angle() {
        for_types!(|Wide: WideFloat| {
            for rotor in random_iter::<Rotor3<Wide>>().flat_map(|r| [r, r.normalize()]) {
                assert_test_eq_or_panic!(
                    rotor.to_axis_angle(),
                    (
                        Vec3::from_lane_fn(|lane| rotor.lane(lane).to_axis_angle().0),
                        Wide::new(std::array::from_fn(|lane| rotor
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
            for rotor in random_iter::<Rotor3<Wide>>().flat_map(|r| [r, r.normalize()]) {
                assert_test_eq_or_panic!(
                    rotor.to_scaled_axis(),
                    Vec3::from_lane_fn(|lane| rotor.lane(lane).to_scaled_axis()),
                    abs <= Wide::splat(1e-5)
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|Wide: WideFloat| {
            for order in EulerRot::values() {
                for rotor in random_iter::<Rotor3<Wide>>().flat_map(|r| [r, r.normalize()]) {
                    assert_test_eq_or_panic!(
                        rotor.to_euler(order),
                        (
                            Wide::new(std::array::from_fn(|lane| rotor
                                .lane(lane)
                                .to_euler(order)
                                .0)),
                            Wide::new(std::array::from_fn(|lane| rotor
                                .lane(lane)
                                .to_euler(order)
                                .1)),
                            Wide::new(std::array::from_fn(|lane| rotor
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
}
