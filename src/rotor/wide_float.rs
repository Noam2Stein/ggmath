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
        /// This function assumes the transform only contains rotation, and
        /// possibly translation, which is ignored.
        #[inline]
        #[must_use]
        pub fn from_affine(_affine: &Affine<N, $Wide, A>) -> Self {
            todo!()
        }

        /// Converts a projective transform with rotation to a rotor.
        ///
        /// This function assumes the transform only contains rotation, and
        /// possibly translation, which is ignored.
        #[inline]
        #[must_use]
        pub fn from_projective(_projective: &Projective<N, $Wide, A>) -> Self {
            todo!()
        }

        /// Returns `true` if any element is NaN.
        #[inline]
        #[must_use]
        pub fn is_nan(self) -> $Wide {
            todo!()
        }

        /// Returns `true` if all elements are neither infinite nor NaN.
        #[inline]
        #[must_use]
        pub fn is_finite(self) -> $Wide {
            todo!()
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
        /// This assumes `self` and `target` are normalized.
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
        pub fn normalize_or(self, _fallback: Self) -> Self {
            todo!()
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
            todo!()
        }

        /// Returns whether the rotor has the length 1 or not.
        ///
        /// This uses a precision threshold of approximately `1e-4`.
        #[inline]
        #[must_use]
        pub fn is_normalized(self) -> $Wide {
            todo!()
        }

        /// Returns `true` if for all lanes, the absolute difference of all
        /// elements between `self` and `other` is less than or equal to
        /// `max_abs_diff`.
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
        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Y`.
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
        pub fn from_rotation_xy(_angle: $Wide) -> Self {
            todo!()
        }

        /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_xz(_angle: $Wide) -> Self {
            todo!()
        }

        /// Creates a rotor from an `angle` (in radians) rotating `+Y` to `+Z`.
        #[inline]
        #[must_use]
        pub fn from_rotation_yz(_angle: $Wide) -> Self {
            todo!()
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
        pub fn from_axis_angle(_axis: Vector<3, $Wide, A>, _angle: $Wide) -> Self {
            todo!()
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
        pub fn from_scaled_axis(_scaled_axis: Vector<3, $Wide, A>) -> Self {
            todo!()
        }

        /// Creates a rotor from an Euler rotation order/sequence and angles (in
        /// radians).
        #[inline]
        #[must_use]
        pub fn from_euler(_order: EulerRot, _a: $Wide, _b: $Wide, _c: $Wide) -> Self {
            todo!()
        }

        /// Creates a rotor from a facing direction and an up direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_to_lh(_dir: Vector<3, $Wide, A>, _up: Vector<3, $Wide, A>) -> Self {
            todo!()
        }

        /// Creates a rotor from a facing direction and an up direction.
        ///
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_to_rh(_dir: Vector<3, $Wide, A>, _up: Vector<3, $Wide, A>) -> Self {
            todo!()
        }

        /// Creates a rotor from a camera position, a focal point and an up
        /// direction.
        ///
        /// For a left-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=forward`.
        #[inline]
        #[must_use]
        pub fn look_at_lh(
            _eye: Vector<3, $Wide, A>,
            _center: Vector<3, $Wide, A>,
            _up: Vector<3, $Wide, A>,
        ) -> Self {
            todo!()
        }

        /// Creates a rotor from a camera position, a focal point and an up
        /// direction.
        ///
        /// For a right-handed view coordinate system with `+X=right`, `+Y=up`
        /// and `+Z=back`.
        #[inline]
        #[must_use]
        pub fn look_at_rh(
            _eye: Vector<3, $Wide, A>,
            _center: Vector<3, $Wide, A>,
            _up: Vector<3, $Wide, A>,
        ) -> Self {
            todo!()
        }

        /// Converts the rotor `self` to a normalized rotation axis and an angle
        /// (in radians), using the right-hand rule.
        #[inline]
        #[must_use]
        pub fn to_axis_angle(self) -> (Vector<3, $Wide, A>, $Wide) {
            todo!()
        }

        // Converts the rotor `self` to a rotation axis scaled by an angle (in
        /// radians), using the right-hand rule.
        #[inline]
        #[must_use]
        pub fn to_scaled_axis(self) -> Vector<3, $Wide, A> {
            todo!()
        }

        /// Returns the Euler angles forming `self` for the given Euler rotation
        /// order/sequence.
        #[inline]
        #[must_use]
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_constants() {
        todo!()
    }

    #[test]
    fn test_from_rotation_arc() {
        todo!()
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        todo!()
    }

    #[test]
    fn test_from_matrix() {
        todo!()
    }

    #[test]
    fn test_from_affine() {
        todo!()
    }

    #[test]
    fn test_from_projective() {
        todo!()
    }

    #[test]
    fn test_is_nan() {
        todo!()
    }

    #[test]
    fn test_is_finite() {
        todo!()
    }

    #[test]
    fn test_inverse() {
        todo!()
    }

    #[test]
    fn test_angle_between() {
        todo!()
    }

    #[test]
    fn test_lerp() {
        todo!()
    }

    #[test]
    fn test_slerp() {
        todo!()
    }

    #[test]
    fn test_rotate_towards() {
        todo!()
    }

    #[test]
    fn test_length() {
        todo!()
    }

    #[test]
    fn test_normalize() {
        todo!()
    }

    // `try_normalize` is exluded on purpose.

    #[test]
    fn test_normalize_or() {
        todo!()
    }

    #[test]
    fn test_normalize_and_length() {
        todo!()
    }

    #[test]
    fn test_is_normalized() {
        todo!()
    }

    #[test]
    fn test_abs_diff_eq() {
        todo!()
    }

    #[test]
    fn test_from_angle() {
        todo!()
    }

    #[test]
    fn test_to_angle() {
        todo!()
    }

    #[test]
    fn test_from_rotation_xy() {
        todo!()
    }

    #[test]
    fn test_from_rotation_xz() {
        todo!()
    }

    #[test]
    fn test_from_rotation_yz() {
        todo!()
    }

    #[test]
    fn test_from_axis_angle() {
        todo!()
    }

    #[test]
    fn test_from_scaled_axis() {
        todo!()
    }

    #[test]
    fn test_from_euler() {
        todo!()
    }

    #[test]
    fn test_look_to_lh() {
        todo!()
    }

    #[test]
    fn test_look_to_rh() {
        todo!()
    }

    #[test]
    fn test_look_at_lh() {
        todo!()
    }

    #[test]
    fn test_look_at_rh() {
        todo!()
    }

    #[test]
    fn test_to_axis_angle() {
        todo!()
    }

    #[test]
    fn test_to_scaled_axis() {
        todo!()
    }

    #[test]
    fn test_to_euler() {
        todo!()
    }
}
