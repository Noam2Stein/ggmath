use crate::{Alignment, Quaternion, Scalar, Vector, utils::PrimitiveFloat};
#[cfg(backend)]
use crate::{EulerRot, Matrix};

#[expect(private_bounds)]
impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Creates a quaternion from an `angle` (in radians) around the x axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = (angle * T::as_from(0.5)).sin_cos();
        Self::from_xyzw(sin, T::ZERO, T::ZERO, cos)
    }

    /// Creates a quaternion from an `angle` (in radians) around the y axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = (angle * T::as_from(0.5)).sin_cos();
        Self::from_xyzw(T::ZERO, sin, T::ZERO, cos)
    }

    /// Creates a quaternion from an `angle` (in radians) around the z axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
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

        let (sin, cos) = (angle * T::as_from(0.5)).sin_cos();
        let xyz = axis * sin;
        Self::from_xyzw(xyz.x, xyz.y, xyz.z, cos)
    }

    /// Creates a quaternion that rotates `scaled_axis.length()` radians around
    /// `scaled_axis.normalize()`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_scaled_axis(scaled_axis: Vector<3, T, A>) -> Self {
        let angle = scaled_axis.length();
        if angle == T::ZERO {
            Self::IDENTITY
        } else {
            Self::from_axis_angle(scaled_axis / angle, angle)
        }
    }

    /// Creates a quaternion from an Euler rotation order/sequence and angles
    /// (in radians).
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

        Self::from_vector(result)
    }

    #[cfg(backend)]
    #[inline(always)]
    fn from_matrix3(matrix: &Matrix<3, T, A>) -> Quaternion<T, A> {
        // Ported from https://github.com/bitshifter/glam-rs `Quat::from_rotation_axes`
        // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`

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

                Quaternion::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = T::as_from(0.5) / four_ysq.sqrt();

                Quaternion::from_xyzw(
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

                Quaternion::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = T::as_from(0.5) / four_wsq.sqrt();

                Quaternion::from_xyzw(
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `dir` or `up` are not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_lh(dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        Self::from_matrix3(&Matrix::<3, T, A>::look_to_lh(dir, up))
    }

    /// Creates a quaternion from a facing direction and an up direction.
    ///
    /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=back`.
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
        Self::from_matrix3(&Matrix::<3, T, A>::look_to_rh(dir, up))
    }

    /// Creates a quaternion from a camera position, a focal point and an up
    /// direction.
    ///
    /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=forward`.
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
        Self::from_matrix3(&Matrix::<3, T, A>::look_at_lh(eye, center, up))
    }

    /// Creates a quaternion from a camera position, a focal point and an up
    /// direction.
    ///
    /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=back`.
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
        Self::from_matrix3(&Matrix::<3, T, A>::look_at_rh(eye, center, up))
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
        self.to_vector().is_nan()
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
        self.to_vector().is_finite()
    }

    /// Returns the inverse of the quaternion `self`.
    ///
    /// `self` must be normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panic if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn inverse(self) -> Self {
        #[cfg(assertions)]
        assert!(self.is_normalized());

        self.conjugate()
    }

    /// Returns the angle (in radians) for the minimal rotation for transforming
    /// `self` into `other`.
    ///
    /// `self` and `other` must be normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `self` or `other` are not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn angle_between(self, other: Self) -> T {
        #[cfg(assertions)]
        assert!(self.is_normalized() && other.is_normalized());

        self.dot(other).abs().acos() * T::as_from(2.0)
    }

    /// Returns the length/magnitude of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let quat = Quat::new(2.0, 3.0, 1.0, 1.0);
    ///
    /// assert_eq!(quat.length(), 15.0_f32.sqrt());
    /// ```
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.to_vector().length()
    }

    /// Returns `self` normalized to length `1`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `self` is a zero quaternion, or if the result is non finite or
    /// zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let quat = Quat::new(1.0, 2.0, 3.0, 4.0);
    ///
    /// assert_eq!(quat.normalize(), quat / quat.length());
    /// ```
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn normalize(self) -> Self {
        #[cfg(assertions)]
        assert!(
            self != Self::from_vector(Vector::ZERO),
            "cannot normalize a zero quaternion"
        );

        let result = self / self.length();

        #[cfg(assertions)]
        assert!(
            result.is_finite() && result != Self::from_vector(Vector::ZERO),
            "non finite result: {self:?}.normalize()"
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
    /// let non_zero = Quat::new(1.0, 2.0, 3.0, 4.0);
    /// let zero = Quat::new(0.0, 0.0, 0.0, 0.0);
    ///
    /// assert_eq!(non_zero.try_normalize(), Some(non_zero.normalize()));
    /// assert_eq!(zero.try_normalize(), None);
    /// ```
    ///
    /// [`normalize`]: Self::normalize
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        self.to_vector().try_normalize().map(Self::from_vector)
    }

    /// Returns [`normalize`], or `fallback` if `self` is zero or if the result
    /// is non finite or zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let non_zero = Quat::new(1.0, 2.0, 3.0, 4.0);
    /// let zero = Quat::new(0.0, 0.0, 0.0, 0.0);
    /// let fallback = Quat::new(2.0, 4.0, 0.0, 1.0);
    ///
    /// assert_eq!(non_zero.normalize_or(fallback), non_zero.normalize());
    /// assert_eq!(zero.normalize_or(fallback), fallback);
    /// ```
    ///
    /// [`normalize`]: Self::normalize
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        Self::from_vector(self.to_vector().normalize_or(fallback.to_vector()))
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
        self.to_vector().is_normalized()
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two quaternions that should be equal, but
    /// may have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool {
        self.to_vector()
            .abs_diff_eq(other.to_vector(), max_abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Matrix, Quat, Quaternion, Vector,
        utils::{assert_assertions_panic, assert_float_eq, assert_panic_float_eq, for_parameters},
    };

    #[test]
    fn test_from_rotation_x() {
        for_parameters!(|T: PrimitiveFloat, A, angle| {
            assert_float_eq!(
                Quaternion::<T, A>::from_rotation_x(angle),
                Quaternion::from_xyzw((angle * 0.5).sin(), 0.0, 0.0, (angle * 0.5).cos())
            );
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_parameters!(|T: PrimitiveFloat, A, angle| {
            assert_float_eq!(
                Quaternion::<T, A>::from_rotation_y(angle),
                Quaternion::from_xyzw(0.0, (angle * 0.5).sin(), 0.0, (angle * 0.5).cos())
            );
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_parameters!(|T: PrimitiveFloat, A, angle| {
            assert_float_eq!(
                Quaternion::<T, A>::from_rotation_z(angle),
                Quaternion::from_xyzw(0.0, 0.0, (angle * 0.5).sin(), (angle * 0.5).cos())
            );
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let angle = x + y + z + 1.5;

            if let Some(axis) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                let quat = Quaternion::<T, A>::from_axis_angle(axis, angle);

                assert_panic_float_eq!(
                    quat.canonical(),
                    Matrix::<4, T, A>::from_axis_angle(axis, angle)
                        .to_scale_rotation_translation()
                        .1
                        .canonical(),
                    abs <= Quaternion::from_vector(quat.to_vector().abs() * 1e-4 + 1e-6),
                    0.0 = -0.0
                );
            }

            let axis = Vector::<3, T, A>::new(x, y, z);
            if !axis.is_normalized() {
                assert_assertions_panic!(Quaternion::<T, A>::from_axis_angle(axis, angle));
            }
        });
    }

    #[test]
    fn test_from_scaled_axis() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let angle = x + y + z + 1.0;

            if let Some(axis) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                assert_panic_float_eq!(
                    Quaternion::<T, A>::from_scaled_axis(axis * angle),
                    Quaternion::<T, A>::from_axis_angle(axis, angle),
                    abs <= Quaternion::from_vector(
                        Vector::splat(1e-6) * x.abs().max(y.abs()).max(z.abs())
                    ),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_euler() {
        for_parameters!(|T: PrimitiveFloat, A, order, a, b, c| {
            let _: [T; 3] = [a, b, c];

            if !a.is_finite() || !b.is_finite() || !c.is_finite() || a > 1e6 || b > 1e6 || c > 1e6 {
                return;
            }

            assert_float_eq!(
                Quaternion::<T, A>::from_euler(order, a, b, c).canonical(),
                Matrix::<4, T, A>::from_euler(order, a, b, c)
                    .to_scale_rotation_translation()
                    .1
                    .canonical(),
                abs <= Quaternion::from_vector(Vector::splat(1e-6)),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let Some(dir) = Vector::<3, T, A>::new(x, y, z).try_normalize() else {
                return;
            };

            let up = (dir * 0.4 + dir.zxy().with_z(0.3)).normalize();

            assert_float_eq!(
                Quaternion::<T, A>::look_to_lh(dir, up).canonical(),
                Matrix::<4, T, A>::look_to_lh(Vector::ZERO, dir, up)
                    .to_scale_rotation_translation()
                    .1
                    .canonical(),
                abs <= Quaternion::from_vector(Vector::splat(1e-6))
            );

            if !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_assertions_panic!(Quaternion::<T, A>::look_to_lh(
                    Vector::<3, T, A>::new(x, y, z),
                    up
                ));
                assert_assertions_panic!(Quaternion::<T, A>::look_to_lh(
                    dir,
                    Vector::<3, T, A>::new(x, y, z)
                ));
            }
        })
    }

    #[test]
    fn test_look_to_rh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let Some(dir) = Vector::<3, T, A>::new(x, y, z).try_normalize() else {
                return;
            };

            let up = (dir * 0.4 + dir.zxy().with_z(0.3)).normalize();

            assert_float_eq!(
                Quaternion::<T, A>::look_to_rh(dir, up).canonical(),
                Matrix::<4, T, A>::look_to_rh(Vector::ZERO, dir, up)
                    .to_scale_rotation_translation()
                    .1
                    .canonical(),
                abs <= Quaternion::from_vector(Vector::splat(1e-6))
            );

            if !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_assertions_panic!(Quaternion::<T, A>::look_to_rh(
                    Vector::<3, T, A>::new(x, y, z),
                    up
                ));
                assert_assertions_panic!(Quaternion::<T, A>::look_to_rh(
                    dir,
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

            assert_panic_float_eq!(
                Quaternion::<T, A>::look_at_lh(eye, center, up).canonical(),
                Matrix::<4, T, A>::look_at_lh(eye, center, up)
                    .to_scale_rotation_translation()
                    .1
                    .canonical(),
                abs <= Quaternion::from_vector(Vector::splat(1e-6))
            );

            if !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_assertions_panic!(Quaternion::<T, A>::look_at_lh(
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

            assert_panic_float_eq!(
                Quaternion::<T, A>::look_at_rh(eye, center, up).canonical(),
                Matrix::<4, T, A>::look_at_rh(eye, center, up)
                    .to_scale_rotation_translation()
                    .1
                    .canonical(),
                abs <= Quaternion::from_vector(Vector::splat(1e-6))
            );

            if !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_assertions_panic!(Quaternion::<T, A>::look_at_rh(
                    eye,
                    center,
                    Vector::<3, T, A>::new(x, y, z),
                ));
            }
        })
    }

    #[test]
    fn test_is_nan() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).is_nan(),
                x.is_nan() || y.is_nan() || z.is_nan() || w.is_nan()
            );
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).is_finite(),
                x.is_finite() && y.is_finite() && z.is_finite() && w.is_finite()
            );
        });
    }

    #[test]
    fn test_inverse() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let quat = Quaternion::<T, A>::from_xyzw(x, y, z, w);
            if quat.is_normalized() {
                assert_float_eq!(quat.inverse(), quat.conjugate());
            } else {
                assert_assertions_panic!(quat.inverse());
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_parameters!(|T: PrimitiveFloat, A, x, y| {
            let _: [T; 2] = [x, y];

            if !x.is_finite() || !y.is_finite() || x.abs() > 1e3 || y.abs() > 1e3 {
                return;
            }

            let tau = core::f64::consts::TAU as T;
            let angle_between = ((x - y).abs() % tau).min(tau - (x - y).abs() % tau);

            assert_float_eq!(
                Quaternion::<T, A>::from_rotation_x(x)
                    .angle_between(Quaternion::<T, A>::from_rotation_x(y)),
                angle_between,
                r2nd <= 1e-6 * x.abs().max(y.abs())
            );
            assert_float_eq!(
                Quaternion::<T, A>::from_rotation_y(x)
                    .angle_between(Quaternion::<T, A>::from_rotation_y(y)),
                angle_between,
                r2nd <= 1e-6 * x.abs().max(y.abs())
            );
            assert_float_eq!(
                Quaternion::<T, A>::from_rotation_z(x)
                    .angle_between(Quaternion::<T, A>::from_rotation_z(y)),
                angle_between,
                r2nd <= 1e-6 * x.abs().max(y.abs())
            );
        });
    }

    #[test]
    fn test_length() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).length(),
                Vector::<4, T, A>::new(x, y, z, w).length()
            );
        });
    }

    #[test]
    fn test_normalize() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_float_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).normalize(),
                Quaternion::from_vector(Vector::<4, T, A>::new(x, y, z, w).normalize())
            );
        });
    }

    #[test]
    fn test_try_normalize() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_float_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w)
                    .try_normalize()
                    .unwrap(),
                Quaternion::from_vector(
                    Vector::<4, T, A>::new(x, y, z, w).try_normalize().unwrap()
                )
            );
        });
    }

    #[test]
    fn test_normalize_or() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_float_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::NAN),
                Quaternion::from_vector(
                    Vector::<4, T, A>::new(x, y, z, w).normalize_or(Vector::NAN)
                )
            );
        });
    }

    #[test]
    fn test_is_normalized() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).is_normalized(),
                Vector::<4, T, A>::new(x, y, z, w).is_normalized()
            );
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|T: PrimitiveFloat| {
            assert!(
                Quat::<T>::from_xyzw(0.0, 1.0, 2.0, 3.0)
                    .abs_diff_eq(Quat::from_xyzw(0.0, 1.1, 2.05, 2.9), 0.125)
            );
            assert!(
                !Quat::<T>::from_xyzw(0.0, 1.0, 2.0, 3.0)
                    .abs_diff_eq(Quat::from_xyzw(0.0, 1.1, 2.5, 2.9), 0.125)
            );
        });
    }
}
