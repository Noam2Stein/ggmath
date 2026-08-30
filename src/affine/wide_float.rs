use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Affine, Alignment, EulerRot, Length, Matrix, Projective, Quaternion, SupportedLength, Vector,
    length::TwoOrThree,
    utils::{specialize, specialize_23},
};

macro_rules! items {
    ($Wide:ident) => {
        /// An affine transform with all elements set to NaN (Not a Number).
        pub const NAN: Self =
            Self::from_matrix_translation(&Matrix::<N, $Wide, A>::NAN, Vector::<N, $Wide, A>::NAN);

        /// Creates an affine transform from a projective transform.
        ///
        /// This assumes `projective` does not contain projections.
        #[inline]
        #[must_use]
        #[track_caller]
        #[expect(private_bounds)]
        pub fn from_projective(projective: &Projective<N, $Wide, A>) -> Self
        where
            Length<N>: TwoOrThree,
        {
            specialize_23!(Affine::<N, $Wide, A>::from_projective_backend(projective))
        }

        /// Returns `true` if any element is NaN.
        #[inline]
        #[must_use]
        pub fn is_nan(&self) -> $Wide {
            self.matrix.is_nan() | self.translation.is_nan()
        }

        /// Returns `true` if all elements are neither infinite nor NaN.
        #[inline]
        #[must_use]
        pub fn is_finite(&self) -> $Wide {
            self.matrix.is_finite() & self.translation.is_finite()
        }

        /// Returns the inverse of `self`.
        ///
        /// If `self` is not invertable the result is unspecified.
        #[inline]
        #[must_use]
        pub fn inverse(&self) -> Self {
            let matrix = self.matrix.inverse();
            let translation = -self.translation * matrix;

            Self::from_matrix_translation(&matrix, translation)
        }

        // `try_inverse` is exluded on purpose. It would not be useful because
        // it would only return `Some` if all lanes succeed.

        /// For each lane, returns the inverse of `self` or `fallback` if `self`
        /// is not invertable.
        ///
        /// The fallback is only applied for invalid lanes. Other lanes are not
        /// affected.
        #[inline]
        #[must_use]
        pub fn inverse_or(&self, fallback: &Self) -> Self {
            specialize!(Affine::<N, $Wide, A>::inverse_or_backend(self, fallback))
        }

        /// For each lane, returns the inverse of `self` or the zero transform
        /// if `self` is not invertable.
        ///
        /// The fallback is only applied for invalid lanes. Other lanes are not
        /// affected.
        #[inline]
        #[must_use]
        pub fn inverse_or_zero(&self) -> Self {
            specialize!(Affine::<N, $Wide, A>::inverse_or_zero_backend(self))
        }

        /// Returns `true` if the absolute difference of all elements between
        /// `self` and `other` is less than or equal to `max_abs_diff` for all
        /// lanes.
        ///
        /// This can be used to compare two affines that should be equal, but
        /// may have a slight difference due to operations having rounding
        /// errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: $Wide) -> bool {
            self.matrix.abs_diff_eq(&other.matrix, max_abs_diff)
                && self
                    .translation
                    .abs_diff_eq(other.translation, max_abs_diff)
        }
    };
}

macro_rules! items_2 {
    ($Wide:ident) => {
        /// Creates an affine transform containing a rotation from an `angle`
        /// (in radians) rotating `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_angle(angle: $Wide) -> Self {
            Self::from_matrix(&Matrix::<2, $Wide, A>::from_angle(angle))
        }

        /// Creates an affine transform containing a rotation of `angle` (in
        /// radians) and `translation`.
        ///
        /// This rotates `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_angle_translation(angle: $Wide, translation: Vector<2, $Wide, A>) -> Self {
            Self::from_matrix_translation(&Matrix::<2, $Wide, A>::from_angle(angle), translation)
        }

        /// Creates an affine transform containing a non-uniform `scale` and
        /// rotation of `angle` (in radians).
        ///
        /// This rotates `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_scale_angle(scale: Vector<2, $Wide, A>, angle: $Wide) -> Self {
            Self::from_matrix(&Matrix::<2, $Wide, A>::from_scale_angle(scale, angle))
        }

        /// Creates an affine transform containing a non-uniform `scale`,
        /// rotation of `angle` (in radians) and `translation`.
        ///
        /// This rotates `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_scale_angle_translation(
            scale: Vector<2, $Wide, A>,
            angle: $Wide,
            translation: Vector<2, $Wide, A>,
        ) -> Self {
            Self::from_matrix_translation(
                &Matrix::<2, $Wide, A>::from_scale_angle(scale, angle),
                translation,
            )
        }

        /// Takes the `N+1`x`N` affine transform part of an `N+1`x`N+1`
        /// homogeneous transformation matrix, removing the last column.
        ///
        /// This assumes `homogeneous` does not contain projections.
        #[inline]
        #[must_use]
        pub fn from_homogeneous(homogeneous: &Matrix<3, $Wide, A>) -> Self {
            Self::from_rows(&[
                homogeneous.x_axis.truncate(),
                homogeneous.y_axis.truncate(),
                homogeneous.z_axis.truncate(),
            ])
        }

        /// For each lane, returns the `scale` and `angle` of `self`.
        ///
        /// `self` must be reversible and not contain shearing. Otherwise the
        /// result is unspecified.
        #[inline]
        #[must_use]
        pub fn to_scale_angle(&self) -> (Vector<2, $Wide, A>, $Wide) {
            self.matrix.to_scale_angle()
        }

        /// For each lane, returns the `scale`, `angle` and `translation` of
        /// `self`.
        ///
        /// `self` must be reversible and not contain shearing. Otherwise the
        /// result is unspecified.
        #[inline]
        #[must_use]
        pub fn to_scale_angle_translation(
            &self,
        ) -> (Vector<2, $Wide, A>, $Wide, Vector<2, $Wide, A>) {
            let (scale, angle) = self.matrix.to_scale_angle();
            (scale, angle, self.translation)
        }
    };
}

macro_rules! items_3 {
    ($Wide:ident) => {
        /// Creates an affine transform containing a rotation from an `angle`
        /// (in radians) rotating `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xy(angle: $Wide) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::from_rotation_xy(angle))
        }

        /// Creates an affine transform containing a rotation from an `angle`
        /// (in radians) rotating `+X` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xz(angle: $Wide) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::from_rotation_xz(angle))
        }

        /// Creates an affine transform containing a rotation from an `angle`
        /// (in radians) rotating `+Y` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_yz(angle: $Wide) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::from_rotation_yz(angle))
        }

        /// Creates an affine transform containing a 3D rotation from a
        /// quaternion.
        #[inline]
        #[must_use]
        pub fn from_quat(quat: Quaternion<$Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::from_quat(quat))
        }

        /// Creates an affine transform containing a rotation from a rotation
        /// `axis` and `angle` (in radians).
        ///
        /// `axis` must be normalized. Otherwise the result is unspecified.
        #[inline]
        #[must_use]
        pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::from_axis_angle(axis, angle))
        }

        /// Creates an affine transform containing a rotation from an Euler
        /// rotation order/sequence and angles (in radians).
        #[inline]
        #[must_use]
        pub fn from_euler(order: EulerRot, a: $Wide, b: $Wide, c: $Wide) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::from_euler(order, a, b, c))
        }

        /// Creates an affine transform containing a non-uniform `scale` and a
        /// 3D `rotation`.
        #[inline]
        #[must_use]
        pub fn from_scale_rotation(
            scale: Vector<3, $Wide, A>,
            rotation: Quaternion<$Wide, A>,
        ) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::from_scale_rotation(scale, rotation))
        }

        /// Creates an affine transform containing a 3D `rotation` and
        /// `translation`.
        #[inline]
        #[must_use]
        pub fn from_rotation_translation(
            rotation: Quaternion<$Wide, A>,
            translation: Vector<3, $Wide, A>,
        ) -> Self {
            Self::from_matrix_translation(&Matrix::<3, $Wide, A>::from_quat(rotation), translation)
        }

        /// Creates an affine transform containing a non-uniform `scale`, a 3D
        /// `rotation` and `translation`.
        #[inline]
        #[must_use]
        pub fn from_scale_rotation_translation(
            scale: Vector<3, $Wide, A>,
            rotation: Quaternion<$Wide, A>,
            translation: Vector<3, $Wide, A>,
        ) -> Self {
            Self::from_matrix_translation(
                &Matrix::<3, $Wide, A>::from_scale_rotation(scale, rotation),
                translation,
            )
        }

        /// Takes the `N+1`x`N` affine transform part of an `N+1`x`N+1`
        /// homogeneous transformation matrix, removing the last column.
        ///
        /// This assumes `homogeneous` does not contain projections.
        #[inline]
        #[must_use]
        pub fn from_homogeneous(homogeneous: &Matrix<4, $Wide, A>) -> Self {
            Self::from_rows(&[
                homogeneous.x_axis.truncate(),
                homogeneous.y_axis.truncate(),
                homogeneous.z_axis.truncate(),
                homogeneous.w_axis.truncate(),
            ])
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
                Vector::<3, $Wide, A>::new(right.x, up.x, forward.x),
                Vector::<3, $Wide, A>::new(right.y, up.y, forward.y),
                Vector::<3, $Wide, A>::new(right.z, up.z, forward.z),
                Vector::<3, $Wide, A>::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward)),
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
                Vector::<3, $Wide, A>::new(right.x, up.x, -forward.x),
                Vector::<3, $Wide, A>::new(right.y, up.y, -forward.y),
                Vector::<3, $Wide, A>::new(right.z, up.z, -forward.z),
                Vector::<3, $Wide, A>::new(-eye.dot(right), -eye.dot(up), eye.dot(forward)),
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

        /// Returns the Euler angles forming `self` for the given Euler rotation
        /// order/sequence.
        ///
        /// `self` must not contain any non-rotation transformations, excluding
        /// translation. Otherwise the result is unspecified.
        #[inline]
        #[must_use]
        pub fn to_euler(&self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
            self.matrix.to_euler(order)
        }

        /// For each lane, returns the `scale` and `rotation` of `self`.
        ///
        /// `self` must be reversible and not contain shearing. Otherwise the
        /// result is unspecified.
        #[inline]
        #[must_use]
        pub fn to_scale_rotation(&self) -> (Vector<3, $Wide, A>, Quaternion<$Wide, A>) {
            self.matrix.to_scale_rotation()
        }

        /// For each lane, returns the `scale`, `rotation` and `translation` of
        /// `self`.
        ///
        /// `self` must be reversible and not contain shearing. Otherwise the
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
            let (scale, rotation) = self.matrix.to_scale_rotation();
            (scale, rotation, self.translation)
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

/// Functionality for [SoA] (Structure of Arrays) float affine transforms.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<const N: usize, Wide, A: Alignment> Affine<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideFloat,
{
    items!(Wide);
}

/// Functionality for [SoA] (Structure of Arrays) 2D float affine transforms.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Affine<2, Wide, A>
where
    Wide: WideFloat,
{
    items_2!(Wide);
}

/// Functionality for [SoA] (Structure of Arrays) 3D float affine transforms.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Affine<3, Wide, A>
where
    Wide: WideFloat,
{
    items_3!(Wide);
}

macro_rules! impl_items {
    ($Wide:ident) => {
        #[cfg(not(doc))]
        impl<const N: usize, A: Alignment> Affine<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            items!($Wide);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Affine<2, $Wide, A> {
            items_2!($Wide);

            #[inline(always)]
            #[track_caller]
            fn from_projective_backend(projective: &Projective<2, $Wide, A>) -> Self {
                Self::from_rows(&[
                    projective.x_axis.truncate(),
                    projective.y_axis.truncate(),
                    projective.z_axis.truncate(),
                ])
            }

            #[inline(always)]
            fn inverse_or_backend(&self, fallback: &Self) -> Self {
                let (matrix, determinant) = self.matrix.inverse_and_determinant();
                let translation = -self.translation * matrix;

                determinant.simd_eq($Wide::ZERO).select(
                    *fallback,
                    Self::from_matrix_translation(&matrix, translation),
                )
            }

            #[inline(always)]
            fn inverse_or_zero_backend(&self) -> Self {
                let (matrix, determinant) = self.matrix.inverse_and_determinant();
                let translation = -self.translation * matrix;

                let non_fallback_mask = determinant.simd_ne($Wide::ZERO);
                Self::from_row_array(&[
                    matrix.x_axis.x & non_fallback_mask,
                    matrix.x_axis.y & non_fallback_mask,
                    matrix.y_axis.x & non_fallback_mask,
                    matrix.y_axis.y & non_fallback_mask,
                    translation.x & non_fallback_mask,
                    translation.y & non_fallback_mask,
                ])
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Affine<3, $Wide, A> {
            items_3!($Wide);

            #[inline(always)]
            #[track_caller]
            fn from_projective_backend(projective: &Projective<3, $Wide, A>) -> Self {
                Self::from_rows(&[
                    projective.x_axis.truncate(),
                    projective.y_axis.truncate(),
                    projective.z_axis.truncate(),
                    projective.w_axis.truncate(),
                ])
            }

            #[inline(always)]
            fn inverse_or_backend(&self, fallback: &Self) -> Self {
                let (matrix, determinant) = self.matrix.inverse_and_determinant();
                let translation = -self.translation * matrix;

                determinant.simd_eq($Wide::ZERO).select(
                    *fallback,
                    Self::from_matrix_translation(&matrix, translation),
                )
            }

            #[inline(always)]
            fn inverse_or_zero_backend(&self) -> Self {
                let (matrix, determinant) = self.matrix.inverse_and_determinant();
                let translation = -self.translation * matrix;

                let non_fallback_mask = determinant.simd_ne($Wide::ZERO);
                Self::from_row_array(&[
                    matrix.x_axis.x & non_fallback_mask,
                    matrix.x_axis.y & non_fallback_mask,
                    matrix.x_axis.z & non_fallback_mask,
                    matrix.y_axis.x & non_fallback_mask,
                    matrix.y_axis.y & non_fallback_mask,
                    matrix.y_axis.z & non_fallback_mask,
                    matrix.z_axis.x & non_fallback_mask,
                    matrix.z_axis.y & non_fallback_mask,
                    matrix.z_axis.z & non_fallback_mask,
                    translation.x & non_fallback_mask,
                    translation.y & non_fallback_mask,
                    translation.z & non_fallback_mask,
                ])
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Affine<4, $Wide, A> {
            #[inline(always)]
            fn inverse_or_backend(&self, fallback: &Self) -> Self {
                let (matrix, determinant) = self.matrix.inverse_and_determinant();
                let translation = -self.translation * matrix;

                determinant.simd_eq($Wide::ZERO).select(
                    *fallback,
                    Self::from_matrix_translation(&matrix, translation),
                )
            }

            #[inline(always)]
            fn inverse_or_zero_backend(&self) -> Self {
                let (matrix, determinant) = self.matrix.inverse_and_determinant();
                let translation = -self.translation * matrix;

                let non_fallback_mask = determinant.simd_ne($Wide::ZERO);
                Self::from_row_array(&[
                    matrix.x_axis.x & non_fallback_mask,
                    matrix.x_axis.y & non_fallback_mask,
                    matrix.x_axis.z & non_fallback_mask,
                    matrix.x_axis.w & non_fallback_mask,
                    matrix.y_axis.x & non_fallback_mask,
                    matrix.y_axis.y & non_fallback_mask,
                    matrix.y_axis.z & non_fallback_mask,
                    matrix.y_axis.w & non_fallback_mask,
                    matrix.z_axis.x & non_fallback_mask,
                    matrix.z_axis.y & non_fallback_mask,
                    matrix.z_axis.z & non_fallback_mask,
                    matrix.z_axis.w & non_fallback_mask,
                    matrix.w_axis.x & non_fallback_mask,
                    matrix.w_axis.y & non_fallback_mask,
                    matrix.w_axis.z & non_fallback_mask,
                    matrix.w_axis.w & non_fallback_mask,
                    translation.x & non_fallback_mask,
                    translation.y & non_fallback_mask,
                    translation.z & non_fallback_mask,
                    translation.w & non_fallback_mask,
                ])
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
    extern crate std;

    use wide::f32x4;

    use crate::{
        Affine, Affine2, Affine3, EulerRot, Mat3, Mat4, Matrix, Projective, Quat, Unaligned, Vec2,
        Vec3, Vector,
        test_utils::{assert_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, Wide: WideFloat| {
            assert_test_eq!(
                Affine::<N, Wide, Unaligned>::NAN,
                Affine::from_matrix_translation(
                    &Matrix::<N, Wide, Unaligned>::NAN,
                    Vector::<N, Wide, Unaligned>::NAN
                )
            );
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|N: TwoOrThree| {
            for projective in random_iter::<Projective<N, f32x4, Unaligned>>() {
                assert_test_eq_or_panic!(
                    Affine::<N, f32x4, Unaligned>::from_projective(&projective),
                    Affine::from_lane_fn(|lane| Affine::<N, f32, Unaligned>::from_projective(
                        &projective.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|N, Wide: WideFloat| {
            for affine in random_iter::<Affine<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    affine.is_nan(),
                    affine.matrix.is_nan() | affine.translation.is_nan()
                );
            }
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|N, Wide: WideFloat| {
            for affine in random_iter::<Affine<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    affine.is_finite(),
                    affine.matrix.is_finite() & affine.translation.is_finite()
                );
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|N, Wide: WideFloat| {
            for affine in random_iter::<Affine<N, Wide, Unaligned>>() {
                assert_test_eq_or_panic!(
                    affine.inverse(),
                    Affine::from_lane_fn(|lane| affine.lane(lane).inverse())
                );
            }
        });
    }

    // `try_inverse` is exluded on purpose.

    #[test]
    fn test_inverse_or() {
        for_types!(|N, Wide: WideFloat| {
            for [affine, fallback] in random_iter::<[Affine<N, Wide, Unaligned>; 2]>() {
                assert_test_eq_or_panic!(
                    affine.inverse_or(&fallback),
                    Affine::from_lane_fn(|lane| affine.lane(lane).inverse_or(&fallback.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_types!(|N, Wide: WideFloat| {
            for affine in random_iter::<Affine<N, Wide, Unaligned>>() {
                assert_test_eq_or_panic!(
                    affine.inverse_or_zero(),
                    Affine::from_lane_fn(|lane| affine.lane(lane).inverse_or_zero())
                );
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|N, Wide: WideFloat| {
            for ([a, b], max_abs_diff) in random_iter::<([Affine<N, Wide, Unaligned>; 2], Wide)>() {
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
                    Affine2::<Wide>::from_angle(angle),
                    Affine2::from_lane_fn(|lane| Affine2::<T>::from_angle(angle.to_array()[lane])),
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
                let condition = scale.length().is_finite();
                let scale = condition.select(scale, Vec2::ONE);
                let angle = condition.select(angle, Wide::ONE);

                assert_test_eq!(
                    Affine2::<Wide>::from_scale_angle(scale, angle),
                    Affine2::from_lane_fn(|lane| Affine2::<T>::from_scale_angle(
                        scale.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= (scale.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for (angle, translation) in random_iter::<(Wide, Vec2<Wide>)>() {
                assert_test_eq!(
                    Affine2::<Wide>::from_angle_translation(angle, translation),
                    Affine2::from_lane_fn(|lane| Affine2::<T>::from_angle_translation(
                        angle.to_array()[lane],
                        translation.lane(lane)
                    )),
                    abs <= (angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for (scale, angle, translation) in random_iter::<(Vec2<Wide>, Wide, Vec2<Wide>)>() {
                let condition = scale.length().is_finite();
                let scale = condition.select(scale, Vec2::ONE);
                let angle = condition.select(angle, Wide::ONE);
                let translation = condition.select(translation, Vec2::ONE);

                assert_test_eq!(
                    Affine2::<Wide>::from_scale_angle_translation(scale, angle, translation),
                    Affine2::from_lane_fn(|lane| Affine2::<T>::from_scale_angle_translation(
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
    fn test_from_homogeneous() {
        for homogeneous in random_iter::<Mat3<f32x4>>() {
            assert_test_eq_or_panic!(
                Affine2::<f32x4>::from_homogeneous(&homogeneous),
                Affine::from_lane_fn(|lane| Affine2::<f32>::from_homogeneous(
                    &homogeneous.lane(lane)
                ))
            );
        }
        for homogeneous in random_iter::<Mat4<f32x4>>() {
            assert_test_eq_or_panic!(
                Affine3::<f32x4>::from_homogeneous(&homogeneous),
                Affine::from_lane_fn(|lane| Affine3::<f32>::from_homogeneous(
                    &homogeneous.lane(lane)
                ))
            );
        }
    }

    #[test]
    fn test_to_scale_angle() {
        for_types!(|Wide: WideFloat| {
            for affine in random_iter::<Affine2<Wide>>().chain(
                random_iter::<(Vec2<Wide>, Wide, Vec2<Wide>)>().map(
                    |(scale, angle, translation)| {
                        Affine2::<Wide>::from_scale_angle_translation(scale, angle, translation)
                    },
                ),
            ) {
                assert_test_eq_or_panic!(
                    affine.to_scale_angle(),
                    (
                        Vector::from_lane_fn(|lane| affine.lane(lane).to_scale_angle().0),
                        Wide::new(core::array::from_fn(|lane| affine
                            .lane(lane)
                            .to_scale_angle()
                            .1)),
                    ),
                    abs <= (Vector::ZERO, Wide::splat(1e-3))
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle_translation() {
        for_types!(|Wide: WideFloat| {
            for affine in random_iter::<Affine2<Wide>>().chain(
                random_iter::<(Vec2<Wide>, Wide, Vec2<Wide>)>().map(
                    |(scale, angle, translation)| {
                        Affine2::<Wide>::from_scale_angle_translation(scale, angle, translation)
                    },
                ),
            ) {
                assert_test_eq_or_panic!(
                    affine.to_scale_angle_translation(),
                    (
                        Vector::from_lane_fn(|lane| affine
                            .lane(lane)
                            .to_scale_angle_translation()
                            .0),
                        Wide::new(core::array::from_fn(|lane| affine
                            .lane(lane)
                            .to_scale_angle_translation()
                            .1)),
                        Vector::from_lane_fn(|lane| affine
                            .lane(lane)
                            .to_scale_angle_translation()
                            .2)
                    ),
                    abs <= (Vector::ZERO, Wide::splat(1e-3), Vector::ZERO)
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_xy() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                assert_test_eq!(
                    Affine3::<Wide>::from_rotation_xy(angle),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::from_rotation_xy(
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
                    Affine3::<Wide>::from_rotation_xz(angle),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::from_rotation_xz(
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
                    Affine3::<Wide>::from_rotation_yz(angle),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::from_rotation_yz(
                        angle.to_array()[lane]
                    )),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_quat() {
        for_types!(|Wide: WideFloat| {
            for quat in random_iter::<Quat<Wide>>().flat_map(|quat| [quat, quat.normalize()]) {
                assert_test_eq_or_panic!(
                    Affine3::<Wide>::from_quat(quat),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::from_quat(quat.lane(lane)))
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
                    Affine3::<Wide>::from_axis_angle(axis, angle),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::from_axis_angle(
                        axis.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= Affine3::from_matrix(
                        &(Mat3::<Wide>::from_axis_angle(axis, angle).abs()
                            * axis.length().max(Wide::ONE)
                            * angle.abs().max(Wide::ONE)
                            * Wide::splat(1e-4)
                            + Mat3::from_row_array(&[Wide::splat(1e-3); 9]))
                    ),
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
                        Affine3::<Wide>::from_euler(order, a, b, c),
                        Affine3::from_lane_fn(|lane| Affine3::<T>::from_euler(
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
    fn test_from_scale_rotation() {
        for_types!(|Wide: WideFloat| {
            for (scale, rotation) in random_iter::<(Vec3<Wide>, Quat<Wide>)>()
                .flat_map(|(scale, quat)| [(scale, quat), (scale, quat.normalize())])
            {
                assert_test_eq_or_panic!(
                    Affine3::<Wide>::from_scale_rotation(scale, rotation),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::from_scale_rotation(
                        scale.lane(lane),
                        rotation.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_translation() {
        for_types!(|Wide: WideFloat| {
            for (rotation, translation) in
                random_iter::<(Quat<Wide>, Vec3<Wide>)>().flat_map(|(rotation, translation)| {
                    [(rotation, translation), (rotation.normalize(), translation)]
                })
            {
                assert_test_eq_or_panic!(
                    Affine3::<Wide>::from_rotation_translation(rotation, translation),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::from_rotation_translation(
                        rotation.lane(lane),
                        translation.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        for_types!(|Wide: WideFloat| {
            for (scale, rotation, translation) in
                random_iter::<(Vec3<Wide>, Quat<Wide>, Vec3<Wide>)>().flat_map(
                    |(scale, rotation, translation)| {
                        [
                            (scale, rotation, translation),
                            (scale, rotation.normalize(), translation),
                        ]
                    },
                )
            {
                assert_test_eq_or_panic!(
                    Affine3::<Wide>::from_scale_rotation_translation(scale, rotation, translation),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::from_scale_rotation_translation(
                        scale.lane(lane),
                        rotation.lane(lane),
                        translation.lane(lane)
                    ))
                );
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
                    Affine3::<Wide>::look_to_lh(eye, dir, up),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::look_to_lh(
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
                    Affine3::<Wide>::look_to_rh(eye, dir, up),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::look_to_rh(
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
                    Affine3::<Wide>::look_at_lh(eye, center, up),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::look_at_lh(
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
                    Affine3::<Wide>::look_at_rh(eye, center, up),
                    Affine3::from_lane_fn(|lane| Affine3::<T>::look_at_rh(
                        eye.lane(lane),
                        center.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|Wide: WideFloat| {
            for order in EulerRot::values() {
                for affine in random_iter::<Affine3<Wide>>().chain(
                    random_iter::<[Wide; 3]>()
                        .map(|[a, b, c]| Affine3::<Wide>::from_euler(order, a, b, c)),
                ) {
                    assert_test_eq_or_panic!(
                        affine.to_euler(order),
                        (
                            Wide::new(std::array::from_fn(|lane| affine
                                .lane(lane)
                                .to_euler(order)
                                .0)),
                            Wide::new(std::array::from_fn(|lane| affine
                                .lane(lane)
                                .to_euler(order)
                                .1)),
                            Wide::new(std::array::from_fn(|lane| affine
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

    #[test]
    fn test_to_scale_rotation() {
        for_types!(|Wide: WideFloat| {
            for affine in random_iter::<Affine3<Wide>>().chain(
                random_iter::<(Vec3<Wide>, Quat<Wide>, Vec3<Wide>)>().map(
                    |(scale, rotation, translation)| {
                        Affine3::<Wide>::from_scale_rotation_translation(
                            scale,
                            rotation.normalize(),
                            translation,
                        )
                    },
                ),
            ) {
                assert_test_eq_or_panic!(
                    affine.to_scale_rotation(),
                    (
                        Vec3::from_lane_fn(|lane| affine.lane(lane).to_scale_rotation().0),
                        Quat::from_lane_fn(|lane| affine.lane(lane).to_scale_rotation().1)
                    )
                );
            }
        });
    }

    #[test]
    fn test_to_scale_rotation_translation() {
        for_types!(|Wide: WideFloat| {
            for affine in random_iter::<Affine3<Wide>>().chain(
                random_iter::<(Vec3<Wide>, Quat<Wide>, Vec3<Wide>)>().map(
                    |(scale, rotation, translation)| {
                        Affine3::<Wide>::from_scale_rotation_translation(
                            scale,
                            rotation.normalize(),
                            translation,
                        )
                    },
                ),
            ) {
                assert_test_eq_or_panic!(
                    affine.to_scale_rotation_translation(),
                    (
                        Vec3::from_lane_fn(|lane| affine
                            .lane(lane)
                            .to_scale_rotation_translation()
                            .0),
                        Quat::from_lane_fn(|lane| affine
                            .lane(lane)
                            .to_scale_rotation_translation()
                            .1),
                        Vec3::from_lane_fn(|lane| affine
                            .lane(lane)
                            .to_scale_rotation_translation()
                            .2)
                    )
                );
            }
        });
    }
}
