use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Affine, Alignment, EulerRot, Length, Matrix, Projective, Rotor, Vector, length::TwoOrThree,
};

macro_rules! items {
    ($Wide:ident) => {
        /// A rotor with all elements set to NaN (Not a Number).
        pub const NAN: Self = todo!();

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
        pub fn from_rotation_arc(_from: Vector<N, $Wide, A>, _to: Vector<N, $Wide, A>) -> Self {
            todo!()
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
            _from: Vector<N, $Wide, A>,
            _to: Vector<N, $Wide, A>,
        ) -> Self {
            todo!()
        }

        /// Converts a rotation matrix to a rotor.
        #[inline]
        #[must_use]
        pub fn from_matrix(_matrix: &Matrix<N, $Wide, A>) -> Self {
            todo!()
        }

        /// Converts an affine transform with rotation to a rotor.
        ///
        /// `affine.matrix` must only contain a rotation. `affine.translation`
        /// is fully ignored.
        #[inline]
        #[must_use]
        pub fn from_affine(_affine: &Affine<N, $Wide, A>) -> Self {
            todo!()
        }

        /// Converts a projective transform with rotation to a rotor.
        ///
        /// This function assumes the transform only contains rotation, and possibly
        /// translation, which is ignored.
        #[inline]
        #[must_use]
        pub fn from_projective(_projective: &Projective<N, $Wide, A>) -> Self {
            todo!()
        }

        /// Returns `true` if any element is NaN.
        #[inline]
        #[must_use]
        pub fn is_nan(self) -> bool {
            todo!()
        }

        /// Returns `true` if all elements are neither infinite nor NaN.
        #[inline]
        #[must_use]
        pub fn is_finite(self) -> bool {
            todo!()
        }

        /// Returns the inverse of a rotor.
        ///
        /// This function is identical to [`conjugate`]. Use whichever function
        /// makes your intentions clearer.
        ///
        /// [`conjugate`]: Self::conjugate
        #[inline]
        #[must_use]
        pub fn inverse(self) -> Self {
            todo!()
        }

        /// Returns the angle (in radians) for the minimal rotation for
        /// transforming `self` into `other`.
        ///
        /// `self` and `other` must be normalized.
        #[inline]
        #[must_use]
        pub fn angle_between(self, _other: Self) -> $Wide {
            todo!()
        }

        /// Computes the linear interpolation between `self` and `other` based
        /// on the value `t`.
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
        pub fn lerp(self, _other: Self, _t: $Wide) -> Self {
            todo!()
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
        pub fn slerp(self, _other: Self, _t: $Wide) -> Self {
            todo!()
        }

        /// Rotates `self` towards `target` by at most `max_angle` (in radians).
        ///
        /// When `max_angle` is `0`, the result is `self`. When `max_angle` is
        /// equal to or greater than `self.angle_between(target)`, the result is
        /// `target`. When `max_angle` is negative, rotates towards the opposite
        /// of `target`.
        ///
        /// `self` and `target` must be normalized.
        #[inline]
        #[must_use]
        pub fn rotate_towards(self, _target: Self, _max_angle: $Wide) -> Self {
            todo!()
        }

        /// Returns the length/magnitude of `self`.
        #[inline]
        #[must_use]
        pub fn length(self) -> $Wide {
            todo!()
        }

        /// Returns `self` normalized to length `1`.
        #[inline]
        #[must_use]
        pub fn normalize(self) -> Self {
            todo!()
        }

        /// Returns [`normalize`], or `None` if `self` is zero or if the result
        /// is non finite or zero.
        ///
        /// [`normalize`]: Self::normalize
        #[inline]
        #[must_use]
        pub fn try_normalize(self) -> Option<Self> {
            todo!()
        }

        /// Returns [`normalize`], or `fallback` if `self` is zero or if the
        /// result is non finite or zero.
        ///
        /// [`normalize`]: Self::normalize
        #[inline]
        #[must_use]
        pub fn normalize_or(self, _fallback: Self) -> Self {
            todo!()
        }

        /// Simultaneously computes [`normalize`] and [`length`].
        ///
        /// [`normalize`]: Self::normalize
        /// [`length`]: Self::length
        #[inline]
        #[must_use]
        pub fn normalize_and_length(self) -> (Self, $Wide) {
            todo!()
        }

        /// Returns whether the rotor has the length 1 or not.
        ///
        /// This uses a precision threshold of approximately `1e-4`.
        #[inline]
        #[must_use]
        pub fn is_normalized(self) -> bool {
            todo!()
        }

        /// Returns `true` if the absolute difference of all elements between `self`
        /// and `other` is less than or equal to `max_abs_diff`.
        ///
        /// This can be used to compare two rotors that should be equal, but may
        /// have a slight difference due to operations having rounding errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(self, _other: Self, _max_abs_diff: $Wide) -> bool {
            todo!()
        }
    };
}

macro_rules! items_2 {
    ($Wide:ident) => {
        /// Creates a 2D rotor from an `angle` (in radians) rotating `+X` to
        /// `+Y`.
        #[inline]
        #[must_use]
        pub fn from_angle(_angle: $Wide) -> Self {
            todo!()
        }

        /// Converts a 2D rotor to an angle (in radians) rotating `+X` to `+Y`.
        ///
        /// This assumes the rotor is normalized.
        #[inline]
        #[must_use]
        pub fn to_angle(self) -> $Wide {
            todo!()
        }
    };
}

macro_rules! items_3 {
    ($Wide:ident) => {
        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Y`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xy(angle: $Wide) -> Self {
            let (sin, cos) = (angle * 0.5).sin_cos();
            Self::from_raw_elements(sin, $Wide::ZERO, $Wide::ZERO, cos)
        }

        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xz(angle: $Wide) -> Self {
            let (sin, cos) = (angle * 0.5).sin_cos();
            Self::from_raw_elements($Wide::ZERO, sin, $Wide::ZERO, cos)
        }

        /// Creates a rotor from an `angle` (in radians) rotating `+Y` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_yz(angle: $Wide) -> Self {
            let (sin, cos) = (angle * 0.5).sin_cos();
            Self::from_raw_elements($Wide::ZERO, $Wide::ZERO, sin, cos)
        }

        /// Creates a rotor from a rotation `axis` and `angle` (in radians), using
        /// the right-hand rule.
        ///
        /// This assumes `axis` is normalized.
        ///
        /// If you are using this to initialize a static rotation, consider using
        /// [`from_rotation_arc`] instead. That function makes it clearer what
        /// direction the rotation happens in, whereas this function requires
        /// remembering the right-hand rule.
        ///
        /// [`from_rotation_arc`]: Self::from_rotation_arc
        #[inline]
        #[must_use]
        #[track_caller]
        pub fn from_axis_angle(axis: Vector<3, $Wide, A>, angle: $Wide) -> Self {
            let (sin, cos) = (angle * 0.5).sin_cos();
            let xyz = axis * sin;
            Self::from_raw_elements(xyz.z, -xyz.y, xyz.x, cos)
        }

        /// Creates a rotor that rotates `scaled_axis.length()` radians around
        /// `scaled_axis.normalize()`, using the right-hand rule.
        ///
        /// If you are using this to initialize a static rotation, consider using
        /// [`from_rotation_arc`] instead. That function makes it clearer what
        /// direction the rotation happens in, whereas this function requires
        /// remembering the right-hand rule.
        ///
        /// [`from_rotation_arc`]: Self::from_rotation_arc
        #[inline]
        #[must_use]
        pub fn from_scaled_axis(scaled_axis: Vector<3, $Wide, A>) -> Self {
            let (axis, angle) = scaled_axis.normalize_and_length();
            if angle == $Wide::ZERO {
                Self::IDENTITY
            } else {
                let (sin, cos) = (angle * 0.5).sin_cos();
                let xyz = axis * sin;
                Self::from_raw_elements(xyz.z, -xyz.y, xyz.x, cos)
            }
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

            let ti = angles.x * 0.5;
            let tj = angles.y * 0.5;
            let th = angles.z * 0.5;
            let (si, ci) = ti.sin_cos();
            let (sj, cj) = tj.sin_cos();
            let (sh, ch) = th.sin_cos();
            let cc = ci * ch;
            let cs = ci * sh;
            let sc = si * ch;
            let ss = si * sh;

            let parity = if !order.parity_even {
                $Wide::ONE
            } else {
                -$Wide::ONE
            };

            let mut result = Vector::ZERO;

            if order.initial_repeated {
                result[i] = cj * (cs + sc);
                result[j] = sj * (cc + ss) * parity;
                result[k] = sj * (cs - sc);
                result[3] = cj * (cc - ss);
            } else {
                result[i] = cj * sc - sj * cs;
                result[j] = (cj * ss + sj * cc) * parity;
                result[k] = cj * cs - sj * sc;
                result[3] = cj * cc + sj * ss;
            }

            Self(result)
        }

        /// Creates a rotor from a facing direction and an up direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and
        /// `+Z=forward`.
        #[inline]
        #[must_use]
        #[track_caller]
        pub fn look_to_lh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_lh(dir, up))
        }

        /// Creates a rotor from a facing direction and an up direction.
        ///
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and
        /// `+Z=back`.
        #[inline]
        #[must_use]
        #[track_caller]
        pub fn look_to_rh(dir: Vector<3, $Wide, A>, up: Vector<3, $Wide, A>) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_to_rh(dir, up))
        }

        /// Creates a rotor from a camera position, a focal point and an up
        /// direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and
        /// `+Z=forward`.
        #[inline]
        #[must_use]
        #[track_caller]
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
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and
        /// `+Z=back`.
        #[inline]
        #[must_use]
        #[track_caller]
        pub fn look_at_rh(
            eye: Vector<3, $Wide, A>,
            center: Vector<3, $Wide, A>,
            up: Vector<3, $Wide, A>,
        ) -> Self {
            Self::from_matrix(&Matrix::<3, $Wide, A>::look_at_rh(eye, center, up))
        }

        /// Converts the rotor `self` to a normalized rotation axis and an angle (in
        /// radians).
        ///
        /// This axis uses the right-hand rule.
        #[inline]
        #[must_use]
        #[track_caller]
        pub fn to_axis_angle(self) -> (Vector<3, $Wide, A>, $Wide) {
            todo!()
        }

        /// Converts the rotor `self` to a rotation axis scaled by an angle (in
        /// radians).
        #[inline]
        #[must_use]
        pub fn to_scaled_axis(self) -> Vector<3, $Wide, A> {
            todo!()
        }

        /// Returns the Euler angles forming `self` for the given Euler rotation
        /// order/sequence.
        #[inline]
        #[must_use]
        #[track_caller]
        pub fn to_euler(self, _order: EulerRot) -> ($Wide, $Wide, $Wide) {
            todo!()
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

/// Functionality for [SoA] (Structure of Arrays) float rotors.
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
impl<const N: usize, Wide, A: Alignment> Rotor<N, Wide, A>
where
    Length<N>: TwoOrThree,
    Wide: WideFloat,
{
    items!(Wide);
}

/// Functionality for [SoA] (Structure of Arrays) 2D float rotors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Rotor<2, Wide, A>
where
    Wide: WideFloat,
{
    items_2!(Wide);
}

/// Functionality for [SoA] (Structure of Arrays) 3D float rotors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
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
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Rotor<3, $Wide, A> {
            items_3!($Wide);
        }
    };
}
impl_items!(f32x4);
impl_items!(f32x8);
impl_items!(f32x16);
impl_items!(f64x2);
impl_items!(f64x4);
impl_items!(f64x8);
