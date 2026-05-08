use wide::{CmpEq, f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{Affine, Alignment, EulerRot, Length, Matrix, Quaternion, SupportedLength, Vector};

macro_rules! impl_wide_float {
    ($Wide:ident) => {
        impl<const N: usize, A: Alignment> Affine<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            /// Returns `true` if any element is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(&self) -> $Wide {
                self.submatrix.is_nan() | self.translation.is_nan()
            }

            /// Returns `true` if all elements are neither infinite nor NaN.
            #[inline]
            #[must_use]
            pub fn is_finite(&self) -> $Wide {
                self.submatrix.is_finite() & self.translation.is_finite()
            }

            /// Returns the inverse of `self`.
            ///
            /// If `self` is not invertable the result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn inverse(&self) -> Self {
                let submatrix = self.submatrix.inverse();
                let translation = -(submatrix * self.translation);

                Self::from_submatrix_translation(submatrix, translation)
            }

            /// Returns the inverse of `self` or `None` if for any lane `self`
            /// is not invertable.
            #[inline]
            #[must_use]
            pub fn try_inverse(&self) -> Option<Self> {
                let submatrix = self.submatrix.try_inverse()?;
                let translation = -(submatrix * self.translation);

                Some(Self::from_submatrix_translation(submatrix, translation))
            }

            /// For each lane, returns the inverse of `self` or `fallback` if
            /// `self` is not invertable.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            #[inline]
            #[must_use]
            pub fn inverse_or(&self, fallback: &Self) -> Self {
                self.submatrix.generic_inverse(
                    |determinant, result| {
                        let fallback_mask = determinant.simd_eq($Wide::ZERO);
                        let submatrix = Matrix::from_column_fn(|c| {
                            Vector::from_fn(|r| {
                                fallback_mask.blend(fallback.column(c)[r], result.column(c)[r])
                            })
                        });
                        let translation = Vector::splat(fallback_mask)
                            .blend(fallback.translation, -(submatrix * self.translation));

                        Self::from_submatrix_translation(submatrix, translation)
                    },
                    |_| Ok(()),
                )
            }

            /// For each lane, returns the inverse of `self` or the zero
            /// transform if `self` is not invertable.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            #[inline]
            #[must_use]
            pub fn inverse_or_zero(&self) -> Self {
                self.submatrix.generic_inverse(
                    |determinant, result| {
                        let fallback_mask = determinant.simd_eq($Wide::ZERO);
                        let submatrix = Matrix::from_column_fn(|c| {
                            Vector::from_fn(|r| {
                                fallback_mask.blend($Wide::ZERO, result.column(c)[r])
                            })
                        });
                        let translation = Vector::splat(fallback_mask)
                            .blend(Vector::ZERO, -(submatrix * self.translation));

                        Self::from_submatrix_translation(submatrix, translation)
                    },
                    |_| Ok(()),
                )
            }
        }

        impl<A: Alignment> Affine<2, $Wide, A> {
            /// Creates an affine transform containing a rotation of `angle`
            /// (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_angle(angle: $Wide) -> Self {
                Self::from_submatrix(Matrix::<2, $Wide, A>::from_angle(angle))
            }

            /// Creates an affine transform containing a rotation of `angle`
            /// (in radians) and `translation`.
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_angle_translation(angle: $Wide, translation: Vector<2, $Wide, A>) -> Self {
                Self::from_submatrix_translation(
                    Matrix::<2, $Wide, A>::from_angle(angle),
                    translation,
                )
            }

            /// Creates an affine transform containing a non-uniform `scale` and
            /// rotation of `angle` (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_scale_angle(scale: Vector<2, $Wide, A>, angle: $Wide) -> Self {
                Self::from_submatrix(Matrix::<2, $Wide, A>::from_scale_angle(scale, angle))
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
                Self::from_submatrix_translation(
                    Matrix::<2, $Wide, A>::from_scale_angle(scale, angle),
                    translation,
                )
            }
        }

        impl<A: Alignment> Affine<3, $Wide, A> {
            /// Creates an affine transform containing a 3D rotation from
            /// `angle` (in radians) around the x axis.
            ///
            /// This rotates `+Y` to `+Z`.
            #[inline]
            #[must_use]
            pub fn from_rotation_x(angle: $Wide) -> Self {
                Self::from_submatrix(Matrix::<3, $Wide, A>::from_rotation_x(angle))
            }

            /// Creates an affine transform containing a 3D rotation from
            /// `angle` (in radians) around the y axis.
            ///
            /// This rotates `+Z` to `+X`.
            #[inline]
            #[must_use]
            pub fn from_rotation_y(angle: $Wide) -> Self {
                Self::from_submatrix(Matrix::<3, $Wide, A>::from_rotation_y(angle))
            }

            /// Creates an affine transform containing a 3D rotation from
            /// `angle` (in radians) around the z axis.
            ///
            /// This rotates `+X` to `+Y`.
            #[inline]
            #[must_use]
            pub fn from_rotation_z(angle: $Wide) -> Self {
                Self::from_submatrix(Matrix::<3, $Wide, A>::from_rotation_z(angle))
            }

            /// Creates an affine transform containing a 3D rotation from a
            /// quaternion.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_quat(quat: Quaternion<$Wide, A>) -> Self {
                Self::from_submatrix(Matrix::<3, $Wide, A>::from_quat(quat))
            }

            /// Creates an affine transform containing a rotation from a
            /// rotation `axis` and `angle` (in radians).
            ///
            /// `axis` must be normalized. Otherwise the result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
                Self::from_submatrix(Matrix::<3, $Wide, A>::from_axis_angle(axis, angle))
            }

            /// Creates an affine transform containing a rotation from an Euler
            /// rotation order/sequence and angles (in radians).
            #[inline]
            #[must_use]
            pub fn from_euler(order: EulerRot, a: $Wide, b: $Wide, c: $Wide) -> Self {
                Self::from_submatrix(Matrix::<3, $Wide, A>::from_euler(order, a, b, c))
            }

            /// Creates an affine transform containing a non-uniform `scale` and
            /// a 3D `rotation`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_scale_rotation(
                scale: Vector<3, $Wide, A>,
                rotation: Quaternion<$Wide, A>,
            ) -> Self {
                Self::from_submatrix(Matrix::<3, $Wide, A>::from_scale_rotation(scale, rotation))
            }

            /// Creates an affine transform containing a 3D `rotation` and
            /// `translation`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_rotation_translation(
                rotation: Quaternion<$Wide, A>,
                translation: Vector<3, $Wide, A>,
            ) -> Self {
                Self::from_submatrix_translation(
                    Matrix::<3, $Wide, A>::from_quat(rotation),
                    translation,
                )
            }

            /// Creates an affine transform containing a non-uniform `scale`, a
            /// 3D `rotation` and `translation`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_scale_rotation_translation(
                scale: Vector<3, $Wide, A>,
                rotation: Quaternion<$Wide, A>,
                translation: Vector<3, $Wide, A>,
            ) -> Self {
                Self::from_submatrix_translation(
                    Matrix::<3, $Wide, A>::from_scale_rotation(scale, rotation),
                    translation,
                )
            }

            /// Creates a left-handed view transform from a camera position, a
            /// facing direction and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=forward`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_to_lh(
                eye: Vector<3, $Wide, A>,
                dir: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                let forward = dir;
                let right = up.cross(forward).normalize();
                let up = forward.cross(right);

                Self::from_columns(&[
                    Vector::<3, $Wide, A>::new(right.x, up.x, forward.x),
                    Vector::<3, $Wide, A>::new(right.y, up.y, forward.y),
                    Vector::<3, $Wide, A>::new(right.z, up.z, forward.z),
                    Vector::<3, $Wide, A>::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward)),
                ])
            }

            /// Creates a right-handed view transform from a camera position, a
            /// facing direction and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_to_rh(
                eye: Vector<3, $Wide, A>,
                dir: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                let forward = dir;
                let right = forward.cross(up).normalize();
                let up = right.cross(forward);

                Self::from_columns(&[
                    Vector::<3, $Wide, A>::new(right.x, up.x, -forward.x),
                    Vector::<3, $Wide, A>::new(right.y, up.y, -forward.y),
                    Vector::<3, $Wide, A>::new(right.z, up.z, -forward.z),
                    Vector::<3, $Wide, A>::new(-eye.dot(right), -eye.dot(up), eye.dot(forward)),
                ])
            }

            /// Creates a left-handed view transform from a camera position, a
            /// focal point and an up direction.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=forward`.
            #[inline]
            #[must_use]
            #[track_caller]
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
            /// For a view coordinate system with `+X=right`, `+Y=up` and
            /// `+Z=back`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn look_at_rh(
                eye: Vector<3, $Wide, A>,
                center: Vector<3, $Wide, A>,
                up: Vector<3, $Wide, A>,
            ) -> Self {
                Self::look_to_rh(eye, (center - eye).normalize(), up)
            }

            /// Returns the Euler angles forming `self` for the given Euler
            /// rotation order/sequence.
            ///
            /// `self` must not contain any non-rotation transformations,
            /// excluding translation. Otherwise the result is unspecified.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn to_euler(&self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
                self.submatrix.to_euler(order)
            }
        }

        // MISSING: abs_diff_eq
        // MISSING: to_scale_angle_translation
        // MISSING: to_scale_rotation_translation
    };
}
impl_wide_float!(f32x4);
impl_wide_float!(f32x8);
impl_wide_float!(f32x16);
impl_wide_float!(f64x2);
impl_wide_float!(f64x4);
impl_wide_float!(f64x8);

#[cfg(test)]
mod tests {
    extern crate std;

    use wide::CmpLt;

    use crate::{
        Affine, Quaternion, Vector,
        utils::{assert_float_eq, assert_float_eq_or_panic, assert_panic_float_eq, for_parameters},
    };

    #[test]
    fn test_is_nan() {
        for_parameters!(|Wide: WideFloat, A, x, y, z, w, a, b, c, d| {
            let [x, y, z, w, a, b, c, d] =
                [x, y, z, w, a, b, c, d].map(|b| if b { Wide::splat(T::NAN) } else { Wide::ONE });

            let affine = Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, a, b]);
            assert_eq!(affine.is_nan().any(), affine.lane(0).is_nan());

            let affine =
                Affine::<3, Wide, A>::from_column_array(&[x, y, z, w, x, y, z, w, a, b, c, d]);
            assert_eq!(affine.is_nan().any(), affine.lane(0).is_nan());

            let affine = Affine::<4, Wide, A>::from_column_array(&[
                x, y, z, w, x, y, z, w, x, y, z, w, x, y, z, w, a, b, c, d,
            ]);
            assert_eq!(affine.is_nan().any(), affine.lane(0).is_nan());
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|Wide: WideFloat, A, x, y, z, w, a, b, c, d| {
            let [x, y, z, w, a, b, c, d] = [x, y, z, w, a, b, c, d].map(|b| {
                if b {
                    Wide::splat(T::INFINITY)
                } else {
                    Wide::ONE
                }
            });

            let affine = Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, a, b]);
            assert_eq!(affine.is_finite().any(), affine.lane(0).is_finite());

            let affine =
                Affine::<3, Wide, A>::from_column_array(&[x, y, z, w, x, y, z, w, a, b, c, d]);
            assert_eq!(affine.is_finite().any(), affine.lane(0).is_finite());

            let affine = Affine::<4, Wide, A>::from_column_array(&[
                x, y, z, w, x, y, z, w, x, y, z, w, x, y, z, w, a, b, c, d,
            ]);
            assert_eq!(affine.is_finite().any(), affine.lane(0).is_finite());
        });
    }

    #[test]
    fn test_inverse() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];

            let affine = Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, a, b]);
            assert_float_eq_or_panic!(
                affine.inverse(),
                Affine::from_lane_fn(|lane| affine.lane(lane).inverse())
            );

            let affine =
                Affine::<3, Wide, A>::from_column_array(&[x, y, z, w, a, b, c, d, e, f, g, h]);
            assert_float_eq_or_panic!(
                affine.inverse(),
                Affine::from_lane_fn(|lane| affine.lane(lane).inverse())
            );
        });
    }

    #[test]
    fn test_try_inverse() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];

            let affine = Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, a, b]);
            assert_panic_float_eq!(
                affine.try_inverse().unwrap(),
                Affine::from_lane_fn(|lane| affine.lane(lane).try_inverse().unwrap())
            );

            let affine =
                Affine::<3, Wide, A>::from_column_array(&[x, y, z, w, a, b, c, d, e, f, g, h]);
            assert_panic_float_eq!(
                affine.try_inverse().unwrap(),
                Affine::from_lane_fn(|lane| affine.lane(lane).try_inverse().unwrap())
            );
        });
    }

    #[test]
    fn test_inverse_or() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];

            let affine = Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, a, b]);
            assert_float_eq_or_panic!(
                affine.inverse_or(&Affine::NAN),
                Affine::from_lane_fn(|lane| affine.lane(lane).inverse_or(&Affine::NAN))
            );

            let affine =
                Affine::<3, Wide, A>::from_column_array(&[x, y, z, w, a, b, c, d, e, f, g, h]);
            assert_float_eq_or_panic!(
                affine.inverse_or(&Affine::NAN),
                Affine::from_lane_fn(|lane| affine.lane(lane).inverse_or(&Affine::NAN))
            );
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];

            let affine = Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, a, b]);
            assert_float_eq_or_panic!(
                affine.inverse_or_zero(),
                Affine::from_lane_fn(|lane| affine.lane(lane).inverse_or_zero())
            );

            let affine =
                Affine::<3, Wide, A>::from_column_array(&[x, y, z, w, a, b, c, d, e, f, g, h]);
            assert_float_eq_or_panic!(
                affine.inverse_or_zero(),
                Affine::from_lane_fn(|lane| affine.lane(lane).inverse_or_zero())
            );
        });
    }

    #[test]
    fn test_from_angle() {
        for_parameters!(|Wide: WideFloat, A, angle| {
            let _: Wide = angle;
            let angle = angle
                .abs()
                .simd_lt(Wide::splat(1e20))
                .blend(angle, Wide::ONE);

            assert_float_eq!(
                Affine::<2, Wide, A>::from_angle(angle),
                Affine::from_lane_fn(|lane| Affine::<2, T, A>::from_angle(angle.to_array()[lane])),
                r2nd <= Affine::<2, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * angle.abs(); 6]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_scale_angle() {
        for_parameters!(|Wide: WideFloat, A, x, y, angle| {
            let _: [Wide; 3] = [x, y, angle];
            let [x, y, angle] = [
                x.abs().simd_lt(Wide::splat(1e20)).blend(x, Wide::ONE),
                y.abs().simd_lt(Wide::splat(1e20)).blend(y, Wide::ONE),
                angle
                    .abs()
                    .simd_lt(Wide::splat(1e20))
                    .blend(angle, Wide::ONE),
            ];
            let scale = Vector::<2, Wide, A>::new(x, y);

            assert_float_eq!(
                Affine::<2, Wide, A>::from_scale_angle(scale, angle),
                Affine::from_lane_fn(|lane| Affine::<2, T, A>::from_scale_angle(
                    scale.lane(lane),
                    angle.to_array()[lane]
                )),
                r2nd <= Affine::<2, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * angle.abs(); 6]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_angle_translation() {
        for_parameters!(|Wide: WideFloat, A, x, y, angle| {
            let _: [Wide; 3] = [x, y, angle];
            let [x, y, angle] = [
                x.abs().simd_lt(Wide::splat(1e20)).blend(x, Wide::ONE),
                y.abs().simd_lt(Wide::splat(1e20)).blend(y, Wide::ONE),
                angle
                    .abs()
                    .simd_lt(Wide::splat(1e20))
                    .blend(angle, Wide::ONE),
            ];
            let translation = Vector::<2, Wide, A>::new(x, y);

            assert_float_eq!(
                Affine::<2, Wide, A>::from_angle_translation(angle, translation),
                Affine::from_lane_fn(|lane| Affine::<2, T, A>::from_angle_translation(
                    angle.to_array()[lane],
                    translation.lane(lane)
                )),
                r2nd <= Affine::<2, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * angle.abs(); 6]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_parameters!(|Wide: WideFloat, A, x, y, angle| {
            let _: [Wide; 3] = [x, y, angle];
            let [x, y, angle] = [
                x.abs().simd_lt(Wide::splat(1e20)).blend(x, Wide::ONE),
                y.abs().simd_lt(Wide::splat(1e20)).blend(y, Wide::ONE),
                angle
                    .abs()
                    .simd_lt(Wide::splat(1e20))
                    .blend(angle, Wide::ONE),
            ];
            let scale = Vector::<2, Wide, A>::new(x, y);
            let translation = Vector::<2, Wide, A>::new(x, y) * Wide::splat(0.7);

            assert_float_eq!(
                Affine::<2, Wide, A>::from_scale_angle_translation(scale, angle, translation),
                Affine::from_lane_fn(|lane| Affine::<2, T, A>::from_scale_angle_translation(
                    scale.lane(lane),
                    angle.to_array()[lane],
                    translation.lane(lane)
                )),
                r2nd <= Affine::<2, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * angle.abs(); 6]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_parameters!(|Wide: WideFloat, A, angle| {
            let _: Wide = angle;
            let angle = angle
                .abs()
                .simd_lt(Wide::splat(1e20))
                .blend(angle, Wide::ONE);

            assert_float_eq!(
                Affine::<3, Wide, A>::from_rotation_x(angle),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_rotation_x(
                    angle.to_array()[lane]
                )),
                r2nd <= Affine::<3, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * angle.abs(); 12]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_parameters!(|Wide: WideFloat, A, angle| {
            let _: Wide = angle;
            let angle = angle
                .abs()
                .simd_lt(Wide::splat(1e20))
                .blend(angle, Wide::ONE);

            assert_float_eq!(
                Affine::<3, Wide, A>::from_rotation_y(angle),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_rotation_y(
                    angle.to_array()[lane]
                )),
                r2nd <= Affine::<3, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * angle.abs(); 12]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_parameters!(|Wide: WideFloat, A, angle| {
            let _: Wide = angle;
            let angle = angle
                .abs()
                .simd_lt(Wide::splat(1e20))
                .blend(angle, Wide::ONE);

            assert_float_eq!(
                Affine::<3, Wide, A>::from_rotation_z(angle),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_rotation_z(
                    angle.to_array()[lane]
                )),
                r2nd <= Affine::<3, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * angle.abs(); 12]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_quat() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let quat = Quaternion::<Wide, A>::from_xyzw(x, y, z, w);

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::from_quat(quat),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_quat(quat.lane(lane)))
            );
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let angle = x ^ y;
            let [x, y, z, angle] = [
                x.abs().simd_lt(Wide::splat(1e20)).blend(x, Wide::ONE),
                y.abs().simd_lt(Wide::splat(1e20)).blend(y, Wide::ONE),
                z.abs().simd_lt(Wide::splat(1e20)).blend(z, Wide::ONE),
                angle
                    .abs()
                    .simd_lt(Wide::splat(1e20))
                    .blend(angle, Wide::ONE),
            ];
            let axis = Vector::<3, Wide, A>::new(x, y, z);

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::from_axis_angle(axis, angle),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_axis_angle(
                    axis.lane(lane),
                    angle.to_array()[lane]
                )),
                r2nd <= Affine::<3, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * angle.abs(); 12]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_euler() {
        for_parameters!(|Wide: WideFloat, A, order, a, b| {
            let _: [Wide; 2] = [a, b];
            let c = a * 0.3 + b * 0.6 + 0.3;

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::from_euler(order, a, b, c),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_euler(
                    order,
                    a.to_array()[lane],
                    b.to_array()[lane],
                    c.to_array()[lane]
                )),
                r2nd <= Affine::<3, Wide, A>::from_column_array(
                    &[Wide::splat(1e-5) * a.abs().max(b.abs()).max(c.abs()); 12]
                ),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_scale_rotation() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let scale = Vector::<3, Wide, A>::new(x, y, z) * Wide::splat(0.7);
            let w = x ^ y;
            let rotation = Quaternion::<Wide, A>::from_xyzw(x, y, z, w);

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::from_scale_rotation(scale, rotation),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_scale_rotation(
                    scale.lane(lane),
                    rotation.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_from_rotation_translation() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let rotation = Quaternion::<Wide, A>::from_xyzw(x, y, z, w);
            let translation = Vector::<3, Wide, A>::new(x, y, z) * Wide::splat(0.8);

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::from_rotation_translation(rotation, translation),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_rotation_translation(
                    rotation.lane(lane),
                    translation.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let scale = Vector::<3, Wide, A>::new(x, y, z) * Wide::splat(0.7);
            let w = x ^ y;
            let rotation = Quaternion::<Wide, A>::from_xyzw(x, y, z, w);
            let translation = Vector::<3, Wide, A>::new(x, y, z) * Wide::splat(0.8);

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::from_scale_rotation_translation(scale, rotation, translation),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::from_scale_rotation_translation(
                    scale.lane(lane),
                    rotation.lane(lane),
                    translation.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let dir = Vector::<3, Wide, A>::new(x, y, z).normalize_or(Vector::<3, Wide, A>::Z);
            let eye = dir * Wide::splat(0.3) + dir.yzx().with_z(Wide::splat(0.6));
            let up = (dir * Wide::splat(0.4) + dir.zxy().with_z(Wide::splat(0.3))).normalize();

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::look_to_lh(eye, dir, up),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::look_to_lh(
                    eye.lane(lane),
                    dir.lane(lane),
                    up.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let dir = Vector::<3, Wide, A>::new(x, y, z).normalize_or(Vector::<3, Wide, A>::Z);
            let eye = dir * Wide::splat(0.3) + dir.yzx().with_z(Wide::splat(0.6));
            let up = (dir * Wide::splat(0.4) + dir.zxy().with_z(Wide::splat(0.3))).normalize();

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::look_to_rh(eye, dir, up),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::look_to_rh(
                    eye.lane(lane),
                    dir.lane(lane),
                    up.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let eye = Vector::<3, Wide, A>::new(x, y, z).normalize_or(Vector::<3, Wide, A>::Z);
            let center = eye * Wide::splat(0.3) + eye.yzx().with_z(Wide::splat(0.6));
            let up = (eye * Wide::splat(0.4) + eye.zxy().with_z(Wide::splat(0.3))).normalize();

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::look_at_lh(eye, center, up),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::look_at_lh(
                    eye.lane(lane),
                    center.lane(lane),
                    up.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_look_at_rh() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let eye = Vector::<3, Wide, A>::new(x, y, z).normalize_or(Vector::<3, Wide, A>::Z);
            let center = eye * Wide::splat(0.3) + eye.yzx().with_z(Wide::splat(0.6));
            let up = (eye * Wide::splat(0.4) + eye.zxy().with_z(Wide::splat(0.3))).normalize();

            assert_float_eq_or_panic!(
                Affine::<3, Wide, A>::look_at_rh(eye, center, up),
                Affine::from_lane_fn(|lane| Affine::<3, T, A>::look_at_rh(
                    eye.lane(lane),
                    center.lane(lane),
                    up.lane(lane)
                ))
            );
        });
    }

    #[test]
    fn test_to_euler() {
        for_parameters!(|Wide: WideFloat, A, order, x, y| {
            let _: [Wide; 2] = [x, y];
            let z = x * 0.1 + y * 0.3 + 0.8;
            let w = x * 0.3 + y * 0.1 + 0.6;
            let affine = Affine::<3, Wide, A>::from_quat(
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w).normalize_or(Quaternion::IDENTITY),
            );

            assert_float_eq!(
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
                r2nd <= (Wide::splat(1e-5), Wide::splat(1e-5), Wide::splat(1e-5))
            );
        });
    }
}
