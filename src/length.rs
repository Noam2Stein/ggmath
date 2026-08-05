/// A marker type to restrict `const N: usize` to `2`, `3` and `4`.
///
/// `Length<N>` and [`SupportedLength`] are markers used by types like
/// [`Vector<N, T, A>`] to restrict `N` to `2`, `3` and `4`.
///
/// [`Vector<N, T, A>`]: crate::Vector
pub struct Length<const N: usize>;

/// A marker trait to restrict `const N: usize` to `2`, `3` and `4`.
///
/// [`Length<N>`] and `SupportedLength` are markers used by types like
/// [`Vector<N, T, A>`] to restrict `N` to `2`, `3` and `4`.
///
/// [`Vector<N, T, A>`]: crate::Vector
#[expect(private_bounds)]
pub trait SupportedLength: Sealed {
    #[doc(hidden)]
    type Select<T2: Copy, T3: Copy, T4: Copy>: Copy;
}

/// An internal trait similar to [`SupportedLength`] that only accepts 2 and 3.
///
/// This is intentionally hidden from the public API.
pub(crate) trait TwoOrThree: SupportedLength {
    #[doc(hidden)]
    type Select<T2: Copy, T3: Copy>: Copy;
}

trait Sealed {}

impl SupportedLength for Length<2> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T2;
}

impl SupportedLength for Length<3> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T3;
}

impl SupportedLength for Length<4> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T4;
}

impl TwoOrThree for Length<2> {
    type Select<T2: Copy, T3: Copy> = T2;
}

impl TwoOrThree for Length<3> {
    type Select<T2: Copy, T3: Copy> = T3;
}

impl Sealed for Length<2> {}
impl Sealed for Length<3> {}
impl Sealed for Length<4> {}
