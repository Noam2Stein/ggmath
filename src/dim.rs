/// A marker type to restrict `const N: usize` to `2`, `3` and `4`.
///
/// `Dim<N>` and [`TwoThreeOrFour`] are markers used by types like
/// [`Vector<N, T, A>`] to restrict `N` to `2`, `3` and `4`.
///
/// [`Vector<N, T, A>`]: crate::Vector
pub struct Dim<const N: usize>;

/// A marker trait to restrict `const N: usize` to `2`, `3` and `4`.
///
/// [`Dim<N>`] and `TwoThreeOrFour` are markers used by types like
/// [`Vector<N, T, A>`] to restrict `N` to `2`, `3` and `4`.
///
/// [`Vector<N, T, A>`]: crate::Vector
#[expect(private_bounds)]
pub trait TwoThreeOrFour: Sealed {
    #[doc(hidden)]
    type Select<T2: Copy, T3: Copy, T4: Copy>: Copy;
}

/// An internal trait similar to [`TwoThreeOrFour`] that only accepts 2 and 3.
///
/// This is intentionally hidden from the public API.
pub(crate) trait TwoOrThree: TwoThreeOrFour {
    #[doc(hidden)]
    type Select<T2: Copy, T3: Copy>: Copy;
}

/// An internal trait similar to [`TwoThreeOrFour`] that only accepts 3.
///
/// This is used for `Rotor`, which currently only accepts `N = 3`, but could
/// accept other dimensions in the future.
///
/// This is intentionally hidden from the public API.
pub(crate) trait Three: TwoOrThree {}

trait Sealed {}

impl TwoThreeOrFour for Dim<2> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T2;
}

impl TwoThreeOrFour for Dim<3> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T3;
}

impl TwoThreeOrFour for Dim<4> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T4;
}

impl TwoOrThree for Dim<2> {
    type Select<T2: Copy, T3: Copy> = T2;
}

impl TwoOrThree for Dim<3> {
    type Select<T2: Copy, T3: Copy> = T3;
}

impl Three for Dim<3> {}

impl Sealed for Dim<2> {}
impl Sealed for Dim<3> {}
impl Sealed for Dim<4> {}
