use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Affine, Alignment, EulerRot, Length, Matrix, Projective, Rotor, Vector, length::TwoOrThree,
};

macro_rules! items {
    ($Wide:ident) => {
        /// TODO
        pub const NAN: Self = todo!();

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_rotation_arc(_from: Vector<N, $Wide, A>, _to: Vector<N, $Wide, A>) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_rotation_arc_colinear(
            _from: Vector<N, $Wide, A>,
            _to: Vector<N, $Wide, A>,
        ) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_matrix(_matrix: &Matrix<N, $Wide, A>) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_affine(_affine: &Affine<N, $Wide, A>) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_projective(_projective: &Projective<N, $Wide, A>) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn is_nan(self) -> $Wide {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn is_finite(self) -> $Wide {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn inverse(self) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn angle_between(self, _other: Self) -> $Wide {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn lerp(self, _other: Self, _t: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn slerp(self, _other: Self, _t: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn rotate_towards(self, _target: Self, _max_angle: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn length(self) -> $Wide {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn normalize(self) -> Self {
            todo!()
        }

        // `try_normalize` is exluded on purpose.

        /// TODO
        #[inline]
        #[must_use]
        pub fn normalize_or(self, _fallback: Self) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn normalize_and_length(self) -> (Self, $Wide) {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn is_normalized(self) -> $Wide {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(self, _other: Self, _max_abs_diff: $Wide) -> bool {
            todo!()
        }
    };
}

macro_rules! items_2 {
    ($Wide:ident) => {
        /// TODO
        #[inline]
        #[must_use]
        pub fn from_angle(_angle: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn to_angle(self) -> $Wide {
            todo!()
        }
    };
}

macro_rules! items_3 {
    ($Wide:ident) => {
        /// TODO
        #[inline]
        #[must_use]
        pub fn from_rotation_xy(_angle: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_rotation_xz(_angle: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_rotation_yz(_angle: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_axis_angle(_axis: Vector<3, $Wide, A>, _angle: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_scaled_axis(_scaled_axis: Vector<3, $Wide, A>) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn from_euler(_order: EulerRot, _a: $Wide, _b: $Wide, _c: $Wide) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn look_to_lh(_dir: Vector<3, $Wide, A>, _up: Vector<3, $Wide, A>) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn look_to_rh(_dir: Vector<3, $Wide, A>, _up: Vector<3, $Wide, A>) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn look_at_lh(
            _eye: Vector<3, $Wide, A>,
            _center: Vector<3, $Wide, A>,
            _up: Vector<3, $Wide, A>,
        ) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn look_at_rh(
            _eye: Vector<3, $Wide, A>,
            _center: Vector<3, $Wide, A>,
            _up: Vector<3, $Wide, A>,
        ) -> Self {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn to_axis_angle(self) -> (Vector<3, $Wide, A>, $Wide) {
            todo!()
        }

        /// TODO
        #[inline]
        #[must_use]
        pub fn to_scaled_axis(self) -> Vector<3, $Wide, A> {
            todo!()
        }

        /// TODO
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
