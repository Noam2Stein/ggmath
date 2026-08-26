use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Alignment, EulerRot, Length, Matrix, Projective, Rotor, SupportedLength, Vector,
    length::TwoOrThree,
    utils::{specialize, specialize_23},
};

macro_rules! items {
    ($Wide:ident, $T:ident) => {
        /// A matrix with all elements set to NaN (Not a Number).
        pub const NAN: Self = Self::from_rows(&[Vector::<N, $Wide, A>::NAN; N]);

        /// Converts a projective transform to a linear transformation matrix.
        ///
        /// This assumes `projective` does not contain projections. If there is
        /// translation, it is ignored.
        #[inline]
        #[must_use]
        #[expect(private_bounds)]
        pub fn from_projective(projective: &Projective<N, $Wide, A>) -> Self
        where
            Length<N>: TwoOrThree,
        {
            specialize_23!(Matrix::<N, $Wide, A>::from_projective_backend(projective))
        }

        /// Creates a rotation matrix from a rotor.
        #[inline]
        #[must_use]
        #[expect(private_bounds)]
        pub fn from_rotor(rotor: Rotor<N, $Wide, A>) -> Self
        where
            Length<N>: TwoOrThree,
        {
            specialize_23!(Matrix::<N, $Wide, A>::from_rotor_backend(rotor))
        }

        /// Creates a matrix containing a non-uniform `scale` and a `rotation`.
        #[inline]
        #[must_use]
        #[expect(private_bounds)]
        pub fn from_scale_rotation(scale: Vector<N, $Wide, A>, rotation: Rotor<N, $Wide, A>) -> Self
        where
            Length<N>: TwoOrThree,
        {
            Self::from_rotor(rotation).prepend_scale(scale)
        }

        /// For each lane, returns `true` if any element is NaN.
        #[inline]
        #[must_use]
        pub fn is_nan(&self) -> $Wide {
            specialize!(Matrix::<N, $Wide, A>::is_nan_backend(self))
        }

        /// For each lane, returns `true` if all elements are neither infinite
        /// nor NaN.
        #[inline]
        #[must_use]
        pub fn is_finite(&self) -> $Wide {
            specialize!(Matrix::<N, $Wide, A>::is_finite_backend(self))
        }

        /// Returns the inverse of `self`.
        ///
        /// If `self` is not invertable the result is unspecified.
        #[must_use]
        pub fn inverse(&self) -> Self {
            self.inverse_and_determinant().0
        }

        // `try_inverse` is exluded on purpose. It would not be useful because
        // it would only return `Some` if all lanes succeed.

        /// Returns the inverse of `self` or `fallback` if `self` is not
        /// invertable.
        ///
        /// The fallback is only applied for invalid lanes. Other lanes are not
        /// affected.
        #[must_use]
        pub fn inverse_or(&self, fallback: &Self) -> Self {
            specialize!(Matrix::<N, $Wide, A>::inverse_or_backend(self, fallback))
        }

        /// Returns the inverse of `self` or the zero matrix if `self` is not
        /// invertable.
        ///
        /// The fallback is only applied for invalid lanes. Other lanes are not
        /// affected.
        #[must_use]
        pub fn inverse_or_zero(&self) -> Self {
            specialize!(Matrix::<N, $Wide, A>::inverse_or_zero_backend(self))
        }

        #[inline]
        pub(crate) fn inverse_and_determinant(&self) -> (Self, $Wide) {
            specialize!(Matrix::<N, $Wide, A>::inverse_and_determinant_backend(self))
        }

        /// Returns the `scale` and `rotation` of `self`.
        ///
        /// `self` must not contain shearing. Otherwise the result is
        /// unspecified.
        #[inline]
        #[must_use]
        #[expect(private_bounds)]
        pub fn to_scale_rotation(&self) -> (Vector<N, $Wide, A>, Rotor<N, $Wide, A>)
        where
            Length<N>: TwoOrThree,
        {
            todo!()
        }

        /// Returns the element-wise reciprocal (inverse) of a matrix,
        /// `1 / self`.
        #[inline]
        #[must_use]
        pub fn recip(&self) -> Self {
            specialize!(Matrix::<N, $Wide, A>::recip_backend(self))
        }

        /// Returns the absolute values of the elements of `self`.
        ///
        /// Equivalent to `(self.x_axis.abs(), self.y_axis.abs(), ...)`.
        #[inline]
        #[must_use]
        pub fn abs(&self) -> Self {
            specialize!(Matrix::<N, $Wide, A>::abs_backend(self))
        }

        /// Returns `true` if the absolute difference of all elements between
        /// `self` and `other` is less than or equal to `max_abs_diff` for all
        /// lanes.
        ///
        /// This can be used to compare two matrices that should be equal, but
        /// may have a slight difference due to operations having rounding
        /// errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: $Wide) -> bool {
            specialize!(Matrix::<N, $Wide, A>::abs_diff_eq_backend(
                self,
                other,
                max_abs_diff
            ))
        }
    };
}

macro_rules! items_2 {
    ($Wide:ident, $T:ident) => {
        /// Creates a matrix containing a rotation of `angle` (in radians).
        ///
        /// This rotates `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_angle(angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_rows(&[
                Vector::<2, $Wide, A>::new(cos, sin),
                Vector::<2, $Wide, A>::new(-sin, cos),
            ])
        }

        /// Creates a matrix containing the non-uniform `scale` and a rotation
        /// of `angle` (in radians).
        ///
        /// This rotates `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_scale_angle(scale: Vector<2, $Wide, A>, angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_rows(&[
                Vector::<2, $Wide, A>::new(cos * scale.x, sin * scale.x),
                Vector::<2, $Wide, A>::new(-sin * scale.y, cos * scale.y),
            ])
        }

        /// Takes the `N`x`N` linear transformation part of an `N+1`x`N+1`
        /// homogeneous transformation matrix, removing the last row and column.
        ///
        /// This assumes `homogeneous` does not contain projections. If there is
        /// translation, it is ignored.
        #[inline]
        #[must_use]
        pub fn from_homogeneous(homogeneous: &Matrix<3, $Wide, A>) -> Self {
            Self::from_rows(&[homogeneous.x_axis.truncate(), homogeneous.y_axis.truncate()])
        }

        /// Returns the `scale` and `angle` of `self`.
        ///
        /// `self` must not contain shearing. Otherwise the result is
        /// unspecified.
        #[inline]
        #[must_use]
        pub fn to_scale_angle(&self) -> (Vector<2, $Wide, A>, $Wide) {
            let determinant = self.determinant();

            let scale = Vector::<2, $Wide, A>::new(
                self.x_axis.length() * determinant.signum(),
                self.y_axis.length(),
            );

            let angle = (-self.y_axis.x).atan2(self.y_axis.y);

            (scale, angle)
        }
    };
}

macro_rules! items_3 {
    ($Wide:ident, $T:ident) => {
        /// Creates a 3D rotation matrix from an `angle` (in radians) rotating
        /// `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xy(angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_rows(&[
                Vector::<3, $Wide, A>::new(cos, sin, $Wide::ZERO),
                Vector::<3, $Wide, A>::new(-sin, cos, $Wide::ZERO),
                Vector::<3, $Wide, A>::Z,
            ])
        }

        /// Creates a 3D rotation matrix from an `angle` (in radians) rotating
        /// `+X` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xz(angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_rows(&[
                Vector::<3, $Wide, A>::new(cos, $Wide::ZERO, sin),
                Vector::<3, $Wide, A>::Y,
                Vector::<3, $Wide, A>::new(-sin, $Wide::ZERO, cos),
            ])
        }

        /// Creates a 3D rotation matrix from an `angle` (in radians) rotating
        /// `+Y` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_yz(angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_rows(&[
                Vector::<3, $Wide, A>::X,
                Vector::<3, $Wide, A>::new($Wide::ZERO, cos, sin),
                Vector::<3, $Wide, A>::new($Wide::ZERO, -sin, cos),
            ])
        }

        /// Creates a 3D rotation matrix from a rotation `axis` and `angle` (in
        /// radians).
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
                Vector::<3, $Wide, A>::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
                Vector::<3, $Wide, A>::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
                Vector::<3, $Wide, A>::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
            ])
        }

        /// Creates a 3D rotation matrix from an Euler rotation order/sequence
        /// and angles (in radians).
        #[inline]
        #[must_use]
        pub fn from_euler(order: EulerRot, a: $Wide, b: $Wide, c: $Wide) -> Self {
            // Ported from https://github.com/bitshifter/glam-rs.

            // Based on Ken Shoemake. 1994. Euler angle conversion. Graphics gems IV.
            // Academic Press Professional, Inc., USA, 222–229.

            let order = order.properties();
            let (i, j, k) = order.axes_indices();

            let mut angles = if order.frame_static {
                Vector::<3, $Wide, A>::new(a, b, c)
            } else {
                Vector::<3, $Wide, A>::new(c, b, a)
            };

            // Rotation direction is reverse from original paper.
            if order.parity_even {
                angles = -angles;
            }

            let (si, ci) = angles.x.sin_cos();
            let (sj, cj) = angles.y.sin_cos();
            let (sh, ch) = angles.z.sin_cos();

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

        /// Takes the `N`x`N` linear transformation part of an `N+1`x`N+1`
        /// homogeneous transformation matrix, removing the last row and column.
        ///
        /// This assumes `homogeneous` does not contain projections. If there is
        /// translation, it is ignored.
        #[inline]
        #[must_use]
        pub fn from_homogeneous(homogeneous: &Matrix<4, $Wide, A>) -> Self {
            Self::from_rows(&[
                homogeneous.x_axis.truncate(),
                homogeneous.y_axis.truncate(),
                homogeneous.z_axis.truncate(),
            ])
        }

        /// Creates a left-handed view matrix from a facing direction and an up
        /// direction.
        ///
        /// For a view coordinate system with `+X=right`, `+Y=up` and
        /// `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_to_lh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            let forward = dir;
            let right = up.cross(forward).normalize();
            let up = forward.cross(right);

            Self::from_rows(&[
                Vector::<3, $Wide, A>::new(right.x, up.x, forward.x),
                Vector::<3, $Wide, A>::new(right.y, up.y, forward.y),
                Vector::<3, $Wide, A>::new(right.z, up.z, forward.z),
            ])
        }

        /// Creates a right-handed view matrix from a facing direction and an up
        /// direction.
        ///
        /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_to_rh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            let forward = dir;
            let right = forward.cross(up).normalize();
            let up = right.cross(forward);

            Self::from_rows(&[
                Vector::<3, $Wide, A>::new(right.x, up.x, -forward.x),
                Vector::<3, $Wide, A>::new(right.y, up.y, -forward.y),
                Vector::<3, $Wide, A>::new(right.z, up.z, -forward.z),
            ])
        }

        /// Creates a left-handed view matrix from a camera position, a focal
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
            Self::look_to_lh((center - eye).normalize(), up)
        }

        /// Creates a right-handed view matrix from a camera position, a focal
        /// point and an up direction.
        ///
        /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_at_rh(
            eye: Vector<3, $Wide, A>,
            center: Vector<3, $Wide, A>,
            up: Vector<3, $Wide, A>,
        ) -> Self {
            Self::look_to_rh((center - eye).normalize(), up)
        }

        /// Returns the Euler angles forming `self` for the given Euler rotation
        /// order/sequence.
        ///
        /// `self` must not contain any non-rotation transformations. Otherwise
        /// the result is unspecified.
        #[inline]
        #[must_use]
        pub fn to_euler(&self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
            // Ported from https://github.com/bitshifter/glam-rs.

            // Based on Ken Shoemake. 1994. Euler angle conversion. Graphics
            // gems IV. Academic Press Professional, Inc., USA, 222–229.

            let order = order.properties();
            let (i, j, k) = order.axes_indices();

            let mut ea = Vector::<3, $Wide, A>::ZERO;
            if order.initial_repeated {
                let sy = (self[i][j] * self[i][j] + self[i][k] * self[i][k]).sqrt();

                let mask = sy.simd_gt($Wide::splat(16.0 * $T::EPSILON));
                ea.x = mask.blend(
                    self[i][j].atan2(self[i][k]),
                    (-self[j][k]).atan2(self[j][j]),
                );
                ea.y = sy.atan2(self[i][i]);
                ea.z = mask & self[j][i].atan2(-self[k][i]);
            } else {
                let cy = (self[i][i] * self[i][i] + self[j][i] * self[j][i]).sqrt();

                let mask = cy.simd_gt($Wide::splat(16.0 * $T::EPSILON));
                ea.x = mask.blend(
                    self[k][j].atan2(self[k][k]),
                    (-self[j][k]).atan2(self[j][j]),
                );
                ea.y = (-self[k][i]).atan2(cy);
                ea.z = mask & self[j][i].atan2(self[i][i]);
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

/// Functionality for [SoA] (Structure of Arrays) float matrices.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<const N: usize, Wide, A: Alignment> Matrix<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideFloat,
{
    items!(Wide, f32);
}

/// Functionality for [SoA] (Structure of Arrays) 2x2 float matrices.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Matrix<2, Wide, A>
where
    Wide: WideFloat,
{
    items_2!(Wide, f32);
}

/// Functionality for [SoA] (Structure of Arrays) 3x3 float matrices.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Matrix<3, Wide, A>
where
    Wide: WideFloat,
{
    items_3!(Wide, f32);
}

macro_rules! impl_items {
    ($Wide:ident, $T:ident) => {
        #[cfg(not(doc))]
        impl<const N: usize, A: Alignment> Matrix<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            items!($Wide, $T);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Matrix<2, $Wide, A> {
            items_2!($Wide, $T);

            #[inline(always)]
            fn from_projective_backend(projective: &Projective<2, $Wide, A>) -> Self {
                Self::from_rows(&[projective.x_axis.truncate(), projective.y_axis.truncate()])
            }

            #[inline(always)]
            fn from_rotor_backend(rotor: Rotor<2, $Wide, A>) -> Self {
                let xx = rotor.s * rotor.s - rotor.xy * rotor.xy;
                let half_xy = rotor.xy * rotor.s;
                let xy = half_xy + half_xy;

                Self::from_row_array(&[xx, xy, -xy, xx])
            }

            #[inline(always)]
            fn is_nan_backend(&self) -> $Wide {
                self.x_axis.is_nan() | self.y_axis.is_nan()
            }

            #[inline(always)]
            fn is_finite_backend(&self) -> $Wide {
                self.x_axis.is_finite() & self.y_axis.is_finite()
            }

            #[inline(always)]
            fn inverse_or_backend(&self, fallback: &Self) -> Self {
                let (inverse, determinant) = self.inverse_and_determinant();

                let fallback_mask = determinant.simd_eq($Wide::ZERO);
                Self::from_row_array(&[
                    fallback_mask.blend(fallback.x_axis.x, inverse.x_axis.x),
                    fallback_mask.blend(fallback.x_axis.y, inverse.x_axis.y),
                    fallback_mask.blend(fallback.y_axis.x, inverse.y_axis.x),
                    fallback_mask.blend(fallback.y_axis.y, inverse.y_axis.y),
                ])
            }

            #[inline(always)]
            fn inverse_or_zero_backend(&self) -> Self {
                let (inverse, determinant) = self.inverse_and_determinant();

                let non_fallback_mask = determinant.simd_ne($Wide::ZERO);
                Self::from_row_array(&[
                    inverse.x_axis.x & non_fallback_mask,
                    inverse.x_axis.y & non_fallback_mask,
                    inverse.y_axis.x & non_fallback_mask,
                    inverse.y_axis.y & non_fallback_mask,
                ])
            }

            #[inline(always)]
            fn inverse_and_determinant_backend(&self) -> (Self, $Wide) {
                let determinant = self.determinant();

                let determinant_recip = $Wide::ONE / determinant;
                let inverse = Matrix::<2, $Wide, A>::from_row_array(&[
                    self.y_axis.y * determinant_recip,
                    self.x_axis.y * -determinant_recip,
                    self.y_axis.x * -determinant_recip,
                    self.x_axis.x * determinant_recip,
                ]);

                (inverse, determinant)
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
            fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: $Wide) -> bool {
                self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
                    && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Matrix<3, $Wide, A> {
            items_3!($Wide, $T);

            #[inline(always)]
            fn from_projective_backend(projective: &Projective<3, $Wide, A>) -> Self {
                Self::from_rows(&[
                    projective.x_axis.truncate(),
                    projective.y_axis.truncate(),
                    projective.z_axis.truncate(),
                ])
            }

            #[inline(always)]
            fn from_rotor_backend(rotor: Rotor<3, $Wide, A>) -> Self {
                let bivector = rotor.0.xyz();
                let bivector_2 = bivector + bivector;
                let [xy_xy_2, xy_xz_2, xy_yz_2] = (bivector * bivector_2.x).to_array();
                let [xz_xz_2, xz_yz_2, yz_yz_2] = (bivector.yzz() * bivector_2.yyz()).to_array();
                let [s_xy_2, s_xz_2, s_yz_2] = (bivector_2 * rotor.s).to_array();

                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new($Wide::ONE, s_xy_2, xy_yz_2)
                        - Vector::<3, $Wide, A>::new(xz_xz_2 + xy_xy_2, xz_yz_2, -s_xz_2),
                    Vector::<3, $Wide, A>::new(-xz_yz_2, $Wide::ONE, s_yz_2)
                        - Vector::<3, $Wide, A>::new(s_xy_2, yz_yz_2 + xy_xy_2, xy_xz_2),
                    Vector::<3, $Wide, A>::new(xy_yz_2, -xy_xz_2, $Wide::ONE)
                        - Vector::<3, $Wide, A>::new(s_xz_2, s_yz_2, yz_yz_2 + xz_xz_2),
                ])
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
            fn inverse_or_backend(&self, fallback: &Self) -> Self {
                let (inverse, determinant) = self.inverse_and_determinant();

                let fallback_mask = determinant.simd_eq($Wide::ZERO);
                Self::from_row_array(&[
                    fallback_mask.blend(fallback.x_axis.x, inverse.x_axis.x),
                    fallback_mask.blend(fallback.x_axis.y, inverse.x_axis.y),
                    fallback_mask.blend(fallback.x_axis.z, inverse.x_axis.z),
                    fallback_mask.blend(fallback.y_axis.x, inverse.y_axis.x),
                    fallback_mask.blend(fallback.y_axis.y, inverse.y_axis.y),
                    fallback_mask.blend(fallback.y_axis.z, inverse.y_axis.z),
                    fallback_mask.blend(fallback.z_axis.x, inverse.z_axis.x),
                    fallback_mask.blend(fallback.z_axis.y, inverse.z_axis.y),
                    fallback_mask.blend(fallback.z_axis.z, inverse.z_axis.z),
                ])
            }

            #[inline(always)]
            fn inverse_or_zero_backend(&self) -> Self {
                let (inverse, determinant) = self.inverse_and_determinant();

                let non_fallback_mask = determinant.simd_ne($Wide::ZERO);
                Self::from_row_array(&[
                    inverse.x_axis.x & non_fallback_mask,
                    inverse.x_axis.y & non_fallback_mask,
                    inverse.x_axis.z & non_fallback_mask,
                    inverse.y_axis.x & non_fallback_mask,
                    inverse.y_axis.y & non_fallback_mask,
                    inverse.y_axis.z & non_fallback_mask,
                    inverse.z_axis.x & non_fallback_mask,
                    inverse.z_axis.y & non_fallback_mask,
                    inverse.z_axis.z & non_fallback_mask,
                ])
            }

            #[inline(always)]
            fn inverse_and_determinant_backend(&self) -> (Self, $Wide) {
                let x_cross_y = self.x_axis.cross(self.y_axis);
                let determinant = x_cross_y.dot(self.z_axis);

                // Compute cross products but avoid the `.zxy()` at the end.
                let y_cross_z_yzx =
                    self.y_axis.zxy() * self.z_axis - self.y_axis * self.z_axis.zxy();
                let z_cross_x_yzx =
                    self.z_axis.zxy() * self.x_axis - self.z_axis * self.x_axis.zxy();

                // Simultaneously perform `{cross-product-yzx}.zxy()` and `{matrix}.transpose()`.
                let adjugate = Self::from_row_array(&[
                    y_cross_z_yzx.z,
                    z_cross_x_yzx.z,
                    x_cross_y.x,
                    y_cross_z_yzx.x,
                    z_cross_x_yzx.x,
                    x_cross_y.y,
                    y_cross_z_yzx.y,
                    z_cross_x_yzx.y,
                    x_cross_y.z,
                ]);

                let inverse = adjugate * ($Wide::ONE / determinant);

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
            fn abs_diff_eq_backend(&self, other: &Self, max_abs_diff: $Wide) -> bool {
                self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
                    && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
                    && self.z_axis.abs_diff_eq(other.z_axis, max_abs_diff)
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Matrix<4, $Wide, A> {
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
            fn inverse_or_backend(&self, fallback: &Self) -> Self {
                let (inverse, determinant) = self.inverse_and_determinant();

                let fallback_mask = determinant.simd_eq($Wide::ZERO);
                Self::from_row_array(&[
                    fallback_mask.blend(fallback.x_axis.x, inverse.x_axis.x),
                    fallback_mask.blend(fallback.x_axis.y, inverse.x_axis.y),
                    fallback_mask.blend(fallback.x_axis.z, inverse.x_axis.z),
                    fallback_mask.blend(fallback.x_axis.w, inverse.x_axis.w),
                    fallback_mask.blend(fallback.y_axis.x, inverse.y_axis.x),
                    fallback_mask.blend(fallback.y_axis.y, inverse.y_axis.y),
                    fallback_mask.blend(fallback.y_axis.z, inverse.y_axis.z),
                    fallback_mask.blend(fallback.y_axis.w, inverse.y_axis.w),
                    fallback_mask.blend(fallback.z_axis.x, inverse.z_axis.x),
                    fallback_mask.blend(fallback.z_axis.y, inverse.z_axis.y),
                    fallback_mask.blend(fallback.z_axis.z, inverse.z_axis.z),
                    fallback_mask.blend(fallback.z_axis.w, inverse.z_axis.w),
                    fallback_mask.blend(fallback.w_axis.x, inverse.w_axis.x),
                    fallback_mask.blend(fallback.w_axis.y, inverse.w_axis.y),
                    fallback_mask.blend(fallback.w_axis.z, inverse.w_axis.z),
                    fallback_mask.blend(fallback.w_axis.w, inverse.w_axis.w),
                ])
            }

            #[inline(always)]
            fn inverse_or_zero_backend(&self) -> Self {
                let (inverse, determinant) = self.inverse_and_determinant();

                let non_fallback_mask = determinant.simd_ne($Wide::ZERO);
                Self::from_row_array(&[
                    inverse.x_axis.x & non_fallback_mask,
                    inverse.x_axis.y & non_fallback_mask,
                    inverse.x_axis.z & non_fallback_mask,
                    inverse.x_axis.w & non_fallback_mask,
                    inverse.y_axis.x & non_fallback_mask,
                    inverse.y_axis.y & non_fallback_mask,
                    inverse.y_axis.z & non_fallback_mask,
                    inverse.y_axis.w & non_fallback_mask,
                    inverse.z_axis.x & non_fallback_mask,
                    inverse.z_axis.y & non_fallback_mask,
                    inverse.z_axis.z & non_fallback_mask,
                    inverse.z_axis.w & non_fallback_mask,
                    inverse.w_axis.x & non_fallback_mask,
                    inverse.w_axis.y & non_fallback_mask,
                    inverse.w_axis.z & non_fallback_mask,
                    inverse.w_axis.w & non_fallback_mask,
                ])
            }

            #[inline(always)]
            fn inverse_and_determinant_backend(&self) -> (Self, $Wide) {
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

                let fac0 = Vector::<4, $Wide, A>::new(coef00, coef00, coef02, coef03);
                let fac1 = Vector::<4, $Wide, A>::new(coef04, coef04, coef06, coef07);
                let fac2 = Vector::<4, $Wide, A>::new(coef08, coef08, coef10, coef11);
                let fac3 = Vector::<4, $Wide, A>::new(coef12, coef12, coef14, coef15);
                let fac4 = Vector::<4, $Wide, A>::new(coef16, coef16, coef18, coef19);
                let fac5 = Vector::<4, $Wide, A>::new(coef20, coef20, coef22, coef23);

                let vec0 = Vector::<4, $Wide, A>::new(m10, m00, m00, m00);
                let vec1 = Vector::<4, $Wide, A>::new(m11, m01, m01, m01);
                let vec2 = Vector::<4, $Wide, A>::new(m12, m02, m02, m02);
                let vec3 = Vector::<4, $Wide, A>::new(m13, m03, m03, m03);

                let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
                let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
                let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
                let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

                let sign_a =
                    Vector::<4, $Wide, A>::new($Wide::ONE, -$Wide::ONE, $Wide::ONE, -$Wide::ONE);
                let sign_b =
                    Vector::<4, $Wide, A>::new(-$Wide::ONE, $Wide::ONE, -$Wide::ONE, $Wide::ONE);

                let inverse = Matrix::<4, $Wide, A>::from_rows(&[
                    inv0 * sign_a,
                    inv1 * sign_b,
                    inv2 * sign_a,
                    inv3 * sign_b,
                ]);

                let inverse_column_0 = Vector::<4, $Wide, A>::new(
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
    extern crate std;

    use wide::f32x4;

    use crate::{
        EulerRot, Mat2, Mat3, Mat4, Matrix, Projective, Unaligned, Vec2, Vec3, Vector,
        test_utils::{assert_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, Wide: WideFloat| {
            assert_test_eq!(
                Matrix::<N, Wide, Unaligned>::NAN,
                Matrix::from_rows(&[Vector::<N, Wide, Unaligned>::NAN; N])
            );
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|N: TwoOrThree| {
            for projective in random_iter::<Projective<N, f32x4, Unaligned>>() {
                assert_test_eq_or_panic!(
                    Matrix::<N, f32x4, Unaligned>::from_projective(&projective),
                    Matrix::from_lane_fn(|lane| Matrix::<N, f32, Unaligned>::from_projective(
                        &projective.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|Wide: WideFloat| {
            for [x, y, z, w] in random_iter::<[Wide; 4]>() {
                assert_test_eq!(
                    Mat2::from_rows(&[x, y].map(Vector::splat)).is_nan(),
                    x.is_nan() | y.is_nan()
                );
                assert_test_eq!(
                    Mat3::from_rows(&[x, y, z].map(Vector::splat)).is_nan(),
                    x.is_nan() | y.is_nan() | z.is_nan()
                );
                assert_test_eq!(
                    Mat4::from_rows(&[x, y, z, w].map(Vector::splat)).is_nan(),
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
                    Mat2::from_rows(&[x, y].map(Vector::splat)).is_finite(),
                    x.is_finite() & y.is_finite()
                );
                assert_test_eq!(
                    Mat3::from_rows(&[x, y, z].map(Vector::splat)).is_finite(),
                    x.is_finite() & y.is_finite() & z.is_finite()
                );
                assert_test_eq!(
                    Mat4::from_rows(&[x, y, z, w].map(Vector::splat)).is_finite(),
                    x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
                );
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_test_eq_or_panic!(
                    matrix.inverse(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).inverse())
                );
            }
        });
    }

    // `try_inverse` is exluded on purpose.

    #[test]
    fn test_inverse_or() {
        for_types!(|N, Wide: WideFloat| {
            for [matrix, fallback] in random_iter::<[Matrix<N, Wide, Unaligned>; 2]>() {
                assert_test_eq!(
                    matrix.inverse_or(&fallback),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).inverse_or(&fallback.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    matrix.inverse_or_zero(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).inverse_or_zero())
                );
            }
        });
    }

    #[test]
    fn test_recip() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    matrix.recip(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).recip())
                );
            }
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|N, Wide: WideFloat| {
            for matrix in random_iter::<Matrix<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    matrix.abs(),
                    Matrix::from_lane_fn(|lane| matrix.lane(lane).abs())
                );
            }
        });
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
                    Mat2::<Wide>::from_angle(angle),
                    Mat2::from_lane_fn(|lane| Mat2::<T>::from_angle(angle.to_array()[lane])),
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
                let scale = Vec2::splat(scale.length().is_finite()).blend(scale, Vec2::ONE);

                assert_test_eq!(
                    Mat2::<Wide>::from_scale_angle(scale, angle),
                    Mat2::from_lane_fn(|lane| Mat2::<T>::from_scale_angle(
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
    fn test_from_homogeneous() {
        for homogeneous in random_iter::<Mat3<f32x4>>() {
            assert_test_eq_or_panic!(
                Mat2::<f32x4>::from_homogeneous(&homogeneous),
                Matrix::from_lane_fn(|lane| Mat2::<f32>::from_homogeneous(&homogeneous.lane(lane)))
            );
        }
        for homogeneous in random_iter::<Mat4<f32x4>>() {
            assert_test_eq_or_panic!(
                Mat3::<f32x4>::from_homogeneous(&homogeneous),
                Matrix::from_lane_fn(|lane| Mat3::<f32>::from_homogeneous(&homogeneous.lane(lane)))
            );
        }
    }

    #[test]
    fn test_to_scale_angle() {
        for_types!(|Wide: WideFloat| {
            for matrix in random_iter::<(Vec2<Wide>, Wide)>()
                .map(|(scale, angle)| Mat2::<Wide>::from_scale_angle(scale, angle))
                .chain(random_iter())
            {
                assert_test_eq_or_panic!(
                    matrix.to_scale_angle(),
                    (
                        Vec2::from_lane_fn(|lane| matrix.lane(lane).to_scale_angle().0),
                        Wide::new(std::array::from_fn(|lane| matrix
                            .lane(lane)
                            .to_scale_angle()
                            .1))
                    ),
                    abs <= (
                        matrix.to_scale_angle().0.abs() * Wide::splat(1e-4) + Wide::splat(1e-3),
                        matrix.to_scale_angle().1.abs() * 1e-4 + 1e-3
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
                    Mat3::<Wide>::from_rotation_xy(angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_rotation_xy(angle.to_array()[lane])),
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
                    Mat3::<Wide>::from_rotation_xz(angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_rotation_xz(angle.to_array()[lane])),
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
                    Mat3::<Wide>::from_rotation_yz(angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_rotation_yz(angle.to_array()[lane])),
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
                let axis = Vec3::splat(condition).blend(axis, Vec3::X);
                let angle = condition.blend(angle, Wide::ONE);

                assert_test_eq_or_panic!(
                    Mat3::<Wide>::from_axis_angle(axis, angle),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::from_axis_angle(
                        axis.lane(lane),
                        angle.to_array()[lane]
                    )),
                    abs <= Mat3::<Wide>::from_axis_angle(axis, angle).abs()
                        * axis.length().max(Wide::ONE)
                        * angle.abs().max(Wide::ONE)
                        * Wide::splat(1e-4)
                        + Mat3::from_row_array(&[Wide::splat(1e-3); 9]),
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
                        Mat3::<Wide>::from_euler(order, a, b, c),
                        Mat3::from_lane_fn(|lane| Mat3::<T>::from_euler(
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
            for [dir, up] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|[dir, up]| [[dir, up], [dir.normalize(), up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Mat3::<Wide>::look_to_lh(dir, up),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::look_to_lh(dir.lane(lane), up.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|Wide: WideFloat| {
            for [dir, up] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|[dir, up]| [[dir, up], [dir.normalize(), up.normalize()]])
            {
                assert_test_eq_or_panic!(
                    Mat3::<Wide>::look_to_rh(dir, up),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::look_to_rh(dir.lane(lane), up.lane(lane)))
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
                    Mat3::<Wide>::look_at_lh(eye, center, up),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::look_at_lh(
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
                    Mat3::<Wide>::look_at_rh(eye, center, up),
                    Mat3::from_lane_fn(|lane| Mat3::<T>::look_at_rh(
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
                for matrix in random_iter::<[Wide; 3]>()
                    .map(|[a, b, c]| Mat3::<Wide>::from_euler(order, a, b, c))
                    .chain(random_iter())
                {
                    assert_test_eq_or_panic!(
                        matrix.to_euler(order),
                        (
                            Wide::new(std::array::from_fn(|lane| matrix
                                .lane(lane)
                                .to_euler(order)
                                .0)),
                            Wide::new(std::array::from_fn(|lane| matrix
                                .lane(lane)
                                .to_euler(order)
                                .1)),
                            Wide::new(std::array::from_fn(|lane| matrix
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
