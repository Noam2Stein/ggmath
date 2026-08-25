use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{
    Affine, Alignment, EulerRot, Length, Matrix, Projective, Quaternion, Rotor, Vector,
    length::TwoOrThree,
    utils::{specialize_23, transmute_generic},
};

macro_rules! items {
    ($Wide:ident) => {};
}

macro_rules! items_2 {
    ($Wide:ident) => {};
}

macro_rules! items_3 {
    ($Wide:ident, $T:ident) => {};
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
    ($Wide:ident, $T:ident) => {
        #[cfg(not(doc))]
        #[expect(private_bounds)]
        impl<const N: usize, A: Alignment> Rotor<N, $Wide, A>
        where
            Length<N>: TwoOrThree,
        {
            items!($Wide, $T);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Rotor<2, $Wide, A> {
            items_2!($Wide, $T);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Rotor<3, $Wide, A> {
            items_3!($Wide, $T);
        }
    };
}
impl_items!(f32x4, f32);
impl_items!(f32x8, f32);
impl_items!(f32x16, f32);
impl_items!(f64x2, f64);
impl_items!(f64x4, f64);
impl_items!(f64x8, f64);
