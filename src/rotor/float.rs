use crate::{
    Affine, Alignment, Dim, EulerRot, FloatExt, Matrix, PrimitiveFloat, Projective, Rotor, Vector,
    length::Three, utils::specialize_3,
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Dim<N>: Three,
    T: PrimitiveFloat,
{
    /// A rotor with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::NAN_INTERNAL_IMPL;

    /// The implementation of [`Self::NAN`].
    ///
    /// We use this helper constant so that IDEs do not show the implementation
    /// of the constant.
    const NAN_INTERNAL_IMPL: Self = Self(Vector::<4, T, A>::NAN);

    /// Returns the minimal rotation transforming `from` to `to`.
    ///
    /// The rotation is in the plane spanned by `from` and `to`. Rotates up to
    /// 180 degrees.
    ///
    /// When `from≈to` this is only accurate to about `0.001` (for `f32`).
    ///
    /// This assumes `from` and `to` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `from` or `to` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc(from: Vector<N, T, A>, to: Vector<N, T, A>) -> Self {
        debug_assert!(
            from.is_normalized() && to.is_normalized(),
            "vectors are not normalized: from_rotation_arc({from:?}, {to:?})"
        );

        specialize_3!(Rotor::<N, T, A>::from_rotation_arc_backend(from, to))
    }

    /// Returns the minimal rotation transforming `from` to either `to` or
    /// `-to`. This rotates `from` so that it is colinear with `to`.
    ///
    /// The rotation is in the plane spanned by `from` and `to`. Rotates up to
    /// 90 degrees.
    ///
    /// When `from≈to` or `from≈-to` this is only accurate to about `0.001` (for
    /// `f32`).
    ///
    /// This assumes `from` and `to` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `from` or `to` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc_colinear(from: Vector<N, T, A>, to: Vector<N, T, A>) -> Self {
        debug_assert!(
            from.is_normalized() && to.is_normalized(),
            "vectors are not normalized: from_rotation_arc_colinear({from:?}, {to:?})"
        );

        specialize_3!(Rotor::<N, T, A>::from_rotation_arc_colinear_backend(
            from, to
        ))
    }

    /// Converts a rotation matrix to a rotor.
    ///
    /// This assumes `matrix` only contains rotation.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `matrix` is not a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_matrix(matrix: &Matrix<N, T, A>) -> Self {
        specialize_3!(Rotor::<N, T, A>::from_matrix_backend(matrix))
    }

    /// Converts an affine transform with rotation to a rotor.
    ///
    /// This assumes `affine` only contains rotation, and translation which is
    /// ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `affine.matrix` is not a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_affine(affine: &Affine<N, T, A>) -> Self {
        Self::from_matrix(&affine.matrix)
    }

    /// Converts a projective transform with rotation to a rotor.
    ///
    /// This assumes `projective` only contains rotation, and translation which
    /// is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `projective` contains anything but rotation and translation.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_projective(projective: &Projective<N, T, A>) -> Self {
        specialize_3!(Rotor::<N, T, A>::from_projective_backend(projective))
    }

    /// Returns `true` if any element is NaN.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Returns the inverse of a rotor.
    ///
    /// This assumes `self` is normalized.
    ///
    /// This performs the same operation as [`conjugate`]. Use whichever
    /// function makes your intentions clearer.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    ///
    /// [`conjugate`]: Self::conjugate
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn inverse(self) -> Self {
        debug_assert!(
            self.is_normalized(),
            "rotor is not normalized: {self:?}.inverse()"
        );

        self.conjugate()
    }

    /// Returns the angle (in radians) for the minimal rotation for transforming
    /// `self` into `other`.
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn angle_between(self, other: Self) -> T {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "rotors are not normalized: {self:?}.angle_between({other:?})"
        );

        let half_angle = self.dot(other).abs().acos_approx();
        half_angle + half_angle
    }

    /// Computes the linear interpolation between two rotors, then normalizes
    /// the result.
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
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    ///
    /// [`slerp`]: Self::slerp
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lerp(self, other: Self, t: T) -> Self {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "rotors are not normalized: {self:?}.lerp({other:?}, {t:?})"
        );

        let other = other * self.dot(other).signum();

        (self * (T::ONE - t) + other * t).normalize()
    }

    /// Computes the spherical linear interpolation between two rotors.
    ///
    /// When `t` is `0`, the result is `self`. When `t` is `1`, the result is
    /// `other`. This interpolates the angle at a constant speed, always taking
    /// the shorter path.
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn slerp(self, other: Self, t: T) -> Self {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "rotors are not normalized: {self:?}.slerp({other:?}, {t:?})"
        );

        specialize_3!(Rotor::<N, T, A>::slerp_backend(self, other, t))
    }

    /// Rotates one rotor towards another by at most `max_angle` (in radians).
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// When `max_angle` is `0`, the result is `self`. When `max_angle` is equal
    /// to or greater than `self.angle_between(target)`, the result is `target`.
    /// When `max_angle` is negative, this rotates towards the opposite of
    /// `target`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn rotate_towards(self, target: Self, max_angle: T) -> Self {
        debug_assert!(
            self.is_normalized() && target.is_normalized(),
            "rotors are not normalized: {self:?}.rotate_towards({target:?}, {max_angle:?})"
        );

        let angle = self.angle_between(target);
        if angle <= T::as_from(1e-4) {
            target
        } else {
            let t = (max_angle / angle).clamp(T::NEG_ONE, T::ONE);
            self.slerp(target, t)
        }
    }

    /// Returns the length/magnitude of `self`.
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.0.length()
    }

    /// Returns `self` normalized to length `1`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is a zero rotor, or if the result is non finite or
    /// zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn normalize(self) -> Self {
        let result = self / self.length();

        debug_assert!(
            result.is_finite() && result != Self(Vector::<4, T, A>::ZERO),
            "rotor is zero or non-finite: {self:?}.normalize()"
        );

        result
    }

    /// Returns [`normalize`], or `None` if `self` is zero or if the result is
    /// non finite or zero.
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        self.0.try_normalize().map(Self)
    }

    /// Returns [`normalize`], or `fallback` if `self` is zero or if the result
    /// is non finite or zero.
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
    pub fn normalize_and_length(self) -> (Self, T) {
        let (normalize, length) = self.0.normalize_and_length();
        (Self(normalize), length)
    }

    /// Returns whether the rotor has the length 1 or not.
    ///
    /// This uses a precision threshold of approximately `1e-4`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        self.0.is_normalized()
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two rotors that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool {
        self.0.abs_diff_eq(other.0, max_abs_diff)
    }
}

impl<T, A: Alignment> Rotor<3, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_rotation_xy(angle: T) -> Self {
        let half_angle = angle * T::as_from(0.5);
        let (xy, s) = half_angle.sin_cos();
        Self::from_elements(T::ZERO, T::ZERO, xy, s)
    }

    /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_xz(angle: T) -> Self {
        let half_angle = angle * T::as_from(0.5);
        let (xz, s) = half_angle.sin_cos();
        Self::from_elements(T::ZERO, -xz, T::ZERO, s)
    }

    /// Creates a rotor from an `angle` (in radians) rotating `+Y` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_yz(angle: T) -> Self {
        let half_angle = angle * T::as_from(0.5);
        let (yz, s) = half_angle.sin_cos();
        Self::from_elements(yz, T::ZERO, T::ZERO, s)
    }

    /// Creates a rotor from a rotation `axis` and `angle` (in radians), using
    /// the right-hand rule.
    ///
    /// This assumes `axis` is normalized.
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
        debug_assert!(
            axis.is_normalized(),
            "axis is not normalized: from_axis_angle({axis:?}, {angle:?})"
        );

        let half_angle = angle * T::as_from(0.5);
        let (sin, s) = half_angle.sin_cos();
        Self((axis * sin).extend(s))
    }

    /// Creates a rotor that rotates `scaled_axis.length()` radians around
    /// `scaled_axis.normalize()`, using the right-hand rule.
    #[inline]
    #[must_use]
    pub fn from_scaled_axis(scaled_axis: Vector<3, T, A>) -> Self {
        let (axis, angle) = scaled_axis.normalize_and_length();
        if angle == T::ZERO {
            Self::IDENTITY
        } else {
            let half_angle = angle * T::as_from(0.5);
            let (sin, s) = half_angle.sin_cos();
            Self((axis * sin).extend(s))
        }
    }

    /// Creates a rotor from an Euler rotation order/sequence and angles (in
    /// radians).
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

        if order.parity_even {
            angles.y = -angles.y;
        }

        let ti = angles.x * T::as_from(0.5);
        let tj = angles.y * T::as_from(0.5);
        let th = angles.z * T::as_from(0.5);
        let (si, ci) = ti.sin_cos();
        let (sj, cj) = tj.sin_cos();
        let (sh, ch) = th.sin_cos();
        let cc = ci * ch;
        let cs = ci * sh;
        let sc = si * ch;
        let ss = si * sh;

        let parity = if !order.parity_even {
            T::ONE
        } else {
            T::NEG_ONE
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

    /// Creates a 3D rotor from a facing direction and an up direction.
    ///
    /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=forward`.
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
        Self::from_matrix(&Matrix::<3, T, A>::look_to_lh(dir, up))
    }

    /// Creates a 3D rotor from a facing direction and an up direction.
    ///
    /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=back`.
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
        Self::from_matrix(&Matrix::<3, T, A>::look_to_rh(dir, up))
    }

    /// Creates a 3D rotor from a camera position, a focal point and an up
    /// direction.
    ///
    /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=forward`.
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
        Self::from_matrix(&Matrix::<3, T, A>::look_at_lh(eye, center, up))
    }

    /// Creates a 3D rotor from a camera position, a focal point and an up
    /// direction.
    ///
    /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=back`.
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
        Self::from_matrix(&Matrix::<3, T, A>::look_at_rh(eye, center, up))
    }

    /// Converts the rotor `self` to a normalized rotation axis and an angle (in
    /// radians), using the right-hand rule.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_axis_angle(self) -> (Vector<3, T, A>, T) {
        debug_assert!(
            self.is_normalized(),
            "rotor is not normalized: {self:?}.to_axis_angle()"
        );

        let bivector = self.0.xyz();
        let sin = bivector.length();

        if sin >= T::as_from(1e-8) {
            let axis = bivector / sin;
            let half_angle = sin.atan2(self.s);
            let angle = half_angle + half_angle;

            (axis, angle)
        } else {
            (Vector::<3, T, A>::X, T::ZERO)
        }
    }

    // Converts the rotor `self` to a rotation axis scaled by an angle (in
    /// radians), using the right-hand rule.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    pub fn to_scaled_axis(self) -> Vector<3, T, A> {
        debug_assert!(
            self.is_normalized(),
            "rotor is not normalized: {self:?}.to_axis_angle()"
        );

        let bivector = self.0.xyz();
        let sin = bivector.length();

        if sin >= T::as_from(1e-8) {
            let axis = bivector / sin;
            let half_angle = sin.atan2(self.s);
            let angle = half_angle + half_angle;

            axis * angle
        } else {
            Vector::ZERO
        }
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(self, order: EulerRot) -> (T, T, T) {
        debug_assert!(
            self.is_normalized(),
            "rotor is not normalized: {self:?}.to_euler({order:?})"
        );

        Matrix::<3, T, A>::from_rotor(self).to_euler(order)
    }

    #[inline(always)]
    fn from_rotation_arc_backend(from: Vector<3, T, A>, to: Vector<3, T, A>) -> Self {
        // Based on https://github.com/bitshifter/glam-rs

        let almost_one = T::ONE - T::as_from(2.0) * T::EPSILON;

        let dot = from.dot(to);
        if dot < -almost_one {
            // 180° singularity: from ≈ -to.
            // Half a turn = 𝛕/2 = 180°.

            // Construct any rotation plane parallel to `from`
            let sign = from.z.signum();
            let tmp = T::NEG_ONE / (sign + from.z);
            let yz = from.x * from.y * tmp;
            let zx = sign + from.y * from.y * tmp;
            let xy = -from.y;

            // sin(angle/2) = sin(𝛕/4) = 1
            // cos(angle/2) = cos(𝛕/4) = 0
            Self::from_elements(yz, zx, xy, T::ZERO)
        } else if dot < almost_one {
            Self(from.cross(to).extend(T::ONE + dot).normalize())
        } else {
            // 0° singularity: from ≈ to.
            Self::IDENTITY
        }
    }

    #[inline(always)]
    fn from_rotation_arc_colinear_backend(from: Vector<3, T, A>, mut to: Vector<3, T, A>) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs

        let almost_one = T::ONE - T::as_from(2.0) * T::EPSILON;

        let mut dot = from.dot(to);
        if dot.is_sign_negative() {
            dot = -dot;
            to = -to;
        }

        if dot < almost_one {
            Self(from.cross(to).extend(T::ONE + dot).normalize())
        } else {
            // 0° singularity: from ≈ to.
            Self::IDENTITY
        }
    }

    #[inline(always)]
    #[track_caller]
    fn from_matrix_backend(matrix: &Matrix<3, T, A>) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs `Quat::from_rotation_axes`
        // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`

        debug_assert!(
            matrix
                .x_axis
                .length_squared()
                .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && matrix
                    .y_axis
                    .length_squared()
                    .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && matrix
                    .x_axis
                    .dot(matrix.y_axis)
                    .abs_diff_eq(T::ZERO, T::as_from(1e-4))
                && matrix
                    .x_axis
                    .cross(matrix.y_axis)
                    .abs_diff_eq(matrix.z_axis, T::as_from(1e-4)),
            "not a rotation matrix: Rotor::from_matrix({matrix:?})"
        );

        let [m00, m01, m02] = matrix.x_axis.to_array();
        let [m10, m11, m12] = matrix.y_axis.to_array();
        let [m20, m21, m22] = matrix.z_axis.to_array();

        if m22 <= T::ZERO {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = T::ONE - m22;

            if dif10 <= T::ZERO {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = T::as_from(0.5) / four_xsq.sqrt();

                Self::from_elements(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = T::as_from(0.5) / four_ysq.sqrt();

                Self::from_elements(
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

                Self::from_elements(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = T::as_from(0.5) / four_wsq.sqrt();

                Self::from_elements(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    #[inline(always)]
    #[track_caller]
    fn from_projective_backend(projective: &Projective<3, T, A>) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs `Quat::from_rotation_axes`
        // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`

        debug_assert!(
            projective
                .column(3)
                .abs_diff_eq(Vector::<4, T, A>::W, T::as_from(1e-6))
                && projective
                    .x_axis
                    .truncate()
                    .length_squared()
                    .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && projective
                    .y_axis
                    .truncate()
                    .length_squared()
                    .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && projective
                    .x_axis
                    .truncate()
                    .dot(projective.y_axis.truncate())
                    .abs_diff_eq(T::ZERO, T::as_from(1e-4))
                && projective
                    .x_axis
                    .truncate()
                    .cross(projective.y_axis.truncate())
                    .abs_diff_eq(projective.z_axis.truncate(), T::as_from(1e-4)),
            "not a rotation: Rotor::from_projective({projective:?})"
        );

        let [m00, m01, m02, _] = projective.x_axis.to_array();
        let [m10, m11, m12, _] = projective.y_axis.to_array();
        let [m20, m21, m22, _] = projective.z_axis.to_array();

        if m22 <= T::ZERO {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = T::ONE - m22;

            if dif10 <= T::ZERO {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = T::as_from(0.5) / four_xsq.sqrt();

                Self::from_elements(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = T::as_from(0.5) / four_ysq.sqrt();

                Self::from_elements(
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

                Self::from_elements(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = T::as_from(0.5) / four_wsq.sqrt();

                Self::from_elements(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    #[inline(always)]
    #[track_caller]
    fn slerp_backend(self, mut other: Self, t: T) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs
        // See http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/

        // Note that a rotation can be represented by two rotors: `r` and `-r`.
        // The slerp path between `r` and `other` will be different from the
        // path between `-r` and `other`. One path will take the long way around
        // and one will take the short way. In order to correct for this, the
        // `dot` product between `self` and `other` should be positive. If the
        // `dot` product is negative, slerp between `self` and `-other`.
        let mut dot = self.dot(other);
        if dot.is_sign_negative() {
            other = -other;
            dot = -dot;
        }

        if dot > T::ONE - T::EPSILON {
            // If above threshold, perform linear interpolation to avoid divide by zero.
            (self * (T::ONE - t) + other * t).normalize()
        } else {
            let half_angle = dot.acos_approx();

            let self_factor = ((T::ONE - t) * half_angle).sin();
            let other_factor = (t * half_angle).sin();

            (self * self_factor + other * other_factor).normalize()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Affine, EulerRot, FloatExt, Matrix, Projective, Rotor, Rotor3A, Vector,
        test_utils::{
            assert_debug_panic, assert_panic_test_eq, assert_test_eq, for_types, random_iter,
        },
        utils::PrimitiveFloatUtils,
    };

    #[test]
    fn test_constants() {
        assert_test_eq!(
            Rotor3A::<f32>::NAN,
            Rotor3A::from_elements(f32::NAN, f32::NAN, f32::NAN, f32::NAN)
        );
    }

    #[test]
    fn test_from_rotation_arc() {
        for_types!(|T: PrimitiveFloat, A| {
            for [from, to] in random_iter::<[Vector<3, T, A>; 2]>() {
                if !from.is_normalized() {
                    assert_debug_panic!(Rotor::<3, T, A>::from_rotation_arc(from, to.normalize()));
                }
                if !to.is_normalized() {
                    assert_debug_panic!(Rotor::<3, T, A>::from_rotation_arc(from.normalize(), to));
                }

                let start = from.normalize_or(Vector::ONE).normalize();
                let end = to.normalize_or(Vector::ONE).normalize();

                let result = Rotor::<3, T, A>::from_rotation_arc(start, end);
                let (result_axis, result_angle) = result.to_axis_angle();

                if ((1.0 as T).to_radians()..(179.0 as T).to_radians())
                    .contains(&start.angle_between(end))
                {
                    assert_test_eq!(start * result, end, abs <= 1e-5, 0.0 = -0.0);
                    assert_test_eq!(result_angle, start.angle_between(end), abs <= 1e-4);
                    assert_test_eq!(result_axis.dot(start), 0.0, abs <= 1e-5, 0.0 = -0.0);
                    assert_test_eq!(result_axis.dot(end), 0.0, abs <= 1e-5, 0.0 = -0.0);
                } else {
                    assert_test_eq!(start * result, end, abs <= 1e-2, 0.0 = -0.0);
                    assert_test_eq!(result_angle, start.angle_between(end), abs <= 1e-2);
                    if result_angle != 0.0 {
                        assert_test_eq!(result_axis.dot(start), 0.0, abs <= 1e-2);
                        assert_test_eq!(result_axis.dot(end), 0.0, abs <= 1e-2);
                    }
                }
                assert!(result_angle <= T::TAU / 2.0 + 0.1);
            }
        });
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        for_types!(|T: PrimitiveFloat, A| {
            for [from, to] in random_iter::<[Vector<3, T, A>; 2]>() {
                if !from.is_normalized() {
                    assert_debug_panic!(Rotor::<3, T, A>::from_rotation_arc_colinear(
                        from,
                        to.normalize()
                    ));
                }
                if !to.is_normalized() {
                    assert_debug_panic!(Rotor::<3, T, A>::from_rotation_arc_colinear(
                        from.normalize(),
                        to
                    ));
                }

                let from = from.normalize_or(Vector::ONE).normalize();
                let to = to.normalize_or(Vector::ONE).normalize();

                assert_test_eq!(
                    Rotor::<3, T, A>::from_rotation_arc_colinear(from, to),
                    if from.dot(to).is_sign_positive() {
                        Rotor::<3, T, A>::from_rotation_arc(from, to)
                    } else {
                        Rotor::<3, T, A>::from_rotation_arc(from, -to)
                    }
                );
            }
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|T: PrimitiveFloat, A| {
            for [xy, xz, yz] in random_iter::<[T; 3]>() {
                if [xy, xz, yz].into_iter().any(|x| !x.is_finite() || x > 1e6) {
                    continue;
                };

                assert_test_eq!(
                    Rotor::<3, T, A>::from_matrix(
                        &(Matrix::<3, T, A>::from_rotation_xy(xy)
                            * Matrix::<3, T, A>::from_rotation_xz(xz)
                            * Matrix::<3, T, A>::from_rotation_yz(yz))
                    ),
                    Rotor::<3, T, A>::from_rotation_xy(xy)
                        * Rotor::<3, T, A>::from_rotation_xz(xz)
                        * Rotor::<3, T, A>::from_rotation_yz(yz),
                    abs <= 1e-6,
                    0.0 = -0.0,
                    rotor = -rotor
                );
            }

            for matrix in random_iter::<Matrix<3, T, A>>().take(10) {
                if !matrix.determinant().abs_diff_eq(1.0, 1e-2)
                    || !matrix.x_axis.length().abs_diff_eq(1.0, 1e-2)
                    || !matrix.y_axis.length().abs_diff_eq(1.0, 1e-2)
                    || !matrix.z_axis.length().abs_diff_eq(1.0, 1e-2)
                    || !matrix.x_axis.dot(matrix.y_axis).abs_diff_eq(0.0, 1e-2)
                    || !matrix.x_axis.dot(matrix.z_axis).abs_diff_eq(0.0, 1e-2)
                    || !matrix.y_axis.dot(matrix.z_axis).abs_diff_eq(0.0, 1e-2)
                {
                    assert_debug_panic!(Rotor::<3, T, A>::from_matrix(&matrix));
                }
            }
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|T: PrimitiveFloat, A| {
            for projective in random_iter::<Rotor<3, T, A>>()
                .map(|rotor| {
                    Projective::<3, T, A>::from_rotor(
                        rotor.normalize_or(Rotor::IDENTITY).normalize(),
                    )
                })
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    Rotor::<3, T, A>::from_projective(&projective),
                    Rotor::<3, T, A>::from_affine(&Affine::<3, T, A>::from_projective(&projective))
                );
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotor<3, T, A>; 2]>() {
                if !a.is_normalized() {
                    assert_debug_panic!(a.angle_between(b.normalize()));
                }
                if !b.is_normalized() {
                    assert_debug_panic!(a.normalize().angle_between(b));
                }

                let [a, b] = [a, b].map(|r| r.normalize_or(Rotor::IDENTITY).normalize());

                assert_test_eq!(
                    a.angle_between(b),
                    (a * b.inverse()).s.abs().acos() * 2.0,
                    abs <= 2e-4
                );
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|T: PrimitiveFloat, A| {
            for [rotor, other] in random_iter::<[Rotor<3, T, A>; 2]>() {
                if !rotor.is_normalized() {
                    assert_debug_panic!(rotor.lerp(other.normalize(), 0.2));
                }
                if !other.is_normalized() {
                    assert_debug_panic!(rotor.normalize().lerp(other, 0.2));
                }

                let [rotor, other] =
                    [rotor, other].map(|r| r.normalize_or(Rotor::IDENTITY).normalize());

                assert_test_eq!(rotor.lerp(other, 0.0), rotor, abs <= 1e-6, 0.0 = -0.0);
                assert_test_eq!(
                    rotor.lerp(other, 0.5).angle_between(rotor),
                    rotor.angle_between(other) / 2.0,
                    abs <= rotor.angle_between(other) * 1e-6 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    rotor.lerp(other, 0.5).angle_between(other),
                    rotor.angle_between(other) / 2.0,
                    abs <= rotor.angle_between(other) * 1e-6 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    rotor.lerp(other, 1.0),
                    other,
                    abs <= 1e-6,
                    0.0 = -0.0,
                    rotor = -rotor
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotor<3, T, A>; 2]>() {
                if !a.is_normalized() {
                    assert_debug_panic!(a.slerp(b.normalize(), 0.2));
                }
                if !b.is_normalized() {
                    assert_debug_panic!(a.normalize().slerp(b, 0.2));
                }

                let [a, b] = [a, b].map(|r| r.normalize_or(Rotor::IDENTITY).normalize());

                assert_test_eq!(a.slerp(b, 0.0), a, abs <= 1e-6, 0.0 = -0.0, rotor = -rotor);
                assert_test_eq!(a.slerp(b, 1.0), b, abs <= 1e-6, 0.0 = -0.0, rotor = -rotor);

                for t in [0.25, 0.5, 0.75] {
                    let result = a.slerp(b, t);

                    if result.angle_between(a).is_nan() && !result.is_nan() {
                        continue;
                    }
                    if result.angle_between(b).is_nan() && !result.is_nan() {
                        continue;
                    }

                    if ((1.0 as T)..(179.0 as T)).contains(&a.angle_between(b)) {
                        assert_test_eq!(
                            result.angle_between(a),
                            a.angle_between(b) * t,
                            abs <= a.angle_between(b) * 1e-6 + 1e-3,
                            0.0 = -0.0
                        );
                        assert_test_eq!(
                            result.angle_between(b),
                            a.angle_between(b) * (1.0 - t),
                            abs <= a.angle_between(b) * 1e-6 + 1e-3,
                            0.0 = -0.0
                        );
                    } else {
                        assert_test_eq!(
                            result.angle_between(a),
                            a.angle_between(b) * t,
                            abs <= a.angle_between(b) * 1e-4 + 1e-2,
                            0.0 = -0.0
                        );
                        assert_test_eq!(
                            result.angle_between(b),
                            a.angle_between(b) * (1.0 - t),
                            abs <= a.angle_between(b) * 1e-4 + 1e-2,
                            0.0 = -0.0
                        );
                    }
                }
            }
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_types!(|T: PrimitiveFloat, A| {
            for [rotor, target] in random_iter::<[Rotor<3, T, A>; 2]>() {
                if !rotor.is_normalized() {
                    assert_debug_panic!(rotor.rotate_towards(target.normalize(), 0.2));
                }
                if !target.is_normalized() {
                    assert_debug_panic!(rotor.normalize().rotate_towards(target, 0.2));
                }

                let [rotor, target] =
                    [rotor, target].map(|r| r.normalize_or(Rotor::IDENTITY).normalize());

                assert_test_eq!(
                    rotor.rotate_towards(target, 0.0),
                    rotor,
                    abs <= 1e-3,
                    0.0 = -0.0,
                    rotor = -rotor
                );
                assert_test_eq!(
                    rotor.rotate_towards(target, rotor.angle_between(target)),
                    target,
                    abs <= 1e-3,
                    0.0 = -0.0,
                    rotor = -rotor
                );
                assert_test_eq!(
                    rotor.rotate_towards(target, rotor.angle_between(target) * 1.5),
                    target,
                    abs <= 1e-3,
                    0.0 = -0.0,
                    rotor = -rotor
                );

                for t in [0.25, 0.5, 0.75] {
                    assert_test_eq!(
                        rotor.rotate_towards(target, rotor.angle_between(target) * t),
                        rotor.slerp(target, t),
                        abs <= 1e-3,
                        0.0 = -0.0,
                        rotor = -rotor
                    );
                }
            }
        });
    }

    #[test]
    fn test_normalize() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<3, T, A>>() {
                assert_panic_test_eq!(rotor.normalize(), Rotor(rotor.0.normalize()));
            }
        });
    }

    #[test]
    fn test_from_rotation_xy() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>()
                .filter(|(vector, angle)| vector.length() < 1e6 && angle.abs() < 1e6)
            {
                assert_test_eq!(
                    vector * Rotor::<3, T, A>::from_rotation_xy(angle),
                    vector.rotate_xy(angle),
                    abs <= vector.length() * 1e-5 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_xz() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>()
                .filter(|(vector, angle)| vector.length() < 1e6 && angle.abs() < 1e6)
            {
                assert_test_eq!(
                    vector * Rotor::<3, T, A>::from_rotation_xz(angle),
                    vector.rotate_xz(angle),
                    abs <= vector.length() * 1e-5 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_yz() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>()
                .filter(|(vector, angle)| vector.length() < 1e6 && angle.abs() < 1e6)
            {
                assert_test_eq!(
                    vector * Rotor::<3, T, A>::from_rotation_yz(angle),
                    vector.rotate_yz(angle),
                    abs <= vector.length() * 1e-5 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (axis, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                if !axis.is_normalized() {
                    assert_debug_panic!(Rotor::<3, T, A>::from_axis_angle(axis, angle));
                }

                let axis = axis.normalize_or(Vector::ONE).normalize();
                let half_angle = angle * 0.5;

                let result = Rotor::<3, T, A>::from_axis_angle(axis, angle);

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
        for_types!(|T: PrimitiveFloat, A| {
            for (axis, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                let axis = axis.normalize_or(Vector::ONE).normalize();
                if !(axis * angle).length().is_finite() {
                    continue;
                };

                assert_panic_test_eq!(
                    Rotor::<3, T, A>::from_scaled_axis(axis * angle),
                    Rotor::<3, T, A>::from_axis_angle(axis, angle),
                    abs <= 1e-6 * axis.abs().max_element().max(angle.abs()),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for [a, b, c] in random_iter::<[T; 3]>() {
                    if [a, b, c].into_iter().any(|x| !x.is_finite() || x > 1e6) {
                        continue;
                    };

                    assert_test_eq!(
                        Rotor::<3, T, A>::from_euler(order, a, b, c),
                        Rotor::<3, T, A>::from_matrix(&Matrix::<3, T, A>::from_euler(
                            order, a, b, c
                        )),
                        abs <= 1e-6,
                        0.0 = -0.0,
                        rotor = -rotor
                    );
                }
            }
        });
    }

    #[test]
    fn test_to_axis_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<3, T, A>>() {
                if !rotor.is_normalized() {
                    assert_debug_panic!(rotor.to_axis_angle());
                }

                let rotor = rotor.normalize_or(Rotor::IDENTITY).normalize();

                let result = rotor.to_axis_angle();
                assert_test_eq!(
                    Rotor::<3, T, A>::from_axis_angle(result.0, result.1),
                    rotor,
                    abs <= 1e-6,
                    0.0 = -0.0,
                    rotor = -rotor
                );
            }
        });
    }

    #[test]
    fn test_to_scaled_axis() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<3, T, A>>() {
                if !rotor.is_normalized() {
                    assert_debug_panic!(rotor.to_scaled_axis());
                }

                let rotor = rotor.normalize_or(Rotor::IDENTITY).normalize();

                assert_test_eq!(
                    Rotor::<3, T, A>::from_scaled_axis(rotor.to_scaled_axis()),
                    rotor,
                    abs <= 1e-6,
                    0.0 = -0.0,
                    rotor = -rotor
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for rotor in random_iter::<Rotor<3, T, A>>() {
                    if !rotor.is_normalized() {
                        assert_debug_panic!(rotor.to_euler(order));
                    }

                    let rotor = rotor.normalize_or(Rotor::IDENTITY).normalize();

                    assert_test_eq!(
                        rotor.to_euler(order),
                        Matrix::<3, T, A>::from_rotor(rotor).to_euler(order)
                    );
                }
            }
        });
    }
}
