use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Alignment, EulerRot, Length, Matrix, Projective, Quaternion, Vector,
    length::TwoOrThree,
    utils::{specialize_23, transmute_generic},
};

macro_rules! impl_wide_float {
    ($Wide:ident, $T:ident) => {
        #[expect(private_bounds)]
        impl<const N: usize, A: Alignment> Projective<N, $Wide, A>
        where
            Length<N>: TwoOrThree,
        {
            /// A matrix with all elements set to NaN (Not a Number).
            pub const NAN: Self = Self::NAN_IMPL;

            /// The implementation of [`Self::NAN`].
            ///
            /// Because of type system limitations, this implementation looks crazy. Use
            /// a separate constant so that IDEs do not show the implementation.
            const NAN_IMPL: Self = match N {
                // SAFETY: We are transmuting a type to itself
                2 => unsafe {
                    transmute_generic::<Projective<2, $Wide, A>, Projective<N, $Wide, A>>(
                        Projective::<2, $Wide, A> {
                            0: Matrix::<3, $Wide, A>::NAN,
                        },
                    )
                },
                // SAFETY: We are transmuting a type to itself
                3 => unsafe {
                    transmute_generic::<Projective<3, $Wide, A>, Projective<N, $Wide, A>>(
                        Projective::<3, $Wide, A> {
                            0: Matrix::<4, $Wide, A>::NAN,
                        },
                    )
                },
                _ => unreachable!(),
            };

            /// For each lane, returns `true` if any element is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(&self) -> $Wide {
                specialize_23!(Projective::<N, $Wide, A>::is_nan_backend(self))
            }

            /// For each lane, returns `true` if all elements are neither
            /// infinite nor NaN.
            #[inline]
            #[must_use]
            pub fn is_finite(&self) -> $Wide {
                specialize_23!(Projective::<N, $Wide, A>::is_finite_backend(self))
            }

            /// Returns the inverse of `self`.
            ///
            /// If `self` is not invertable the result is unspecified.
            #[must_use]
            pub fn inverse(&self) -> Self {
                specialize_23!(Projective::<N, $Wide, A>::inverse_backend(self))
            }

            // `try_inverse` is exluded on purpose. It would not be useful
            // because it would only return `Some` if all lanes succeed.

            /// Returns the inverse of `self` or `fallback` if `self` is not
            /// invertable.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            #[must_use]
            pub fn inverse_or(&self, fallback: &Self) -> Self {
                specialize_23!(Projective::<N, $Wide, A>::inverse_or_backend(
                    self, fallback
                ))
            }

            /// Returns the inverse of `self` or the zero matrix if `self` is
            /// not invertable.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            #[must_use]
            pub fn inverse_or_zero(&self) -> Self {
                specialize_23!(Projective::<N, $Wide, A>::inverse_or_zero_backend(self))
            }

            /// Transforms the given 2D vector as a point.
            ///
            /// Equivalent to `(point, 1) * self` but is faster.
            ///
            /// `self` must contain a valid affine transform, meaning the third column
            /// must be `(0, 0, 1)`.
            #[inline]
            #[must_use]
            pub fn transform_point(&self, point: Vector<N, $Wide, A>) -> Vector<N, $Wide, A> {
                specialize_23!(Projective::<N, $Wide, A>::transform_point_backend(
                    self, point
                ))
            }

            /// Transforms the given 2D vector without applying translation.
            ///
            /// Equivalent to `(vector, 0) * self` but is faster.
            ///
            /// `self` must contain a valid affine transform, meaning the third column
            /// must be `(0, 0, 1)`.
            #[inline]
            #[must_use]
            pub fn transform_vector(&self, vector: Vector<N, $Wide, A>) -> Vector<N, $Wide, A> {
                specialize_23!(Projective::<N, $Wide, A>::transform_vector_backend(
                    self, vector
                ))
            }

            /// Transforms the given 3D vector as a point, applying perspective
            /// projection.
            ///
            /// Equivalent to:
            ///
            /// ```ignore
            /// let result = matrix * (point, 1);
            /// result.xyz / result.w
            /// ```
            #[inline]
            #[must_use]
            pub fn project_point(&self, point: Vector<N, $Wide, A>) -> Vector<N, $Wide, A> {
                specialize_23!(Projective::<N, $Wide, A>::project_point_backend(
                    self, point
                ))
            }

            /// Returns `true` if the absolute difference of all elements
            /// between `self` and `other` is less than or equal to
            /// `max_abs_diff` for all lanes.
            ///
            /// This can be used to compare two matrices that should be equal,
            /// but may have a slight difference due to operations having
            /// rounding errors.
            #[inline]
            #[must_use]
            pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: $Wide) -> bool {
                specialize_23!(Projective::<N, $Wide, A>::abs_diff_eq_backend(
                    self,
                    other,
                    max_abs_diff
                ))
            }
        }

        impl<A: Alignment> Projective<2, $Wide, A> {
            /// Creates an affine transformation matrix containing a rotation of
            /// `angle` (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 2D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_angle(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos, sin, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin, cos, $Wide::ZERO),
                    Vector::<3, $Wide, A>::Z,
                ])
            }

            /// Creates an affine transformation matrix containing the
            /// non-uniform `scale` and a rotation of `angle` (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 2D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_scale_angle(scale: Vector<2, $Wide, A>, angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos * scale.x, sin * scale.x, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin * scale.y, cos * scale.y, $Wide::ZERO),
                    Vector::<3, $Wide, A>::Z,
                ])
            }

            /// Creates an affine transformation matrix containing a rotation of
            /// `angle` (in radians) and `translation`.
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 2D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_angle_translation(angle: $Wide, translation: Vector<2, $Wide, A>) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos, sin, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin, cos, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(translation.x, translation.y, $Wide::ONE),
                ])
            }

            /// Creates an affine transformation matrix containing the
            /// non-uniform `scale`, a rotation of `angle` (in radians) and
            /// `translation`.
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 2D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_scale_angle_translation(
                scale: Vector<2, $Wide, A>,
                angle: $Wide,
                translation: Vector<2, $Wide, A>,
            ) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos * scale.x, sin * scale.x, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin * scale.y, cos * scale.y, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(translation.x, translation.y, $Wide::ONE),
                ])
            }

            /// For each lane, returns the `scale` and `angle` of `self`.
            ///
            /// `self` must contain a valid affine transformation without
            /// shearing. Otherwise the result is unspecified.
            ///
            /// `self` can contain translation which is ignored.
            #[inline]
            #[must_use]
            pub fn to_scale_angle(&self) -> (Vector<2, $Wide, A>, $Wide) {
                Matrix::from_projective(self).to_scale_angle()
            }

            /// For each lane, returns the `scale`, `angle` and `translation` of
            /// `self`.
            ///
            /// `self` must contain a valid affine transformation without
            /// shearing. Otherwise the result is unspecified.
            #[inline]
            #[must_use]
            pub fn to_scale_angle_translation(
                &self,
            ) -> (Vector<2, $Wide, A>, $Wide, Vector<2, $Wide, A>) {
                let (scale, angle) = self.to_scale_angle();
                (scale, angle, self.translation())
            }

            #[inline(always)]
            fn is_nan_backend(&self) -> $Wide {
                self.x_axis.is_nan() | self.y_axis.is_nan() | self.z_axis.is_nan()
            }

            #[inline(always)]
            fn is_finite_backend(&self) -> $Wide {
                self.x_axis.is_finite() & self.y_axis.is_finite() & self.z_axis.is_finite()
            }

            #[inline(always)]
            fn inverse_backend(&self) -> Self {
                Self(self.0.inverse())
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
            fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: $Wide) -> bool {
                self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
                    && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
                    && self.z_axis.abs_diff_eq(other.z_axis, max_abs_diff)
            }

            #[inline(always)]
            fn transform_point_backend(&self, point: Vector<2, $Wide, A>) -> Vector<2, $Wide, A> {
                self.x_axis.xy() * point.x + self.y_axis.xy() * point.y + self.z_axis.xy()
            }

            #[inline(always)]
            fn transform_vector_backend(&self, vector: Vector<2, $Wide, A>) -> Vector<2, $Wide, A> {
                self.x_axis.xy() * vector.x + self.y_axis.xy() * vector.y
            }

            #[inline(always)]
            fn project_point_backend(&self, point: Vector<2, $Wide, A>) -> Vector<2, $Wide, A> {
                let result = self.x_axis * point.x + self.y_axis * point.y + self.z_axis;

                (result / result.z).xy()
            }
        }

        impl<A: Alignment> Projective<3, $Wide, A> {
            /// Creates an affine transformation matrix containing a 3D rotation
            /// from `angle` (in radians) around the x axis.
            ///
            /// This rotates `+Y` to `+Z`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_rotation_x(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<4, $Wide, A>::X,
                    Vector::<4, $Wide, A>::new($Wide::ZERO, cos, sin, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, -sin, cos, $Wide::ZERO),
                    Vector::<4, $Wide, A>::W,
                ])
            }

            /// Creates an affine transformation matrix containing a 3D rotation
            /// from `angle` (in radians) around the y axis.
            ///
            /// This rotates `+Z` to `+X`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_rotation_y(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(cos, $Wide::ZERO, -sin, $Wide::ZERO),
                    Vector::<4, $Wide, A>::Y,
                    Vector::<4, $Wide, A>::new(sin, $Wide::ZERO, cos, $Wide::ZERO),
                    Vector::<4, $Wide, A>::W,
                ])
            }

            /// Creates an affine transformation matrix containing a 3D rotation
            /// from `angle` (in radians) around the z axis.
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_rotation_z(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(cos, sin, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(-sin, cos, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::Z,
                    Vector::<4, $Wide, A>::W,
                ])
            }

            #[inline]
            fn quat_to_axes(quat: Quaternion<$Wide, A>) -> [Vector<4, $Wide, A>; 3] {
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
                    Vector::<4, $Wide, A>::new(
                        $Wide::ONE - (yy2 + zz2),
                        xy2 + wz2,
                        xz2 - wy2,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        xy2 - wz2,
                        $Wide::ONE - (xx2 + zz2),
                        yz2 + wx2,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        xz2 + wy2,
                        yz2 - wx2,
                        $Wide::ONE - (xx2 + yy2),
                        $Wide::ZERO,
                    ),
                ]
            }

            /// Creates an affine transformation matrix containing a 3D rotation
            /// from a quaternion.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_quat(quat: Quaternion<$Wide, A>) -> Self {
                let [x_axis, y_axis, z_axis] = Self::quat_to_axes(quat);
                Self::from_rows(&[x_axis, y_axis, z_axis, Vector::W])
            }

            /// Creates an affine transformation matrix containing a rotation
            /// from a rotation `axis` and `angle` (in radians).
            ///
            /// `axis` must be normalized. Otherwise the result is unspecified.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                let [xsin, ysin, zsin] = (axis * sin).to_array();
                let [x, y, z] = axis.to_array();
                let [x2, y2, z2] = (axis * axis).to_array();
                let omc = $Wide::ONE - cos;
                let xyomc = x * y * omc;
                let xzomc = x * z * omc;
                let yzomc = y * z * omc;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        x2 * omc + cos,
                        xyomc + zsin,
                        xzomc - ysin,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        xyomc - zsin,
                        y2 * omc + cos,
                        yzomc + xsin,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        xzomc + ysin,
                        yzomc - xsin,
                        z2 * omc + cos,
                        $Wide::ZERO,
                    ),
                    Vector::W,
                ])
            }

            /// Creates an affine transformation matrix containing a rotation
            /// from an Euler rotation order/sequence and angles (in radians).
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_euler(order: EulerRot, a: $Wide, b: $Wide, c: $Wide) -> Self {
                Self::from_matrix(&Matrix::<3, $Wide, A>::from_euler(order, a, b, c))
            }

            /// Creates an affine transformation matrix containing a non-uniform
            /// `scale` and a 3D `rotation`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_scale_rotation(
                scale: Vector<3, $Wide, A>,
                rotation: Quaternion<$Wide, A>,
            ) -> Self {
                let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
                Self::from_rows(&[
                    rotation_x * scale.x,
                    rotation_y * scale.y,
                    rotation_z * scale.z,
                    Vector::W,
                ])
            }

            /// Creates an affine transformation matrix containing a 3D
            /// `rotation` and `translation`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_rotation_translation(
                rotation: Quaternion<$Wide, A>,
                translation: Vector<3, $Wide, A>,
            ) -> Self {
                let [x_axis, y_axis, z_axis] = Self::quat_to_axes(rotation);
                Self::from_rows(&[
                    x_axis,
                    y_axis,
                    z_axis,
                    Vector::<4, $Wide, A>::new(
                        translation.x,
                        translation.y,
                        translation.z,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates an affine transformation matrix containing the
            /// non-uniform `scale`, a 3D `rotation` and `translation`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn from_scale_rotation_translation(
                scale: Vector<3, $Wide, A>,
                rotation: Quaternion<$Wide, A>,
                translation: Vector<3, $Wide, A>,
            ) -> Self {
                let [rotation_x, rotation_y, rotation_z] = Self::quat_to_axes(rotation);
                Self::from_rows(&[
                    rotation_x * scale.x,
                    rotation_y * scale.y,
                    rotation_z * scale.z,
                    Vector::<4, $Wide, A>::new(
                        translation.x,
                        translation.y,
                        translation.z,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a left-handed view matrix from a camera position, a
            /// facing direction and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=forward`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn look_to_lh(
                eye: Vector<3, $Wide, A>,
                dir: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                let forward = dir;
                let right = up.cross(forward).normalize();
                let up = forward.cross(right);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(right.x, up.x, forward.x, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(right.y, up.y, forward.y, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(right.z, up.z, forward.z, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        -eye.dot(right),
                        -eye.dot(up),
                        -eye.dot(forward),
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a right-handed view matrix from a camera position, a
            /// facing direction and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn look_to_rh(
                eye: Vector<3, $Wide, A>,
                dir: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                let forward = dir;
                let right = forward.cross(up).normalize();
                let up = right.cross(forward);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(right.x, up.x, -forward.x, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(right.y, up.y, -forward.y, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(right.z, up.z, -forward.z, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        -eye.dot(right),
                        -eye.dot(up),
                        eye.dot(forward),
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a left-handed view matrix from a camera position, a
            /// focal point and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=forward`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn look_at_lh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::look_to_lh(eye, (center - eye).normalize(), up)
            }

            /// Creates a right-handed view matrix from a camera position, a
            /// focal point and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            ///
            /// The resulting matrix can be used to transform 3D points and
            /// vectors. See [`transform_point`] and [`transform_vector`].
            ///
            /// [`transform_point`]: Self::transform_point
            /// [`transform_vector`]: Self::transform_vector
            #[inline]
            #[must_use]
            pub fn look_at_rh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::look_to_rh(eye, (center - eye).normalize(), up)
            }

            /// Creates a left-handed perspective projection matrix with `0..1`
            /// depth range.
            ///
            /// Useful to map the standard left-handed coordinate system into
            /// what WebGPU/Metal/Direct3D expect.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn perspective_lh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::splat(0.5)).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;
                let depth_scale = far_plane / (far_plane - near_plane);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, depth_scale, $Wide::ONE),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        -depth_scale * near_plane,
                        $Wide::ZERO,
                    ),
                ])
            }

            /// Creates a right-handed perspective projection matrix with `0..1`
            /// depth range.
            ///
            /// Useful to map the standard right-handed coordinate system into
            /// what WebGPU/Metal/Direct3D expect.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn perspective_rh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;
                let neg_depth_scale = far_plane / (near_plane - far_plane);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        neg_depth_scale,
                        -$Wide::ONE,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        neg_depth_scale * near_plane,
                        $Wide::ZERO,
                    ),
                ])
            }

            /// Creates a right-handed perspective projection matrix with
            /// `-1..1` depth range.
            ///
            /// Equivalent to the OpenGL [`gluPerspective`] function.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`gluPerspective`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn perspective_rh_gl(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;
                let depth_recip = $Wide::ONE / (near_plane - far_plane);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        (near_plane + far_plane) * depth_recip,
                        -$Wide::ONE,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::splat(2.0) * near_plane * far_plane * depth_recip,
                        $Wide::ZERO,
                    ),
                ])
            }

            /// Creates an infinite left-handed perspective projection matrix
            /// with `0..1` depth range.
            ///
            /// Equivalent to `perspective_lh`, but with an infinite value for
            /// `far_plane`. The result is that points near `near_plane` have
            /// depth `0`, and as they move towards infinity the depth
            /// approaches `1`.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn perspective_infinite_lh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, $Wide::ONE, $Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, -near_plane, $Wide::ZERO),
                ])
            }

            /// Creates an infinite right-handed perspective projection matrix
            /// with `0..1` depth range.
            ///
            /// Equivalent to `perspective_rh`, but with an infinite value for
            /// `far_plane`. The result is that points near `near_plane` have
            /// depth `0`, and as they move towards infinity the depth
            /// approaches `1`.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn perspective_infinite_rh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, -$Wide::ONE, -$Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, -near_plane, $Wide::ZERO),
                ])
            }

            /// Creates an infinite left-handed perspective projection matrix
            /// with reversed `0..1` depth range.
            ///
            /// Equivalent to `perspective_infinite_lh`, but maps points at
            /// `near_plane` to depth `1` and points at infinity to depth `0`.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn perspective_infinite_reverse_lh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, $Wide::ZERO, $Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, near_plane, $Wide::ZERO),
                ])
            }

            /// Creates an infinite right-handed perspective projection matrix
            /// with reversed `0..1` depth range.
            ///
            /// Equivalent to `perspective_infinite_rh`, but maps points at
            /// `near_plane` to depth `1` and points at infinity to depth `0`.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn perspective_infinite_reverse_rh(
                vertical_fov: $Wide,
                aspect_ratio: $Wide,
                near_plane: $Wide,
            ) -> Self {
                let (sin, cos) = (vertical_fov * $Wide::HALF).sin_cos();
                let height_recip = cos / sin;
                let width_recip = height_recip / aspect_ratio;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(width_recip, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, height_recip, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, $Wide::ZERO, -$Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, near_plane, $Wide::ZERO),
                ])
            }

            /// Creates a left-handed perspective projection matrix with `0..1`
            /// depth range.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn frustum_lh(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let depth_recip = $Wide::ONE / (far_plane - near_plane);
                let two_near_plane = $Wide::splat(2.0) * near_plane;
                let a = (right + left) * width_recip;
                let b = (top + bottom) * height_recip;
                let c = far_plane * depth_recip;
                let d = -near_plane * c;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        two_near_plane * width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        two_near_plane * height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(a, b, c, $Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, d, $Wide::ZERO),
                ])
            }

            /// Creates a right-handed perspective projection matrix with `0..1`
            /// depth range.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn frustum_rh(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let depth_recip = $Wide::ONE / (far_plane - near_plane);
                let two_near_plane = $Wide::splat(2.0) * near_plane;
                let a = (right + left) * width_recip;
                let b = (top + bottom) * height_recip;
                let c = -far_plane * depth_recip;
                let d = near_plane * c;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        two_near_plane * width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        two_near_plane * height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(a, b, c, -$Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, d, $Wide::ZERO),
                ])
            }

            /// Creates a right-handed perspective projection matrix with
            /// `-1..1` depth range.
            ///
            /// Equivalent to the OpenGL [`glFrustum`] function.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`glFrustum`]: https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn frustum_rh_gl(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near_plane: $Wide,
                far_plane: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let depth_recip = $Wide::ONE / (far_plane - near_plane);
                let two_near_plane = $Wide::splat(2.0) * near_plane;
                let a = (right + left) * width_recip;
                let b = (top + bottom) * height_recip;
                let c = -(far_plane + near_plane) * depth_recip;
                let d = -two_near_plane * far_plane * depth_recip;

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        two_near_plane * width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        two_near_plane * height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(a, b, c, -$Wide::ONE),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, d, $Wide::ZERO),
                ])
            }

            /// Creates a left-handed orthographic projection matrix with `0..1`
            /// depth range.
            ///
            /// Useful to map a left-handed coordinate system into what
            /// WebGPU/Metal/Direct3D expect.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn orthographic_lh(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near: $Wide,
                far: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let depth_recip = $Wide::ONE / (far - near);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        width_recip + width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        height_recip + height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, depth_recip, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        -(left + right) * width_recip,
                        -(top + bottom) * height_recip,
                        -depth_recip * near,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a right-handed orthographic projection matrix with
            /// `0..1` depth range.
            ///
            /// Useful to map a right-handed coordinate system into what
            /// WebGPU/Metal/Direct3D expect.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn orthographic_rh(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near: $Wide,
                far: $Wide,
            ) -> Self {
                let width_recip = $Wide::ONE / (right - left);
                let height_recip = $Wide::ONE / (top - bottom);
                let neg_depth_recip = $Wide::ONE / (near - far);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(
                        width_recip + width_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        height_recip + height_recip,
                        $Wide::ZERO,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        $Wide::ZERO,
                        $Wide::ZERO,
                        neg_depth_recip,
                        $Wide::ZERO,
                    ),
                    Vector::<4, $Wide, A>::new(
                        -(left + right) * width_recip,
                        -(top + bottom) * height_recip,
                        neg_depth_recip * near,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Creates a right-handed orthographic projection matrix with
            /// `-1..1` depth range.
            ///
            /// Equivalent to the OpenGL [`glOrtho`] function.
            ///
            /// The resulting matrix can be used to transform 3D points using
            /// [`project_point`].
            ///
            /// [`glOrtho`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml
            /// [`project_point`]: Self::project_point
            #[inline]
            #[must_use]
            pub fn orthographic_rh_gl(
                left: $Wide,
                right: $Wide,
                bottom: $Wide,
                top: $Wide,
                near: $Wide,
                far: $Wide,
            ) -> Self {
                let scale_x = $Wide::splat(2.0) / (right - left);
                let scale_y = $Wide::splat(2.0) / (top - bottom);
                let scale_z = $Wide::splat(2.0) / (near - far);
                let translation_x = -(right + left) / (right - left);
                let translation_y = -(top + bottom) / (top - bottom);
                let translation_z = -(far + near) / (far - near);

                Self::from_rows(&[
                    Vector::<4, $Wide, A>::new(scale_x, $Wide::ZERO, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, scale_y, $Wide::ZERO, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, scale_z, $Wide::ZERO),
                    Vector::<4, $Wide, A>::new(
                        translation_x,
                        translation_y,
                        translation_z,
                        $Wide::ONE,
                    ),
                ])
            }

            /// Returns the Euler angles forming `self` for the given Euler
            /// rotation order/sequence.
            ///
            /// The upper 3x3 matrix must not contain any non-rotation
            /// transformations. Otherwise the result is unspecified.
            #[inline]
            #[must_use]
            pub fn to_euler(&self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
                Matrix::from_projective(self).to_euler(order)
            }

            /// For each lane, returns the `scale` and `rotation` of `self`.
            ///
            /// `self` must contain a valid affine transformation. Otherwise the
            /// result is unspecified.
            ///
            /// `self` can contain translation which is ignored.
            #[inline]
            #[must_use]
            pub fn to_scale_rotation(&self) -> (Vector<3, $Wide, A>, Quaternion<$Wide, A>) {
                Matrix::from_projective(self).to_scale_rotation()
            }

            /// For each lane, returns the `scale`, `rotation` and `translation`
            /// of `self`.
            ///
            /// `self` must contain a valid affine transformation. Otherwise the
            /// result is unspecified.
            #[inline]
            #[must_use]
            pub fn to_scale_rotation_translation(
                &self,
            ) -> (
                Vector<3, $Wide, A>,
                Quaternion<$Wide, A>,
                Vector<3, $Wide, A>,
            ) {
                let (scale, rotation) = self.to_scale_rotation();
                (scale, rotation, self.translation())
            }

            #[inline(always)]
            fn transform_point_backend(&self, point: Vector<3, $Wide, A>) -> Vector<3, $Wide, A> {
                self.x_axis.xyz() * point.x
                    + self.y_axis.xyz() * point.y
                    + self.z_axis.xyz() * point.z
                    + self.w_axis.xyz()
            }

            #[inline(always)]
            fn transform_vector_backend(&self, vector: Vector<3, $Wide, A>) -> Vector<3, $Wide, A> {
                self.x_axis.xyz() * vector.x
                    + self.y_axis.xyz() * vector.y
                    + self.z_axis.xyz() * vector.z
            }

            #[inline(always)]
            fn project_point_backend(&self, point: Vector<3, $Wide, A>) -> Vector<3, $Wide, A> {
                let result = self.x_axis * point.x
                    + self.y_axis * point.y
                    + self.z_axis * point.z
                    + self.w_axis;

                (result / result.w).xyz()
            }

            #[inline(always)]
            fn is_nan_backend(&self) -> $Wide {
                self.x_axis.is_nan()
                    | self.y_axis.is_nan()
                    | self.z_axis.is_nan()
                    | self.w_axis.is_nan()
            }

            #[inline(always)]
            fn is_finite_backend(&self) -> $Wide {
                self.x_axis.is_finite()
                    & self.y_axis.is_finite()
                    & self.z_axis.is_finite()
                    & self.w_axis.is_finite()
            }

            #[inline(always)]
            fn inverse_backend(&self) -> Self {
                Self(self.0.inverse())
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
            fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: $Wide) -> bool {
                self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
                    && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
                    && self.z_axis.abs_diff_eq(other.z_axis, max_abs_diff)
                    && self.w_axis.abs_diff_eq(other.w_axis, max_abs_diff)
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
