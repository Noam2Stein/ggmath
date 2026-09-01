use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Affine, Alignment, EulerRot, Length, Matrix, Projective, Rotor, Vector,
    length::TwoOrThree,
    utils::{FloatUtils, specialize_23, transmute_generic},
};

macro_rules! items {
    ($Wide:ident) => {
        /// A rotor with all elements set to NaN (Not a Number).
        pub const NAN: Self = Self::NAN_INTERNAL_IMPL;

        /// The implementation of [`Self::NAN`].
        ///
        /// Because of type system limitations, this implementation looks crazy.
        /// Use a separate constant so that IDEs do not show the implementation.
        const NAN_INTERNAL_IMPL: Self = match N {
            // SAFETY: We are transmuting a type to itself
            2 => unsafe {
                transmute_generic::<Rotor<2, $Wide, A>, Rotor<N, $Wide, A>>(Rotor::<2, $Wide, A>(
                    Vector::<2, $Wide, A>::NAN,
                ))
            },
            // SAFETY: We are transmuting a type to itself
            3 => unsafe {
                transmute_generic::<Rotor<3, $Wide, A>, Rotor<N, $Wide, A>>(Rotor::<3, $Wide, A>(
                    Vector::<4, $Wide, A>::NAN,
                ))
            },
            _ => unreachable!(),
        };

        /// Returns the minimal rotation transforming `from` to `to`.
        ///
        /// The rotation is in the plane spanned by `from` and `to`. Rotates up
        /// to 180 degrees.
        ///
        /// When `from≈to` this is only accurate to about `0.001` (for `f32`).
        ///
        /// `from` and `to` must be normalized.
        #[inline]
        #[must_use]
        pub fn from_rotation_arc(from: Vector<N, $Wide, A>, to: Vector<N, $Wide, A>) -> Self {
            specialize_23!(Rotor::<N, $Wide, A>::from_rotation_arc_backend(from, to))
        }

        /// Returns the minimal rotation transforming `from` to either `to` or
        /// `-to`. This rotates `from` so that it is colinear with `to`.
        ///
        /// The rotation is in the plane spanned by `from` and `to`. Rotates up
        /// to 90 degrees.
        ///
        /// When `from≈to` or `from≈-to` this is only accurate to about `0.001`
        /// (for `f32`).
        ///
        /// `from` and `to` must be normalized.
        #[inline]
        #[must_use]
        pub fn from_rotation_arc_colinear(
            from: Vector<N, $Wide, A>,
            to: Vector<N, $Wide, A>,
        ) -> Self {
            specialize_23!(Rotor::<N, $Wide, A>::from_rotation_arc_colinear_backend(
                from, to
            ))
        }

        /// Converts a rotation matrix to a rotor.
        #[inline]
        #[must_use]
        pub fn from_matrix(matrix: &Matrix<N, $Wide, A>) -> Self {
            specialize_23!(Rotor::<N, $Wide, A>::from_matrix_backend(matrix))
        }

        /// Converts an affine transform with rotation to a rotor.
        ///
        /// This function assumes the transform only contains rotation, and
        /// possibly translation, which is ignored.
        #[inline]
        #[must_use]
        pub fn from_affine(affine: &Affine<N, $Wide, A>) -> Self {
            Self::from_matrix(&affine.matrix)
        }

        /// Converts a projective transform with rotation to a rotor.
        ///
        /// This function assumes the transform only contains rotation, and
        /// possibly translation, which is ignored.
        #[inline]
        #[must_use]
        pub fn from_projective(projective: &Projective<N, $Wide, A>) -> Self {
            specialize_23!(Rotor::<N, $Wide, A>::from_projective_backend(projective))
        }

        /// Returns `true` if any element is NaN.
        #[inline]
        #[must_use]
        pub fn is_nan(self) -> $Wide {
            specialize_23!(Rotor::<N, $Wide, A>::is_nan_backend(self))
        }

        /// Returns `true` if all elements are neither infinite nor NaN.
        #[inline]
        #[must_use]
        pub fn is_finite(self) -> $Wide {
            specialize_23!(Rotor::<N, $Wide, A>::is_finite_backend(self))
        }

        /// Returns the inverse of a rotor.
        ///
        /// This is identical to [`conjugate`]. Use whichever function makes
        /// your intentions clearer.
        ///
        /// [`conjugate`]: Self::conjugate
        #[inline]
        #[must_use]
        pub fn inverse(self) -> Self {
            self.conjugate()
        }

        /// Returns the angle (in radians) for the minimal rotation for
        /// transforming `self` into `other`.
        ///
        /// `self` and `other` must be normalized.
        #[inline]
        #[must_use]
        pub fn angle_between(self, other: Self) -> $Wide {
            let half_angle = self.dot(other).abs().acos_approx();
            half_angle + half_angle
        }

        /// Computes the linear interpolation between `self` and `other` based
        /// on the value `t`, then normalizes the result.
        ///
        /// When `t` is 0, the result is `self`.  When `t` is 1, the result is
        /// `rhs`.
        ///
        /// Note that this does *not* interpolate the angle. For that, use
        /// [`slerp`].
        ///
        /// [`slerp`]: Self::slerp
        #[inline]
        #[must_use]
        pub fn lerp(self, other: Self, t: $Wide) -> Self {
            let other = self.dot(other).is_sign_negative().select(-other, other);

            (self * ($Wide::ONE - t) + other * t).normalize()
        }

        /// Computes the spherical linear interpolation between `self` and
        /// `other` based on the value `t`.
        ///
        /// When `t` is `0`, the result is `self`.  When `t` is `1`, the result
        /// is `other`.
        ///
        /// This function assumes both rotors are normalized.
        #[inline]
        #[must_use]
        pub fn slerp(self, other: Self, t: $Wide) -> Self {
            specialize_23!(Rotor::<N, $Wide, A>::slerp_backend(self, other, t))
        }

        /// Rotates `self` towards `target` by at most `max_angle` (in radians).
        ///
        /// When `max_angle` is `0`, the result is `self`. When `max_angle` is
        /// equal to or greater than `self.angle_between(target)`, the result is
        /// `target`. When `max_angle` is negative, rotates towards the opposite
        /// of `target`.
        ///
        /// This assumes `self` and `target` are normalized.
        #[inline]
        #[must_use]
        pub fn rotate_towards(self, target: Self, max_angle: $Wide) -> Self {
            let angle = self.angle_between(target);
            let t = (max_angle / angle).clamp(-$Wide::ONE, $Wide::ONE);

            angle.simd_le(1e-4).select(target, self.slerp(target, t))
        }

        /// Returns the length/magnitude of `self`.
        #[inline]
        #[must_use]
        pub fn length(self) -> $Wide {
            specialize_23!(Rotor::<N, $Wide, A>::length_backend(self))
        }

        /// Returns `self` normalized to length `1`.
        #[inline]
        #[must_use]
        pub fn normalize(self) -> Self {
            self / self.length()
        }

        // `try_normalize` is exluded on purpose.

        /// Returns [`normalize`], or `fallback` if `self` is zero or if the
        /// result is non finite or zero.
        ///
        /// The fallback is only applied invalid lanes. Other lanes are
        /// not affected.
        ///
        /// [`normalize`]: Self::normalize
        #[inline]
        #[must_use]
        pub fn normalize_or(self, fallback: Self) -> Self {
            specialize_23!(Rotor::<N, $Wide, A>::normalize_or_backend(self, fallback))
        }

        /// Simultaneously computes [`normalize`] and [`length`].
        ///
        /// This assumes the rotor is not zero (so the output for that will be
        /// garbage). Consider manually checking for that case.
        ///
        /// [`normalize`]: Self::normalize
        /// [`length`]: Self::length
        #[inline]
        #[must_use]
        pub fn normalize_and_length(self) -> (Self, $Wide) {
            let length = self.length();
            (self / length, length)
        }

        /// Returns whether the rotor has the length 1 or not.
        ///
        /// This uses a precision threshold of approximately `1e-4`.
        #[inline]
        #[must_use]
        pub fn is_normalized(self) -> $Wide {
            specialize_23!(Rotor::<N, $Wide, A>::is_normalized_backend(self))
        }

        /// Returns `true` if for all lanes, the absolute difference of all
        /// elements between `self` and `other` is less than or equal to
        /// `max_abs_diff`.
        ///
        /// This can be used to compare two rotors that should be equal, but may
        /// have a slight difference due to operations having rounding errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(self, other: Self, max_abs_diff: $Wide) -> bool {
            specialize_23!(Rotor::<N, $Wide, A>::abs_diff_eq_backend(
                self,
                other,
                max_abs_diff
            ))
        }
    };
}

macro_rules! items_2 {
    ($Wide:ident) => {
        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_angle(angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (xy, s) = half_angle.sin_cos();

            Self::from_raw_elements(xy, s)
        }

        /// Converts a 2D rotor to an angle (in radians) rotating `+X` to `+Y`.
        ///
        /// This assumes the rotor is normalized.
        #[inline]
        #[must_use]
        pub fn to_angle(self) -> $Wide {
            let half_angle = self.xy.atan2(self.s);
            half_angle + half_angle
        }
    };
}

macro_rules! items_3 {
    ($Wide:ident) => {
        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xy(angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (sin, cos) = half_angle.sin_cos();
            Self::from_raw_elements(sin, $Wide::ZERO, $Wide::ZERO, cos)
        }

        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xz(angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (sin, cos) = half_angle.sin_cos();
            Self::from_raw_elements($Wide::ZERO, sin, $Wide::ZERO, cos)
        }

        /// Creates a rotor from an `angle` (in radians) rotating `+Y` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_yz(angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (sin, cos) = half_angle.sin_cos();
            Self::from_raw_elements($Wide::ZERO, $Wide::ZERO, sin, cos)
        }

        /// Creates a rotor from a rotation `axis` and `angle` (in radians),
        /// using the right-hand rule.
        ///
        /// This assumes `axis` is normalized.
        ///
        /// If you are using this to initialize a static rotation, consider
        /// using [`from_rotation_arc`] instead. That function makes it clearer
        /// what direction the rotation happens in, whereas this function
        /// requires remembering the right-hand rule.
        ///
        /// [`from_rotation_arc`]: Self::from_rotation_arc
        #[inline]
        #[must_use]
        pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
            let half_angle = angle * $Wide::HALF;
            let (sin, cos) = half_angle.sin_cos();
            let [yz, zx, xy] = (axis * sin).to_array();
            Self::from_raw_elements(xy, -zx, yz, cos)
        }

        /// Creates a rotor that rotates `scaled_axis.length()` radians around
        /// `scaled_axis.normalize()`, using the right-hand rule.
        ///
        /// If you are using this to initialize a static rotation, consider
        /// using [`from_rotation_arc`] instead. That function makes it clearer
        /// what direction the rotation happens in, whereas this function
        /// requires remembering the right-hand rule.
        ///
        /// [`from_rotation_arc`]: Self::from_rotation_arc
        #[inline]
        #[must_use]
        pub fn from_scaled_axis(scaled_axis: Vector<3, $Wide, A>) -> Self {
            let (axis, angle) = scaled_axis.normalize_and_length();

            let half_angle = angle * $Wide::HALF;
            let (sin, cos) = half_angle.sin_cos();
            let [yz, zx, xy] = (axis * sin).to_array();

            let angle_is_not_zero = angle.simd_ne($Wide::ZERO);
            Self::from_raw_elements(
                xy & angle_is_not_zero,
                -zx & angle_is_not_zero,
                yz & angle_is_not_zero,
                cos,
            )
        }

        /// Creates a rotor from an Euler rotation order/sequence and angles (in
        /// radians).
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

            if order.parity_even {
                angles.y = -angles.y;
            }

            let ti = angles.x * $Wide::HALF;
            let tj = angles.y * $Wide::HALF;
            let th = angles.z * $Wide::HALF;
            let (si, ci) = ti.sin_cos();
            let (sj, cj) = tj.sin_cos();
            let (sh, ch) = th.sin_cos();
            let cc = ci * ch;
            let cs = ci * sh;
            let sc = si * ch;
            let ss = si * sh;

            let parity = if order.parity_even {
                -$Wide::ZERO
            } else {
                $Wide::ZERO
            };

            let mut result = Vector::ZERO;

            if order.initial_repeated {
                result[i] = cj * (cs + sc);
                result[j] = sj * (cc + ss) ^ parity;
                result[k] = sj * (cs - sc);
                result[3] = cj * (cc - ss);
            } else {
                result[i] = cj * sc - sj * cs;
                result[j] = (cj * ss + sj * cc) ^ parity;
                result[k] = cj * cs - sj * sc;
                result[3] = cj * cc + sj * ss;
            }

            Self(result)
        }

        /// Creates a rotor from a facing direction and an up direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_to_lh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_lh(dir, up))
        }

        /// Creates a rotor from a facing direction and an up direction.
        ///
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_to_rh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_rh(dir, up))
        }

        /// Creates a rotor from a camera position, a focal point and an up
        /// direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_at_lh(
            eye: Vector<3, $Wide, A>,
            center: Vector<3, $Wide, A>,
            up: Vector<3, $Wide, A>,
        ) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_at_lh(eye, center, up))
        }

        /// Creates a rotor from a camera position, a focal point and an up
        /// direction.
        ///
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_at_rh(
            eye: Vector<3, $Wide, A>,
            center: Vector<3, $Wide, A>,
            up: Vector<3, $Wide, A>,
        ) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_at_rh(eye, center, up))
        }

        /// Converts the rotor `self` to a normalized rotation axis and an angle
        /// (in radians), using the right-hand rule.
        #[inline]
        #[must_use]
        pub fn to_axis_angle(self) -> (Vector<3, $Wide, A>, $Wide) {
            let bivector_rh = Vector::<3, $Wide, A>::new(self.yz, -self.xz, self.xy);
            let (axis, bivector_length) = bivector_rh.normalize_and_length();

            let half_angle = bivector_length.atan2(self.s);
            let angle = half_angle + half_angle;
            let angle_is_not_zero = bivector_length.simd_ge(1e-8);

            (
                Vector::<3, $Wide, A>::new(
                    angle_is_not_zero.select(axis.x, $Wide::ONE),
                    axis.y & angle_is_not_zero,
                    axis.z & angle_is_not_zero,
                ),
                angle & angle_is_not_zero,
            )
        }

        // Converts the rotor `self` to a rotation axis scaled by an angle (in
        /// radians), using the right-hand rule.
        #[inline]
        #[must_use]
        pub fn to_scaled_axis(self) -> Vector<3, $Wide, A> {
            let bivector_rh = Vector::<3, $Wide, A>::new(self.yz, -self.xz, self.xy);
            let (axis, bivector_length) = bivector_rh.normalize_and_length();

            let half_angle = bivector_length.atan2(self.s);
            let angle = half_angle + half_angle;
            let angle_is_not_zero = bivector_length.simd_ge(1e-8);

            (axis * angle) & angle_is_not_zero
        }

        /// Returns the Euler angles forming `self` for the given Euler rotation
        /// order/sequence.
        #[inline]
        #[must_use]
        pub fn to_euler(self, order: EulerRot) -> ($Wide, $Wide, $Wide) {
            Matrix::<3, $Wide, A>::from_rotor(self).to_euler(order)
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

#[cfg(doc)]
#[expect(private_bounds)]
impl<const N: usize, Wide, A: Alignment> Rotor<N, Wide, A>
where
    Length<N>: TwoOrThree,
    Wide: WideFloat,
{
    items!(Wide);
}

#[cfg(doc)]
impl<Wide, A: Alignment> Rotor<2, Wide, A>
where
    Wide: WideFloat,
{
    items_2!(Wide);
}

#[cfg(doc)]
impl<Wide, A: Alignment> Rotor<3, Wide, A>
where
    Wide: WideFloat,
{
    items_3!(Wide);
}

macro_rules! impl_items {
    ($Wide:ident) => {
        #[cfg(not(doc))]
        #[expect(private_bounds)]
        impl<const N: usize, A: Alignment> Rotor<N, $Wide, A>
        where
            Length<N>: TwoOrThree,
        {
            items!($Wide);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Rotor<2, $Wide, A> {
            items_2!($Wide);

            #[inline(always)]
            fn from_rotation_arc_backend(
                from: Vector<2, $Wide, A>,
                to: Vector<2, $Wide, A>,
            ) -> Self {
                // The formula for this rotor is `normalize(1+(from)(to))`. This formula
                // breaks for `to = -from`, and gets less stable as we get closer to
                // that value. To fix this, if the angle is greater than 90 degrees, we
                // use `-to` instead of `to` then add 180 degrees to the resulting
                // rotor.

                let dot = from.dot(to);
                let wedge = from.wedge(to);
                let dot_abs_p1 = dot.abs() + $Wide::ONE;
                let dot_is_negative = dot.is_sign_negative();

                Self::from_raw_elements(
                    dot_is_negative.select(dot_abs_p1, wedge),
                    dot_is_negative.select(wedge, dot_abs_p1),
                )
                .normalize()
            }

            #[inline(always)]
            fn from_rotation_arc_colinear_backend(
                from: Vector<2, $Wide, A>,
                to: Vector<2, $Wide, A>,
            ) -> Self {
                let dot = from.dot(to);
                let dot_signbit = dot & -$Wide::ZERO;
                let dot = dot ^ dot_signbit;
                let to = to ^ dot_signbit;

                Self::from_raw_elements(from.wedge(to), $Wide::ONE + dot).normalize()
            }

            #[inline(always)]
            fn from_matrix_backend(matrix: &Matrix<2, $Wide, A>) -> Self {
                let cos = matrix.x_axis.x;
                let sin = matrix.x_axis.y;
                let cos_abs_p1 = cos.abs() + $Wide::ONE;
                let cos_is_negative = cos.is_sign_negative();

                Self::from_raw_elements(
                    cos_is_negative.select(cos_abs_p1, sin),
                    cos_is_negative.select(sin, cos_abs_p1),
                )
                .normalize()
            }

            #[inline(always)]
            fn from_projective_backend(projective: &Projective<2, $Wide, A>) -> Self {
                let cos = projective.x_axis.x;
                let sin = projective.x_axis.y;
                let cos_abs_p1 = cos.abs() + $Wide::ONE;
                let cos_is_negative = cos.is_sign_negative();

                Self::from_raw_elements(
                    cos_is_negative.select(cos_abs_p1, sin),
                    cos_is_negative.select(sin, cos_abs_p1),
                )
                .normalize()
            }

            #[inline(always)]
            fn is_nan_backend(self) -> $Wide {
                self.0.is_nan()
            }

            #[inline(always)]
            fn is_finite_backend(self) -> $Wide {
                self.0.is_finite()
            }

            #[inline(always)]
            fn slerp_backend(self, other: Self, t: $Wide) -> Self {
                let dot = self.dot(other);

                // A rotation can be represented by two rotors: `r` and `-r`. The slerp
                // path between `self` and `other` will be different from the path
                // between `-self` and `other`. One path will take the long way around
                // and one will take the short way. In order to correct for this, the
                // `dot` product between `self` and `other` should be positive. If the
                // `dot` product is negative, slerp between `self` and `-other`.
                let dot_signbit = dot & -$Wide::ZERO;
                let other = Self(other.0 ^ dot_signbit);
                let dot = dot ^ dot_signbit;

                let half_angle = dot.acos_approx();
                let rotation = half_angle * self.0.wedge(other.0).signum() * t;

                let (sin, cos) = rotation.sin_cos();
                Self::from_raw_elements(self.xy * cos - self.s * sin, self.xy * sin + self.s * cos)
            }

            #[inline(always)]
            fn length_backend(self) -> $Wide {
                self.0.length()
            }

            #[inline(always)]
            fn normalize_or_backend(self, fallback: Self) -> Self {
                Self(self.0.normalize_or(fallback.0))
            }

            #[inline(always)]
            fn is_normalized_backend(self) -> $Wide {
                self.0.is_normalized()
            }

            #[inline(always)]
            fn abs_diff_eq_backend(self, other: Self, max_abs_diff: $Wide) -> bool {
                self.0.abs_diff_eq(other.0, max_abs_diff)
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Rotor<3, $Wide, A> {
            items_3!($Wide);

            #[inline(always)]
            fn from_rotation_arc_backend(
                from: Vector<3, $Wide, A>,
                to: Vector<3, $Wide, A>,
            ) -> Self {
                // Based on https://github.com/bitshifter/glam-rs

                let almost_one = $Wide::ONE - 2.0 * $Wide::EPSILON;

                let dot = from.dot(to);
                dot.simd_gt(almost_one).select(
                    // 0° singularity: from ≈ to.
                    Self::IDENTITY,
                    dot.simd_lt(-almost_one).select(
                        {
                            // 180° singularity: from ≈ -to.
                            // Half a turn = 𝛕/2 = 180°.

                            // Construct any rotation plane parallel to `from`
                            let sign = from.z.signum();
                            let tmp = -$Wide::ONE / (sign + from.z);
                            let xy = -from.y;
                            let xz = -sign - from.y * from.y * tmp;
                            let yz = from.x * from.y * tmp;

                            // sin(angle/2) = sin(𝛕/4) = 1
                            // cos(angle/2) = cos(𝛕/4) = 0
                            Self::from_raw_elements(xy, xz, yz, $Wide::ZERO)
                        },
                        {
                            // This computes `xy, zx, yz`, so we flip `y` to make it `xz`
                            let bivector = (from.zxy() * to - from * to.zxy()).yxz();
                            let bivector = bivector
                                * Vector::<3, $Wide, A>::new($Wide::ONE, -$Wide::ONE, $Wide::ONE);

                            Self(bivector.extend($Wide::ONE + dot).normalize())
                        },
                    ),
                )
            }

            #[inline(always)]
            fn from_rotation_arc_colinear_backend(
                from: Vector<3, $Wide, A>,
                to: Vector<3, $Wide, A>,
            ) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs

                let almost_one = $Wide::ONE - 2.0 * $Wide::EPSILON;

                let dot = from.dot(to);
                let dot_signbit = dot & -$Wide::ZERO;
                let dot = dot ^ dot_signbit;
                let to = to ^ dot_signbit;

                dot.simd_gt(almost_one).select(
                    // 0° singularity: from ≈ to.
                    Self::IDENTITY,
                    {
                        // This computes `xy, zx, yz`, so we flip `y` to make it `xz`
                        let bivector = (from.zxy() * to - from * to.zxy()).yxz();
                        let bivector = bivector
                            * Vector::<3, $Wide, A>::new($Wide::ONE, -$Wide::ONE, $Wide::ONE);

                        Self(bivector.extend($Wide::ONE + dot).normalize())
                    },
                )
            }

            #[inline(always)]
            fn from_matrix_backend(matrix: &Matrix<3, $Wide, A>) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs `Quat::from_rotation_axes`
                // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`

                let [xx, xy, xz] = matrix.x_axis.to_array();
                let [yx, yy, yz] = matrix.y_axis.to_array();
                let [zx, zy, zz] = matrix.z_axis.to_array();

                zz.simd_le($Wide::ZERO).select(
                    {
                        // x^2 + y^2 >= z^2 + w^2
                        let dif10 = yy - xx;
                        let omm22 = $Wide::ONE - zz;

                        dif10.simd_le($Wide::ZERO).select(
                            {
                                // x^2 >= y^2
                                let four_xsq = omm22 - dif10;
                                let inv4x = $Wide::HALF / four_xsq.sqrt();

                                Self::from_raw_elements(xz + zx, -xy - yx, four_xsq, yz - zy)
                                    * inv4x
                            },
                            {
                                // y^2 >= x^2
                                let four_ysq = omm22 + dif10;
                                let inv4y = $Wide::HALF / four_ysq.sqrt();

                                Self::from_raw_elements(yz + zy, -four_ysq, xy + yx, zx - xz)
                                    * inv4y
                            },
                        )
                    },
                    {
                        // z^2 + w^2 >= x^2 + y^2
                        let sum10 = yy + xx;
                        let opm22 = $Wide::ONE + zz;

                        sum10.simd_le($Wide::ZERO).select(
                            {
                                // z^2 >= w^2
                                let four_zsq = opm22 - sum10;
                                let inv4z = $Wide::HALF / four_zsq.sqrt();

                                Self::from_raw_elements(four_zsq, -yz - zy, xz + zx, xy - yx)
                                    * inv4z
                            },
                            {
                                // w^2 >= z^2
                                let four_wsq = opm22 + sum10;
                                let inv4w = $Wide::HALF / four_wsq.sqrt();

                                Self::from_raw_elements(xy - yx, xz - zx, yz - zy, four_wsq) * inv4w
                            },
                        )
                    },
                )
            }

            #[inline(always)]
            fn from_projective_backend(projective: &Projective<3, $Wide, A>) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs `Quat::from_rotation_axes`
                // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`

                let [xx, xy, xz, _] = projective.x_axis.to_array();
                let [yx, yy, yz, _] = projective.y_axis.to_array();
                let [zx, zy, zz, _] = projective.z_axis.to_array();

                zz.simd_le($Wide::ZERO).select(
                    {
                        // x^2 + y^2 >= z^2 + w^2
                        let dif10 = yy - xx;
                        let omm22 = $Wide::ONE - zz;

                        dif10.simd_le($Wide::ZERO).select(
                            {
                                // x^2 >= y^2
                                let four_xsq = omm22 - dif10;
                                let inv4x = $Wide::HALF / four_xsq.sqrt();

                                Self::from_raw_elements(xz + zx, -xy - yx, four_xsq, yz - zy)
                                    * inv4x
                            },
                            {
                                // y^2 >= x^2
                                let four_ysq = omm22 + dif10;
                                let inv4y = $Wide::HALF / four_ysq.sqrt();

                                Self::from_raw_elements(yz + zy, -four_ysq, xy + yx, zx - xz)
                                    * inv4y
                            },
                        )
                    },
                    {
                        // z^2 + w^2 >= x^2 + y^2
                        let sum10 = yy + xx;
                        let opm22 = $Wide::ONE + zz;

                        sum10.simd_le($Wide::ZERO).select(
                            {
                                // z^2 >= w^2
                                let four_zsq = opm22 - sum10;
                                let inv4z = $Wide::HALF / four_zsq.sqrt();

                                Self::from_raw_elements(four_zsq, -yz - zy, xz + zx, xy - yx)
                                    * inv4z
                            },
                            {
                                // w^2 >= z^2
                                let four_wsq = opm22 + sum10;
                                let inv4w = $Wide::HALF / four_wsq.sqrt();

                                Self::from_raw_elements(xy - yx, xz - zx, yz - zy, four_wsq) * inv4w
                            },
                        )
                    },
                )
            }

            #[inline(always)]
            fn is_nan_backend(self) -> $Wide {
                self.0.is_nan()
            }

            #[inline(always)]
            fn is_finite_backend(self) -> $Wide {
                self.0.is_finite()
            }

            #[inline(always)]
            fn slerp_backend(self, other: Self, t: $Wide) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs
                // See http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/

                // Note that a rotation can be represented by two rotors: `r` and `-r`.
                // The slerp path between `r` and `other` will be different from the
                // path between `-r` and `other`. One path will take the long way around
                // and one will take the short way. In order to correct for this, the
                // `dot` product between `self` and `other` should be positive. If the
                // `dot` product is negative, slerp between `self` and `-other`.
                let dot = self.dot(other);
                let dot_signbit = dot & -$Wide::ZERO;
                let other = Self(other.0 ^ dot_signbit);
                let dot = dot ^ dot_signbit;

                dot.simd_gt($Wide::ONE - $Wide::EPSILON).select(
                    // If above threshold, perform linear interpolation to avoid divide by zero.
                    (self * ($Wide::ONE - t) + other * t).normalize(),
                    {
                        let half_angle = dot.acos_approx();

                        let self_factor = (($Wide::ONE - t) * half_angle).sin();
                        let other_factor = (t * half_angle).sin();

                        (self * self_factor + other * other_factor).normalize()
                    },
                )
            }

            #[inline(always)]
            fn length_backend(self) -> $Wide {
                self.0.length()
            }

            #[inline(always)]
            fn normalize_or_backend(self, fallback: Self) -> Self {
                Self(self.0.normalize_or(fallback.0))
            }

            #[inline(always)]
            fn is_normalized_backend(self) -> $Wide {
                self.0.is_normalized()
            }

            #[inline(always)]
            fn abs_diff_eq_backend(self, other: Self, max_abs_diff: $Wide) -> bool {
                self.0.abs_diff_eq(other.0, max_abs_diff)
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
    use wide::f32x4;

    use crate::{
        EulerRot, Mat2, Mat3, Rot2, Rot3, Rotor, Unaligned, Vec2, Vec3, Vector,
        test_utils::{assert_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        assert_test_eq!(
            Rot2::<f32x4>::NAN,
            Rot2::from_raw_elements(f32x4::NAN, f32x4::NAN)
        );
        assert_test_eq!(
            Rot3::<f32x4>::NAN,
            Rot3::from_raw_elements(f32x4::NAN, f32x4::NAN, f32x4::NAN, f32x4::NAN)
        );
    }

    #[test]
    fn test_from_rotation_arc() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for [from, to] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>()
                .flat_map(|from_to| [from_to, from_to.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Rotor::<N, Wide, Unaligned>::from_rotation_arc(from, to),
                    Rotor::from_lane_fn(|lane| Rotor::<N, T, Unaligned>::from_rotation_arc(
                        from.lane(lane),
                        to.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for [from, to] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>()
                .flat_map(|from_to| [from_to, from_to.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Rotor::<N, Wide, Unaligned>::from_rotation_arc_colinear(from, to),
                    Rotor::from_lane_fn(|lane| {
                        Rotor::<N, T, Unaligned>::from_rotation_arc_colinear(
                            from.lane(lane),
                            to.lane(lane),
                        )
                    })
                );
            }
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|Wide: WideFloat| {
            for xy in random_iter::<Wide>() {
                let matrix = Mat2::<Wide>::from_angle(xy);

                assert_test_eq_or_panic!(
                    Rot2::<Wide>::from_matrix(&matrix),
                    Rot2::from_lane_fn(|lane| Rot2::<T>::from_matrix(&matrix.lane(lane)))
                );
            }
            for [xy, xz, yz] in random_iter::<[Wide; 3]>() {
                let matrix = Mat3::<Wide>::from_rotation_xy(xy)
                    * Mat3::<Wide>::from_rotation_xz(xz)
                    * Mat3::<Wide>::from_rotation_yz(yz);

                assert_test_eq_or_panic!(
                    Rot3::<Wide>::from_matrix(&matrix),
                    Rot3::from_lane_fn(|lane| Rot3::<T>::from_matrix(&matrix.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|Wide: WideFloat| {
            for xy in random_iter::<Wide>() {
                let matrix = Mat2::<Wide>::from_angle(xy);

                assert_test_eq_or_panic!(
                    Rot2::<Wide>::from_matrix(&matrix),
                    Rot2::from_lane_fn(|lane| Rot2::<T>::from_matrix(&matrix.lane(lane)))
                );
            }
            for [xy, xz, yz] in random_iter::<[Wide; 3]>() {
                let matrix = Mat3::<Wide>::from_rotation_xy(xy)
                    * Mat3::<Wide>::from_rotation_xz(xz)
                    * Mat3::<Wide>::from_rotation_yz(yz);

                assert_test_eq_or_panic!(
                    Rot3::<Wide>::from_matrix(&matrix),
                    Rot3::from_lane_fn(|lane| Rot3::<T>::from_matrix(&matrix.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_is_nan() {
        for [x, y, z, w] in random_iter::<[f32x4; 4]>() {
            assert_eq!(
                Rot2::from_raw_elements(x, y).is_nan(),
                x.is_nan() | y.is_nan()
            );
            assert_eq!(
                Rot3::from_raw_elements(x, y, z, w).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan()
            );
        }
    }

    #[test]
    fn test_is_finite() {
        for [x, y, z, w] in random_iter::<[f32x4; 4]>() {
            assert_eq!(
                Rot2::from_raw_elements(x, y).is_finite(),
                x.is_finite() & y.is_finite()
            );
            assert_eq!(
                Rot3::from_raw_elements(x, y, z, w).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
            );
        }
    }

    #[test]
    fn test_angle_between() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for [a, b] in random_iter::<[Rotor<N, Wide, Unaligned>; 2]>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotor::IDENTITY));

                assert_test_eq_or_panic!(
                    a.angle_between(b),
                    Wide::new(std::array::from_fn(|lane| a
                        .lane(lane)
                        .angle_between(b.lane(lane))))
                );
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Rotor<N, Wide, Unaligned>; 2], Wide)>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotor::IDENTITY));

                assert_test_eq_or_panic!(
                    a.lerp(b, t),
                    Rotor::from_lane_fn(|lane| a.lane(lane).lerp(b.lane(lane), t.as_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Rotor<N, Wide, Unaligned>; 2], Wide)>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotor::IDENTITY));

                assert_test_eq_or_panic!(
                    a.slerp(b, t),
                    Rotor::from_lane_fn(|lane| a
                        .lane(lane)
                        .slerp(b.lane(lane), t.as_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for ([a, b], max_angle) in random_iter::<([Rotor<N, Wide, Unaligned>; 2], Wide)>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotor::IDENTITY));

                assert_test_eq_or_panic!(
                    a.slerp(b, max_angle),
                    Rotor::from_lane_fn(|lane| a
                        .lane(lane)
                        .rotate_towards(b.lane(lane), max_angle.as_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_length() {
        for rotor in random_iter::<Rot2<f32x4>>() {
            assert_test_eq!(rotor.length(), rotor.0.length());
        }
        for rotor in random_iter::<Rot3<f32x4>>() {
            assert_test_eq!(rotor.length(), rotor.0.length());
        }
    }

    // `try_normalize` is exluded on purpose.

    #[test]
    fn test_normalize_or() {
        for_types!(|Wide: WideFloat| {
            for [rotor, fallback] in random_iter::<[Rot2<Wide>; 2]>() {
                assert_test_eq!(
                    rotor.normalize_or(fallback),
                    Rotor(rotor.0.normalize_or(fallback.0))
                );
            }
            for [rotor, fallback] in random_iter::<[Rot3<Wide>; 2]>() {
                assert_test_eq!(
                    rotor.normalize_or(fallback),
                    Rotor(rotor.0.normalize_or(fallback.0))
                );
            }
        });
    }

    #[test]
    fn test_normalize_and_length() {
        for_types!(|Wide: WideFloat| {
            for rotor in random_iter::<Rot2<Wide>>() {
                let expected = rotor.0.normalize_and_length();
                assert_test_eq!(
                    rotor.normalize_and_length(),
                    (Rotor(expected.0), expected.1)
                );
            }
            for rotor in random_iter::<Rot3<Wide>>() {
                let expected = rotor.0.normalize_and_length();
                assert_test_eq!(
                    rotor.normalize_and_length(),
                    (Rotor(expected.0), expected.1)
                );
            }
        });
    }

    #[test]
    fn test_is_normalized() {
        for_types!(|Wide: WideFloat| {
            for rotor in random_iter::<Rot2<Wide>>() {
                assert_test_eq!(rotor.is_normalized(), rotor.0.is_normalized());
            }
            for rotor in random_iter::<Rot3<Wide>>() {
                assert_test_eq!(rotor.is_normalized(), rotor.0.is_normalized());
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|N: TwoOrThree, Wide: WideFloat| {
            for ([a, b], max_abs_diff) in random_iter::<([Rotor<N, Wide, Unaligned>; 2], Wide)>() {
                assert_test_eq!(
                    a.abs_diff_eq(b, max_abs_diff),
                    (0..LANES).all(|lane| a
                        .lane(lane)
                        .abs_diff_eq(b.lane(lane), max_abs_diff.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_from_angle() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec2<Wide>, Wide)>() {
                assert_test_eq!(
                    vector * Rot2::<Wide>::from_angle(angle),
                    vector.rotate(angle)
                );
            }
        });
    }

    #[test]
    fn test_to_angle() {
        for_types!(|Wide: WideFloat| {
            for angle in random_iter::<Wide>() {
                let rotor = Rot2::<Wide>::from_angle(angle);

                assert_test_eq!(rotor.to_angle(), angle);
                assert_test_eq!((-rotor).to_angle(), angle);
            }
        });
    }

    #[test]
    fn test_from_rotation_xy() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                assert_test_eq!(
                    vector * Rot3::<Wide>::from_rotation_xy(angle),
                    vector.rotate_xy(angle)
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_xz() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                assert_test_eq!(
                    vector * Rot3::<Wide>::from_rotation_xz(angle),
                    vector.rotate_xz(angle)
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_yz() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                assert_test_eq!(
                    vector * Rot3::<Wide>::from_rotation_yz(angle),
                    vector.rotate_yz(angle)
                );
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_types!(|Wide: WideFloat| {
            for (axis, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                let axis = axis.normalize_or(Vector::ONE).normalize();
                let half_angle = angle * 0.5;

                let result = Rot3::<Wide>::from_axis_angle(axis, angle);

                assert_test_eq!(
                    result.s,
                    half_angle.cos(),
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    result.yz,
                    half_angle.sin() * axis.x,
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    -result.xz,
                    half_angle.sin() * axis.y,
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    result.xy,
                    half_angle.sin() * axis.z,
                    abs <= angle.abs() * 1e-4 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scaled_axis() {
        for_types!(|Wide: WideFloat| {
            for (axis, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                let axis = axis.normalize_or(Vector::ONE).normalize();

                let skip = !(axis * angle).length().is_finite();
                let axis = skip.select(Vec3::X, axis);
                let angle = skip.select(Wide::ZERO, angle);

                assert_test_eq!(
                    Rot3::<Wide>::from_scaled_axis(axis * angle),
                    Rot3::<Wide>::from_axis_angle(axis, angle),
                    abs <= 1e-6 * axis.abs().max_element().max(angle.abs()),
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
                    if [a, b, c]
                        .into_iter()
                        .flat_map(Wide::to_array)
                        .any(|x| !x.is_finite() || x > 1e6)
                    {
                        continue;
                    };

                    assert_test_eq!(
                        Rot3::<Wide>::from_euler(order, a, b, c),
                        Rot3::<Wide>::from_matrix(&Mat3::<Wide>::from_euler(order, a, b, c)),
                        abs <= Wide::splat(1e-6),
                        0.0 = -0.0,
                        rotor = -rotor
                    );
                }
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|Wide: WideFloat| {
            for [dir, up] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|dir_up| [dir_up, dir_up.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Rot3::<Wide>::look_to_lh(dir, up),
                    Rot3::from_lane_fn(|lane| Rot3::<T>::look_to_lh(dir.lane(lane), up.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|Wide: WideFloat| {
            for [dir, up] in random_iter::<[Vec3<Wide>; 2]>()
                .flat_map(|dir_up| [dir_up, dir_up.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Rot3::<Wide>::look_to_rh(dir, up),
                    Rot3::from_lane_fn(|lane| Rot3::<T>::look_to_rh(dir.lane(lane), up.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|Wide: WideFloat| {
            for [eye, center, up] in random_iter::<[Vec3<Wide>; 3]>()
                .flat_map(|eye_center_up| [eye_center_up, eye_center_up.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Rot3::<Wide>::look_at_lh(eye, center, up),
                    Rot3::from_lane_fn(|lane| Rot3::<T>::look_at_lh(
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
                .flat_map(|eye_center_up| [eye_center_up, eye_center_up.map(|v| v.normalize())])
            {
                assert_test_eq_or_panic!(
                    Rot3::<Wide>::look_at_rh(eye, center, up),
                    Rot3::from_lane_fn(|lane| Rot3::<T>::look_at_rh(
                        eye.lane(lane),
                        center.lane(lane),
                        up.lane(lane)
                    ))
                );
            }
        });
    }

    #[test]
    fn test_to_axis_angle() {
        for_types!(|Wide: WideFloat| {
            for rotor in random_iter::<Rot3<Wide>>().flat_map(|r| [r, r.normalize()]) {
                assert_test_eq_or_panic!(
                    rotor.to_axis_angle(),
                    (
                        Vec3::from_lane_fn(|lane| rotor.lane(lane).to_axis_angle().0),
                        Wide::new(std::array::from_fn(|lane| rotor
                            .lane(lane)
                            .to_axis_angle()
                            .1))
                    ),
                    abs <= (Wide::splat(1e-5), Wide::splat(1e-5))
                );
            }
        });
    }

    #[test]
    fn test_to_scaled_axis() {
        for_types!(|Wide: WideFloat| {
            for rotor in random_iter::<Rot3<Wide>>().flat_map(|r| [r, r.normalize()]) {
                assert_test_eq_or_panic!(
                    rotor.to_scaled_axis(),
                    Vec3::from_lane_fn(|lane| rotor.lane(lane).to_scaled_axis()),
                    abs <= Wide::splat(1e-5)
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|Wide: WideFloat| {
            for order in EulerRot::values() {
                for rotor in random_iter::<Rot3<Wide>>().flat_map(|r| [r, r.normalize()]) {
                    assert_test_eq_or_panic!(
                        rotor.to_euler(order),
                        (
                            Wide::new(std::array::from_fn(|lane| rotor
                                .lane(lane)
                                .to_euler(order)
                                .0)),
                            Wide::new(std::array::from_fn(|lane| rotor
                                .lane(lane)
                                .to_euler(order)
                                .1)),
                            Wide::new(std::array::from_fn(|lane| rotor
                                .lane(lane)
                                .to_euler(order)
                                .2))
                        ),
                        abs <= (Wide::splat(1e-5), Wide::splat(1e-5), Wide::splat(1e-5))
                    );
                }
            }
        });
    }
}
