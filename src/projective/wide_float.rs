use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Affine, Alignment, EulerRot, Length, Matrix, Projective, Rotor, Vector,
    length::TwoOrThree,
    utils::{specialize_23, transmute_generic},
};

macro_rules! items {
    ($Wide:ident, $T:ident) => {
        /// A matrix with all elements set to NaN (Not a Number).
        pub const NAN: Self = Self::NAN_INTERNAL_IMPL;

        /// The implementation of [`Self::NAN`].
        ///
        /// Because of type system limitations, this implementation looks crazy.
        /// Use a separate constant so that IDEs do not show the implementation.
        #[allow(
            clippy::init_numbered_fields,
            reason = "due to some sort of compiler bug, tuple initialization fails here"
        )]
        const NAN_INTERNAL_IMPL: Self =
            match N {
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

        /// Creates a projective transform from a rotor.
        ///
        /// This assumes the rotor is normalized.
        #[inline]
        #[must_use]
        pub fn from_rotor(rotor: Rotor<N, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<N, $Wide, A>::from_rotor(rotor))
        }

        /// Creates a projective transform from non-uniform `scale` and
        /// `rotation`.
        ///
        /// This assumes `rotation` is normalized.
        #[inline]
        #[must_use]
        #[track_caller]
        #[expect(private_bounds)]
        pub fn from_scale_rotation(scale: Vector<N, $Wide, A>, rotation: Rotor<N, $Wide, A>) -> Self
        where
            Length<N>: TwoOrThree,
        {
            Self::from_matrix(&Matrix::<N, $Wide, A>::from_scale_rotation(scale, rotation))
        }

        /// Creates a projective transform from `rotation` and `translation`.
        ///
        /// This assumes `rotation` is normalized.
        #[inline]
        #[must_use]
        #[track_caller]
        #[expect(private_bounds)]
        pub fn from_rotation_translation(
            rotation: Rotor<N, $Wide, A>,
            translation: Vector<N, $Wide, A>,
        ) -> Self
        where
            Length<N>: TwoOrThree,
        {
            Self::from_matrix_translation(&Matrix::<N, $Wide, A>::from_rotor(rotation), translation)
        }

        /// Creates a projective transform from non-uniform `scale`, `rotation`
        /// and `translation`.
        ///
        /// This assumes `rotation` is normalized.
        #[inline]
        #[must_use]
        #[track_caller]
        #[expect(private_bounds)]
        pub fn from_scale_rotation_translation(
            scale: Vector<N, $Wide, A>,
            rotation: Rotor<N, $Wide, A>,
            translation: Vector<N, $Wide, A>,
        ) -> Self
        where
            Length<N>: TwoOrThree,
        {
            Self::from_matrix_translation(
                &Matrix::<N, $Wide, A>::from_scale_rotation(scale, rotation),
                translation,
            )
        }

        /// For each lane, returns `true` if any element is NaN.
        #[inline]
        #[must_use]
        pub fn is_nan(&self) -> $Wide {
            specialize_23!(Projective::<N, $Wide, A>::is_nan_backend(self))
        }

        /// For each lane, returns `true` if all elements are neither infinite
        /// nor NaN.
        #[inline]
        #[must_use]
        pub fn is_finite(&self) -> $Wide {
            specialize_23!(Projective::<N, $Wide, A>::is_finite_backend(self))
        }

        /// Returns the inverse of `self`.
        ///
        /// If `self` is not invertable the result is unspecified.
        ///
        /// This computes the inverse of the inner homogeneous matrix.
        #[must_use]
        pub fn inverse(&self) -> Self {
            specialize_23!(Projective::<N, $Wide, A>::inverse_backend(self))
        }

        // `try_inverse` is exluded on purpose. It would not be useful because
        // it would only return `Some` if all lanes succeed.

        /// Returns the inverse of `self` or `fallback` if `self` is not
        /// invertable.
        ///
        /// The fallback is only applied for invalid lanes. Other lanes are not
        /// affected.
        ///
        /// This computes the inverse of the inner homogeneous matrix.
        #[must_use]
        pub fn inverse_or(&self, fallback: &Self) -> Self {
            specialize_23!(Projective::<N, $Wide, A>::inverse_or_backend(
                self, fallback
            ))
        }

        /// Returns the inverse of `self` or the zero transform if `self` is not
        /// invertable.
        ///
        /// The fallback is only applied for invalid lanes. Other lanes are not
        /// affected.
        ///
        /// This computes the inverse of the inner homogeneous matrix.
        #[must_use]
        pub fn inverse_or_zero(&self) -> Self {
            specialize_23!(Projective::<N, $Wide, A>::inverse_or_zero_backend(self))
        }

        /// Transforms the given vector as a point.
        ///
        /// Equivalent to `(point, 1) * self` but is faster.
        ///
        /// This function assumes `self` contains an affine transformation, with
        /// no projections, meaning the last column must be `(0, 0, ..., 1)`.
        #[inline]
        #[must_use]
        pub fn transform_point(&self, point: Vector<N, $Wide, A>) -> Vector<N, $Wide, A> {
            specialize_23!(Projective::<N, $Wide, A>::transform_point_backend(
                self, point
            ))
        }

        /// Transforms the given vector without applying translation.
        ///
        /// Equivalent to `(vector, 0) * self` but is faster.
        ///
        /// This function assumes `self` contains an affine transformation, with
        /// no projections, meaning the last column must be `(0, 0, ..., 1)`.
        #[inline]
        #[must_use]
        pub fn transform_vector(&self, vector: Vector<N, $Wide, A>) -> Vector<N, $Wide, A> {
            specialize_23!(Projective::<N, $Wide, A>::transform_vector_backend(
                self, vector
            ))
        }

        /// Transforms the given vector as a point, applying perspective divide.
        #[inline]
        #[must_use]
        pub fn project_point(&self, point: Vector<N, $Wide, A>) -> Vector<N, $Wide, A> {
            specialize_23!(Projective::<N, $Wide, A>::project_point_backend(
                self, point
            ))
        }

        /// Returns the absolute values of the elements of `self`.
        ///
        /// Equivalent to `(self.x_axis.abs(), self.y_axis.abs(), ...)`.
        #[inline]
        #[must_use]
        pub fn abs(&self) -> Self {
            specialize_23!(Projective::<N, $Wide, A>::abs_backend(self))
        }

        /// Converts a projective transform to non-uniform scale and rotation.
        ///
        /// This assumes the transform only contains scale, rotation, and
        /// translation which is ignored.
        #[inline]
        #[must_use]
        #[track_caller]
        #[expect(private_bounds)]
        pub fn to_scale_rotation(&self) -> (Vector<N, $Wide, A>, Rotor<N, $Wide, A>)
        where
            Length<N>: TwoOrThree,
        {
            Matrix::<N, $Wide, A>::from_projective(self).to_scale_rotation()
        }

        /// Converts a projective transform to rotation and translation.
        ///
        /// This assumes the transform only contains rotation and translation.
        #[inline]
        #[must_use]
        #[track_caller]
        #[expect(private_bounds)]
        pub fn to_rotation_translation(&self) -> (Rotor<N, $Wide, A>, Vector<N, $Wide, A>)
        where
            Length<N>: TwoOrThree,
        {
            Affine::<N, $Wide, A>::from_projective(self).to_rotation_translation()
        }

        /// Converts a projective transform to non-uniform scale, rotation and
        /// translation.
        ///
        /// This assumes the transform only contains scale, rotation and
        /// translation.
        #[inline]
        #[must_use]
        #[track_caller]
        #[expect(private_bounds)]
        pub fn to_scale_rotation_translation(
            &self,
        ) -> (Vector<N, $Wide, A>, Rotor<N, $Wide, A>, Vector<N, $Wide, A>)
        where
            Length<N>: TwoOrThree,
        {
            Affine::<N, $Wide, A>::from_projective(self).to_scale_rotation_translation()
        }

        /// Returns `true` if the absolute difference of all elements between
        /// `self` and `other` is less than or equal to `max_abs_diff` for all
        /// lanes.
        ///
        /// This can be used to compare two transforms that should be equal, but
        /// may have a slight difference due to operations having rounding
        /// errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: $Wide) -> bool {
            specialize_23!(Projective::<N, $Wide, A>::abs_diff_eq_backend(
                self,
                other,
                max_abs_diff
            ))
        }
    };
}

macro_rules! items_2 {
    ($Wide:ident, $T:ident) => {
        /// Creates a projective transform containing a rotation from an `angle`
        /// (in radians) rotating `+X` to `+Y`.
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

        /// Creates a 2D projective transform containing a non-uniform `scale`
        /// and a rotation of `angle` (in radians).
        ///
        /// This rotates `+X` to `+Y`.
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

        /// Creates a 2D projective transform containing a rotation of `angle`
        /// (in radians) and `translation`.
        ///
        /// This rotates `+X` to `+Y`.
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

        /// Creates a 2D projective transform containing a non-uniform `scale`,
        /// a rotation of `angle` (in radians) and `translation`.
        ///
        /// This rotates `+X` to `+Y`.
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

        /// Returns the `scale` and `angle` of `self`.
        ///
        /// This function assumes `self` contains an affine transformation with
        /// no shearing.
        ///
        /// `self` can contain translation, which is ignored.
        #[inline]
        #[must_use]
        pub fn to_scale_angle(&self) -> (Vector<2, $Wide, A>, $Wide) {
            Matrix::<2, $Wide, A>::from_projective(self).to_scale_angle()
        }

        /// Returns the `scale`, `angle` and `translation` of `self`.
        ///
        /// This function assumes `self` contains an affine transformation with
        /// no shearing.
        #[inline]
        #[must_use]
        pub fn to_scale_angle_translation(
            &self,
        ) -> (Vector<2, $Wide, A>, $Wide, Vector<2, $Wide, A>) {
            let (scale, angle) = self.to_scale_angle();
            (scale, angle, self.translation())
        }
    };
}

macro_rules! items_3 {
    ($Wide:ident, $T:ident) => {
        /// Creates a projective transform containing a rotation from an `angle`
        /// (in radians) rotating `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xy(angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_rows(&[
                Vector::<4, $Wide, A>::new(cos, sin, $Wide::ZERO, $Wide::ZERO),
                Vector::<4, $Wide, A>::new(-sin, cos, $Wide::ZERO, $Wide::ZERO),
                Vector::<4, $Wide, A>::Z,
                Vector::<4, $Wide, A>::W,
            ])
        }

        /// Creates a projective transform containing a rotation from an `angle`
        /// (in radians) rotating `+X` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xz(angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_rows(&[
                Vector::<4, $Wide, A>::new(cos, $Wide::ZERO, sin, $Wide::ZERO),
                Vector::<4, $Wide, A>::Y,
                Vector::<4, $Wide, A>::new(-sin, $Wide::ZERO, cos, $Wide::ZERO),
                Vector::<4, $Wide, A>::W,
            ])
        }

        /// Creates a projective transform containing a rotation from an `angle`
        /// (in radians) rotating `+Y` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_yz(angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_rows(&[
                Vector::<4, $Wide, A>::X,
                Vector::<4, $Wide, A>::new($Wide::ZERO, cos, sin, $Wide::ZERO),
                Vector::<4, $Wide, A>::new($Wide::ZERO, -sin, cos, $Wide::ZERO),
                Vector::<4, $Wide, A>::W,
            ])
        }

        /// Creates a 3D projective transform containing a rotation from a
        /// rotation `axis` and `angle` (in radians) using the right-hand rule.
        ///
        /// `axis` must be normalized. Otherwise the result is unspecified.
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
                Vector::<4, $Wide, A>::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin, $Wide::ZERO),
                Vector::<4, $Wide, A>::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin, $Wide::ZERO),
                Vector::<4, $Wide, A>::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos, $Wide::ZERO),
                Vector::W,
            ])
        }

        /// Creates a 3D projective transform containing a rotation from an
        /// Euler rotation order/sequence and angles (in radians).
        #[inline]
        #[must_use]
        pub fn from_euler(order: EulerRot, a: $Wide, b: $Wide, c: $Wide) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::from_euler(order, a, b, c))
        }

        /// Creates a left-handed view transform from a camera position, a
        /// facing direction and an up direction.
        ///
        /// For a view coordinate system with `+X=right`, `+Y=up` and
        /// `+Z=forward`.
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

        /// Creates a right-handed view transform from a camera position, a
        /// facing direction and an up direction.
        ///
        /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
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

        /// Creates a left-handed view transform from a camera position, a focal
        /// point and an up direction.
        ///
        /// For a view coordinate system with `+X=right`, `+Y=up` and
        /// `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_at_lh(
            eye: Vector<3, $Wide, A>,
            center: Vector<3, $Wide, A>,
            up: Vector<3, $Wide, A>,
        ) -> Self {
            Self::look_to_lh(eye, (center - eye).normalize(), up)
        }

        /// Creates a right-handed view transform from a camera position, a
        /// focal point and an up direction.
        ///
        /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_at_rh(
            eye: Vector<3, $Wide, A>,
            center: Vector<3, $Wide, A>,
            up: Vector<3, $Wide, A>,
        ) -> Self {
            Self::look_to_rh(eye, (center - eye).normalize(), up)
        }

        /// Creates a left-handed perspective projection with `0..1` depth
        /// range.
        ///
        /// Useful to map the standard left-handed coordinate system into what
        /// WebGPU/Metal/Direct3D expect.
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

        /// Creates a right-handed perspective projection with `0..1` depth
        /// range.
        ///
        /// Useful to map the standard right-handed coordinate system into what
        /// WebGPU/Metal/Direct3D expect.
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
                Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, neg_depth_scale, -$Wide::ONE),
                Vector::<4, $Wide, A>::new(
                    $Wide::ZERO,
                    $Wide::ZERO,
                    neg_depth_scale * near_plane,
                    $Wide::ZERO,
                ),
            ])
        }

        /// Creates a right-handed perspective projection with `-1..1` depth
        /// range.
        ///
        /// Equivalent to the OpenGL [`gluPerspective`] function.
        ///
        /// [`gluPerspective`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
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

        /// Creates an infinite left-handed perspective projection with `0..1`
        /// depth range.
        ///
        /// Equivalent to `perspective_lh`, but with an infinite value for
        /// `far_plane`. The result is that points near `near_plane` have depth
        /// `0`, and as they move towards infinity the depth approaches `1`.
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

        /// Creates an infinite right-handed perspective projection with `0..1`
        /// depth range.
        ///
        /// Equivalent to `perspective_rh`, but with an infinite value for
        /// `far_plane`. The result is that points near `near_plane` have depth
        /// `0`, and as they move towards infinity the depth approaches `1`.
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

        /// Creates an infinite left-handed perspective projection with reversed
        /// `0..1` depth range.
        ///
        /// Equivalent to `perspective_infinite_lh`, but maps points at
        /// `near_plane` to depth `1` and points at infinity to depth `0`.
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

        /// Creates an infinite right-handed perspective projection with
        /// reversed `0..1` depth range.
        ///
        /// Equivalent to `perspective_infinite_rh`, but maps points at
        /// `near_plane` to depth `1` and points at infinity to depth `0`.
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

        /// Creates a left-handed perspective projection with `0..1` depth
        /// range.
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

        /// Creates a right-handed perspective projection with `0..1` depth
        /// range.
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

        /// Creates a right-handed perspective projection with `-1..1` depth
        /// range.
        ///
        /// Equivalent to the OpenGL [`glFrustum`] function.
        ///
        /// [`glFrustum`]: https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml
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

        /// Creates a left-handed orthographic projection with `0..1` depth
        /// range.
        ///
        /// Useful to map a left-handed coordinate system into what
        /// WebGPU/Metal/Direct3D expect.
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

        /// Creates a right-handed orthographic projection with `0..1` depth
        /// range.
        ///
        /// Useful to map a right-handed coordinate system into what
        /// WebGPU/Metal/Direct3D expect.
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
                Vector::<4, $Wide, A>::new($Wide::ZERO, $Wide::ZERO, neg_depth_recip, $Wide::ZERO),
                Vector::<4, $Wide, A>::new(
                    -(left + right) * width_recip,
                    -(top + bottom) * height_recip,
                    neg_depth_recip * near,
                    $Wide::ONE,
                ),
            ])
        }

        /// Creates a right-handed orthographic projection with `-1..1` depth
        /// range.
        ///
        /// Equivalent to the OpenGL [`glOrtho`] function.
        ///
        /// [`glOrtho`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml
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
                Vector::<4, $Wide, A>::new(translation_x, translation_y, translation_z, $Wide::ONE),
            ])
        }

        /// Returns the Euler angles forming `self` for the given Euler rotation
        /// order/sequence.
        ///
        /// The upper-left 3x3 matrix of `self` must not contain any
        /// non-rotation transformations. Otherwise the result is unspecified.
        #[inline]
        #[must_use]
        pub fn to_euler(&self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
            Matrix::<3, $Wide, A>::from_projective(self).to_euler(order)
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

/// Functionality for [SoA] (Structure of Arrays) float projective transforms.
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
impl<const N: usize, Wide, A: Alignment> Projective<N, Wide, A>
where
    Length<N>: TwoOrThree,
    Wide: WideFloat,
{
    items!(Wide, f32);
}

/// Functionality for [SoA] (Structure of Arrays) 2D float projective
/// transforms.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Projective<2, Wide, A>
where
    Wide: WideFloat,
{
    items_2!(Wide, f32);
}

/// Functionality for [SoA] (Structure of Arrays) 3D float projective
/// transforms.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Projective<3, Wide, A>
where
    Wide: WideFloat,
{
    items_3!(Wide, f32);
}

macro_rules! impl_items {
    ($Wide:ident, $T:ident) => {
        #[cfg(not(doc))]
        #[expect(private_bounds)]
        impl<const N: usize, A: Alignment> Projective<N, $Wide, A>
        where
            Length<N>: TwoOrThree,
        {
            items!($Wide, $T);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Projective<2, $Wide, A> {
            items_2!($Wide, $T);

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

            #[inline(always)]
            fn abs_backend(&self) -> Self {
                Self(self.0.abs())
            }

            #[inline(always)]
            fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: $Wide) -> bool {
                self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
                    && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
                    && self.z_axis.abs_diff_eq(other.z_axis, max_abs_diff)
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Projective<3, $Wide, A> {
            items_3!($Wide, $T);

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
            fn abs_backend(&self) -> Self {
                Self(self.0.abs())
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
impl_items!(f32x4, f32);
impl_items!(f32x8, f32);
impl_items!(f32x16, f32);
impl_items!(f64x2, f64);
impl_items!(f64x4, f64);
impl_items!(f64x8, f64);

#[cfg(test)]
mod tests {
    use crate::{
        Affine, EulerRot, Mat3, Mat4, Matrix, Proj2, Proj3, Projective, Unaligned, Vec2, Vec3,
        Vector,
        test_utils::{assert_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|Wide: WideFloat| {
            assert_test_eq!(Proj2::<Wide>::NAN, Projective(Mat3::<Wide>::NAN));
            assert_test_eq!(Proj3::<Wide>::NAN, Projective(Mat4::<Wide>::NAN));
        });
    }

    #[test]
    fn test_from_rotor() {
        todo!()
    }

    #[test]
    fn test_from_scale_rotation() {
        todo!()
    }

    #[test]
    fn test_from_rotation_translation() {
        todo!()
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        todo!()
    }

    #[test]
    fn test_is_nan() {
        for_types!(|Wide: WideFloat| {
            for [x, y, z, w] in random_iter::<[Wide; 4]>() {
                assert_test_eq!(
                    Proj2::from_rows(&[x, y, z].map(Vector::splat)).is_nan(),
                    x.is_nan() | y.is_nan() | z.is_nan()
                );
                assert_test_eq!(
                    Proj3::from_rows(&[x, y, z, w].map(Vector::splat)).is_nan(),
                    x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan()
                );
            }
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|Wide: WideFloat| {
            for [x, y, z, w] in random_iter::<[Wide; 4]>() {
                assert_test_eq!(
                    Proj2::from_rows(&[x, y, z].map(Vector::splat)).is_finite(),
                    x.is_finite() & y.is_finite() & z.is_finite()
                );
                assert_test_eq!(
                    Proj3::from_rows(&[x, y, z, w].map(Vector::splat)).is_finite(),
                    x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
                );
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for projective in random_iter::<Projective<N, Wide, Unaligned>>() {
                assert_test_eq_or_panic!(
                    projective.inverse(),
                    Projective::from_lane_fn(|lane| projective.lane(lane).inverse())
                );
            }
        });
    }

    // `try_inverse` is exluded on purpose.

    #[test]
    fn test_inverse_or() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for [projective, fallback] in random_iter::<[Projective<N, Wide, Unaligned>; 2]>() {
                assert_test_eq!(
                    projective.inverse_or(&fallback),
                    Projective::from_lane_fn(|lane| projective
                        .lane(lane)
                        .inverse_or(&fallback.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for projective in random_iter::<Projective<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    projective.inverse_or_zero(),
                    Projective::from_lane_fn(|lane| projective.lane(lane).inverse_or_zero())
                );
            }
        });
    }

    #[test]
    fn test_transform_point() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for (projective, point) in
                random_iter::<(Projective<N, Wide, Unaligned>, Vector<N, Wide, Unaligned>)>()
                    .flat_map(|(projective, point)| {
                        [
                            (projective, point),
                            (
                                Projective::from_affine(
                                    &Affine::<N, Wide, Unaligned>::from_projective(&projective),
                                ),
                                point,
                            ),
                        ]
                    })
            {
                assert_test_eq_or_panic!(
                    projective.transform_point(point),
                    Vector::from_lane_fn(|lane| projective
                        .lane(lane)
                        .transform_point(point.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_transform_vector() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for (projective, vector) in
                random_iter::<(Projective<N, Wide, Unaligned>, Vector<N, Wide, Unaligned>)>()
                    .flat_map(|(projective, vector)| {
                        [
                            (projective, vector),
                            (
                                Projective::from_affine(
                                    &Affine::<N, Wide, Unaligned>::from_projective(&projective),
                                ),
                                vector,
                            ),
                        ]
                    })
            {
                assert_test_eq_or_panic!(
                    projective.transform_vector(vector),
                    Vector::from_lane_fn(|lane| projective
                        .lane(lane)
                        .transform_vector(vector.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_project_point() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for (projective, point) in
                random_iter::<(Projective<N, Wide, Unaligned>, Vector<N, Wide, Unaligned>)>()
            {
                assert_test_eq!(
                    projective.project_point(point),
                    Vector::from_lane_fn(|lane| projective
                        .lane(lane)
                        .project_point(point.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for projective in random_iter::<Projective<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    projective.abs(),
                    Projective::from_lane_fn(|lane| projective.lane(lane).abs())
                );
            }
        });
    }

    #[test]
    fn test_to_scale_rotation() {
        todo!()
    }

    #[test]
    fn test_to_rotation_translation() {
        todo!()
    }

    #[test]
    fn test_to_scale_rotation_translation() {
        todo!()
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|N, Wide: WideFloat| {
            for ([a, b], max_abs_diff) in random_iter::<([Matrix<N, Wide, Unaligned>; 2], Wide)>() {
                assert_test_eq!(
                    a.abs_diff_eq(&b, max_abs_diff),
                    (0..LANES).all(|lane| a
                        .lane(lane)
                        .abs_diff_eq(&b.lane(lane), max_abs_diff.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_from_angle() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Proj2::<Wide>::from_angle(angle),
                    Proj2::from_lane_fn(|lane| Proj2::<T>::from_angle(angle.to_array()[lane])),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle() {
        for_types!(|Wide: WideFloat| {
            for (scale, angle) in random_iter::<(Vec2<Wide>, Wide)>() {
                let scale = scale.length().is_finite().select(scale, Vec2::ONE);

                assert_test_eq!(
                    Proj2::<Wide>::from_scale_angle(scale, angle),
                    Proj2::from_lane_fn(|lane| Proj2::<T>::from_scale_angle(
                        scale.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= (scale.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }

    #[test]
    fn test_from_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for (angle, translation) in random_iter::<(Wide, Vec2<Wide>)>() {
                assert_test_eq!(
                    Proj2::<Wide>::from_angle_translation(angle, translation),
                    Proj2::from_lane_fn(|lane| Proj2::<T>::from_angle_translation(
                        angle.to_array()[lane],
                        translation.lane(lane)
                    )),
                    abs <= angle.abs() * 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for (scale, angle, translation) in random_iter::<(Vec2<Wide>, Wide, Vec2<Wide>)>() {
                let scale = scale.length().is_finite().select(scale, Vec2::ONE);

                assert_test_eq!(
                    Proj2::<Wide>::from_scale_angle_translation(scale, angle, translation),
                    Proj2::from_lane_fn(|lane| Proj2::<T>::from_scale_angle_translation(
                        scale.lane(lane),
                        angle.to_array()[lane],
                        translation.lane(lane)
                    )),
                    abs <= (scale.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle() {
        for_types!(|Wide: WideFloat| {
            for projective in random_iter::<(Vec2<Wide>, Wide, Vec2<Wide>)>()
                .map(|(scale, angle, translation)| {
                    Proj2::<Wide>::from_scale_angle_translation(scale, angle, translation)
                })
                .chain(random_iter())
            {
                assert_test_eq_or_panic!(
                    projective.to_scale_angle(),
                    (
                        Vec2::from_lane_fn(|lane| projective.lane(lane).to_scale_angle().0),
                        Wide::new(std::array::from_fn(|lane| projective
                            .lane(lane)
                            .to_scale_angle()
                            .1))
                    ),
                    abs <= (
                        projective.to_scale_angle().0.abs() * Wide::splat(1e-4) + Wide::splat(1e-3),
                        projective.to_scale_angle().1.abs() * 1e-4 + 1e-3
                    )
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for projective in random_iter::<(Vec2<Wide>, Wide, Vec2<Wide>)>()
                .map(|(scale, angle, translation)| {
                    Proj2::<Wide>::from_scale_angle_translation(scale, angle, translation)
                })
                .chain(random_iter())
            {
                assert_test_eq_or_panic!(
                    projective.to_scale_angle_translation(),
                    (
                        Vec2::from_lane_fn(|lane| projective
                            .lane(lane)
                            .to_scale_angle_translation()
                            .0),
                        Wide::new(std::array::from_fn(|lane| projective
                            .lane(lane)
                            .to_scale_angle_translation()
                            .1)),
                        Vec2::from_lane_fn(|lane| projective
                            .lane(lane)
                            .to_scale_angle_translation()
                            .2)
                    ),
                    abs <= (
                        projective.to_scale_angle_translation().0.abs() * Wide::splat(1e-4)
                            + Wide::splat(1e-3),
                        projective.to_scale_angle_translation().1.abs() * 1e-4 + 1e-3,
                        Vector::ZERO
                    )
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_xy() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Proj3::<Wide>::from_rotation_xy(angle),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::from_rotation_xy(
                        angle.to_array()[lane]
                    )),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_xz() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Proj3::<Wide>::from_rotation_xz(angle),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::from_rotation_xz(
                        angle.to_array()[lane]
                    )),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_yz() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Proj3::<Wide>::from_rotation_yz(angle),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::from_rotation_yz(
                        angle.to_array()[lane]
                    )),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
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
                let angle = condition.select(angle, Wide::ONE);

                assert_test_eq_or_panic!(
                    Proj3::<Wide>::from_axis_angle(axis, angle),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::from_axis_angle(
                        axis.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= Proj3::<Wide>::from_axis_angle(axis, angle).abs()
                        * axis.length().max(Wide::ONE)
                        * angle.abs().max(Wide::ONE)
                        * Wide::splat(1e-4)
                        + Proj3::from_row_array(&[Wide::splat(1e-3); 16]),
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
                        Proj3::<Wide>::from_euler(order, a, b, c),
                        Proj3::from_lane_fn(|lane| Proj3::<T>::from_euler(
                            order,
                            a.to_array()[lane],
                            b.to_array()[lane],
                            c.to_array()[lane]
                        )),
                        abs <= a.abs().max(b.abs()).max(c.abs()) * 1e-4,
                        0.0 = -0.0
                    );
                }
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|Wide: WideFloat| {
            for [eye, dir, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|[eye, dir, up]| [[eye, dir, up], [eye, dir.normalize(), up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::look_to_lh(eye, dir, up),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::look_to_lh(
                        eye.lane(lane),
                        dir.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|Wide: WideFloat| {
            for [eye, dir, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|[eye, dir, up]| [[eye, dir, up], [eye, dir.normalize(), up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::look_to_rh(eye, dir, up),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::look_to_rh(
                        eye.lane(lane),
                        dir.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|Wide: WideFloat| {
            for [eye, center, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|[eye, center, up]| [[eye, center, up], [eye, center, up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::look_at_lh(eye, center, up),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::look_at_lh(
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
                .flat_map(|[eye, center, up]| [[eye, center, up], [eye, center, up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::look_at_rh(eye, center, up),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::look_at_rh(
                        eye.lane(lane),
                        center.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_perspective_lh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, far_plane, aspect_ratio] in random_iter::<[Wide; 4]>() {
                let [vertical_fov, near_plane, far_plane, aspect_ratio] =
                    [vertical_fov, near_plane, far_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).select(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Proj3::<Wide>::perspective_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    ),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::perspective_lh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    )),
                    abs <= Proj3::<Wide>::perspective_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Proj3::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_rh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, far_plane, aspect_ratio] in random_iter::<[Wide; 4]>() {
                let [vertical_fov, near_plane, far_plane, aspect_ratio] =
                    [vertical_fov, near_plane, far_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).select(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Proj3::<Wide>::perspective_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    ),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::perspective_rh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    )),
                    abs <= Proj3::<Wide>::perspective_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Proj3::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_rh_gl() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, far_plane, aspect_ratio] in random_iter::<[Wide; 4]>() {
                let [vertical_fov, near_plane, far_plane, aspect_ratio] =
                    [vertical_fov, near_plane, far_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).select(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Proj3::<Wide>::perspective_rh_gl(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    ),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::perspective_rh_gl(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    )),
                    abs <= Proj3::<Wide>::perspective_rh_gl(
                        vertical_fov,
                        aspect_ratio,
                        near_plane,
                        far_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Proj3::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_lh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, aspect_ratio] in random_iter::<[Wide; 3]>() {
                let [vertical_fov, near_plane, aspect_ratio] =
                    [vertical_fov, near_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).select(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Proj3::<Wide>::perspective_infinite_lh(vertical_fov, aspect_ratio, near_plane),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::perspective_infinite_lh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane]
                    )),
                    abs <= Proj3::<Wide>::perspective_infinite_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Proj3::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_rh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, aspect_ratio] in random_iter::<[Wide; 3]>() {
                let [vertical_fov, near_plane, aspect_ratio] =
                    [vertical_fov, near_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).select(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Proj3::<Wide>::perspective_infinite_rh(vertical_fov, aspect_ratio, near_plane),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::perspective_infinite_rh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane]
                    )),
                    abs <= Proj3::<Wide>::perspective_infinite_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Proj3::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_lh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, aspect_ratio] in random_iter::<[Wide; 3]>() {
                let [vertical_fov, near_plane, aspect_ratio] =
                    [vertical_fov, near_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).select(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Proj3::<Wide>::perspective_infinite_reverse_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::perspective_infinite_reverse_lh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane]
                    )),
                    abs <= Proj3::<Wide>::perspective_infinite_reverse_lh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Proj3::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_perspective_infinite_reverse_rh() {
        for_types!(|Wide: WideFloat| {
            for [vertical_fov, near_plane, aspect_ratio] in random_iter::<[Wide; 3]>() {
                let [vertical_fov, near_plane, aspect_ratio] =
                    [vertical_fov, near_plane, aspect_ratio]
                        .map(|x| (x.is_finite() & x.abs().simd_lt(1e3)).select(x, Wide::ONE));

                assert_test_eq_or_panic!(
                    Proj3::<Wide>::perspective_infinite_reverse_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    ),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::perspective_infinite_reverse_rh(
                        vertical_fov.to_array()[lane],
                        aspect_ratio.to_array()[lane],
                        near_plane.to_array()[lane]
                    )),
                    abs <= Proj3::<Wide>::perspective_infinite_reverse_rh(
                        vertical_fov,
                        aspect_ratio,
                        near_plane
                    )
                    .abs()
                        * Wide::splat(1e-3)
                        + Proj3::from_row_array(&[Wide::splat(1e-3); 16])
                );
            }
        });
    }

    #[test]
    fn test_frustum_lh() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near_plane, far_plane] in random_iter::<[Wide; 6]>()
                .flat_map(|[left, right, bottom, top, near_plane, far_plane]| {
                    [
                        [left, right, bottom, top, near_plane, far_plane],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near_plane.min(far_plane),
                            near_plane.max(far_plane),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::frustum_lh(left, right, bottom, top, near_plane, far_plane),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::frustum_lh(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_frustum_rh() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near_plane, far_plane] in random_iter::<[Wide; 6]>()
                .flat_map(|[left, right, bottom, top, near_plane, far_plane]| {
                    [
                        [left, right, bottom, top, near_plane, far_plane],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near_plane.min(far_plane),
                            near_plane.max(far_plane),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::frustum_rh(left, right, bottom, top, near_plane, far_plane),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::frustum_rh(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_frustum_rh_gl() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near_plane, far_plane] in random_iter::<[Wide; 6]>()
                .flat_map(|[left, right, bottom, top, near_plane, far_plane]| {
                    [
                        [left, right, bottom, top, near_plane, far_plane],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near_plane.min(far_plane),
                            near_plane.max(far_plane),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::frustum_rh_gl(left, right, bottom, top, near_plane, far_plane),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::frustum_rh_gl(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near_plane.to_array()[lane],
                        far_plane.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_orthographic_lh() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near, far] in
                random_iter::<[Wide; 6]>().flat_map(|[left, right, bottom, top, near, far]| {
                    [
                        [left, right, bottom, top, near, far],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near.min(far),
                            near.max(far),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::orthographic_lh(left, right, bottom, top, near, far),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::orthographic_lh(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near.to_array()[lane],
                        far.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_orthographic_rh() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near, far] in
                random_iter::<[Wide; 6]>().flat_map(|[left, right, bottom, top, near, far]| {
                    [
                        [left, right, bottom, top, near, far],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near.min(far),
                            near.max(far),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::orthographic_rh(left, right, bottom, top, near, far),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::orthographic_rh(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near.to_array()[lane],
                        far.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_orthographic_rh_gl() {
        for_types!(|Wide: WideFloat| {
            for [left, right, bottom, top, near, far] in
                random_iter::<[Wide; 6]>().flat_map(|[left, right, bottom, top, near, far]| {
                    [
                        [left, right, bottom, top, near, far],
                        [
                            left.min(right),
                            left.max(right),
                            bottom.min(top),
                            bottom.max(top),
                            near.min(far),
                            near.max(far),
                        ],
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    Proj3::<Wide>::orthographic_rh_gl(left, right, bottom, top, near, far),
                    Proj3::from_lane_fn(|lane| Proj3::<T>::orthographic_rh_gl(
                        left.to_array()[lane],
                        right.to_array()[lane],
                        bottom.to_array()[lane],
                        top.to_array()[lane],
                        near.to_array()[lane],
                        far.to_array()[lane]
                    ))
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|Wide: WideFloat| {
            for order in EulerRot::values() {
                for projective in random_iter::<[Wide; 3]>()
                    .map(|[a, b, c]| Proj3::<Wide>::from_euler(order, a, b, c))
                    .chain(random_iter())
                {
                    assert_test_eq_or_panic!(
                        projective.to_euler(order),
                        (
                            Wide::new(std::array::from_fn(|lane| projective
                                .lane(lane)
                                .to_euler(order)
                                .0)),
                            Wide::new(std::array::from_fn(|lane| projective
                                .lane(lane)
                                .to_euler(order)
                                .1)),
                            Wide::new(std::array::from_fn(|lane| projective
                                .lane(lane)
                                .to_euler(order)
                                .2))
                        ),
                        abs <= (Wide::splat(1e-4), Wide::splat(1e-4), Wide::splat(1e-4)),
                        0.0 = -0.0
                    );
                }
            }
        });
    }
}
