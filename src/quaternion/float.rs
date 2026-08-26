use crate::{Alignment, EulerRot, FloatExt, Matrix, PrimitiveFloat, Quaternion, Vector};

impl<T, A: Alignment> Quaternion<T, A>
where
    T: PrimitiveFloat,
{
    /// A quaternion with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::from_vector(Vector::<4, T, A>::NAN);

    /// Creates a quaternion from an `angle` (in radians) around the x axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = (angle * T::as_from(0.5)).sin_cos();
        Self::from_xyzw(sin, T::ZERO, T::ZERO, cos)
    }

    /// Creates a quaternion from an `angle` (in radians) around the y axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = (angle * T::as_from(0.5)).sin_cos();
        Self::from_xyzw(T::ZERO, sin, T::ZERO, cos)
    }

    /// Creates a quaternion from an `angle` (in radians) around the z axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = (angle * T::as_from(0.5)).sin_cos();
        Self::from_xyzw(T::ZERO, T::ZERO, sin, cos)
    }

    /// Creates a quaternion from a rotation `axis` and `angle` (in radians).
    ///
    /// `axis` must be normalized.
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

        let (sin, cos) = (angle * T::as_from(0.5)).sin_cos();
        let xyz = axis * sin;
        Self::from_xyzw(xyz.x, xyz.y, xyz.z, cos)
    }

    /// Creates a quaternion that rotates `scaled_axis.length()` radians around
    /// `scaled_axis.normalize()`.
    #[inline]
    #[must_use]
    pub fn from_scaled_axis(scaled_axis: Vector<3, T, A>) -> Self {
        let angle = scaled_axis.length();
        if angle == T::ZERO {
            Self::IDENTITY
        } else {
            let (sin, cos) = (angle * T::as_from(0.5)).sin_cos();
            let xyz = scaled_axis / angle * sin;
            Self::from_xyzw(xyz.x, xyz.y, xyz.z, cos)
        }
    }

    /// Returns the minimal rotation transforming `from` to `to`.
    ///
    /// The rotation is in the plane spanned by `from` and `to`. Rotates up to
    /// 180 degrees.
    ///
    /// When `from≈to` this is only accurate to about `0.001` (for `f32`).
    ///
    /// `from` and `to` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `from` or `to` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc(from: Vector<3, T, A>, to: Vector<3, T, A>) -> Self {
        // Ported from `https://github.com/bitshifter/glam-rs`.

        debug_assert!(
            from.is_normalized() && to.is_normalized(),
            "vectors are not normalized: from_rotation_arc({from:?}, {to:?})"
        );

        let almost_one = T::ONE - T::as_from(2.0) * T::EPSILON;

        let dot = from.dot(to);
        if dot > almost_one {
            // 0° singularity: from ≈ to.
            Self::IDENTITY
        } else if dot < -almost_one {
            // 180° singularity: from ≈ -to.
            // Half a turn = 𝛕/2 = 180°.
            Self::from_axis_angle(from.any_orthonormal_vector(), T::PI)
        } else {
            let cross = from.cross(to);
            Self::from_xyzw(cross.x, cross.y, cross.z, T::ONE + dot).normalize()
        }
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
    /// `from` and `to` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `from` or `to` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc_colinear(from: Vector<3, T, A>, mut to: Vector<3, T, A>) -> Self {
        // Ported from `https://github.com/bitshifter/glam-rs`.

        debug_assert!(
            from.is_normalized() && to.is_normalized(),
            "vectors are not normalized: from_rotation_arc_colinear({from:?}, {to:?})"
        );

        let almost_one = T::ONE - T::as_from(2.0) * T::EPSILON;

        let mut dot = from.dot(to);
        if dot.is_sign_negative() {
            dot = -dot;
            to = -to;
        }

        if dot > almost_one {
            // 0° singularity: from ≈ to.
            Self::IDENTITY
        } else {
            let cross = from.cross(to);
            Self::from_xyzw(cross.x, cross.y, cross.z, T::ONE + dot).normalize()
        }
    }

    /// Creates a quaternion from an Euler rotation order/sequence and angles
    /// (in radians).
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

    /// Creates a quaternion from a 3D rotation matrix.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `matrix` is not a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_matrix(matrix: &Matrix<3, T, A>) -> Self {
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
            "not a rotation matrix: Quaternion::from_matrix({matrix:?})"
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

                Self::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = T::as_from(0.5) / four_ysq.sqrt();

                Self::from_xyzw(
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

                Self::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = T::as_from(0.5) / four_wsq.sqrt();

                Self::from_xyzw(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    /// Creates a quaternion from a facing direction and an up direction.
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

    /// Creates a quaternion from a facing direction and an up direction.
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

    /// Creates a quaternion from a camera position, a focal point and an up
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

    /// Creates a quaternion from a camera position, a focal point and an up
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

    /// Converts the quaternion `self` to a normalized rotation axis and an
    /// angle (in radians).
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
            "quaternion is not normalized: {self:?}.to_axis_angle()"
        );

        let xyz = Vector::<3, T, A>::new(self.x, self.y, self.z);
        let length = xyz.length();

        if length >= T::as_from(1e-8) {
            let axis = xyz / length;
            let angle = length.atan2(self.w) * T::as_from(2.0);

            (axis, angle)
        } else {
            (Vector::<3, T, A>::X, T::ZERO)
        }
    }

    /// Converts the quaternion `self` to a rotation axis scaled by an angle (in
    /// radians).
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
            "quaternion is not normalized: {self:?}.to_scaled_axis()"
        );

        let xyz = Vector::<3, T, A>::new(self.x, self.y, self.z);
        let length = xyz.length();

        if length >= T::as_from(1e-8) {
            let axis = xyz / length;
            let angle = length.atan2(self.w) * T::as_from(2.0);

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
            "quaternion is not normalized: {self:?}.to_euler({order:?})"
        );

        Matrix::<3, T, A>::from_quat(self).to_euler(order)
    }

    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let nan = Quat::from_xyzw(1.0, 2.0, 3.0, f32::NAN);
    /// let normal = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    ///
    /// assert!(nan.is_nan());
    /// assert!(!normal.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let finite = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    /// let inf = Quat::from_xyzw(1.0, f32::INFINITY, 3.0, 4.0);
    /// let neg_inf = Quat::from_xyzw(1.0, f32::NEG_INFINITY, 3.0, 4.0);
    /// let nan = Quat::from_xyzw(1.0, f32::NEG_INFINITY, 3.0, 4.0);
    ///
    /// assert!(finite.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// assert!(!nan.is_finite());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Returns the inverse of the quaternion `self`.
    ///
    /// `self` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panic if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn inverse(self) -> Self {
        debug_assert!(
            self.is_normalized(),
            "quaternion is not normalized: {self:?}.inverse()"
        );

        self.conjugate()
    }

    /// Returns the angle (in radians) for the minimal rotation for transforming
    /// `self` into `other`.
    ///
    /// `self` and `other` must be normalized.
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
            "quaternions are not normalized: {self:?}.angle_between({other:?})"
        );

        self.dot(other).abs().min(T::ONE).acos() * T::as_from(2.0)
    }

    /// Computes the linear interpolation between `self` and `other` based on
    /// the value `t`.
    ///
    /// When `t` is `0`, the result is `self`.  When `t` is `1`, the result is
    /// `rhs`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lerp(self, other: Self, t: T) -> Self {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "quaternions are not normalized: {self:?}.lerp({other:?}, {t:?})"
        );

        let other = if self.dot(other).is_sign_negative() {
            -other
        } else {
            other
        };

        (self * (T::ONE - t) + other * t).normalize()
    }

    /// Computes the spherical linear interpolation between `self` and `other`
    /// based on the value `t`.
    ///
    /// When `t` is `0`, the result is `self`.  When `t` is `1`, the result is
    /// `other`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn slerp(self, mut other: Self, t: T) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs
        // See http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/

        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "quaternions are not normalized: {self:?}.slerp({other:?}, {t:?})"
        );

        // Note that a rotation can be represented by two quaternions: `q` and
        // `-q`. The slerp path between `q` and `other` will be different from
        // the path between `-q` and `other`. One path will take the long way
        // around and one will take the short way. In order to correct for this,
        // the `dot` product between `self` and `other` should be positive. If
        // the `dot` product is negative, slerp between `self` and `-other`.
        let mut dot = self.dot(other);
        if dot.is_sign_negative() {
            other = -other;
            dot = -dot;
        }

        if dot > T::ONE - T::EPSILON {
            // If above threshold, perform linear interpolation to avoid divide by zero.
            (self * (T::ONE - t) + other * t).normalize()
        } else {
            let theta = dot.acos();

            let x = T::ONE - t;
            let y = t;
            let z = T::ONE;

            let tmp = Vector::<4, T, A>::new(x, y, z, T::ZERO) * theta;
            let tmp = tmp.sin();

            Self((self.0 * tmp.x + other.0 * tmp.y) / tmp.z)
        }
    }

    /// Rotates `self` towards `target` by at most `max_angle` (in radians).
    ///
    /// When `max_angle` is `0`, the result is `self`. When `max_angle` is equal
    /// to or greater than `self.angle_between(target)`, the result is `target`.
    /// When `max_angle` is negative, rotates towards the opposite of `target`.
    ///
    /// `self` and `target` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `target` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn rotate_towards(self, target: Self, max_angle: T) -> Self {
        debug_assert!(
            self.is_normalized() && target.is_normalized(),
            "quaternions are not normalized: {self:?}.rotate_towards({target:?}, {max_angle:?})"
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let quat = Quat::from_xyzw(2.0, 3.0, 1.0, 1.0);
    ///
    /// assert_eq!(quat.length(), 15.0_f32.sqrt());
    /// ```
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
    /// Panics if `self` is a zero quaternion, or if the result is non finite or
    /// zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let quat = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    ///
    /// assert_eq!(quat.normalize(), quat / quat.length());
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn normalize(self) -> Self {
        let result = self / self.length();

        debug_assert!(
            result.is_finite() && result != Self(Vector::ZERO),
            "quaternion is zero or non-finite: {self:?}.normalize()"
        );

        result
    }

    /// Returns [`normalize`], or `None` if `self` is zero or if the result is
    /// non finite or zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let non_zero = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    /// let zero = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
    ///
    /// assert_eq!(non_zero.try_normalize(), Some(non_zero.normalize()));
    /// assert_eq!(zero.try_normalize(), None);
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let non_zero = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    /// let zero = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
    /// let fallback = Quat::from_xyzw(2.0, 4.0, 0.0, 1.0);
    ///
    /// assert_eq!(non_zero.normalize_or(fallback), non_zero.normalize());
    /// assert_eq!(zero.normalize_or(fallback), fallback);
    /// ```
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
    pub fn normalize_and_length(self) -> (Self, T) {
        let (normalize, length) = self.0.normalize_and_length();

        (Self(normalize), length)
    }

    /// Returns whether the quaternion has the length `1.0` or not.
    ///
    /// This uses a precision threshold of approximately `1e-4`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let unit = Quat::from_xyzw(0.5, 0.5, 0.5, 0.5);
    /// let non_unit = Quat::from_xyzw(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert!(unit.is_normalized());
    /// assert!(!non_unit.is_normalized());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        self.0.is_normalized()
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two quaternions that should be equal, but
    /// may have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool {
        self.0.abs_diff_eq(other.0, max_abs_diff)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        EulerRot, FloatExt, Matrix, QuatA, Quaternion, Vector,
        test_utils::{
            assert_debug_panic, assert_panic_test_eq, assert_test_eq, for_types, random_iter,
            test_eq,
        },
        utils::PrimitiveFloatUtils,
    };

    #[test]
    fn test_constants() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Quaternion::<T, A>::NAN,
                Quaternion::from_xyzw(T::NAN, T::NAN, T::NAN, T::NAN)
            );
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter() {
                assert_test_eq!(
                    Quaternion::<T, A>::from_rotation_x(angle),
                    Quaternion::from_xyzw((angle * 0.5).sin(), 0.0, 0.0, (angle * 0.5).cos()),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter() {
                assert_test_eq!(
                    Quaternion::<T, A>::from_rotation_y(angle),
                    Quaternion::from_xyzw(0.0, (angle * 0.5).sin(), 0.0, (angle * 0.5).cos()),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter() {
                assert_test_eq!(
                    Quaternion::<T, A>::from_rotation_z(angle),
                    Quaternion::from_xyzw(0.0, 0.0, (angle * 0.5).sin(), (angle * 0.5).cos()),
                    abs <= angle.abs() * 1e-4 + 1e-3,
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
                    assert_debug_panic!(Quaternion::<T, A>::from_axis_angle(axis, angle));
                }

                let axis = axis.normalize_or(Vector::ONE).normalize();

                let result = Quaternion::<T, A>::from_axis_angle(axis, angle);

                assert_test_eq!(
                    result.w,
                    (angle * 0.5).cos(),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    result.x,
                    (angle * 0.5).sin() * axis.x,
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    result.y,
                    (angle * 0.5).sin() * axis.y,
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    result.z,
                    (angle * 0.5).sin() * axis.z,
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
                    Quaternion::<T, A>::from_scaled_axis(axis * angle),
                    Quaternion::<T, A>::from_axis_angle(axis, angle),
                    abs <= 1e-6 * axis.abs().max_element().max(angle.abs()),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_arc() {
        for_types!(|T: PrimitiveFloat, A| {
            for [start, end] in random_iter::<[Vector<3, T, A>; 2]>() {
                if !start.is_normalized() {
                    assert_debug_panic!(Quaternion::<T, A>::from_rotation_arc(
                        start,
                        end.normalize()
                    ));
                }
                if !end.is_normalized() {
                    assert_debug_panic!(Quaternion::<T, A>::from_rotation_arc(
                        start.normalize(),
                        end
                    ));
                }

                let start = start.normalize_or(Vector::ONE).normalize();
                let end = end.normalize_or(Vector::ONE).normalize();

                let result = Quaternion::<T, A>::from_rotation_arc(start, end);
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
            for [start, end] in random_iter::<[Vector<3, T, A>; 2]>() {
                if !start.is_normalized() {
                    assert_debug_panic!(Quaternion::<T, A>::from_rotation_arc_colinear(
                        start,
                        end.normalize()
                    ));
                }
                if !end.is_normalized() {
                    assert_debug_panic!(Quaternion::<T, A>::from_rotation_arc_colinear(
                        start.normalize(),
                        end
                    ));
                }

                let start = start.normalize_or(Vector::ONE).normalize();
                let end = end.normalize_or(Vector::ONE).normalize();

                let result = Quaternion::<T, A>::from_rotation_arc_colinear(start, end);
                let (result_axis, result_angle) = result.to_axis_angle();

                if ((1.0 as T).to_radians()..(179.0 as T).to_radians())
                    .contains(&start.angle_between(end))
                {
                    assert!(
                        test_eq!(start * result, end, abs <= 1e-5, 0.0 = -0.0)
                            || test_eq!(start * result, -end, abs <= 1e-5, 0.0 = -0.0)
                    );
                    assert_test_eq!(
                        result_angle,
                        start.angle_between(end).min(start.angle_between(-end)),
                        abs <= 1e-4
                    );
                    assert_test_eq!(result_axis.dot(start), 0.0, abs <= 1e-5, 0.0 = -0.0);
                    assert_test_eq!(result_axis.dot(end), 0.0, abs <= 1e-5, 0.0 = -0.0);
                } else {
                    assert!(
                        test_eq!(start * result, end, abs <= 1e-2, 0.0 = -0.0)
                            || test_eq!(start * result, -end, abs <= 1e-2, 0.0 = -0.0)
                    );
                    assert_test_eq!(
                        result_angle,
                        start.angle_between(end).min(start.angle_between(-end)),
                        abs <= 1e-2
                    );
                    if result_angle != 0.0 {
                        assert_test_eq!(result_axis.dot(start), 0.0, abs <= 1e-2, 0.0 = -0.0);
                        assert_test_eq!(result_axis.dot(end), 0.0, abs <= 1e-2, 0.0 = -0.0);
                    }
                }
                assert!(result_angle <= T::TAU / 4.0 + 0.1);
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
                        Quaternion::<T, A>::from_euler(order, a, b, c),
                        Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::from_euler(
                            order, a, b, c
                        )),
                        abs <= 1e-6,
                        0.0 = -0.0,
                        quat = -quat
                    );
                }
            }
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|T: PrimitiveFloat, A| {
            for [x, y, z] in random_iter::<[T; 3]>() {
                if [x, y, z].into_iter().any(|x| !x.is_finite() || x > 1e6) {
                    continue;
                };

                assert_test_eq!(
                    Quaternion::<T, A>::from_matrix(
                        &(Matrix::<3, T, A>::from_rotation_yz(x)
                            * Matrix::<3, T, A>::from_rotation_xz(-y)
                            * Matrix::<3, T, A>::from_rotation_yz(z))
                    ),
                    Quaternion::<T, A>::from_rotation_x(x)
                        * Quaternion::<T, A>::from_rotation_y(y)
                        * Quaternion::<T, A>::from_rotation_z(z),
                    abs <= 1e-6,
                    0.0 = -0.0,
                    quat = -quat
                );
            }

            for matrix in random_iter::<Matrix<3, T, A>>().take(10) {
                if !matrix.determinant().abs_diff_eq(1.0, 1e-4)
                    || !matrix.x_axis.dot(matrix.y_axis).abs_diff_eq(0.0, 1e-4)
                    || !matrix.x_axis.dot(matrix.z_axis).abs_diff_eq(0.0, 1e-4)
                    || !matrix.y_axis.dot(matrix.z_axis).abs_diff_eq(0.0, 1e-4)
                    || !matrix
                        .x_axis
                        .cross(matrix.y_axis)
                        .abs_diff_eq(matrix.z_axis, 1e-4)
                {
                    assert_debug_panic!(Quaternion::<T, A>::from_matrix(&matrix));
                }
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [dir, up] in random_iter::<[Vector<3, T, A>; 2]>() {
                assert_panic_test_eq!(
                    Quaternion::<T, A>::look_to_lh(dir, up),
                    Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::look_to_lh(dir, up))
                );

                let dir = dir.normalize_or(Vector::<3, T, A>::Z);
                let up = up.normalize_or(Vector::<3, T, A>::Y);

                assert_panic_test_eq!(
                    Quaternion::<T, A>::look_to_lh(dir, up),
                    Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::look_to_lh(dir, up))
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [dir, up] in random_iter::<[Vector<3, T, A>; 2]>() {
                assert_panic_test_eq!(
                    Quaternion::<T, A>::look_to_rh(dir, up),
                    Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::look_to_rh(dir, up))
                );

                let dir = dir.normalize_or(Vector::<3, T, A>::Z);
                let up = up.normalize_or(Vector::<3, T, A>::Y);

                assert_panic_test_eq!(
                    Quaternion::<T, A>::look_to_rh(dir, up),
                    Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::look_to_rh(dir, up))
                );
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Quaternion::<T, A>::look_at_lh(eye, center, up),
                    Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::look_at_lh(
                        eye, center, up
                    ))
                );

                let up = up.normalize_or(Vector::<3, T, A>::Y);

                assert_panic_test_eq!(
                    Quaternion::<T, A>::look_at_lh(eye, center, up),
                    Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::look_at_lh(
                        eye, center, up
                    ))
                );
            }
        });
    }

    #[test]
    fn test_look_at_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Quaternion::<T, A>::look_at_rh(eye, center, up),
                    Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::look_at_rh(
                        eye, center, up
                    ))
                );

                let up = up.normalize_or(Vector::<3, T, A>::Y);

                assert_panic_test_eq!(
                    Quaternion::<T, A>::look_at_rh(eye, center, up),
                    Quaternion::<T, A>::from_matrix(&Matrix::<3, T, A>::look_at_rh(
                        eye, center, up
                    ))
                );
            }
        });
    }

    #[test]
    fn test_to_axis_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(quat.to_axis_angle());
                }

                let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();

                let result = quat.to_axis_angle();
                assert_test_eq!(
                    Quaternion::<T, A>::from_axis_angle(result.0, result.1),
                    quat,
                    abs <= 1e-6,
                    0.0 = -0.0,
                    quat = -quat
                );
            }
        });
    }

    #[test]
    fn test_to_scaled_axis() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(quat.to_scaled_axis());
                }

                let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();

                assert_test_eq!(
                    Quaternion::<T, A>::from_scaled_axis(quat.to_scaled_axis()),
                    quat,
                    abs <= 1e-6,
                    0.0 = -0.0,
                    quat = -quat
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for quat in random_iter::<Quaternion<T, A>>() {
                    if !quat.is_normalized() {
                        assert_debug_panic!(quat.to_euler(order));
                    }

                    let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();

                    assert_test_eq!(
                        quat.to_euler(order),
                        Matrix::<3, T, A>::from_quat(quat).to_euler(order)
                    );
                }
            }
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_eq!(quat.is_nan(), quat.to_vector().is_nan());
            }
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_eq!(quat.is_finite(), quat.to_vector().is_finite());
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(quat.inverse());
                }

                let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();

                assert_test_eq!(quat.inverse(), quat.conjugate());
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat, other] in random_iter::<[Quaternion<T, A>; 2]>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(quat.angle_between(other.normalize()));
                }
                if !other.is_normalized() {
                    assert_debug_panic!(quat.normalize().angle_between(other));
                }

                let [quat, other] =
                    [quat, other].map(|q| q.normalize_or(Quaternion::IDENTITY).normalize());

                assert_test_eq!(
                    quat.angle_between(other),
                    (quat * other.inverse()).w.abs().acos() * 2.0,
                    abs <= 2e-4
                );
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat, other] in random_iter::<[Quaternion<T, A>; 2]>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(quat.lerp(other.normalize(), 0.2));
                }
                if !other.is_normalized() {
                    assert_debug_panic!(quat.normalize().lerp(other, 0.2));
                }

                let [quat, other] =
                    [quat, other].map(|q| q.normalize_or(Quaternion::IDENTITY).normalize());

                assert_test_eq!(quat.lerp(other, 0.0), quat, abs <= 1e-6, 0.0 = -0.0);
                assert_test_eq!(
                    quat.lerp(other, 0.5).angle_between(quat),
                    quat.angle_between(other) / 2.0,
                    abs <= quat.angle_between(other) * 1e-6 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    quat.lerp(other, 0.5).angle_between(other),
                    quat.angle_between(other) / 2.0,
                    abs <= quat.angle_between(other) * 1e-6 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    quat.lerp(other, 1.0),
                    other,
                    abs <= 1e-6,
                    0.0 = -0.0,
                    quat = -quat
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat, other] in random_iter::<[Quaternion<T, A>; 2]>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(quat.slerp(other.normalize(), 0.2));
                }
                if !other.is_normalized() {
                    assert_debug_panic!(quat.normalize().slerp(other, 0.2));
                }

                let [quat, other] =
                    [quat, other].map(|q| q.normalize_or(Quaternion::IDENTITY).normalize());

                assert_test_eq!(
                    quat.slerp(other, 0.0),
                    quat,
                    abs <= 1e-6,
                    0.0 = -0.0,
                    quat = -quat
                );
                assert_test_eq!(
                    quat.slerp(other, 1.0),
                    other,
                    abs <= 1e-6,
                    0.0 = -0.0,
                    quat = -quat
                );

                for t in [0.25, 0.5, 0.75] {
                    let result = quat.slerp(other, t);

                    if result.angle_between(quat).is_nan() && !result.is_nan() {
                        continue;
                    }
                    if result.angle_between(other).is_nan() && !result.is_nan() {
                        continue;
                    }

                    if ((1.0 as T)..(179.0 as T)).contains(&quat.angle_between(other)) {
                        assert_test_eq!(
                            result.angle_between(quat),
                            quat.angle_between(other) * t,
                            abs <= quat.angle_between(other) * 1e-6 + 1e-3,
                            0.0 = -0.0
                        );
                        assert_test_eq!(
                            result.angle_between(other),
                            quat.angle_between(other) * (1.0 - t),
                            abs <= quat.angle_between(other) * 1e-6 + 1e-3,
                            0.0 = -0.0
                        );
                    } else {
                        assert_test_eq!(
                            result.angle_between(quat),
                            quat.angle_between(other) * t,
                            abs <= quat.angle_between(other) * 1e-4 + 1e-2,
                            0.0 = -0.0
                        );
                        assert_test_eq!(
                            result.angle_between(other),
                            quat.angle_between(other) * (1.0 - t),
                            abs <= quat.angle_between(other) * 1e-4 + 1e-2,
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
            for [quat, target] in random_iter::<[Quaternion<T, A>; 2]>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(quat.rotate_towards(target.normalize(), 0.2));
                }
                if !target.is_normalized() {
                    assert_debug_panic!(quat.normalize().rotate_towards(target, 0.2));
                }

                let [quat, target] =
                    [quat, target].map(|q| q.normalize_or(Quaternion::IDENTITY).normalize());

                assert_test_eq!(
                    quat.rotate_towards(target, 0.0),
                    quat,
                    abs <= 1e-3,
                    0.0 = -0.0,
                    quat = -quat
                );
                assert_test_eq!(
                    quat.rotate_towards(target, quat.angle_between(target)),
                    target,
                    abs <= 1e-3,
                    0.0 = -0.0,
                    quat = -quat
                );
                assert_test_eq!(
                    quat.rotate_towards(target, quat.angle_between(target) * 1.5),
                    target,
                    abs <= 1e-3,
                    0.0 = -0.0,
                    quat = -quat
                );

                for t in [0.25, 0.5, 0.75] {
                    assert_test_eq!(
                        quat.rotate_towards(target, quat.angle_between(target) * t),
                        quat.slerp(target, t),
                        abs <= 1e-3,
                        0.0 = -0.0,
                        quat = -quat
                    );
                }
            }
        });
    }

    #[test]
    fn test_length() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_test_eq!(quat.length(), quat.to_vector().length());
            }
        });
    }

    #[test]
    fn test_normalize() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_panic_test_eq!(
                    quat.normalize(),
                    Quaternion::from_vector(quat.to_vector().normalize())
                );
            }
        });
    }

    #[test]
    fn test_try_normalize() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_panic_test_eq!(
                    quat.try_normalize().unwrap(),
                    Quaternion::from_vector(quat.to_vector().try_normalize().unwrap())
                );
            }
        });
    }

    #[test]
    fn test_normalize_or() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat, fallback] in random_iter::<[Quaternion<T, A>; 2]>() {
                assert_panic_test_eq!(
                    quat.normalize_or(fallback),
                    Quaternion::from_vector(quat.to_vector().normalize_or(fallback.to_vector()))
                );
            }
        });
    }

    #[test]
    fn test_normalize_and_length() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_test_eq!(
                    quat.normalize_and_length(),
                    (
                        Quaternion::from_vector(quat.to_vector().normalize_and_length().0),
                        quat.normalize_and_length().1
                    )
                );
            }
        });
    }

    #[test]
    fn test_is_normalized() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_eq!(quat.is_normalized(), quat.to_vector().is_normalized());
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|T: PrimitiveFloat| {
            assert!(
                QuatA::<T>::from_xyzw(0.0, 1.0, 2.0, 3.0)
                    .abs_diff_eq(QuatA::from_xyzw(0.0, 1.1, 2.05, 2.9), 0.125)
            );
            assert!(
                !QuatA::<T>::from_xyzw(0.0, 1.0, 2.0, 3.0)
                    .abs_diff_eq(QuatA::from_xyzw(0.0, 1.1, 2.5, 2.9), 0.125)
            );
        });
    }
}
