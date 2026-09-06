use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{Affine, Alignment, Matrix, Projective, Rotation2, Vector, utils::FloatUtils};

macro_rules! items {
    ($Wide:ident) => {
        /// A 2D rotation with all elements set to NaN (Not a Number).
        pub const NAN: Self = Self::from_cos_sin($Wide::NAN, $Wide::NAN);

        /// Creates a 2D rotation from an `angle` (in radians) rotating `+X` to
        /// `+Y`.
        #[inline]
        #[must_use]
        pub fn from_angle(angle: $Wide) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::from_cos_sin(cos, sin)
        }

        /// Returns the rotation transforming `from` to `to`.
        ///
        /// This assumes `from` and `to` are normalized.
        #[inline]
        #[must_use]
        pub fn from_rotation_arc(from: Vector<2, $Wide, A>, to: Vector<2, $Wide, A>) -> Self {
            Self::from_cos_sin(from.dot(to), from.perp_dot(to))
        }

        /// Returns the rotation transforming `from` to either `to` or `-to`,
        /// rotating up to 90 degrees.
        ///
        /// This assumes `from` and `to` are normalized.
        #[inline]
        #[must_use]
        pub fn from_rotation_arc_colinear(
            from: Vector<2, $Wide, A>,
            to: Vector<2, $Wide, A>,
        ) -> Self {
            let dot = from.dot(to);
            Self::from_cos_sin(dot, from.perp_dot(to)) * dot.signum()
        }

        /// Converts a rotation matrix to a 2D rotation represented by a complex
        /// number.
        ///
        /// This assumes `matrix` only contains rotation.
        #[inline]
        #[must_use]

        pub fn from_matrix(matrix: &Matrix<2, $Wide, A>) -> Self {
            Self(matrix.x_axis)
        }

        /// Converts an affine transform to a 2D rotation represented by a complex
        /// number.
        ///
        /// This assumes `affine` only contains rotation, and translation which is
        /// ignored.
        #[inline]
        #[must_use]
        pub fn from_affine(affine: &Affine<2, $Wide, A>) -> Self {
            Self::from_matrix(&affine.matrix)
        }

        /// Converts a projective transform to a 2D rotation represented by a
        /// complex number.
        ///
        /// This assumes `projective` only contains rotation, and translation which
        /// is ignored.
        #[inline]
        #[must_use]
        pub fn from_projective(projective: &Projective<2, $Wide, A>) -> Self {
            Self(projective.x_axis.truncate())
        }

        /// Converts a 2D rotation to an angle (in radians) rotating `+X` to `+Y`.
        ///
        /// This assumes `self` is normalized.
        #[inline]
        #[must_use]
        pub fn to_angle(self) -> $Wide {
            self.sin.atan2(self.cos)
        }

        /// Returns `true` if any element is NaN.
        #[inline]
        #[must_use]
        pub fn is_nan(self) -> $Wide {
            self.0.is_nan()
        }

        /// Returns `true` if all elements are neither infinite nor NaN.
        #[inline]
        #[must_use]
        pub fn is_finite(self) -> $Wide {
            self.0.is_finite()
        }

        /// Returns the inverse of a 2D rotation.
        ///
        /// This assumes `self` is normalized.
        ///
        /// This is the same as [`conjugate`].
        ///
        /// [`conjugate`]: Self::conjugate
        #[inline]
        #[must_use]
        pub fn inverse(self) -> Self {
            self.conjugate()
        }

        /// Returns the absolute angle (in radians) between two rotations.
        ///
        /// This assumes `self` and `other` are normalized.
        #[inline]
        #[must_use]
        pub fn angle_between(self, other: Self) -> $Wide {
            self.dot(other).acos_approx()
        }

        /// Returns the signed angle (in radians) transforming `self` to `other`.
        ///
        /// The result is in the range `-π..π`. Positive is counter-clockwise and
        /// negative is clockwise.
        ///
        /// This assumes `self` and `other` are normalized.
        ///
        /// `self.angle_to(other)` is identical to `other.angle_from(self)`.
        #[inline]
        #[must_use]
        pub fn angle_to(self, other: Self) -> $Wide {
            self.dot(other).acos_approx() * self.perp_dot(other).signum()
        }

        /// Returns the signed angle (in radians) transforming `other` to `self`.
        ///
        /// The result is in the range `-π..π`. Positive is counter-clockwise and
        /// negative is clockwise.
        ///
        /// This assumes `self` and `other` are normalized.
        ///
        /// `self.angle_from(other)` is identical to `other.angle_to(self)`.
        #[inline]
        #[must_use]
        pub fn angle_from(self, other: Self) -> $Wide {
            self.dot(other).acos_approx() * other.perp_dot(self).signum()
        }

        /// Computes the linear interpolation between two rotations, then normalizes
        /// the result.
        ///
        /// When `t` is `0`, the result is `self`. When `t` is `1`, the result is
        /// `other`.
        ///
        /// This assumes `self` and `other` are normalized.
        ///
        /// This does not interpolate the angle at a constant speed. For that use
        /// [`slerp`]. This function is more efficient as it avoids calling
        /// trigonometric functions. This function breaks when rotations are exactly
        /// 180 degrees apart, so only use this if you know that is not a
        /// possibility.
        ///
        /// [`slerp`]: Self::slerp
        #[inline]
        #[must_use]
        pub fn lerp(self, other: Self, t: $Wide) -> Self {
            (self * ($Wide::ONE - t) + other * t).normalize()
        }

        /// Computes the spherical linear interpolation between two rotations.
        ///
        /// When `t` is `0`, the result is `self`. When `t` is `1`, the result is
        /// `other`. This interpolates the angle at a constant speed, always taking
        /// the shorter path.
        ///
        /// This assumes `self` and `other` are normalized.
        #[inline]
        #[must_use]
        pub fn slerp(self, other: Self, t: $Wide) -> Self {
            // diff = other * self.inverse()
            let diff = Self::from_cos_sin(
                other.cos * self.cos + other.sin * self.sin,
                other.sin * self.cos - other.cos * self.sin,
            );

            Self::from_angle(diff.to_angle() * t) * self
        }

        /// Rotates one rotation towards another by at most `max_angle` (in
        /// radians).
        ///
        /// This assumes `self` and `other` are normalized.
        ///
        /// When `max_angle` is `0`, the result is `self`. When `max_angle` is equal
        /// to or greater than `self.angle_between(target)`, the result is `target`.
        /// When `max_angle` is negative, this rotates towards the opposite of
        /// `target`.
        #[inline]
        #[must_use]
        pub fn rotate_towards(self, target: Self, max_angle: $Wide) -> Self {
            // diff = target * self.inverse()
            let diff = Self::from_cos_sin(
                target.cos * self.cos + target.sin * self.sin,
                target.sin * self.cos - target.cos * self.sin,
            );

            // Handle negative `max_angle` by rotating towards `-target`
            let diff = diff * max_angle.signum();
            let max_angle = max_angle.abs();

            Self::from_angle(diff.to_angle().clamp(-max_angle, max_angle)) * self
        }

        /// Returns the length/magnitude of a complex number.
        #[inline]
        #[must_use]
        pub fn length(self) -> $Wide {
            self.0.length()
        }

        /// Returns `self` normalized to length `1`.
        #[inline]
        #[must_use]
        pub fn normalize(self) -> Self {
            self / self.length()
        }

        // `try_normalize` is discluded purposefully.

        /// Returns [`normalize`], or `fallback` if `self` is zero or if the
        /// result is non finite or zero.
        ///
        /// The fallback is only applied to invalid lanes. Other lanes are
        /// unaffected.
        ///
        /// [`normalize`]: Self::normalize
        #[inline]
        #[must_use]
        pub fn normalize_or(self, fallback: Self) -> Self {
            Self(self.0.normalize_or(fallback.0))
        }

        /// Simultaneously computes [`normalize`] and [`length`].
        ///
        /// [`normalize`]: Self::normalize
        /// [`length`]: Self::length
        #[inline]
        #[must_use]
        pub fn normalize_and_length(self) -> (Self, $Wide) {
            let (normalize, length) = self.0.normalize_and_length();
            (Self(normalize), length)
        }

        /// Returns whether the rotation has the length `1` or not.
        ///
        /// This uses a precision threshold of approximately `1e-4`.
        #[inline]
        #[must_use]
        pub fn is_normalized(self) -> $Wide {
            self.0.is_normalized()
        }

        /// Returns `true` if the absolute difference of all elements between `self`
        /// and `other` is less than or equal to `max_abs_diff`.
        ///
        /// This can be used to compare two rotations that should be equal, but may
        /// have a slight difference due to operations having rounding errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(self, other: Self, max_abs_diff: $Wide) -> bool {
            self.0.abs_diff_eq(other.0, max_abs_diff)
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

/// Functionality for [SoA] (Structure of Arrays) float 2D rotations.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Rotation2<Wide, A>
where
    Wide: WideFloat,
{
    items!(Wide);
}

macro_rules! impl_items {
    ($Wide:ident) => {
        #[cfg(not(doc))]
        impl<A: Alignment> Rotation2<$Wide, A> {
            items!($Wide);
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
    use crate::{
        Mat2, Proj2, Rot2, Vec2,
        test_utils::{assert_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_from_rotation_arc() {
        for_types!(|Wide: WideFloat| {
            for [from, to] in random_iter::<[Vec2<Wide>; 2]>() {
                let [from, to] = [from, to].map(|v| v.normalize_or(Vec2::<Wide>::X).normalize());

                assert_test_eq_or_panic!(
                    Rot2::<Wide>::from_rotation_arc(from, to),
                    Rot2::from_lane_fn(|lane| Rot2::<T>::from_rotation_arc(
                        from.lane(lane),
                        to.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        for_types!(|Wide: WideFloat| {
            for [from, to] in random_iter::<[Vec2<Wide>; 2]>() {
                let [from, to] = [from, to].map(|v| v.normalize_or(Vec2::<Wide>::X).normalize());

                assert_test_eq_or_panic!(
                    Rot2::<Wide>::from_rotation_arc_colinear(from, to),
                    Rot2::from_lane_fn(|lane| Rot2::<T>::from_rotation_arc_colinear(
                        from.lane(lane),
                        to.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|Wide: WideFloat| {
            for matrix in random_iter::<Wide>()
                .map(Mat2::<Wide>::from_angle)
                .chain(random_iter())
            {
                assert_test_eq_or_panic!(
                    Rot2::<Wide>::from_matrix(&matrix),
                    Rot2::from_lane_fn(|lane| Rot2::<T>::from_matrix(&matrix.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|Wide: WideFloat| {
            for projective in random_iter::<Wide>()
                .map(Proj2::<Wide>::from_angle)
                .chain(random_iter())
            {
                assert_test_eq_or_panic!(
                    Rot2::<Wide>::from_projective(&projective),
                    Rot2::from_lane_fn(|lane| Rot2::<T>::from_projective(&projective.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_to_angle() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                let angle = (angle % 3.0) & angle.is_finite();

                assert_test_eq!(
                    Rot2::<Wide>::from_angle(angle).to_angle(),
                    angle,
                    abs <= Wide::splat(1e-4)
                );
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|Wide: WideFloat| {
            for rotation in random_iter::<Rot2<Wide>>() {
                let rotation = rotation.normalize_or(Rot2::IDENTITY).normalize();

                assert_test_eq!(
                    rotation * rotation.inverse(),
                    Rot2::IDENTITY,
                    abs <= Wide::splat(1e-4),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_types!(|Wide: WideFloat| {
            for [a, b] in random_iter::<[Rot2<Wide>; 2]>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rot2::IDENTITY).normalize());

                assert_test_eq!(
                    a.angle_between(b),
                    a.0.angle_between(b.0),
                    abs <= Wide::splat(1e-2)
                );
            }
        });
    }

    #[test]
    fn test_angle_to() {
        for_types!(|Wide: WideFloat| {
            for [a, b] in random_iter::<[Rot2<Wide>; 2]>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rot2::IDENTITY).normalize());

                assert_test_eq!(a.angle_to(b), a.0.angle_to(b.0), abs <= Wide::splat(1e-2));
            }
        });
    }

    #[test]
    fn test_angle_from() {
        for_types!(|Wide: WideFloat| {
            for [a, b] in random_iter::<[Rot2<Wide>; 2]>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rot2::IDENTITY).normalize());

                assert_test_eq!(
                    a.angle_from(b),
                    a.0.angle_from(b.0),
                    abs <= Wide::splat(1e-2)
                );
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Rot2<Wide>; 2], Wide)>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rot2::IDENTITY).normalize());

                assert_test_eq_or_panic!(
                    a.lerp(b, t),
                    Rot2::from_lane_fn(|lane| a.lane(lane).lerp(b.lane(lane), t.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Rot2<Wide>; 2], Wide)>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rot2::IDENTITY).normalize());
                let t = (t % 5.0) & t.is_finite();

                assert_test_eq!(
                    a.slerp(b, Wide::ZERO),
                    a,
                    abs <= Wide::splat(1e-3),
                    0.0 = -0.0
                );
                assert_test_eq!(
                    a.slerp(b, Wide::ONE),
                    b,
                    abs <= Wide::splat(1e-3),
                    0.0 = -0.0
                );

                assert_test_eq!(
                    a.slerp(b, t),
                    a * Rot2::<Wide>::from_angle(a.angle_to(b) * t),
                    abs <= Wide::splat(2e-3),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_types!(|Wide: WideFloat| {
            for ([current, target], max_angle) in random_iter::<([Rot2<Wide>; 2], Wide)>() {
                let [current, target] =
                    [current, target].map(|r| r.normalize_or(Rot2::IDENTITY).normalize());
                let max_angle = (max_angle % 10.0) & max_angle.is_finite();

                for lane in 0..LANES {
                    if max_angle.is_sign_negative().to_array()[lane].is_sign_negative() {
                        if current.to_array()[lane] != target.to_array()[lane] {
                            assert_test_eq!(
                                current.rotate_towards(target, max_angle).lane(lane),
                                current.rotate_towards(-target, -max_angle).lane(lane)
                            )
                        } else {
                            assert_test_eq!(
                                current
                                    .rotate_towards(target, max_angle)
                                    .angle_between(-target)
                                    .to_array()[lane],
                                (current.angle_between(-target) - max_angle.abs())
                                    .max(Wide::ZERO)
                                    .to_array()[lane],
                                abs <= 1e-3,
                                0.0 = -0.0
                            );
                        }
                    } else {
                        assert_test_eq!(
                            current
                                .rotate_towards(target, max_angle)
                                .angle_between(target)
                                .to_array()[lane],
                            (current.angle_between(target) - max_angle)
                                .max(Wide::ZERO)
                                .to_array()[lane],
                            abs <= 1e-3,
                            0.0 = -0.0
                        );
                    }
                }
            }
        });
    }
}
