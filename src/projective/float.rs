use crate::{
    Alignment, EulerRot, Length, Matrix, PrimitiveFloat, Projective, Quaternion, Vector,
    length::TwoOrThree,
    utils::{specialize_23, transmute_generic},
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: PrimitiveFloat,
{
    /// A transform with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::NAN_INTERNAL_IMPL;

    /// The implementation of [`Self::NAN`].
    ///
    /// Because of type system limitations, this implementation looks crazy. Use
    /// a separate constant so that IDEs do not show the implementation.
    #[allow(
        clippy::init_numbered_fields,
        reason = "due to some sort of compiler bug, tuple initialization fails here"
    )]
    const NAN_INTERNAL_IMPL: Self = match N {
        // SAFETY: We are transmuting a type to itself
        2 => unsafe {
            transmute_generic::<Projective<2, T, A>, Projective<N, T, A>>(Projective::<2, T, A> {
                0: Matrix::<3, T, A>::NAN,
            })
        },
        // SAFETY: We are transmuting a type to itself
        3 => unsafe {
            transmute_generic::<Projective<3, T, A>, Projective<N, T, A>>(Projective::<3, T, A> {
                0: Matrix::<4, T, A>::NAN,
            })
        },
        _ => unreachable!(),
    };

    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Proj2, Vec3};
    /// #
    /// let normal = Proj2::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let nan = Proj2::from_rows(&[
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
        specialize_23!(Projective::<N, T, A>::is_nan_backend(self))
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Proj2, Vec3};
    /// #
    /// let finite = Proj2::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let infinite = Proj2::from_rows(&[
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
        specialize_23!(Projective::<N, T, A>::is_finite_backend(self))
    }

    /// Returns the inverse of `self`.
    ///
    /// If `self` is not invertable, the result is unspecified.
    ///
    /// This computes the inverse of the inner homogeneous matrix.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant of the homogeneous matrix is `0`.
    #[must_use]
    #[track_caller]
    pub fn inverse(&self) -> Self {
        specialize_23!(Projective::<N, T, A>::inverse_backend(self))
    }

    /// Returns the inverse of `self` or `None` if `self` is not invertable.
    ///
    /// This computes the inverse of the inner homogeneous matrix.
    #[must_use]
    pub fn try_inverse(&self) -> Option<Self> {
        specialize_23!(Projective::<N, T, A>::try_inverse_backend(self))
    }

    /// Returns the inverse of `self` or `fallback` if `self` is not invertable.
    ///
    /// This computes the inverse of the inner homogeneous matrix.
    #[must_use]
    pub fn inverse_or(&self, fallback: &Self) -> Self {
        specialize_23!(Projective::<N, T, A>::inverse_or_backend(self, fallback))
    }

    /// Returns the inverse of `self` or the zero transform if `self` is not
    /// invertable.
    ///
    /// This computes the inverse of the inner homogeneous matrix.
    #[must_use]
    pub fn inverse_or_zero(&self) -> Self {
        specialize_23!(Projective::<N, T, A>::inverse_or_zero_backend(self))
    }

    /// Transforms the given vector as a point.
    ///
    /// Equivalent to `(point, 1) * self` but is faster.
    ///
    /// This function assumes `self` contains an affine transformation, with no
    /// projections, meaning the last column must be `(0, 0, ..., 1)`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the last column of `self` is not `(0, 0, ..., 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_point(&self, point: Vector<N, T, A>) -> Vector<N, T, A> {
        specialize_23!(Projective::<N, T, A>::transform_point_backend(self, point))
    }

    /// Transforms the given vector without applying translation.
    ///
    /// Equivalent to `(vector, 0) * self` but is faster.
    ///
    /// This function assumes `self` contains an affine transformation, with no
    /// projections, meaning the last column must be `(0, 0, ..., 1)`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the last column of `self` is not `(0, 0, ..., 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_vector(&self, vector: Vector<N, T, A>) -> Vector<N, T, A> {
        specialize_23!(Projective::<N, T, A>::transform_vector_backend(
            self, vector
        ))
    }

    /// Transforms the given vector as a point, applying perspective divide.
    #[inline]
    #[must_use]
    pub fn project_point(&self, point: Vector<N, T, A>) -> Vector<N, T, A> {
        specialize_23!(Projective::<N, T, A>::project_point_backend(self, point))
    }

    /// Returns the absolute values of the elements of `self`.
    ///
    /// Equivalent to `(self.x_axis.abs(), self.y_axis.abs(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Proj2, Vec3};
    /// #
    /// let projective = Proj2::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, -1.0, 0.0),
    ///     Vec3::new(0.0, 0.0, -1.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     projective.abs(),
    ///     Proj2::from_rows(&[
    ///         Vec3::new(1.0, 0.0, 0.0),
    ///         Vec3::new(0.0, 1.0, 0.0),
    ///         Vec3::new(0.0, 0.0, 1.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        specialize_23!(Projective::<N, T, A>::abs_backend(self))
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two transforms that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool {
        specialize_23!(Projective::<N, T, A>::abs_diff_eq_backend(
            self,
            other,
            max_abs_diff
        ))
    }
}

impl<T, A: Alignment> Projective<2, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a 2D projective transform containing a rotation of `angle` (in
    /// radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos, sin, T::ZERO),
            Vector::<3, T, A>::new(-sin, cos, T::ZERO),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Creates a 2D projective transform containing a non-uniform `scale` and a
    /// rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos * scale.x, sin * scale.x, T::ZERO),
            Vector::<3, T, A>::new(-sin * scale.y, cos * scale.y, T::ZERO),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Creates a 2D projective transform containing a rotation of `angle` (in
    /// radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle_translation(angle: T, translation: Vector<2, T, A>) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos, sin, T::ZERO),
            Vector::<3, T, A>::new(-sin, cos, T::ZERO),
            Vector::<3, T, A>::new(translation.x, translation.y, T::ONE),
        ])
    }

    /// Creates a 2D projective transform containing a non-uniform `scale`, a
    /// rotation of `angle` (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_scale_angle_translation(
        scale: Vector<2, T, A>,
        angle: T,
        translation: Vector<2, T, A>,
    ) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos * scale.x, sin * scale.x, T::ZERO),
            Vector::<3, T, A>::new(-sin * scale.y, cos * scale.y, T::ZERO),
            Vector::<3, T, A>::new(translation.x, translation.y, T::ONE),
        ])
    }

    /// Returns the `scale` and `angle` of `self`.
    ///
    /// This function assumes `self` contains an affine transformation with no
    /// shearing.
    ///
    /// `self` can contain translation, which is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the 2D determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle(&self) -> (Vector<2, T, A>, T) {
        Matrix::<2, T, A>::from_projective(self).to_scale_angle()
    }

    /// Returns the `scale`, `angle` and `translation` of `self`.
    ///
    /// This function assumes `self` contains an affine transformation with no
    /// shearing.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the 2D determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle_translation(&self) -> (Vector<2, T, A>, T, Vector<2, T, A>) {
        let (scale, angle) = self.to_scale_angle();
        (scale, angle, self.translation())
    }

    #[inline(always)]
    fn is_nan_backend(&self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan() || self.z_axis.is_nan()
    }

    #[inline(always)]
    fn is_finite_backend(&self) -> bool {
        self.x_axis.is_finite() && self.y_axis.is_finite() && self.z_axis.is_finite()
    }

    #[inline(always)]
    fn inverse_backend(&self) -> Self {
        Self(self.0.inverse())
    }

    #[inline(always)]
    fn try_inverse_backend(&self) -> Option<Self> {
        self.0.try_inverse().map(Self)
    }

    #[inline(always)]
    fn inverse_or_backend(&self, fallback: &Self) -> Self {
        Self(self.0.inverse_or(&fallback.0))
    }

    #[inline(always)]
    fn inverse_or_zero_backend(&self) -> Self {
        Self(self.0.inverse_or_zero())
    }

    #[inline(always)]
    #[track_caller]
    fn transform_point_backend(&self, point: Vector<2, T, A>) -> Vector<2, T, A> {
        debug_assert!(
            self.column(2)
                .abs_diff_eq(Vector::<3, T, A>::Z, T::as_from(1e-6)),
            "matrix contains projection (which transform_point does not handle)"
        );

        self.x_axis.xy() * point.x + self.y_axis.xy() * point.y + self.z_axis.xy()
    }

    #[inline(always)]
    #[track_caller]
    fn transform_vector_backend(&self, vector: Vector<2, T, A>) -> Vector<2, T, A> {
        debug_assert!(
            self.column(2)
                .abs_diff_eq(Vector::<3, T, A>::Z, T::as_from(1e-6)),
            "matrix contains projection (which transform_vector does not handle)"
        );

        self.x_axis.xy() * vector.x + self.y_axis.xy() * vector.y
    }

    #[inline(always)]
    fn project_point_backend(&self, point: Vector<2, T, A>) -> Vector<2, T, A> {
        let result = self.x_axis * point.x + self.y_axis * point.y + self.z_axis;

        (result / result.z).truncate()
    }

    #[inline(always)]
    fn abs_backend(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline(always)]
    fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: T) -> bool {
        self.0.abs_diff_eq(&other.0, max_abs_diff)
    }
}

impl<T, A: Alignment> Projective<3, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a 3D projective transform containing a rotation from `angle` (in
    /// radians) around the x axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_rows(&[
            Vector::<4, T, A>::X,
            Vector::<4, T, A>::new(T::ZERO, cos, sin, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, -sin, cos, T::ZERO),
            Vector::<4, T, A>::W,
        ])
    }

    /// Creates a 3D projective transform containing a rotation from `angle` (in
    /// radians) around the y axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_rows(&[
            Vector::<4, T, A>::new(cos, T::ZERO, -sin, T::ZERO),
            Vector::<4, T, A>::Y,
            Vector::<4, T, A>::new(sin, T::ZERO, cos, T::ZERO),
            Vector::<4, T, A>::W,
        ])
    }

    /// Creates a 3D projective transform containing a rotation from `angle` (in
    /// radians) around the z axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_rows(&[
            Vector::<4, T, A>::new(cos, sin, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(-sin, cos, T::ZERO, T::ZERO),
            Vector::<4, T, A>::Z,
            Vector::<4, T, A>::W,
        ])
    }

    #[inline(always)]
    fn quat_to_axes(quat: Quaternion<T, A>) -> [Vector<4, T, A>; 3] {
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

    /// Creates a 3D projective transform containing a rotation from a
    /// quaternion.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the quaternion is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_quat(quat: Quaternion<T, A>) -> Self {
        debug_assert!(
            quat.is_normalized(),
            "quat is not normalized: Matrix::from_quat({quat:?})"
        );

        let [x_axis, y_axis, z_axis] = Self::quat_to_axes(quat);
        Self::from_rows(&[x_axis, y_axis, z_axis, Vector::W])
    }

    /// Creates a 3D projective transform containing a rotation from a
    /// rotation `axis` and `angle` (in radians).
    ///
    /// `axis` must be normalized. Otherwise the result is unspecified.
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

        let (sin, cos) = angle.sin_cos();
        let [xsin, ysin, zsin] = (axis * sin).to_array();
        let [x, y, z] = axis.to_array();
        let [x2, y2, z2] = (axis * axis).to_array();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;

        Self::from_rows(&[
            Vector::<4, T, A>::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin, T::ZERO),
            Vector::<4, T, A>::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin, T::ZERO),
            Vector::<4, T, A>::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos, T::ZERO),
            Vector::W,
        ])
    }

    /// Creates a 3D projective transform containing a rotation from an Euler
    /// rotation order/sequence and angles (in radians).
    #[inline]
    #[must_use]
    pub fn from_euler(order: EulerRot, a: T, b: T, c: T) -> Self {
        Self::from_matrix(&Matrix::<3, T, A>::from_euler(order, a, b, c))
    }

    /// Creates a 3D projective transform containing a non-uniform `scale` and a
    /// 3D `rotation`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_scale_rotation(scale: Vector<3, T, A>, rotation: Quaternion<T, A>) -> Self {
        debug_assert!(
            rotation.is_normalized(),
            "rotation is not normalized: from_scale_rotation({scale:?}, {rotation:?})"
        );

        let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
        Self::from_rows(&[
            rotation_x * scale.x,
            rotation_y * scale.y,
            rotation_z * scale.z,
            Vector::W,
        ])
    }

    /// Creates a 3D projective transform containing a 3D `rotation` and
    /// `translation`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_translation(
        rotation: Quaternion<T, A>,
        translation: Vector<3, T, A>,
    ) -> Self {
        debug_assert!(
            rotation.is_normalized(),
            "rotation is not normalized: from_rotation_translation({rotation:?}, {translation:?})"
        );

        let [x_axis, y_axis, z_axis] = Self::quat_to_axes(rotation);
        Self::from_rows(&[
            x_axis,
            y_axis,
            z_axis,
            Vector::<4, T, A>::new(translation.x, translation.y, translation.z, T::ONE),
        ])
    }

    /// Creates a 3D projective transform containing a non-uniform `scale`, a 3D
    /// `rotation` and `translation`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_scale_rotation_translation(
        scale: Vector<3, T, A>,
        rotation: Quaternion<T, A>,
        translation: Vector<3, T, A>,
    ) -> Self {
        debug_assert!(
            rotation.is_normalized(),
            "rotation is not normalized: from_scale_rotation_translation({scale:?}, {rotation:?}, {translation:?})"
        );

        let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
        Self::from_rows(&[
            rotation_x * scale.x,
            rotation_y * scale.y,
            rotation_z * scale.z,
            Vector::<4, T, A>::new(translation.x, translation.y, translation.z, T::ONE),
        ])
    }

    /// Creates a left-handed view transform from a camera position, a facing
    /// direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
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
    pub fn look_to_lh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(
            dir.is_normalized() && up.is_normalized(),
            "directions are not normalized: look_to_lh({eye:?}, {dir:?}, {up:?})"
        );

        let forward = dir;

        let right = up.cross(forward);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "dir and up are parallel: look_to_lh({eye:?}, {dir:?}, {up:?})"
        );

        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<4, T, A>::new(right.x, up.x, forward.x, T::ZERO),
            Vector::<4, T, A>::new(right.y, up.y, forward.y, T::ZERO),
            Vector::<4, T, A>::new(right.z, up.z, forward.z, T::ZERO),
            Vector::<4, T, A>::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward), T::ONE),
        ])
    }

    /// Creates a right-handed view transform from a camera position, a facing
    /// direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
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
    pub fn look_to_rh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(
            dir.is_normalized() && up.is_normalized(),
            "directions are not normalized: look_to_rh({eye:?}, {dir:?}, {up:?})"
        );

        let forward = dir;

        let right = forward.cross(up);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "dir and up are parallel: look_to_rh({eye:?}, {dir:?}, {up:?})"
        );

        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<4, T, A>::new(right.x, up.x, -forward.x, T::ZERO),
            Vector::<4, T, A>::new(right.y, up.y, -forward.y, T::ZERO),
            Vector::<4, T, A>::new(right.z, up.z, -forward.z, T::ZERO),
            Vector::<4, T, A>::new(-eye.dot(right), -eye.dot(up), eye.dot(forward), T::ONE),
        ])
    }

    /// Creates a left-handed view transform from a camera position, a focal point
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
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
        debug_assert!(
            up.is_normalized(),
            "up is not normalized: look_at_lh({eye:?}, {center:?}, {up:?})"
        );

        let forward = center - eye;
        let forward = forward / forward.length();
        debug_assert!(
            forward.is_finite() && forward != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_lh({eye:?}, {center:?}, {up:?})"
        );

        let right = up.cross(forward);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_lh({eye:?}, {center:?}, {up:?})"
        );

        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<4, T, A>::new(right.x, up.x, forward.x, T::ZERO),
            Vector::<4, T, A>::new(right.y, up.y, forward.y, T::ZERO),
            Vector::<4, T, A>::new(right.z, up.z, forward.z, T::ZERO),
            Vector::<4, T, A>::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward), T::ONE),
        ])
    }

    /// Creates a right-handed view transform from a camera position, a focal point
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
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
        debug_assert!(
            up.is_normalized(),
            "up is not normalized: look_at_rh({eye:?}, {center:?}, {up:?})"
        );

        let forward = center - eye;
        let forward = forward / forward.length();
        debug_assert!(
            forward.is_finite() && forward != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_rh({eye:?}, {center:?}, {up:?})"
        );

        let right = forward.cross(up);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_rh({eye:?}, {center:?}, {up:?})"
        );

        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<4, T, A>::new(right.x, up.x, -forward.x, T::ZERO),
            Vector::<4, T, A>::new(right.y, up.y, -forward.y, T::ZERO),
            Vector::<4, T, A>::new(right.z, up.z, -forward.z, T::ZERO),
            Vector::<4, T, A>::new(-eye.dot(right), -eye.dot(up), eye.dot(forward), T::ONE),
        ])
    }

    /// Creates a left-handed perspective projection with `0..1` depth range.
    ///
    /// Useful to map the standard left-handed coordinate system into what
    /// WebGPU/Metal/Direct3D expect.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_lh(vertical_fov: T, aspect_ratio: T, near_plane: T, far_plane: T) -> Self {
        debug_assert!(
            near_plane > T::ZERO && far_plane > near_plane,
            "near_plane < 0 or far_plane < near_plane"
        );

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let depth_scale = far_plane / (far_plane - near_plane);

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, depth_scale, T::ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, -depth_scale * near_plane, T::ZERO),
        ])
    }

    /// Creates a right-handed perspective projection with `0..1` depth range.
    ///
    /// Useful to map the standard right-handed coordinate system into what
    /// WebGPU/Metal/Direct3D expect.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_rh(vertical_fov: T, aspect_ratio: T, near_plane: T, far_plane: T) -> Self {
        debug_assert!(
            near_plane > T::ZERO && far_plane > near_plane,
            "near_plane < 0 or far_plane < near_plane"
        );

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let neg_depth_scale = far_plane / (near_plane - far_plane);

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, neg_depth_scale, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, neg_depth_scale * near_plane, T::ZERO),
        ])
    }

    /// Creates a right-handed perspective projection with `-1..1` depth range.
    ///
    /// Equivalent to the OpenGL [`gluPerspective`] function.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`gluPerspective`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_rh_gl(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
        far_plane: T,
    ) -> Self {
        debug_assert!(
            near_plane > T::ZERO && far_plane > near_plane,
            "near_plane < 0 or far_plane < near_plane"
        );

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;
        let depth_recip = (near_plane - far_plane).recip();

        Self::from_rows(&[
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

    /// Creates an infinite left-handed perspective projection with `0..1` depth
    /// range.
    ///
    /// Equivalent to `perspective_lh`, but with an infinite value for
    /// `far_plane`. The result is that points near `near_plane` have depth `0`,
    /// and as they move towards infinity the depth approaches `1`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_lh(vertical_fov: T, aspect_ratio: T, near_plane: T) -> Self {
        debug_assert!(near_plane > T::ZERO, "near_plane < 0");

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ONE, T::ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, -near_plane, T::ZERO),
        ])
    }

    /// Creates an infinite right-handed perspective projection with `0..1`
    /// depth range.
    ///
    /// Equivalent to `perspective_rh`, but with an infinite value for
    /// `far_plane`. The result is that points near `near_plane` have depth `0`,
    /// and as they move towards infinity the depth approaches `1`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_rh(vertical_fov: T, aspect_ratio: T, near_plane: T) -> Self {
        debug_assert!(near_plane > T::ZERO, "near_plane < 0");

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::NEG_ONE, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, -near_plane, T::ZERO),
        ])
    }

    /// Creates an infinite left-handed perspective projection with reversed
    /// `0..1` depth range.
    ///
    /// Equivalent to `perspective_infinite_lh`, but maps points at `near_plane`
    /// to depth `1` and points at infinity to depth `0`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_reverse_lh(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
    ) -> Self {
        debug_assert!(near_plane > T::ZERO, "near_plane < 0");

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, near_plane, T::ZERO),
        ])
    }

    /// Creates an infinite right-handed perspective projection with reversed
    /// `0..1` depth range.
    ///
    /// Equivalent to `perspective_infinite_rh`, but maps points at `near_plane`
    /// to depth `1` and points at infinity to depth `0`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perspective_infinite_reverse_rh(
        vertical_fov: T,
        aspect_ratio: T,
        near_plane: T,
    ) -> Self {
        debug_assert!(near_plane > T::ZERO, "near_plane < 0");

        let (sin, cos) = (vertical_fov * T::as_from(0.5)).sin_cos();
        let height_recip = cos / sin;
        let width_recip = height_recip / aspect_ratio;

        Self::from_rows(&[
            Vector::<4, T, A>::new(width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, near_plane, T::ZERO),
        ])
    }

    /// Creates a left-handed perspective projection with `0..1` depth range.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn frustum_lh(left: T, right: T, bottom: T, top: T, near_plane: T, far_plane: T) -> Self {
        debug_assert!(
            near_plane > T::ZERO && far_plane > near_plane,
            "near_plane < 0 or far_plane < near_plane"
        );

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = far_plane * depth_recip;
        let d = -near_plane * c;

        Self::from_rows(&[
            Vector::<4, T, A>::new(two_near_plane * width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, two_near_plane * height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(a, b, c, T::ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, d, T::ZERO),
        ])
    }

    /// Creates a right-handed perspective projection with `0..1` depth range.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn frustum_rh(left: T, right: T, bottom: T, top: T, near_plane: T, far_plane: T) -> Self {
        debug_assert!(
            near_plane > T::ZERO && far_plane > near_plane,
            "near_plane < 0 or far_plane < near_plane"
        );

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = -far_plane * depth_recip;
        let d = near_plane * c;

        Self::from_rows(&[
            Vector::<4, T, A>::new(two_near_plane * width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, two_near_plane * height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(a, b, c, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, d, T::ZERO),
        ])
    }

    /// Creates a right-handed perspective projection with `-1..1` depth range.
    ///
    /// Equivalent to the OpenGL [`glFrustum`] function.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `near_plane` is less than or equal to `0`, or if `far_plane`
    /// is less than or equal to `near_plane`.
    ///
    /// [`glFrustum`]: https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml
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
        debug_assert!(
            near_plane > T::ZERO && far_plane > near_plane,
            "near_plane < 0 or far_plane < near_plane"
        );

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far_plane - near_plane).recip();
        let two_near_plane = T::as_from(2.0) * near_plane;
        let a = (right + left) * width_recip;
        let b = (top + bottom) * height_recip;
        let c = -(far_plane + near_plane) * depth_recip;
        let d = -two_near_plane * far_plane * depth_recip;

        Self::from_rows(&[
            Vector::<4, T, A>::new(two_near_plane * width_recip, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, two_near_plane * height_recip, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(a, b, c, T::NEG_ONE),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, d, T::ZERO),
        ])
    }

    /// Creates a left-handed orthographic projection with `0..1` depth range.
    ///
    /// Useful to map a left-handed coordinate system into what
    /// WebGPU/Metal/Direct3D expect.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `far` is less than or equal to `near`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_lh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        debug_assert!(far > near, "far < near");

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let depth_recip = (far - near).recip();

        Self::from_rows(&[
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

    /// Creates a right-handed orthographic projection with `0..1` depth range.
    ///
    /// Useful to map a right-handed coordinate system into what
    /// WebGPU/Metal/Direct3D expect.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `far` is less than or equal to `near`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_rh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        debug_assert!(far > near, "far < near");

        let width_recip = (right - left).recip();
        let height_recip = (top - bottom).recip();
        let neg_depth_recip = (near - far).recip();

        Self::from_rows(&[
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

    /// Creates a right-handed orthographic projection with `-1..1` depth range.
    ///
    /// Equivalent to the OpenGL [`glOrtho`] function.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `far` is less than or equal to `near`.
    ///
    /// [`glOrtho`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn orthographic_rh_gl(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        debug_assert!(far > near, "far < near");

        let scale_x = T::as_from(2.0) / (right - left);
        let scale_y = T::as_from(2.0) / (top - bottom);
        let scale_z = T::as_from(2.0) / (near - far);
        let translation_x = -(right + left) / (right - left);
        let translation_y = -(top + bottom) / (top - bottom);
        let translation_z = -(far + near) / (far - near);

        Self::from_rows(&[
            Vector::<4, T, A>::new(scale_x, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, scale_y, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, scale_z, T::ZERO),
            Vector::<4, T, A>::new(translation_x, translation_y, translation_z, T::ONE),
        ])
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// The upper-left 3x3 matrix of `self` must not contain any non-rotation
    /// transformations. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(&self, order: EulerRot) -> (T, T, T) {
        Matrix::<3, T, A>::from_projective(self).to_euler(order)
    }

    /// Returns the `scale` and `rotation` of `self`.
    ///
    /// This function assumes `self` contains an affine transform with no shear
    /// or projections. `self` can contain translation which is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation(&self) -> (Vector<3, T, A>, Quaternion<T, A>) {
        Matrix::<3, T, A>::from_projective(self).to_scale_rotation()
    }

    /// Returns the `scale`, `rotation` and `translation` of `self`.
    ///
    /// This function assumes `self` contains an affine transform with no shear
    /// or projections.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation_translation(
        &self,
    ) -> (Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>) {
        let (scale, rotation) = self.to_scale_rotation();
        (scale, rotation, self.translation())
    }

    #[inline(always)]
    fn is_nan_backend(&self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan() || self.z_axis.is_nan() || self.w_axis.is_nan()
    }

    #[inline(always)]
    fn is_finite_backend(&self) -> bool {
        self.x_axis.is_finite()
            && self.y_axis.is_finite()
            && self.z_axis.is_finite()
            && self.w_axis.is_finite()
    }

    #[inline(always)]
    fn inverse_backend(&self) -> Self {
        Self(self.0.inverse())
    }

    #[inline(always)]
    fn try_inverse_backend(&self) -> Option<Self> {
        self.0.try_inverse().map(Self)
    }

    #[inline(always)]
    fn inverse_or_backend(&self, fallback: &Self) -> Self {
        Self(self.0.inverse_or(&fallback.0))
    }

    #[inline(always)]
    fn inverse_or_zero_backend(&self) -> Self {
        Self(self.0.inverse_or_zero())
    }

    #[inline(always)]
    #[track_caller]
    fn transform_point_backend(&self, point: Vector<3, T, A>) -> Vector<3, T, A> {
        debug_assert!(
            self.column(3)
                .abs_diff_eq(Vector::<4, T, A>::W, T::as_from(1e-6)),
            "matrix contains projection (which transform_point does not handle)"
        );

        self.x_axis.xyz() * point.x
            + self.y_axis.xyz() * point.y
            + self.z_axis.xyz() * point.z
            + self.w_axis.xyz()
    }

    #[inline(always)]
    #[track_caller]
    fn transform_vector_backend(&self, vector: Vector<3, T, A>) -> Vector<3, T, A> {
        debug_assert!(
            self.column(3)
                .abs_diff_eq(Vector::<4, T, A>::W, T::as_from(1e-6)),
            "matrix contains projection (which transform_vector does not handle)"
        );

        self.x_axis.xyz() * vector.x + self.y_axis.xyz() * vector.y + self.z_axis.xyz() * vector.z
    }

    #[inline(always)]
    fn project_point_backend(&self, point: Vector<3, T, A>) -> Vector<3, T, A> {
        let result =
            self.x_axis * point.x + self.y_axis * point.y + self.z_axis * point.z + self.w_axis;

        (result / result.w).truncate()
    }

    #[inline(always)]
    fn abs_backend(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline(always)]
    fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: T) -> bool {
        self.0.abs_diff_eq(&other.0, max_abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Affine, EulerRot, Matrix, Proj2A, Proj3A, Projective, Quaternion, Vec2A, Vec3A, Vec4A,
        Vector,
        test_utils::{
            assert_debug_panic, assert_panic_test_eq, assert_test_eq, for_types, random_iter,
        },
    };

    #[test]
    fn test_constants() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Projective::<2, T, A>::NAN,
                Projective(Matrix::<3, T, A>::NAN)
            );
            assert_test_eq!(
                Projective::<3, T, A>::NAN,
                Projective(Matrix::<4, T, A>::NAN)
            );
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let nan = Vector::<3, T, A>::NAN;
            assert!(!Projective::<2, T, A>::from_rows(&[one; 3]).is_nan());
            assert!(Projective::<2, T, A>::from_rows(&[nan, one, one]).is_nan());
            assert!(Projective::<2, T, A>::from_rows(&[one, nan, one]).is_nan());
            assert!(Projective::<2, T, A>::from_rows(&[one, one, nan]).is_nan());
            assert!(Projective::<2, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::<4, T, A>::NAN;
            assert!(!Projective::<3, T, A>::from_rows(&[one; 4]).is_nan());
            assert!(Projective::<3, T, A>::from_rows(&[nan, one, one, one]).is_nan());
            assert!(Projective::<3, T, A>::from_rows(&[one, nan, one, one]).is_nan());
            assert!(Projective::<3, T, A>::from_rows(&[one, one, nan, one]).is_nan());
            assert!(Projective::<3, T, A>::from_rows(&[one, one, one, nan]).is_nan());
            assert!(Projective::<3, T, A>::NAN.is_nan());
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let inf = Vector::<3, T, A>::INFINITY;
            assert!(Projective::<2, T, A>::from_rows(&[one, one, one]).is_finite());
            assert!(!Projective::<2, T, A>::from_rows(&[inf, one, one]).is_finite());
            assert!(!Projective::<2, T, A>::from_rows(&[one, inf, one]).is_finite());
            assert!(!Projective::<2, T, A>::from_rows(&[one, one, inf]).is_finite());
            assert!(!Projective::<2, T, A>::from_rows(&[inf, inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::<4, T, A>::INFINITY;
            assert!(Projective::<3, T, A>::from_rows(&[one, one, one, one]).is_finite());
            assert!(!Projective::<3, T, A>::from_rows(&[inf, one, one, one]).is_finite());
            assert!(!Projective::<3, T, A>::from_rows(&[one, inf, one, one]).is_finite());
            assert!(!Projective::<3, T, A>::from_rows(&[one, one, inf, one]).is_finite());
            assert!(!Projective::<3, T, A>::from_rows(&[one, one, one, inf]).is_finite());
            assert!(!Projective::<3, T, A>::from_rows(&[inf, inf, inf, inf]).is_finite());
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|T: PrimitiveFloat, A| {
            for projective in random_iter::<Projective<2, T, A>>() {
                assert_panic_test_eq!(projective.inverse(), Projective(projective.0.inverse()));
            }
            for projective in random_iter::<Projective<3, T, A>>() {
                assert_panic_test_eq!(projective.inverse(), Projective(projective.0.inverse()));
            }
        });
    }

    #[test]
    fn test_try_inverse() {
        for_types!(|T: PrimitiveFloat, A| {
            for projective in random_iter::<Projective<2, T, A>>() {
                assert_test_eq!(
                    projective.try_inverse(),
                    projective.0.try_inverse().map(Projective)
                );
            }
            for projective in random_iter::<Projective<3, T, A>>() {
                assert_test_eq!(
                    projective.try_inverse(),
                    projective.0.try_inverse().map(Projective)
                );
            }
        });
    }

    #[test]
    fn test_inverse_or() {
        for_types!(|T: PrimitiveFloat, A| {
            for [projective, fallback] in random_iter::<[Projective<2, T, A>; 2]>() {
                assert_panic_test_eq!(
                    projective.inverse_or(&fallback),
                    Projective(projective.0.inverse_or(&fallback.0))
                );
            }
            for [projective, fallback] in random_iter::<[Projective<3, T, A>; 2]>() {
                assert_panic_test_eq!(
                    projective.inverse_or(&fallback),
                    Projective(projective.0.inverse_or(&fallback.0))
                );
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_types!(|T: PrimitiveFloat, A| {
            for projective in random_iter::<Projective<2, T, A>>() {
                assert_panic_test_eq!(
                    projective.inverse_or_zero(),
                    Projective(projective.0.inverse_or_zero())
                );
            }
            for projective in random_iter::<Projective<3, T, A>>() {
                assert_panic_test_eq!(
                    projective.inverse_or_zero(),
                    Projective(projective.0.inverse_or_zero())
                );
            }
        });
    }

    #[test]
    fn test_transform_point() {
        assert_eq!(
            Proj2A::from_rows(&[
                Vec3A::new(2.0, 3.0, 0.0),
                Vec3A::new(4.0, 5.0, 0.0),
                Vec3A::new(6.0, 7.0, 1.0)
            ])
            .transform_point(Vec2A::new(-1.0, -2.0)),
            Vec2A::new(-4.0, -6.0)
        );
        assert_eq!(
            Proj3A::from_rows(&[
                Vec4A::new(2.0, 3.0, 4.0, 0.0),
                Vec4A::new(5.0, 6.0, 7.0, 0.0),
                Vec4A::new(8.0, 9.0, 10.0, 0.0),
                Vec4A::new(11.0, 12.0, 13.0, 1.0)
            ])
            .transform_point(Vec3A::new(-1.0, -2.0, -3.0)),
            Vec3A::new(-25.0, -30.0, -35.0)
        );

        assert_debug_panic!(
            Proj2A::from_rows(&[
                Vec3A::new(2.0, 3.0, 0.0),
                Vec3A::new(4.0, 5.0, 1.0),
                Vec3A::new(6.0, 7.0, 1.0)
            ])
            .transform_point(Vec2A::new(-1.0, -2.0))
        );
        assert_debug_panic!(
            Proj3A::from_rows(&[
                Vec4A::new(2.0, 3.0, 4.0, 0.0),
                Vec4A::new(5.0, 6.0, 7.0, 0.0),
                Vec4A::new(8.0, 9.0, 10.0, 1.0),
                Vec4A::new(11.0, 12.0, 13.0, 1.0)
            ])
            .transform_point(Vec3A::new(-1.0, -2.0, -3.0))
        );
    }

    #[test]
    fn test_transform_vector() {
        assert_eq!(
            Proj2A::from_rows(&[
                Vec3A::new(2.0, 3.0, 0.0),
                Vec3A::new(4.0, 5.0, 0.0),
                Vec3A::new(6.0, 7.0, 1.0)
            ])
            .transform_vector(Vec2A::new(-1.0, -2.0)),
            Vec2A::new(-10.0, -13.0)
        );
        assert_eq!(
            Proj3A::from_rows(&[
                Vec4A::new(2.0, 3.0, 4.0, 0.0),
                Vec4A::new(5.0, 6.0, 7.0, 0.0),
                Vec4A::new(8.0, 9.0, 10.0, 0.0),
                Vec4A::new(11.0, 12.0, 13.0, 1.0)
            ])
            .transform_vector(Vec3A::new(-1.0, -2.0, -3.0)),
            Vec3A::new(-36.0, -42.0, -48.0)
        );

        assert_debug_panic!(
            Proj2A::from_rows(&[
                Vec3A::new(2.0, 3.0, 0.0),
                Vec3A::new(4.0, 5.0, 1.0),
                Vec3A::new(6.0, 7.0, 1.0)
            ])
            .transform_vector(Vec2A::new(-1.0, -2.0))
        );
        assert_debug_panic!(
            Proj3A::from_rows(&[
                Vec4A::new(2.0, 3.0, 4.0, 0.0),
                Vec4A::new(5.0, 6.0, 7.0, 0.0),
                Vec4A::new(8.0, 9.0, 10.0, 1.0),
                Vec4A::new(11.0, 12.0, 13.0, 1.0)
            ])
            .transform_vector(Vec3A::new(-1.0, -2.0, -3.0))
        );
    }

    #[test]
    fn test_project_point() {
        for_types!(|T: PrimitiveFloat, A| {
            for (projective, point) in random_iter::<(Projective<2, T, A>, Vector<2, T, A>)>() {
                assert_test_eq!(
                    projective.project_point(point),
                    Vector::<2, T, A>::from_homogeneous(point.to_homogeneous() * projective)
                );
            }
            for (projective, point) in random_iter::<(Projective<3, T, A>, Vector<3, T, A>)>() {
                assert_test_eq!(
                    projective.project_point(point),
                    Vector::<3, T, A>::from_homogeneous(point.to_homogeneous() * projective)
                );
            }
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|T: PrimitiveFloat, A| {
            for projective in random_iter::<Projective<2, T, A>>() {
                assert_test_eq!(projective.abs(), Projective(projective.0.abs()));
            }
            for projective in random_iter::<Projective<3, T, A>>() {
                assert_test_eq!(projective.abs(), Projective(projective.0.abs()));
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|T: PrimitiveFloat, A| {
            let x_axis = Vector::<3, T, A>::new(0.0, 1.0, 2.0);
            let y_axis = Vector::<3, T, A>::new(3.0, 4.0, 5.0);
            let z_axis = Vector::<3, T, A>::new(6.0, 7.0, 8.0);
            assert!(
                Projective::<2, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Projective::<2, T, A>::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Projective::<2, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Projective::<2, T, A>::from_rows(&[x_axis + 0.5, y_axis - 0.1, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Projective::<2, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Projective::<2, T, A>::from_rows(&[x_axis + 0.1, y_axis - 0.5, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Projective::<2, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Projective::<2, T, A>::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.5]),
                    0.125
                )
            );

            let x_axis = Vector::<4, T, A>::new(0.0, 1.0, 2.0, 3.0);
            let y_axis = Vector::<4, T, A>::new(4.0, 5.0, 6.0, 7.0);
            let z_axis = Vector::<4, T, A>::new(8.0, 9.0, 10.0, 11.0);
            let w_axis = Vector::<4, T, A>::new(12.0, 13.0, 14.0, 15.0);
            assert!(
                Projective::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Projective::<3, T, A>::from_rows(&[
                        x_axis + 0.1,
                        y_axis - 0.1,
                        z_axis + 0.05,
                        w_axis - 0.05
                    ]),
                    0.125
                )
            );
            assert!(
                !Projective::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Projective::<3, T, A>::from_rows(&[
                        x_axis + 0.5,
                        y_axis - 0.1,
                        z_axis + 0.05,
                        w_axis - 0.05
                    ]),
                    0.125
                )
            );
            assert!(
                !Projective::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Projective::<3, T, A>::from_rows(&[
                        x_axis + 0.1,
                        y_axis - 0.5,
                        z_axis + 0.05,
                        w_axis - 0.05
                    ]),
                    0.125
                )
            );
            assert!(
                !Projective::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Projective::<3, T, A>::from_rows(&[
                        x_axis + 0.1,
                        y_axis - 0.1,
                        z_axis + 0.5,
                        w_axis - 0.05
                    ]),
                    0.125
                )
            );
            assert!(
                !Projective::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Projective::<3, T, A>::from_rows(&[
                        x_axis + 0.1,
                        y_axis - 0.1,
                        z_axis + 0.05,
                        w_axis - 0.5
                    ]),
                    0.125
                )
            );
        });
    }

    #[test]
    fn test_from_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                assert_test_eq!(
                    Projective::<2, T, A>::from_angle(angle).transform_point(vector),
                    vector.rotate(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                if !scale.is_finite() || !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Projective::<2, T, A>::from_scale_angle(scale, angle),
                    Projective::<2, T, A>::from_scale(scale)
                        * Projective::<2, T, A>::from_angle(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (angle, translation) in random_iter::<(T, Vector<2, T, A>)>() {
                if !angle.is_finite() || !translation.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Projective::<2, T, A>::from_angle_translation(angle, translation),
                    Projective::<2, T, A>::from_angle(angle)
                        * Projective::<2, T, A>::from_translation(translation),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, angle, translation) in
                random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
            {
                if !scale.is_finite() || !angle.is_finite() || !translation.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Projective::<2, T, A>::from_scale_angle_translation(scale, angle, translation),
                    Projective::<2, T, A>::from_scale(scale)
                        * Projective::<2, T, A>::from_angle(angle)
                        * Projective::<2, T, A>::from_translation(translation),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_debug_panic!(Projective::<2, T, A>::ZERO.to_scale_angle());
            assert_debug_panic!(
                Projective::<2, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(0.3, 0.4, 0.0),
                    Vector::<3, T, A>::new(0.4, 0.6, 0.0),
                    Vector::<3, T, A>::new(0.0, 0.0, 1.0)
                ])
                .to_scale_angle()
            );

            for (scale, angle, translation) in
                random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
            {
                let projective =
                    Projective::<2, T, A>::from_scale_angle_translation(scale, angle, translation);

                if scale.iter().any(|x| x > 1e10)
                    || !projective.as_homogeneous().determinant().is_finite()
                    || projective.as_homogeneous().determinant().abs() < 1e-8
                {
                    continue;
                }

                let (result_scale, result_angle) = projective.to_scale_angle();
                assert_test_eq!(
                    Projective::<2, T, A>::from_scale_angle_translation(
                        result_scale,
                        result_angle,
                        translation
                    ),
                    projective,
                    abs <= scale.max_element() * 1e-5 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for projective in random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
                .map(|(scale, angle, translation)| {
                    Projective::<2, T, A>::from_scale_angle_translation(scale, angle, translation)
                })
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    projective.to_scale_angle_translation(),
                    (
                        projective.to_scale_angle().0,
                        projective.to_scale_angle().1,
                        projective.translation()
                    )
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>() {
                assert_test_eq!(
                    Projective::<3, T, A>::from_rotation_x(angle),
                    Projective::<3, T, A>::from_matrix(&Matrix::<3, T, A>::from_rotation_x(angle))
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>() {
                assert_test_eq!(
                    Projective::<3, T, A>::from_rotation_y(angle),
                    Projective::<3, T, A>::from_matrix(&Matrix::<3, T, A>::from_rotation_y(angle))
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>() {
                assert_test_eq!(
                    Projective::<3, T, A>::from_rotation_z(angle),
                    Projective::<3, T, A>::from_matrix(&Matrix::<3, T, A>::from_rotation_z(angle))
                );
            }
        });
    }

    #[test]
    fn test_from_quat() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_panic_test_eq!(
                    Projective::<3, T, A>::from_quat(quat),
                    Projective::<3, T, A>::from_matrix(&Matrix::<3, T, A>::from_quat(quat))
                );
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (axis, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                assert_panic_test_eq!(
                    Projective::<3, T, A>::from_axis_angle(axis, angle),
                    Projective::<3, T, A>::from_matrix(&Matrix::<3, T, A>::from_axis_angle(
                        axis, angle
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for [a, b, c] in random_iter::<[T; 3]>() {
                    assert_test_eq!(
                        Projective::<3, T, A>::from_euler(order, a, b, c),
                        Projective::<3, T, A>::from_matrix(&Matrix::<3, T, A>::from_euler(
                            order, a, b, c
                        ))
                    );
                }
            }
        });
    }

    #[test]
    fn test_from_scale_rotation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation) in random_iter::<(Vector<3, T, A>, Quaternion<T, A>)>() {
                if scale.is_finite() && rotation.is_finite() {
                    assert_panic_test_eq!(
                        Projective::<3, T, A>::from_scale_rotation(scale, rotation),
                        Projective::<3, T, A>::from_matrix(
                            &Matrix::<3, T, A>::from_scale_rotation(scale, rotation)
                        ),
                        0.0 = -0.0
                    );
                }
            }
        });
    }

    #[test]
    fn test_from_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (rotation, translation) in random_iter::<(Quaternion<T, A>, Vector<3, T, A>)>() {
                assert_panic_test_eq!(
                    Projective::<3, T, A>::from_rotation_translation(rotation, translation),
                    Projective::<3, T, A>::from_affine(
                        &Affine::<3, T, A>::from_rotation_translation(rotation, translation)
                    ),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation, translation) in
                random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
            {
                if scale.is_finite() && rotation.is_finite() {
                    assert_panic_test_eq!(
                        Projective::<3, T, A>::from_scale_rotation_translation(
                            scale,
                            rotation,
                            translation
                        ),
                        Projective::<3, T, A>::from_affine(
                            &Affine::<3, T, A>::from_scale_rotation_translation(
                                scale,
                                rotation,
                                translation,
                            )
                        ),
                        0.0 = -0.0
                    );
                }
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, dir, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Projective::<3, T, A>::look_to_lh(eye, dir, up),
                    Projective::from_affine(&Affine::<3, T, A>::look_to_lh(eye, dir, up))
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, dir, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Projective::<3, T, A>::look_to_rh(eye, dir, up),
                    Projective::from_affine(&Affine::<3, T, A>::look_to_rh(eye, dir, up))
                );
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Projective::<3, T, A>::look_at_lh(eye, center, up),
                    Projective::from_affine(&Affine::<3, T, A>::look_at_lh(eye, center, up))
                );
            }
        });
    }

    #[test]
    fn test_look_at_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Projective::<3, T, A>::look_at_rh(eye, center, up),
                    Projective::from_affine(&Affine::<3, T, A>::look_at_rh(eye, center, up))
                );
            }
        });
    }

    #[test]
    fn test_perspective_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                assert_debug_panic!(Projective::<3, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Projective::<3, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));

                let projective = Projective::<3, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                );

                let half_size = Vector::<2, T, A>::new(
                    (vertical_fov / 2.0).tan() * aspect_ratio,
                    (vertical_fov / 2.0).tan(),
                );

                for point in random_iter::<Vector<2, T, A>>() {
                    let point = point.map(|x| if x.abs() < 1e7 { x } else { 0.0 });

                    for (z, projection_z) in [(near_plane, 0.0), (far_plane, 1.0)] {
                        let projection = point / z / half_size;

                        assert_test_eq!(
                            projective.project_point(point.extend(z)),
                            projection.extend(projection_z),
                            abs <= point.abs().max_element().max(1.0) * 1e-3,
                            0.0 = -0.0
                        );
                    }
                }
            }
        });
    }

    #[test]
    fn test_perspective_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                assert_debug_panic!(Projective::<3, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Projective::<3, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));

                assert_test_eq!(
                    Projective::<3, T, A>::perspective_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane,
                    ),
                    Projective::<3, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                        * Projective::<3, T, A>::perspective_lh(
                            vertical_fov,
                            aspect_ratio,
                            near_plane,
                            far_plane,
                        )
                );
            }
        });
    }

    #[test]
    fn test_perspective_rh_gl() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                assert_debug_panic!(Projective::<3, T, A>::perspective_rh_gl(
                    vertical_fov,
                    aspect_ratio,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Projective::<3, T, A>::perspective_rh_gl(
                    vertical_fov,
                    aspect_ratio,
                    6.0,
                    4.0
                ));

                let expected =
                    Projective::<3, T, A>::perspective_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane,
                    ) * Projective::<3, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, 2.0))
                        * Projective::<3, T, A>::from_translation(Vector::<3, T, A>::NEG_Z);
                assert_test_eq!(
                    Projective::<3, T, A>::perspective_rh_gl(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane,
                    ),
                    expected,
                    abs <= expected.abs() * 1e-4
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5),
                ((120.0 as T).to_radians(), 20.0, 1e-3),
            ] {
                assert_debug_panic!(Projective::<3, T, A>::perspective_infinite_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));

                let projective = Projective::<3, T, A>::perspective_infinite_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                );

                let half_size = Vector::<2, T, A>::new(
                    (vertical_fov / 2.0).tan() * aspect_ratio,
                    (vertical_fov / 2.0).tan(),
                );

                for point in random_iter::<Vector<2, T, A>>() {
                    let point = point.map(|x| if x.abs() < 1e7 { x } else { 0.0 });

                    for (z, projection_z) in [(near_plane, 0.0), (1000.0, 1.0 - 1.0 / 1000.0)] {
                        let projection = point / z / half_size;

                        assert_test_eq!(
                            projective.project_point(point.extend(z)),
                            projection.extend(projection_z),
                            abs <= point.abs().max_element().max(1.0) * 1e-3,
                            0.0 = -0.0
                        );
                    }
                }
            }
        });
    }

    #[test]
    fn test_perspective_infinite_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5),
                ((120.0 as T).to_radians(), 20.0, 1e-3),
            ] {
                assert_debug_panic!(Projective::<3, T, A>::perspective_infinite_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));

                assert_test_eq!(
                    Projective::<3, T, A>::perspective_infinite_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Projective::<3, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                        * Projective::<3, T, A>::perspective_infinite_lh(
                            vertical_fov,
                            aspect_ratio,
                            near_plane
                        )
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5),
                ((120.0 as T).to_radians(), 20.0, 1e-3),
            ] {
                assert_debug_panic!(Projective::<3, T, A>::perspective_infinite_reverse_lh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));

                assert_test_eq!(
                    Projective::<3, T, A>::perspective_infinite_reverse_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Projective::<3, T, A>::perspective_infinite_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ) * Projective::<3, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                        * Projective::<3, T, A>::from_translation(Vector::<3, T, A>::Z)
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5),
                ((120.0 as T).to_radians(), 20.0, 1e-3),
            ] {
                assert_debug_panic!(Projective::<3, T, A>::perspective_infinite_reverse_rh(
                    vertical_fov,
                    aspect_ratio,
                    -1.0
                ));

                assert_test_eq!(
                    Projective::<3, T, A>::perspective_infinite_reverse_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Projective::<3, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                        * Projective::<3, T, A>::perspective_infinite_reverse_lh(
                            vertical_fov,
                            aspect_ratio,
                            near_plane
                        )
                );
            }
        });
    }

    #[test]
    fn test_frustum_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                let half_height = (vertical_fov / 2.0).tan() * near_plane;
                let half_width = half_height * aspect_ratio;

                assert_debug_panic!(Projective::<3, T, A>::frustum_lh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Projective::<3, T, A>::frustum_lh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    6.0,
                    4.0
                ));

                let expected = Projective::<3, T, A>::perspective_lh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                );
                assert_test_eq!(
                    Projective::<3, T, A>::frustum_lh(
                        -half_width,
                        half_width,
                        -half_height,
                        half_height,
                        near_plane,
                        far_plane
                    ),
                    expected,
                    abs <= expected.abs() * 1e-4
                );
            }
        });
    }

    #[test]
    fn test_frustum_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vertical_fov, aspect_ratio, near_plane, far_plane) in [
                ((70.0 as T).to_radians(), 16.0 / 9.0, 0.33, 400.0),
                ((60.0 as T).to_radians(), 10.0 / 9.0, 0.5, 1e5),
                ((120.0 as T).to_radians(), 20.0, 1e-3, 1e6),
            ] {
                let half_height = (vertical_fov / 2.0).tan() * near_plane;
                let half_width = half_height * aspect_ratio;

                assert_debug_panic!(Projective::<3, T, A>::frustum_rh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    -1.0,
                    4.0
                ));
                assert_debug_panic!(Projective::<3, T, A>::frustum_rh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    6.0,
                    4.0
                ));

                let expected = Projective::<3, T, A>::perspective_rh(
                    vertical_fov,
                    aspect_ratio,
                    near_plane,
                    far_plane,
                );
                assert_test_eq!(
                    Projective::<3, T, A>::frustum_rh(
                        -half_width,
                        half_width,
                        -half_height,
                        half_height,
                        near_plane,
                        far_plane
                    ),
                    expected,
                    abs <= expected.abs() * 1e-4
                );
            }
        });
    }

    #[test]
    fn test_frustum_rh_gl() {
        for_types!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near_plane = 0.34;
            let far_plane = 420.0;

            assert_debug_panic!(Projective::<3, T, A>::frustum_rh_gl(
                left, right, bottom, top, -1.0, 4.0
            ));
            assert_debug_panic!(Projective::<3, T, A>::frustum_rh_gl(
                left, right, bottom, top, 6.0, 4.0
            ));

            let expected =
                Projective::<3, T, A>::frustum_rh(left, right, bottom, top, near_plane, far_plane)
                    * Projective::<3, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, 2.0))
                    * Projective::<3, T, A>::from_translation(Vector::<3, T, A>::NEG_Z);
            assert_test_eq!(
                Projective::<3, T, A>::frustum_rh_gl(
                    left, right, bottom, top, near_plane, far_plane
                ),
                expected,
                abs <= expected.abs() * 1e-4
            );
        });
    }

    #[test]
    fn test_orthographic_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            assert_debug_panic!(Projective::<3, T, A>::orthographic_lh(
                left, right, bottom, top, 6.0, 4.0
            ));

            let projective =
                Projective::<3, T, A>::orthographic_lh(left, right, bottom, top, near, far);

            for (x, projection_x) in [(left, -1.0), (right, 1.0), (left.midpoint(right), 0.0)] {
                for (y, projection_y) in [(bottom, -1.0), (top, 1.0), (bottom.midpoint(top), 0.0)] {
                    for (z, projection_z) in [(near, 0.0), (far, 1.0), (near.midpoint(far), 0.5)] {
                        let point = Vector::<3, T, A>::new(x, y, z);
                        let projection =
                            Vector::<3, T, A>::new(projection_x, projection_y, projection_z);

                        assert_test_eq!(projective.project_point(point), projection, abs <= 1e-5);
                    }
                }
            }
        });
    }

    #[test]
    fn test_orthographic_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            assert_debug_panic!(Projective::<3, T, A>::orthographic_rh(
                left, right, bottom, top, 6.0, 4.0
            ));

            assert_test_eq!(
                Projective::<3, T, A>::orthographic_rh(left, right, bottom, top, near, far),
                Projective::<3, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, -1.0))
                    * Projective::<3, T, A>::orthographic_lh(left, right, bottom, top, near, far)
            );
        });
    }

    #[test]
    fn test_orthographic_rh_gl() {
        for_types!(|T: PrimitiveFloat, A| {
            let left = -0.6;
            let right = 2.8;
            let bottom = -0.4;
            let top = 1.3;
            let near = 0.34;
            let far = 420.0;

            assert_debug_panic!(Projective::<3, T, A>::orthographic_rh_gl(
                left, right, bottom, top, 6.0, 4.0
            ));

            let expected =
                Projective::<3, T, A>::orthographic_rh(left, right, bottom, top, near, far)
                    * Projective::<3, T, A>::from_scale(Vector::<3, T, A>::new(1.0, 1.0, 2.0))
                    * Projective::<3, T, A>::from_translation(Vector::<3, T, A>::NEG_Z);
            assert_test_eq!(
                Projective::<3, T, A>::orthographic_rh_gl(left, right, bottom, top, near, far),
                expected,
                abs <= expected.abs() * 1e-4
            );
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for projective in random_iter::<Quaternion<T, A>>()
                    .map(|quat| {
                        let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();
                        Projective::<3, T, A>::from_quat(quat)
                    })
                    .chain(random_iter::<Projective<3, T, A>>().take(20))
                {
                    assert_panic_test_eq!(
                        projective.to_euler(order),
                        Matrix::<3, T, A>::from_projective(&projective).to_euler(order)
                    );
                }
            }
        });
    }

    #[test]
    fn test_to_scale_rotation() {
        for_types!(|T: PrimitiveFloat, A| {
            for projective in random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
                .map(|(scale, rotation, translation)| {
                    let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();
                    Projective::<3, T, A>::from_scale_rotation_translation(
                        scale,
                        rotation,
                        translation,
                    )
                })
                .chain(random_iter::<Projective<3, T, A>>().take(20))
            {
                assert_panic_test_eq!(
                    projective.to_scale_rotation(),
                    Matrix::<3, T, A>::from_projective(&projective).to_scale_rotation()
                );
            }
        });
    }

    #[test]
    fn test_to_scale_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for projective in random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
                .map(|(scale, rotation, translation)| {
                    let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();
                    Projective::<3, T, A>::from_scale_rotation_translation(
                        scale,
                        rotation,
                        translation,
                    )
                })
                .chain(random_iter::<Projective<3, T, A>>().take(20))
            {
                assert_panic_test_eq!(
                    projective.to_scale_rotation_translation(),
                    Affine::<3, T, A>::from_projective(&projective).to_scale_rotation_translation()
                );
            }
        });
    }
}
