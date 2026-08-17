use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Alignment, EulerRot, Length, Matrix, Quaternion, SupportedLength, Vector, utils::specialize,
};

macro_rules! impl_wide_float {
    ($Wide:ident, $T:ident) => {
        impl<const N: usize, A: Alignment> Matrix<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            /// A matrix with all elements set to NaN (Not a Number).
            pub const NAN: Self = Self::from_rows(&[Vector::<N, $Wide, A>::NAN; N]);

            /// For each lane, returns `true` if any element is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(&self) -> $Wide {
                specialize!(Matrix::<N, $Wide, A>::is_nan_backend(self))
            }

            /// For each lane, returns `true` if all elements are neither
            /// infinite nor NaN.
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

            // `try_inverse` is exluded on purpose. It would not be useful
            // because it would only return `Some` if all lanes succeed.

            /// Returns the inverse of `self` or `fallback` if `self` is not
            /// invertable.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            #[must_use]
            pub fn inverse_or(&self, fallback: &Self) -> Self {
                specialize!(Matrix::<N, $Wide, A>::inverse_or_backend(self, fallback))
            }

            /// Returns the inverse of `self` or the zero matrix if `self` is
            /// not invertable.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            #[must_use]
            pub fn inverse_or_zero(&self) -> Self {
                specialize!(Matrix::<N, $Wide, A>::inverse_or_zero_backend(self))
            }

            #[inline]
            pub(crate) fn inverse_and_determinant(&self) -> (Self, $Wide) {
                specialize!(Matrix::<N, $Wide, A>::inverse_and_determinant_backend(self))
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
                specialize!(Matrix::<N, $Wide, A>::abs_diff_eq_backend(
                    self,
                    other,
                    max_abs_diff
                ))
            }
        }

        impl<A: Alignment> Matrix<2, $Wide, A> {
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

            /// Creates a matrix containing the non-uniform `scale` and a
            /// rotation of `angle` (in radians).
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

        impl<A: Alignment> Matrix<3, $Wide, A> {
            /// Creates a 3D rotation matrix from `angle` (in radians) around
            /// the x axis.
            ///
            /// This rotates `+Y` to `+Z`.
            #[inline]
            #[must_use]
            pub fn from_rotation_x(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::X,
                    Vector::<3, $Wide, A>::new($Wide::ZERO, cos, sin),
                    Vector::<3, $Wide, A>::new($Wide::ZERO, -sin, cos),
                ])
            }

            /// Creates a 3D rotation matrix from `angle` (in radians) around
            /// the y axis.
            ///
            /// This rotates `+Z` to `+X`.
            #[inline]
            #[must_use]
            pub fn from_rotation_y(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos, $Wide::ZERO, -sin),
                    Vector::<3, $Wide, A>::Y,
                    Vector::<3, $Wide, A>::new(sin, $Wide::ZERO, cos),
                ])
            }

            /// Creates a 3D rotation matrix from `angle` (in radians) around
            /// the z axis.
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_rotation_z(angle: $Wide) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::from_rows(&[
                    Vector::<3, $Wide, A>::new(cos, sin, $Wide::ZERO),
                    Vector::<3, $Wide, A>::new(-sin, cos, $Wide::ZERO),
                    Vector::<3, $Wide, A>::Z,
                ])
            }

            #[inline]
            fn quat_to_axes(quat: Quaternion<$Wide, A>) -> [Vector<3, $Wide, A>; 3] {
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
                    Vector::<3, $Wide, A>::new($Wide::ONE - (yy2 + zz2), xy2 + wz2, xz2 - wy2),
                    Vector::<3, $Wide, A>::new(xy2 - wz2, $Wide::ONE - (xx2 + zz2), yz2 + wx2),
                    Vector::<3, $Wide, A>::new(xz2 + wy2, yz2 - wx2, $Wide::ONE - (xx2 + yy2)),
                ]
            }

            /// Creates a 3D rotation matrix from a quaternion.
            #[inline]
            #[must_use]
            pub fn from_quat(quat: Quaternion<$Wide, A>) -> Self {
                let [x_axis, y_axis, z_axis] = Self::quat_to_axes(quat);
                Self::from_rows(&[x_axis, y_axis, z_axis])
            }

            /// Creates a 3D rotation matrix from a rotation `axis` and `angle`
            /// (in radians).
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

            /// Creates a 3D rotation matrix from an Euler rotation
            /// order/sequence and angles (in radians).
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

            /// Creates a matrix containing a non-uniform `scale` and a 3D
            /// `rotation`.
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
                ])
            }

            /// Creates a left-handed view matrix from a facing direction and an
            /// up direction.
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

            /// Creates a right-handed view matrix from a facing direction and
            /// an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
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

            /// Creates a left-handed view matrix from a camera position, a
            /// focal point and an up direction.
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

            /// Creates a right-handed view matrix from a camera position, a
            /// focal point and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            #[inline]
            #[must_use]
            pub fn look_at_rh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::look_to_rh((center - eye).normalize(), up)
            }

            /// Returns the Euler angles forming `self` for the given Euler
            /// rotation order/sequence.
            ///
            /// `self` must not contain any non-rotation transformations.
            /// Otherwise the result is unspecified.
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

            /// For each lane, returns the `scale` and `rotation` of `self`.
            ///
            /// `self` must not contain shearing. Otherwise the result is
            /// unspecified.
            #[inline]
            #[must_use]
            pub fn to_scale_rotation(&self) -> (Vector<3, $Wide, A>, Quaternion<$Wide, A>) {
                let determinant = self.determinant();

                let scale = Vector::<3, $Wide, A>::new(
                    self.x_axis.length() * determinant.signum(),
                    self.y_axis.length(),
                    self.z_axis.length(),
                );

                let scale_recip = scale.recip();

                let rotation = Quaternion::<$Wide, A>::from_matrix(&Self::from_rows(&[
                    self.x_axis * scale_recip.x,
                    self.y_axis * scale_recip.y,
                    self.z_axis * scale_recip.z,
                ]));

                (scale, rotation)
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
impl_wide_float!(f32x4, f32);
impl_wide_float!(f32x8, f32);
impl_wide_float!(f32x16, f32);
impl_wide_float!(f64x2, f64);
impl_wide_float!(f64x4, f64);
impl_wide_float!(f64x8, f64);
