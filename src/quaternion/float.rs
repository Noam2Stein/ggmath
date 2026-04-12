use crate::{Alignment, Quaternion, Scalar, Vector, utils::PrimitiveFloat};

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
        assert!(self != Self::ZERO, "cannot normalize a zero quaternion");

        let result = self / self.length();

        #[cfg(assertions)]
        assert!(
            result.is_finite() && result != Self::ZERO,
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
