use crate::{
    Affine, Alignment, EulerRot, FloatExt, Length, Matrix, PrimitiveFloat, Projective, Rotor,
    Vector,
    length::TwoOrThree,
    utils::{PrimitiveFloatUtils, specialize_23, transmute_generic},
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: PrimitiveFloat,
{
    /// A rotor with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::NAN_INTERNAL_IMPL;

    /// The implementation of [`Self::NAN`].
    ///
    /// Because of type system limitations, this implementation looks crazy. Use
    /// a separate constant so that IDEs do not show the implementation.
    const NAN_INTERNAL_IMPL: Self = match N {
        // SAFETY: We are transmuting a type to itself
        2 => unsafe {
            transmute_generic::<Rotor<2, T, A>, Rotor<N, T, A>>(Rotor::<2, T, A>(
                Vector::<2, T, A>::NAN,
            ))
        },
        // SAFETY: We are transmuting a type to itself
        3 => unsafe {
            transmute_generic::<Rotor<3, T, A>, Rotor<N, T, A>>(Rotor::<3, T, A>(
                Vector::<4, T, A>::NAN,
            ))
        },
        _ => unreachable!(),
    };

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
    pub fn from_rotation_arc(from: Vector<N, T, A>, to: Vector<N, T, A>) -> Self {
        debug_assert!(
            from.is_normalized() && to.is_normalized(),
            "vectors are not normalized: from_rotation_arc({from:?}, {to:?})"
        );

        specialize_23!(Rotor::<N, T, A>::from_rotation_arc_backend(from, to))
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
    pub fn from_rotation_arc_colinear(from: Vector<N, T, A>, to: Vector<N, T, A>) -> Self {
        debug_assert!(
            from.is_normalized() && to.is_normalized(),
            "vectors are not normalized: from_rotation_arc_colinear({from:?}, {to:?})"
        );

        specialize_23!(Rotor::<N, T, A>::from_rotation_arc_colinear_backend(
            from, to
        ))
    }

    /// Converts a rotation matrix to a rotor.
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
        specialize_23!(Rotor::<N, T, A>::from_matrix_backend(matrix))
    }

    /// Converts an affine transform with rotation to a rotor.
    ///
    /// `affine.matrix` must only contain a rotation. `affine.translation` is
    /// fully ignored.
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
    /// This function assumes the transform only contains rotation, and possibly
    /// translation, which is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `projective` is not a rotation transform.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_projective(projective: &Projective<N, T, A>) -> Self {
        specialize_23!(Rotor::<N, T, A>::from_projective_backend(projective))
    }

    /// Returns `true` if any element is NaN.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        specialize_23!(Rotor::<N, T, A>::is_nan_backend(self))
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        specialize_23!(Rotor::<N, T, A>::is_finite_backend(self))
    }

    /// Returns the inverse of a rotor.
    ///
    /// The only difference between this and [`conjugate`] is that this asserts
    /// `self.is_normalized()` when debug assertions are enabled. Use whichever
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
            "rotors are not normalized: {self:?}.angle_between({other:?})"
        );

        let half_angle = self.dot(other).abs().acos_approx();
        half_angle + half_angle
    }

    /// Computes the linear interpolation between `self` and `other` based on
    /// the value `t`.
    ///
    /// When `t` is 0, the result is `self`.  When `t` is 1, the result is
    /// `rhs`.
    ///
    /// Note that this does *not* interpolate the angle. For that, use [`slerp`].
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
    /// This function assumes both rotors are normalized.
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

        specialize_23!(Rotor::<N, T, A>::slerp_backend(self, other, t))
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
            "rotors are not normalized: {self:?}.rotate_towards({target:?}, {max_angle:?})"
        );

        let angle = self.angle_between(target);
        if angle <= T::as_from(1e-4) {
            target
        } else {
            let t = (max_angle / angle).clamp(T::NEG_ONE, T::ONE);
            specialize_23!(Rotor::<N, T, A>::slerp_backend(self, target, t))
        }
    }

    /// Returns the length/magnitude of `self`.
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        specialize_23!(Rotor::<N, T, A>::length_backend(self))
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
            result.is_finite() && result != Self::ZERO,
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
        specialize_23!(Rotor::<N, T, A>::try_normalize_backend(self))
    }

    /// Returns [`normalize`], or `fallback` if `self` is zero or if the result
    /// is non finite or zero.
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        specialize_23!(Rotor::<N, T, A>::normalize_or_backend(self, fallback))
    }

    /// Simultaneously computes [`normalize`] and [`length`].
    ///
    /// [`normalize`]: Self::normalize
    /// [`length`]: Self::length
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, T) {
        specialize_23!(Rotor::<N, T, A>::normalize_and_length_backend(self))
    }

    /// Returns whether the rotor has the length 1 or not.
    ///
    /// This uses a precision threshold of approximately `1e-4`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        specialize_23!(Rotor::<N, T, A>::is_normalized_backend(self))
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two rotors that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool {
        specialize_23!(Rotor::<N, T, A>::abs_diff_eq_backend(
            self,
            other,
            max_abs_diff
        ))
    }
}

impl<T, A: Alignment> Rotor<2, T, A>
where
    T: PrimitiveFloat,
{
    #[inline(always)]
    #[track_caller]
    fn from_rotation_arc_backend(from: Vector<2, T, A>, to: Vector<2, T, A>) -> Self {
        Self::new(from.wedge(to), T::ONE + from.dot(to)).normalize()
    }

    #[inline(always)]
    #[track_caller]
    fn from_rotation_arc_colinear_backend(from: Vector<2, T, A>, mut to: Vector<2, T, A>) -> Self {
        let mut dot = from.dot(to);
        if dot.is_sign_negative() {
            dot = -dot;
            to = -to;
        }

        Self::new(from.wedge(to), T::ONE + dot).normalize()
    }

    #[inline(always)]
    #[track_caller]
    fn from_matrix_backend(matrix: &Matrix<2, T, A>) -> Self {
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
                    .wedge(matrix.y_axis)
                    .abs_diff_eq(T::ONE, T::as_from(1e-4)),
            "not a rotation matrix: Rotor::from_matrix({matrix:?})"
        );

        Self::new(matrix.x_axis.y, matrix.x_axis.x + T::ONE).normalize()
    }

    #[inline(always)]
    #[track_caller]
    fn from_projective_backend(projective: &Projective<2, T, A>) -> Self {
        debug_assert!(
            projective
                .column(2)
                .abs_diff_eq(Vector::<3, T, A>::Z, T::as_from(1e-4))
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
                    .wedge(projective.y_axis.truncate())
                    .abs_diff_eq(T::ONE, T::as_from(1e-4)),
            "not a rotation: Rotor::from_projective({projective:?})"
        );

        Self::new(projective.x_axis.y, projective.x_axis.x + T::ONE).normalize()
    }

    #[inline(always)]
    fn is_nan_backend(self) -> bool {
        self.0.is_nan()
    }

    #[inline(always)]
    fn is_finite_backend(self) -> bool {
        self.0.is_finite()
    }

    #[inline(always)]
    #[track_caller]
    fn slerp_backend(self, other: Self, t: T) -> Self {
        let dot = self.dot(other);

        // A rotation can be represented by two rotors: `r` and `-r`. The slerp
        // path between `self` and `other` will be different from the path
        // between `-self` and `other`. One path will take the long way around
        // and one will take the short way. In order to correct for this, the
        // `dot` product between `self` and `other` should be positive. If the
        // `dot` product is negative, slerp between `self` and `-other`.
        let other = if dot.is_sign_negative() {
            -other
        } else {
            other
        };
        let dot = dot.abs();

        let half_angle = dot.acos_approx();
        let rotation = half_angle * self.0.wedge(other.0).signum() * t;

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(rotation);
        Self::new(self.xy * cos - self.s * sin, self.xy * sin + self.s * cos)
    }

    #[inline(always)]
    fn length_backend(self) -> T {
        self.0.length()
    }

    #[inline(always)]
    fn try_normalize_backend(self) -> Option<Self> {
        self.0.try_normalize().map(Self)
    }

    #[inline(always)]
    fn normalize_or_backend(self, fallback: Self) -> Self {
        Self(self.0.normalize_or(fallback.0))
    }

    #[inline(always)]
    fn normalize_and_length_backend(self) -> (Self, T) {
        let (normalize, length) = self.0.normalize_and_length();

        (Self(normalize), length)
    }

    #[inline(always)]
    fn is_normalized_backend(self) -> bool {
        self.0.is_normalized()
    }

    #[inline(always)]
    fn abs_diff_eq_backend(self, other: Self, max_abs_diff: T) -> bool {
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
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle * T::as_from(0.5));
        Self::new(sin, T::ZERO, T::ZERO, cos)
    }

    /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_xz(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle * T::as_from(0.5));
        Self::new(T::ZERO, sin, T::ZERO, cos)
    }

    /// Creates a rotor from an `angle` (in radians) rotating `+Y` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_yz(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle * T::as_from(0.5));
        Self::new(T::ZERO, T::ZERO, sin, cos)
    }

    /// Creates a rotor from a rotation `axis` and `angle` (in radians), using
    /// the right-hand rule.
    ///
    /// This assumes `axis` is normalized.
    ///
    /// If you are using this to initialize a static rotation, consider using
    /// [`from_rotation_arc`] instead. That function makes it clearer what
    /// direction the rotation happens in, whereas this function requires
    /// remembering the right-hand rule.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `axis` is not normalized.
    ///
    /// [`from_rotation_arc`]: Self::from_rotation_arc
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(axis: Vector<3, T, A>, angle: T) -> Self {
        debug_assert!(
            axis.is_normalized(),
            "axis is not normalized: Rotor::from_axis_angle({axis:?}, {angle:?})"
        );

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle * T::as_from(0.5));
        let xyz = axis * sin;
        Self::new(xyz.z, -xyz.y, xyz.x, cos)
    }

    /// Creates a rotor that rotates `scaled_axis.length()` radians around
    /// `scaled_axis.normalize()`, using the right-hand rule.
    ///
    /// If you are using this to initialize a static rotation, consider using
    /// [`from_rotation_arc`] instead. That function makes it clearer what
    /// direction the rotation happens in, whereas this function requires
    /// remembering the right-hand rule.
    ///
    /// [`from_rotation_arc`]: Self::from_rotation_arc
    #[inline]
    #[must_use]
    pub fn from_scaled_axis(scaled_axis: Vector<3, T, A>) -> Self {
        let (axis, angle) = scaled_axis.normalize_and_length();
        if angle == T::ZERO {
            Self::IDENTITY
        } else {
            let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle * T::as_from(0.5));
            let xyz = axis * sin;
            Self::new(xyz.z, -xyz.y, xyz.x, cos)
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
        let (si, ci) = PrimitiveFloatUtils::sin_cos(ti);
        let (sj, cj) = PrimitiveFloatUtils::sin_cos(tj);
        let (sh, ch) = PrimitiveFloatUtils::sin_cos(th);
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

    /// Creates a rotor from a facing direction and an up direction.
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

    /// Creates a rotor from a facing direction and an up direction.
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

    /// Creates a rotor from a camera position, a focal point and an up
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

    /// Creates a rotor from a camera position, a focal point and an up
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
    /// radians).
    ///
    /// This axis uses the right-hand rule.
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

        let bivector_rh = Vector::<3, T, A>::new(self.yz, -self.xz, self.xy);
        let bivector_length = bivector_rh.length();

        if bivector_length >= T::as_from(1e-8) {
            let axis = bivector_rh / bivector_length;
            let half_angle = PrimitiveFloatUtils::atan2(bivector_length, self.s);
            let angle = half_angle + half_angle;

            (axis, angle)
        } else {
            (Vector::<3, T, A>::X, T::ZERO)
        }
    }

    /// Converts the rotor `self` to a rotation axis scaled by an angle (in
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
            "rotor is not normalized: {self:?}.to_axis_angle()"
        );

        let bivector_rh = Vector::<3, T, A>::new(self.yz, -self.xz, self.xy);
        let bivector_length = bivector_rh.length();

        if bivector_length >= T::as_from(1e-8) {
            let axis = bivector_rh / bivector_length;
            let half_angle = PrimitiveFloatUtils::atan2(bivector_length, self.s);
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
    #[track_caller]
    fn from_rotation_arc_backend(from: Vector<3, T, A>, to: Vector<3, T, A>) -> Self {
        // Based on https://github.com/bitshifter/glam-rs

        let almost_one = T::ONE - T::as_from(2.0) * T::EPSILON;

        let dot = from.dot(to);
        if dot > almost_one {
            // 0° singularity: from ≈ to.
            Self::IDENTITY
        } else if dot < -almost_one {
            // 180° singularity: from ≈ -to.
            // Half a turn = 𝛕/2 = 180°.

            // Construct any rotation plane parallel to `from`
            let sign = from.z.signum();
            let tmp = T::NEG_ONE / (sign + from.z);
            let xy = -from.y;
            let xz = -sign - from.y * from.y * tmp;
            let yz = from.x * from.y * tmp;

            // sin(angle/2) = sin(𝛕/4) = 1
            // cos(angle/2) = cos(𝛕/4) = 0
            Self::new(xy, xz, yz, T::ZERO)
        } else {
            // This computes `xy, zx, yz`, so we flip `y` to make it `xz`
            let bivector = (from.zxy() * to - from * to.zxy()).yxz();
            let bivector = bivector * Vector::<3, T, A>::new(T::ONE, T::NEG_ONE, T::ONE);

            Self(bivector.extend(T::ONE + dot).normalize())
        }
    }

    #[inline(always)]
    #[track_caller]
    fn from_rotation_arc_colinear_backend(from: Vector<3, T, A>, mut to: Vector<3, T, A>) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs

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
            // This computes `xy, zx, yz`, so we flip `y` to make it `xz`
            let bivector = (from.zxy() * to - from * to.zxy()).yxz();
            let bivector = bivector * Vector::<3, T, A>::new(T::ONE, T::NEG_ONE, T::ONE);

            Self(bivector.extend(T::ONE + dot).normalize())
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

        let [xx, xy, xz] = matrix.x_axis.to_array();
        let [yx, yy, yz] = matrix.y_axis.to_array();
        let [zx, zy, zz] = matrix.z_axis.to_array();

        if zz <= T::ZERO {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = yy - xx;
            let omm22 = T::ONE - zz;

            if dif10 <= T::ZERO {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = T::as_from(0.5) / PrimitiveFloatUtils::sqrt(four_xsq);

                Self::new(xz + zx, -xy - yx, four_xsq, yz - zy) * inv4x
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = T::as_from(0.5) / PrimitiveFloatUtils::sqrt(four_ysq);

                Self::new(yz + zy, -four_ysq, xy + yx, zx - xz) * inv4y
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = yy + xx;
            let opm22 = T::ONE + zz;

            if sum10 <= T::ZERO {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = T::as_from(0.5) / PrimitiveFloatUtils::sqrt(four_zsq);

                Self::new(four_zsq, -yz - zy, xz + zx, xy - yx) * inv4z
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = T::as_from(0.5) / PrimitiveFloatUtils::sqrt(four_wsq);

                Self::new(xy - yx, xz - zx, yz - zy, four_wsq) * inv4w
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
                .abs_diff_eq(Vector::<4, T, A>::W, T::as_from(1e-4))
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

        let [xx, xy, xz, _] = projective.x_axis.to_array();
        let [yx, yy, yz, _] = projective.y_axis.to_array();
        let [zx, zy, zz, _] = projective.z_axis.to_array();

        if zz <= T::ZERO {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = yy - xx;
            let omm22 = T::ONE - zz;

            if dif10 <= T::ZERO {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = T::as_from(0.5) / PrimitiveFloatUtils::sqrt(four_xsq);

                Self::new(xz + zx, -xy - yx, four_xsq, yz - zy) * inv4x
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = T::as_from(0.5) / PrimitiveFloatUtils::sqrt(four_ysq);

                Self::new(yz + zy, -four_ysq, xy + yx, zx - xz) * inv4y
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = yy + xx;
            let opm22 = T::ONE + zz;

            if sum10 <= T::ZERO {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = T::as_from(0.5) / PrimitiveFloatUtils::sqrt(four_zsq);

                Self::new(four_zsq, -yz - zy, xz + zx, xy - yx) * inv4z
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = T::as_from(0.5) / PrimitiveFloatUtils::sqrt(four_wsq);

                Self::new(xy - yx, xz - zx, yz - zy, four_wsq) * inv4w
            }
        }
    }

    #[inline(always)]
    fn is_nan_backend(self) -> bool {
        self.0.is_nan()
    }

    #[inline(always)]
    fn is_finite_backend(self) -> bool {
        self.0.is_finite()
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

            let self_factor = PrimitiveFloatUtils::sin((T::ONE - t) * half_angle);
            let other_factor = PrimitiveFloatUtils::sin(t * half_angle);

            (self * self_factor + other * other_factor).normalize()
        }
    }

    #[inline(always)]
    fn length_backend(self) -> T {
        self.0.length()
    }

    #[inline(always)]
    fn try_normalize_backend(self) -> Option<Self> {
        self.0.try_normalize().map(Self)
    }

    #[inline(always)]
    fn normalize_or_backend(self, fallback: Self) -> Self {
        Self(self.0.normalize_or(fallback.0))
    }

    #[inline(always)]
    fn normalize_and_length_backend(self) -> (Self, T) {
        let (normalize, length) = self.0.normalize_and_length();

        (Self(normalize), length)
    }

    #[inline(always)]
    fn is_normalized_backend(self) -> bool {
        self.0.is_normalized()
    }

    #[inline(always)]
    fn abs_diff_eq_backend(self, other: Self, max_abs_diff: T) -> bool {
        self.0.abs_diff_eq(other.0, max_abs_diff)
    }
}
