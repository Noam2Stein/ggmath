use crate::{
    Alignment, EulerRot, FloatExt, Length, Matrix, PrimitiveFloat, Projective, Quaternion,
    SupportedLength, Vector,
    length::TwoOrThree,
    utils::{PrimitiveFloatUtils, specialize, specialize_23},
};

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveFloat,
{
    /// A matrix with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::from_rows(&[Vector::<N, T, A>::NAN; N]);

    /// Converts a projective transform to a linear transformation matrix.
    ///
    /// This assumes `projective` does not contain projections. If there is
    /// translation, it is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the last column of `projective` is not approximately
    /// `(0, 0, ..., 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Proj2, Vec2, Vec3};
    /// #
    /// let projective = Proj2::from_rows(&[
    ///     Vec3::new(11.0, 12.0, 0.0),
    ///     Vec3::new(21.0, 22.0, 0.0),
    ///     Vec3::new(5.0, 8.0, 1.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     Mat2::<f32>::from_projective(&projective),
    ///     Mat2::from_rows(&[
    ///         Vec2::new(11.0, 12.0),
    ///         Vec2::new(21.0, 22.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    #[expect(private_bounds)]
    pub fn from_projective(projective: &Projective<N, T, A>) -> Self
    where
        Length<N>: TwoOrThree,
    {
        specialize_23!(Matrix::<N, T, A>::from_projective_backend(projective))
    }

    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let normal = Mat3::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let nan = Mat3::from_rows(&[
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
        specialize!(Matrix::<N, T, A>::is_nan_backend(self))
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let finite = Mat3::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let infinite = Mat3::from_rows(&[
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
        specialize!(Matrix::<N, T, A>::is_finite_backend(self))
    }

    /// Returns the inverse of `self`.
    ///
    /// If `self` is not invertable the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant is `0`.
    #[must_use]
    #[track_caller]
    pub fn inverse(&self) -> Self {
        #[cfg(debug_assertions)]
        {
            let (inverse, determinant) = self.inverse_and_determinant();

            if determinant == T::ZERO {
                panic!("matrix is not invertable: {self:?}.inverse()");
            }

            inverse
        }
        #[cfg(not(debug_assertions))]
        {
            self.inverse_and_determinant().0
        }
    }

    /// Returns the inverse of `self` or `None` if `self` is not invertable.
    #[must_use]
    pub fn try_inverse(&self) -> Option<Self> {
        let (inverse, determinant) = self.inverse_and_determinant();
        (determinant != T::ZERO).then_some(inverse)
    }

    /// Returns the inverse of `self` or `fallback` if `self` is not invertable.
    #[must_use]
    pub fn inverse_or(&self, fallback: &Self) -> Self {
        let (inverse, determinant) = self.inverse_and_determinant();
        if determinant == T::ZERO {
            *fallback
        } else {
            inverse
        }
    }

    /// Returns the inverse of `self` or the zero matrix if `self` is not
    /// invertable.
    #[must_use]
    pub fn inverse_or_zero(&self) -> Self {
        let (inverse, determinant) = self.inverse_and_determinant();
        if determinant == T::ZERO {
            Self::ZERO
        } else {
            inverse
        }
    }

    #[inline]
    fn inverse_and_determinant(&self) -> (Self, T) {
        specialize!(Matrix::<N, T, A>::inverse_and_determinant_backend(self))
    }

    /// Returns the element-wise reciprocal (inverse) of a matrix, `1 / self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let matrix = Mat3::from_rows(&[
    ///     Vec3::new(2.0, 4.0, 1.0),
    ///     Vec3::new(1.0, 2.0, 4.0),
    ///     Vec3::new(4.0, 1.0, 2.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     matrix.recip(),
    ///     Mat3::from_rows(&[
    ///         Vec3::new(0.5, 0.25, 1.0),
    ///         Vec3::new(1.0, 0.5, 0.25),
    ///         Vec3::new(0.25, 1.0, 0.5),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn recip(&self) -> Self {
        specialize!(Matrix::<N, T, A>::recip_backend(self))
    }

    /// Returns the absolute values of the elements of `self`.
    ///
    /// Equivalent to `(self.x_axis.abs(), self.y_axis.abs(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let matrix = Mat3::from_rows(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, -1.0, 0.0),
    ///     Vec3::new(0.0, 0.0, -1.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     matrix.abs(),
    ///     Mat3::from_rows(&[
    ///         Vec3::new(1.0, 0.0, 0.0),
    ///         Vec3::new(0.0, 1.0, 0.0),
    ///         Vec3::new(0.0, 0.0, 1.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        specialize!(Matrix::<N, T, A>::abs_backend(self))
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two matrices that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool {
        specialize!(Matrix::<N, T, A>::abs_diff_eq_backend(
            self,
            other,
            max_abs_diff
        ))
    }
}

impl<T, A: Alignment> Matrix<2, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a matrix containing a rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<2, T, A>::new(cos, sin),
            Vector::<2, T, A>::new(-sin, cos),
        ])
    }

    /// Creates a matrix containing the non-uniform `scale` and a rotation of
    /// `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<2, T, A>::new(cos * scale.x, sin * scale.x),
            Vector::<2, T, A>::new(-sin * scale.y, cos * scale.y),
        ])
    }

    /// Returns the `scale` and `angle` of `self`.
    ///
    /// `self` must not contain shearing. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle(&self) -> (Vector<2, T, A>, T) {
        let determinant = self.determinant();

        let scale = Vector::<2, T, A>::new(
            self.x_axis.length() * determinant.signum(),
            self.y_axis.length(),
        );

        debug_assert!(
            determinant != T::ZERO
                && (self.x_axis / scale.x)
                    .dot(self.y_axis / scale.y)
                    .abs_diff_eq(T::ZERO, T::as_from(1e-4)),
            "matrix contains shearing or determinant is zero: {self:?}.to_scale_angle()"
        );

        let angle = PrimitiveFloatUtils::atan2(-self.y_axis.x, self.y_axis.y);

        (scale, angle)
    }

    #[inline(always)]
    #[track_caller]
    fn from_projective_backend(projective: &Projective<2, T, A>) -> Self {
        debug_assert!(
            projective
                .column(2)
                .abs_diff_eq(Vector::<3, T, A>::Z, T::as_from(1e-4))
        );

        Self::from_rows(&[projective.x_axis.truncate(), projective.y_axis.truncate()])
    }

    #[inline(always)]
    fn is_nan_backend(&self) -> bool {
        self.0.is_nan()
    }

    #[inline(always)]
    fn is_finite_backend(&self) -> bool {
        self.0.is_finite()
    }

    #[inline(always)]
    fn inverse_and_determinant_backend(&self) -> (Self, T) {
        if const { align_of::<Self>() > align_of::<T>() } {
            // `[a*d, b*c, b*c, a*d]`
            let products = self.0 * self.0.wzyx();

            // `[a*d-b*c, b*c-a*d, b*c-a*d, a*d-b*c]`
            // `[det, -det, -det, det]`
            let determinant = products - products.yxxy();

            let determinant_recip = determinant.recip();
            let inverse = Self(self.0.wyzx() * determinant_recip);

            (inverse, determinant.x)
        } else {
            let determinant = self.determinant();

            let determinant_recip = determinant.recip();
            let inverse = Self::from_row_array(&[
                self.y_axis.y * determinant_recip,
                self.x_axis.y * -determinant_recip,
                self.y_axis.x * -determinant_recip,
                self.x_axis.x * determinant_recip,
            ]);

            (inverse, determinant)
        }
    }

    #[inline(always)]
    fn recip_backend(&self) -> Self {
        Self(self.0.recip())
    }

    #[inline(always)]
    fn abs_backend(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline(always)]
    fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: T) -> bool {
        self.0.abs_diff_eq(other.0, max_abs_diff)
    }
}

impl<T, A: Alignment> Matrix<3, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a 3D rotation matrix from `angle` (in radians) around the x
    /// axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::X,
            Vector::<3, T, A>::new(T::ZERO, cos, sin),
            Vector::<3, T, A>::new(T::ZERO, -sin, cos),
        ])
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the y
    /// axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos, T::ZERO, -sin),
            Vector::<3, T, A>::Y,
            Vector::<3, T, A>::new(sin, T::ZERO, cos),
        ])
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the z
    /// axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::from_rows(&[
            Vector::<3, T, A>::new(cos, sin, T::ZERO),
            Vector::<3, T, A>::new(-sin, cos, T::ZERO),
            Vector::<3, T, A>::Z,
        ])
    }

    #[inline(always)]
    fn quat_to_axes(quat: Quaternion<T, A>) -> [Vector<3, T, A>; 3] {
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
            Vector::<3, T, A>::new(T::ONE - (yy2 + zz2), xy2 + wz2, xz2 - wy2),
            Vector::<3, T, A>::new(xy2 - wz2, T::ONE - (xx2 + zz2), yz2 + wx2),
            Vector::<3, T, A>::new(xz2 + wy2, yz2 - wx2, T::ONE - (xx2 + yy2)),
        ]
    }

    /// Creates a 3D rotation matrix from a quaternion.
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
        Self::from_rows(&[x_axis, y_axis, z_axis])
    }

    /// Creates a 3D rotation matrix from a rotation `axis` and `angle` (in
    /// radians).
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

        let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
        let [xsin, ysin, zsin] = (axis * sin).to_array();
        let [x, y, z] = axis.to_array();
        let [x2, y2, z2] = (axis * axis).to_array();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;

        Self::from_rows(&[
            Vector::<3, T, A>::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            Vector::<3, T, A>::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            Vector::<3, T, A>::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
        ])
    }

    /// Creates a 3D rotation matrix from an Euler rotation order/sequence and
    /// angles (in radians).
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

        // Rotation direction is reverse from original paper.
        if order.parity_even {
            angles = -angles;
        }

        let (si, ci) = PrimitiveFloatUtils::sin_cos(angles.x);
        let (sj, cj) = PrimitiveFloatUtils::sin_cos(angles.y);
        let (sh, ch) = PrimitiveFloatUtils::sin_cos(angles.z);

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

    /// Creates a matrix containing a non-uniform `scale` and a 3D `rotation`.
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
        ])
    }

    /// Creates a left-handed view matrix from a facing direction and an up
    /// direction.
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
    pub fn look_to_lh(dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(
            dir.is_normalized() && up.is_normalized(),
            "directions are not normalized: look_to_lh({dir:?}, {up:?})"
        );

        let forward = dir;

        let right = up.cross(forward);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "dir and up are parallel: look_to_lh({dir:?}, {up:?})"
        );

        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, forward.x),
            Vector::<3, T, A>::new(right.y, up.y, forward.y),
            Vector::<3, T, A>::new(right.z, up.z, forward.z),
        ])
    }

    /// Creates a right-handed view matrix from a facing direction and an up
    /// direction.
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
    pub fn look_to_rh(dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(
            dir.is_normalized() && up.is_normalized(),
            "directions are not normalized: look_to_rh({dir:?}, {up:?})"
        );

        let forward = dir;

        let right = forward.cross(up);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "dir and up are parallel: look_to_lh({dir:?}, {up:?})"
        );

        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, -forward.x),
            Vector::<3, T, A>::new(right.y, up.y, -forward.y),
            Vector::<3, T, A>::new(right.z, up.z, -forward.z),
        ])
    }

    /// Creates a left-handed view matrix from a camera position, a focal point
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
            "center = eye: look_at_lh({eye:?}, {center:?}, {up:?})"
        );

        let right = up.cross(forward);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_lh({eye:?}, {center:?}, {up:?})"
        );

        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, forward.x),
            Vector::<3, T, A>::new(right.y, up.y, forward.y),
            Vector::<3, T, A>::new(right.z, up.z, forward.z),
        ])
    }

    /// Creates a right-handed view matrix from a camera position, a focal point
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
            "center = eye: look_at_rh({eye:?}, {center:?}, {up:?})"
        );

        let right = forward.cross(up);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_rh({eye:?}, {center:?}, {up:?})"
        );

        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, -forward.x),
            Vector::<3, T, A>::new(right.y, up.y, -forward.y),
            Vector::<3, T, A>::new(right.z, up.z, -forward.z),
        ])
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// `self` must not contain any non-rotation transformations. Otherwise the
    /// result is unspecified.
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
        // Ported from https://github.com/bitshifter/glam-rs.

        // Based on Ken Shoemake. 1994. Euler angle conversion. Graphics gems IV.
        // Academic Press Professional, Inc., USA, 222–229.

        debug_assert!(
            self.x_axis
                .length_squared()
                .abs_diff_eq(T::ONE, T::as_from(2e-2))
                && self
                    .y_axis
                    .length_squared()
                    .abs_diff_eq(T::ONE, T::as_from(2e-2))
                && self
                    .x_axis
                    .dot(self.y_axis)
                    .abs_diff_eq(T::ZERO, T::as_from(2e-2))
                && self
                    .x_axis
                    .cross(self.y_axis)
                    .abs_diff_eq(self.z_axis, T::as_from(2e-2)),
            "not a rotation matrix"
        );

        let order = order.properties();
        let (i, j, k) = order.axes_indices();

        let mut ea = Vector::<3, T, A>::ZERO;
        if order.initial_repeated {
            let sy = PrimitiveFloatUtils::sqrt(self[i][j] * self[i][j] + self[i][k] * self[i][k]);

            if sy > T::as_from(16.0) * T::EPSILON {
                ea.x = PrimitiveFloatUtils::atan2(self[i][j], self[i][k]);
                ea.y = PrimitiveFloatUtils::atan2(sy, self[i][i]);
                ea.z = PrimitiveFloatUtils::atan2(self[j][i], -self[k][i]);
            } else {
                ea.x = PrimitiveFloatUtils::atan2(-self[j][k], self[j][j]);
                ea.y = PrimitiveFloatUtils::atan2(sy, self[i][i]);
            }
        } else {
            let cy = PrimitiveFloatUtils::sqrt(self[i][i] * self[i][i] + self[j][i] * self[j][i]);

            if cy > T::as_from(16.0) * T::EPSILON {
                ea.x = PrimitiveFloatUtils::atan2(self[k][j], self[k][k]);
                ea.y = PrimitiveFloatUtils::atan2(-self[k][i], cy);
                ea.z = PrimitiveFloatUtils::atan2(self[j][i], self[i][i]);
            } else {
                ea.x = PrimitiveFloatUtils::atan2(-self[j][k], self[j][j]);
                ea.y = PrimitiveFloatUtils::atan2(-self[k][i], cy);
            }
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

    /// Returns the `scale` and `rotation` of `self`.
    ///
    /// `self` must not contain shearing. Otherwise the result is unspecified.
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
        let determinant = self.determinant();

        let scale = Vector::<3, T, A>::new(
            self.x_axis.length() * determinant.signum(),
            self.y_axis.length(),
            self.z_axis.length(),
        );

        let scale_recip = scale.recip();

        let rotation_matrix = Self::from_rows(&[
            self.x_axis * scale_recip.x,
            self.y_axis * scale_recip.y,
            self.z_axis * scale_recip.z,
        ]);

        debug_assert!(
            rotation_matrix
                .x_axis
                .dot(rotation_matrix.y_axis)
                .abs_diff_eq(T::ZERO, T::as_from(1e-4))
                && rotation_matrix
                    .x_axis
                    .dot(rotation_matrix.z_axis)
                    .abs_diff_eq(T::ZERO, T::as_from(1e-4))
                && rotation_matrix
                    .y_axis
                    .dot(rotation_matrix.z_axis)
                    .abs_diff_eq(T::ZERO, T::as_from(1e-4))
                && determinant != T::ZERO,
            "matrix contains shearing or determinant is zero"
        );

        let rotation = Quaternion::<T, A>::from_matrix(&rotation_matrix);

        (scale, rotation)
    }

    #[inline(always)]
    #[track_caller]
    fn from_projective_backend(projective: &Projective<3, T, A>) -> Self {
        debug_assert!(
            projective
                .column(3)
                .abs_diff_eq(Vector::<4, T, A>::W, T::as_from(1e-4))
        );

        Self::from_rows(&[
            projective.x_axis.truncate(),
            projective.y_axis.truncate(),
            projective.z_axis.truncate(),
        ])
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
    fn inverse_and_determinant_backend(&self) -> (Self, T) {
        let x_cross_y = self.x_axis.cross(self.y_axis);
        let determinant = x_cross_y.dot(self.z_axis);

        // Compute cross products but avoid the `.zxy()` at the end.
        let y_cross_z_yzx = self.y_axis.zxy() * self.z_axis - self.y_axis * self.z_axis.zxy();
        let z_cross_x_yzx = self.z_axis.zxy() * self.x_axis - self.z_axis * self.x_axis.zxy();

        // Simultaneously perform `{cross-product-yzx}.zxy()` and `{matrix}.transpose()`.
        let adjugate = if const { align_of::<Self>() > align_of::<T>() } {
            // SIMD shuffles usually support taking elements from two input
            // registers. These intermediate shuffles help the optimizer.

            let shuffle_1 = Vector::<4, T, A>::new(
                y_cross_z_yzx.x,
                y_cross_z_yzx.y,
                z_cross_x_yzx.x,
                z_cross_x_yzx.y,
            );
            let shuffle_2 = Vector::<4, T, A>::new(
                y_cross_z_yzx.z,
                y_cross_z_yzx.z,
                z_cross_x_yzx.z,
                z_cross_x_yzx.z,
            );

            Self::from_rows(&[
                Vector::<3, T, A>::new(shuffle_2.x, shuffle_2.z, x_cross_y.x),
                Vector::<3, T, A>::new(shuffle_1.x, shuffle_1.z, x_cross_y.y),
                Vector::<3, T, A>::new(shuffle_1.y, shuffle_1.w, x_cross_y.z),
            ])
        } else {
            Self::from_row_array(&[
                y_cross_z_yzx.z,
                z_cross_x_yzx.z,
                x_cross_y.x,
                y_cross_z_yzx.x,
                z_cross_x_yzx.x,
                x_cross_y.y,
                y_cross_z_yzx.y,
                z_cross_x_yzx.y,
                x_cross_y.z,
            ])
        };

        let inverse = adjugate * determinant.recip();

        (inverse, determinant)
    }

    #[inline(always)]
    fn recip_backend(&self) -> Self {
        Self::from_rows(&[
            self.x_axis.recip(),
            self.y_axis.recip(),
            self.z_axis.recip(),
        ])
    }

    #[inline(always)]
    fn abs_backend(&self) -> Self {
        Self::from_rows(&[self.x_axis.abs(), self.y_axis.abs(), self.z_axis.abs()])
    }

    #[inline(always)]
    fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: T) -> bool {
        self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
            && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
            && self.z_axis.abs_diff_eq(other.z_axis, max_abs_diff)
    }
}

impl<T, A: Alignment> Matrix<4, T, A>
where
    T: PrimitiveFloat,
{
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
    fn inverse_and_determinant_backend(&self) -> (Self, T) {
        let [m00, m01, m02, m03] = self.x_axis.to_array();
        let [m10, m11, m12, m13] = self.y_axis.to_array();
        let [m20, m21, m22, m23] = self.z_axis.to_array();
        let [m30, m31, m32, m33] = self.w_axis.to_array();

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

        let fac0 = Vector::<4, T, A>::new(coef00, coef00, coef02, coef03);
        let fac1 = Vector::<4, T, A>::new(coef04, coef04, coef06, coef07);
        let fac2 = Vector::<4, T, A>::new(coef08, coef08, coef10, coef11);
        let fac3 = Vector::<4, T, A>::new(coef12, coef12, coef14, coef15);
        let fac4 = Vector::<4, T, A>::new(coef16, coef16, coef18, coef19);
        let fac5 = Vector::<4, T, A>::new(coef20, coef20, coef22, coef23);

        let vec0 = Vector::<4, T, A>::new(m10, m00, m00, m00);
        let vec1 = Vector::<4, T, A>::new(m11, m01, m01, m01);
        let vec2 = Vector::<4, T, A>::new(m12, m02, m02, m02);
        let vec3 = Vector::<4, T, A>::new(m13, m03, m03, m03);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = Vector::<4, T, A>::new(T::ONE, T::NEG_ONE, T::ONE, T::NEG_ONE);
        let sign_b = Vector::<4, T, A>::new(T::NEG_ONE, T::ONE, T::NEG_ONE, T::ONE);

        let inverse = Matrix::<4, T, A>::from_rows(&[
            inv0 * sign_a,
            inv1 * sign_b,
            inv2 * sign_a,
            inv3 * sign_b,
        ]);

        let inverse_column_0 = Vector::<4, T, A>::new(
            inverse.x_axis.x,
            inverse.y_axis.x,
            inverse.z_axis.x,
            inverse.w_axis.x,
        );

        let determinant = self.x_axis.dot(inverse_column_0);
        let inverse = inverse / determinant;

        (inverse, determinant)
    }

    #[inline(always)]
    fn recip_backend(&self) -> Self {
        Self::from_rows(&[
            self.x_axis.recip(),
            self.y_axis.recip(),
            self.z_axis.recip(),
            self.w_axis.recip(),
        ])
    }

    #[inline(always)]
    fn abs_backend(&self) -> Self {
        Self::from_rows(&[
            self.x_axis.abs(),
            self.y_axis.abs(),
            self.z_axis.abs(),
            self.w_axis.abs(),
        ])
    }

    #[inline(always)]
    fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: T) -> bool {
        self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
            && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
            && self.z_axis.abs_diff_eq(other.z_axis, max_abs_diff)
            && self.w_axis.abs_diff_eq(other.w_axis, max_abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        EulerRot, FloatExt, Matrix, Projective, Quaternion, Vector,
        test_utils::{assert_debug_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, T: PrimitiveFloat, A| {
            assert_test_eq!(
                Matrix::<N, T, A>::NAN,
                Matrix::from_rows(&[Vector::<N, T, A>::NAN; N])
            );
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|T: PrimitiveFloat, A| {
            let projective = Projective::<2, T, A>::from_rows(&[
                Vector::<3, T, A>::new(0.9, 0.2, 1e-5),
                Vector::<3, T, A>::new(0.1, 0.8, 1e-5),
                Vector::<3, T, A>::new(5.3, 3.2, 1.0 + 1e-5),
            ]);
            assert_eq!(
                Matrix::<2, T, A>::from_projective(&projective),
                Matrix::<2, T, A>::from_rows(&[
                    projective.x_axis.truncate(),
                    projective.y_axis.truncate(),
                ])
            );

            let projective = Projective::<3, T, A>::from_rows(&[
                Vector::<4, T, A>::new(0.9, 0.2, 0.1, 1e-5),
                Vector::<4, T, A>::new(0.1, 0.8, 0.3, 1e-5),
                Vector::<4, T, A>::new(0.2, 0.1, 0.8, 1e-5),
                Vector::<4, T, A>::new(5.3, 3.2, 9.8, 1.0 + 1e-5),
            ]);
            assert_eq!(
                Matrix::<3, T, A>::from_projective(&projective),
                Matrix::<3, T, A>::from_rows(&[
                    projective.x_axis.truncate(),
                    projective.y_axis.truncate(),
                    projective.z_axis.truncate(),
                ])
            );

            assert_debug_panic!(Matrix::<2, T, A>::from_projective(
                &Projective::<2, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(0.9, 0.2, 2.0),
                    Vector::<3, T, A>::new(0.1, 0.8, 0.0),
                    Vector::<3, T, A>::new(5.3, 3.2, 1.0),
                ])
            ));
            assert_debug_panic!(Matrix::<3, T, A>::from_projective(
                &Projective::<3, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(0.9, 0.2, 0.1, 2.0),
                    Vector::<4, T, A>::new(0.1, 0.8, 0.3, 3.1),
                    Vector::<4, T, A>::new(0.2, 0.1, 0.8, 0.0),
                    Vector::<4, T, A>::new(5.3, 3.2, 9.8, 1.0),
                ])
            ));
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let nan = Vector::<2, T, A>::NAN;
            assert!(!Matrix::<2, T, A>::from_rows(&[one; 2]).is_nan());
            assert!(Matrix::<2, T, A>::from_rows(&[nan, one]).is_nan());
            assert!(Matrix::<2, T, A>::from_rows(&[one, nan]).is_nan());
            assert!(Matrix::<2, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::<3, T, A>::NAN;
            assert!(!Matrix::<3, T, A>::from_rows(&[one; 3]).is_nan());
            assert!(Matrix::<3, T, A>::from_rows(&[nan, one, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_rows(&[one, nan, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_rows(&[one, one, nan]).is_nan());
            assert!(Matrix::<3, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::<4, T, A>::NAN;
            assert!(!Matrix::<4, T, A>::from_rows(&[one; 4]).is_nan());
            assert!(Matrix::<4, T, A>::from_rows(&[nan, one, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_rows(&[one, nan, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_rows(&[one, one, nan, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_rows(&[one, one, one, nan]).is_nan());
            assert!(Matrix::<4, T, A>::NAN.is_nan());
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let inf = Vector::<2, T, A>::INFINITY;
            assert!(Matrix::<2, T, A>::from_rows(&[one, one]).is_finite());
            assert!(!Matrix::<2, T, A>::from_rows(&[inf, one]).is_finite());
            assert!(!Matrix::<2, T, A>::from_rows(&[one, inf]).is_finite());
            assert!(!Matrix::<2, T, A>::from_rows(&[inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::<3, T, A>::INFINITY;
            assert!(Matrix::<3, T, A>::from_rows(&[one, one, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_rows(&[inf, one, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_rows(&[one, inf, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_rows(&[one, one, inf]).is_finite());
            assert!(!Matrix::<3, T, A>::from_rows(&[inf, inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::<4, T, A>::INFINITY;
            assert!(Matrix::<4, T, A>::from_rows(&[one, one, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[inf, one, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[one, inf, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[one, one, inf, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[one, one, one, inf]).is_finite());
            assert!(!Matrix::<4, T, A>::from_rows(&[inf, inf, inf, inf]).is_finite());
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                if matrix.determinant() == 0.0 {
                    assert_debug_panic!(matrix.inverse());
                }

                if !matrix.is_finite()
                    || matrix.as_rows().iter().flatten().any(|x| x.abs() > 1e6)
                    || !(1e-2..=1e2).contains(
                        &(matrix.determinant()
                            / matrix
                                .as_rows()
                                .iter()
                                .flatten()
                                .reduce(T::max)
                                .unwrap()
                                .powi(N as i32)),
                    )
                {
                    continue;
                }

                assert_test_eq!(
                    matrix.inverse() * matrix,
                    Matrix::IDENTITY,
                    abs <= matrix
                        .determinant()
                        .abs()
                        .max(matrix.determinant().recip().abs())
                        * 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_try_inverse() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                let Some(try_inverse) = matrix.try_inverse() else {
                    assert_debug_panic!(matrix.inverse());
                    continue;
                };

                assert_test_eq!(try_inverse, matrix.inverse())
            }
        });
    }

    #[test]
    fn test_inverse_or() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [matrix, fallback] in random_iter::<[Matrix<N, T, A>; 2]>() {
                let Some(inverse) = matrix.try_inverse() else {
                    assert_test_eq!(matrix.inverse_or(&fallback), fallback);
                    continue;
                };

                assert_test_eq!(matrix.inverse_or(&fallback), inverse);
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                assert_test_eq!(matrix.inverse_or_zero(), matrix.inverse_or(&Matrix::ZERO));
            }
        });
    }

    #[test]
    fn test_recip() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                assert_test_eq!(matrix.recip(), Matrix::from_row_fn(|r| matrix[r].recip()));
            }
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                assert_test_eq!(matrix.abs(), Matrix::from_row_fn(|r| matrix[r].abs()));
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|T: PrimitiveFloat, A| {
            let x_axis = Vector::<2, T, A>::new(0.0, 1.0);
            let y_axis = Vector::<2, T, A>::new(2.0, 3.0);
            assert!(
                Matrix::<2, T, A>::from_rows(&[x_axis, y_axis])
                    .abs_diff_eq(&Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1]), 0.125)
            );
            assert!(
                !Matrix::<2, T, A>::from_rows(&[x_axis, y_axis])
                    .abs_diff_eq(&Matrix::from_rows(&[x_axis + 0.5, y_axis - 0.1]), 0.125)
            );
            assert!(
                !Matrix::<2, T, A>::from_rows(&[x_axis, y_axis])
                    .abs_diff_eq(&Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.5]), 0.125)
            );

            let x_axis = Vector::<3, T, A>::new(0.0, 1.0, 2.0);
            let y_axis = Vector::<3, T, A>::new(3.0, 4.0, 5.0);
            let z_axis = Vector::<3, T, A>::new(6.0, 7.0, 8.0);
            assert!(
                Matrix::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.5, y_axis - 0.1, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.5, z_axis + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_rows(&[x_axis, y_axis, z_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.5]),
                    0.125
                )
            );

            let x_axis = Vector::<4, T, A>::new(0.0, 1.0, 2.0, 3.0);
            let y_axis = Vector::<4, T, A>::new(4.0, 5.0, 6.0, 7.0);
            let z_axis = Vector::<4, T, A>::new(8.0, 9.0, 10.0, 11.0);
            let w_axis = Vector::<4, T, A>::new(12.0, 13.0, 14.0, 15.0);
            assert!(
                Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.05, w_axis - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.5, y_axis - 0.1, z_axis + 0.05, w_axis - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.5, z_axis + 0.05, w_axis - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.5, w_axis - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_rows(&[x_axis, y_axis, z_axis, w_axis]).abs_diff_eq(
                    &Matrix::from_rows(&[x_axis + 0.1, y_axis - 0.1, z_axis + 0.05, w_axis - 0.5]),
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
                    vector * Matrix::<2, T, A>::from_angle(angle),
                    vector.rotate(angle)
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
                    Matrix::<2, T, A>::from_scale_angle(scale, angle),
                    Matrix::<2, T, A>::from_scale(scale) * Matrix::<2, T, A>::from_angle(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_debug_panic!(Matrix::<2, T, A>::ZERO.to_scale_angle());
            assert_debug_panic!(
                Matrix::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(0.3, 0.4),
                    Vector::<2, T, A>::new(0.4, 0.6)
                ])
                .to_scale_angle()
            );

            for (scale, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                let matrix = Matrix::<2, T, A>::from_scale_angle(scale, angle);

                if scale.iter().any(|x| x > 1e10)
                    || !matrix.determinant().is_finite()
                    || matrix.determinant().abs() < 1e-8
                {
                    continue;
                }

                let (result_scale, result_angle) = matrix.to_scale_angle();
                assert_test_eq!(
                    Matrix::<2, T, A>::from_scale_angle(result_scale, result_angle),
                    matrix,
                    abs <= scale.max_element() * 1e-5 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                if !vector.is_finite() || !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    vector * Matrix::<3, T, A>::from_rotation_x(angle),
                    vector.rotate_x(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                if !vector.is_finite() || !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    vector * Matrix::<3, T, A>::from_rotation_y(angle),
                    vector.rotate_y(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                if !vector.is_finite() || !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    vector * Matrix::<3, T, A>::from_rotation_z(angle),
                    vector.rotate_z(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_quat() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Matrix::<3, T, A>::from_quat(Quaternion::IDENTITY),
                Matrix::IDENTITY
            );

            for quat in random_iter::<Quaternion<T, A>>() {
                if !quat.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::from_quat(quat));
                }

                let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();
                assert_test_eq!(
                    Matrix::<3, T, A>::from_quat(quat).determinant(),
                    1.0,
                    abs <= 1e-5
                );

                let (axis, angle) = quat.to_axis_angle();
                assert_test_eq!(
                    Matrix::<3, T, A>::from_quat(quat),
                    Matrix::<3, T, A>::from_axis_angle(axis, angle),
                    abs <= 1e-5,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>() {
                if !angle.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::X, angle),
                    Matrix::<3, T, A>::from_rotation_x(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::Y, angle),
                    Matrix::<3, T, A>::from_rotation_y(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    Matrix::<3, T, A>::from_axis_angle(Vector::<3, T, A>::Z, angle),
                    Matrix::<3, T, A>::from_rotation_z(angle),
                    abs <= 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for [x, y, z] in random_iter::<[T; 3]>() {
                let rot_x = Matrix::<3, T, A>::from_rotation_x(x);
                let rot_y = Matrix::<3, T, A>::from_rotation_y(y);
                let rot_z = Matrix::<3, T, A>::from_rotation_z(z);
                let rot_x_by_y = Matrix::<3, T, A>::from_rotation_x(y);
                let rot_x_by_z = Matrix::<3, T, A>::from_rotation_x(z);
                let rot_y_by_x = Matrix::<3, T, A>::from_rotation_y(x);
                let rot_y_by_z = Matrix::<3, T, A>::from_rotation_y(z);
                let rot_z_by_x = Matrix::<3, T, A>::from_rotation_z(x);
                let rot_z_by_y = Matrix::<3, T, A>::from_rotation_z(y);

                for (order, a, b, c, result) in [
                    (EulerRot::Xyz, x, y, z, rot_z * rot_y * rot_x),
                    (EulerRot::Xzy, x, z, y, rot_y * rot_z * rot_x),
                    (EulerRot::Yxz, y, x, z, rot_z * rot_x * rot_y),
                    (EulerRot::Yzx, y, z, x, rot_x * rot_z * rot_y),
                    (EulerRot::Zxy, z, x, y, rot_y * rot_x * rot_z),
                    (EulerRot::Zyx, z, y, x, rot_x * rot_y * rot_z),
                    (EulerRot::Xyx, x, y, z, rot_x_by_z * rot_y * rot_x),
                    (EulerRot::Xzx, x, z, y, rot_x_by_y * rot_z * rot_x),
                    (EulerRot::Yxy, y, x, z, rot_y_by_z * rot_x * rot_y),
                    (EulerRot::Yzy, y, z, x, rot_y_by_x * rot_z * rot_y),
                    (EulerRot::Zxz, z, x, y, rot_z_by_y * rot_x * rot_z),
                    (EulerRot::Zyz, z, y, x, rot_z_by_x * rot_y * rot_z),
                    (EulerRot::XyzEx, x, y, z, rot_x * rot_y * rot_z),
                    (EulerRot::XzyEx, x, z, y, rot_x * rot_z * rot_y),
                    (EulerRot::YxzEx, y, x, z, rot_y * rot_x * rot_z),
                    (EulerRot::YzxEx, y, z, x, rot_y * rot_z * rot_x),
                    (EulerRot::ZxyEx, z, x, y, rot_z * rot_x * rot_y),
                    (EulerRot::ZyxEx, z, y, x, rot_z * rot_y * rot_x),
                    (EulerRot::XyxEx, x, y, z, rot_x * rot_y * rot_x_by_z),
                    (EulerRot::XzxEx, x, z, y, rot_x * rot_z * rot_x_by_y),
                    (EulerRot::YxyEx, y, x, z, rot_y * rot_x * rot_y_by_z),
                    (EulerRot::YzyEx, y, z, x, rot_y * rot_z * rot_y_by_x),
                    (EulerRot::ZxzEx, z, x, y, rot_z * rot_x * rot_z_by_y),
                    (EulerRot::ZyzEx, z, y, x, rot_z * rot_y * rot_z_by_x),
                ] {
                    assert_test_eq!(
                        Matrix::<3, T, A>::from_euler(order, a, b, c),
                        result,
                        abs <= 1e-5,
                        0.0 = -0.0
                    );
                }
            }
        });
    }

    #[test]
    fn test_from_scale_rotation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation) in random_iter::<(Vector<3, T, A>, Quaternion<T, A>)>() {
                if !rotation.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::from_scale_rotation(scale, rotation));
                }

                let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();

                assert_test_eq!(
                    Matrix::<3, T, A>::from_scale_rotation(scale, rotation),
                    Matrix::<3, T, A>::from_scale(scale) * Matrix::<3, T, A>::from_quat(rotation),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [dir, up] in random_iter::<[Vector<3, T, A>; 2]>() {
                if !dir.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_lh(dir, up.normalize()));
                }
                if !up.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_lh(dir.normalize(), up));
                }

                let dir = dir.normalize_or(Vector::<3, T, A>::Z).normalize();
                let up = up.normalize_or(Vector::<3, T, A>::Y).normalize();
                if dir.cross(up).try_normalize().is_none() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_lh(dir, up));
                    continue;
                }

                let result = Matrix::<3, T, A>::look_to_lh(dir, up);
                assert_test_eq!(result.determinant(), 1.0, abs <= 1e-2);
                assert_test_eq!(dir * result, Vector::<3, T, A>::Z, abs <= 1e-5, 0.0 = -0.0);
                assert_test_eq!((up * result).x, 0.0, abs <= 1e-6, 0.0 = -0.0);
                assert!((0.0..=1.00001).contains(&(up * result).y));
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [dir, up] in random_iter::<[Vector<3, T, A>; 2]>() {
                if !dir.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_rh(dir, up.normalize()));
                }
                if !up.is_normalized() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_rh(dir.normalize(), up));
                }

                let dir = dir.normalize_or(Vector::<3, T, A>::Z).normalize();
                let up = up.normalize_or(Vector::<3, T, A>::Y).normalize();
                if dir.cross(up).try_normalize().is_none() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_to_rh(dir, up));
                    continue;
                }

                let result = Matrix::<3, T, A>::look_to_rh(dir, up);
                assert_test_eq!(result.determinant(), 1.0, abs <= 1e-2);
                assert_test_eq!(
                    dir * result,
                    Vector::<3, T, A>::NEG_Z,
                    abs <= 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!((up * result).x, 0.0, abs <= 1e-6, 0.0 = -0.0);
                assert!((0.0..=1.00001).contains(&(up * result).y));
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                if !up.is_normalized() || center == eye {
                    assert_debug_panic!(Matrix::<3, T, A>::look_at_lh(eye, center, up));
                }

                let up = up.normalize_or(Vector::<3, T, A>::Y);
                let Some(dir) = (center - eye).try_normalize() else {
                    continue;
                };
                if up.cross(dir).try_normalize().is_none() {
                    assert_debug_panic!(Matrix::<3, T, A>::look_at_lh(eye, center, up));
                    continue;
                }

                let result = Matrix::<3, T, A>::look_at_lh(eye, center, up);
                assert_test_eq!(result.determinant(), 1.0, abs <= 1e-5);
                assert_test_eq!(dir * result, Vector::<3, T, A>::Z, abs <= 1e-5, 0.0 = -0.0);
                assert_test_eq!((up * result).x, 0.0, abs <= 1e-6, 0.0 = -0.0);
                assert!((0.0..=1.00001).contains(&(up * result).y));
            }
        });
    }

    #[test]
    fn test_look_at_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                if !up.is_normalized() || center == eye {
                    assert_debug_panic!(Matrix::<3, T, A>::look_at_rh(eye, center, up));
                }

                let up = up.normalize_or(Vector::<3, T, A>::Y);
                let Some(dir) = (center - eye).try_normalize() else {
                    continue;
                };
                if up.cross(dir).try_normalize().is_none() {
                    continue;
                }

                let result = Matrix::<3, T, A>::look_at_rh(eye, center, up);
                assert_test_eq!(result.determinant(), 1.0, abs <= 1e-5);
                assert_test_eq!(
                    dir * result,
                    Vector::<3, T, A>::NEG_Z,
                    abs <= 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!((up * result).x, 0.0, abs <= 1e-6, 0.0 = -0.0);
                assert!((0.0..=1.00001).contains(&(up * result).y));
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for matrix in random_iter::<Matrix<3, T, A>>().take(20) {
                    let is_rotation = matrix.determinant().abs_diff_eq(1.0, 1e-4)
                        && matrix.x_axis.dot(matrix.y_axis).abs_diff_eq(0.0, 1e-4)
                        && matrix.x_axis.dot(matrix.z_axis).abs_diff_eq(0.0, 1e-4)
                        && matrix.y_axis.dot(matrix.z_axis).abs_diff_eq(0.0, 1e-4)
                        && matrix
                            .x_axis
                            .cross(matrix.y_axis)
                            .abs_diff_eq(matrix.z_axis, 1e-4);

                    if !is_rotation {
                        assert_debug_panic!(matrix.to_euler(order));
                    }
                }

                for quat in random_iter::<Quaternion<T, A>>() {
                    let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();
                    let matrix = Matrix::<3, T, A>::from_quat(quat);

                    let result = matrix.to_euler(order);
                    assert_test_eq!(
                        Quaternion::<T, A>::from_euler(order, result.0, result.1, result.2),
                        quat,
                        abs <= quat.to_vector().abs() * 1e-3 + 1e-2,
                        0.0 = -0.0,
                        quat = -quat
                    );
                }
            }
        });
    }

    #[test]
    fn test_to_scale_rotation() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_debug_panic!(Matrix::<3, T, A>::ZERO.to_scale_rotation());
            assert_debug_panic!(
                Matrix::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(0.3, 0.4, -0.2),
                    Vector::<3, T, A>::new(0.4, 0.6, -0.1),
                    Vector::<3, T, A>::new(1.0, 1.0, 1.0)
                ])
                .to_scale_rotation()
            );

            for (scale, rotation) in random_iter::<(Vector<3, T, A>, Quaternion<T, A>)>() {
                let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();

                let matrix = Matrix::<3, T, A>::from_scale_rotation(scale, rotation);

                if scale.iter().any(|x| x > 1e10)
                    || !matrix.is_finite()
                    || !(1e-5..1e8).contains(&matrix.determinant().abs())
                {
                    continue;
                }

                let (result_scale, result_rotation) = matrix.to_scale_rotation();
                assert_test_eq!(
                    Matrix::<3, T, A>::from_scale_rotation(result_scale, result_rotation),
                    matrix,
                    abs <= matrix.abs() * 1e-4 + Matrix::<3, T, A>::from_row_array(&[1e-3; 9]),
                    0.0 = -0.0
                );
            }
        });
    }
}
