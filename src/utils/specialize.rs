use crate::{
    Affine, Aligned, Alignment, Length, Mask, Matrix, Projective, Rotor, Scalar, SupportedLength,
    Unaligned, Vector, length::TwoOrThree, utils::transmute_generic,
};

/// Bypasses a type system limitation to perform specialization.
///
/// Types that implement [`Scalar`] can override the implementation of math
/// functions. Implementations are overriden via the [`ScalarBackend<N, A>`]
/// trait, which has to be implemented for all lengths and both alignments.
///
/// Implementations can be generic over `N` and `A`, but there can also be
/// seperate implementations for each concrete case. To make this possible,
/// [`Scalar`] is defined as:
///
/// ```ignore
/// trait Scalar:
///     ScalarBackend<2, Aligned>
///     + ScalarBackend<3, Aligned>
///     + ScalarBackend<4, Aligned>
///     + ScalarBackend<2, Unaligned>
///     + ScalarBackend<3, Unaligned>
///     + ScalarBackend<4, Unaligned>
/// {
/// }
/// ```
///
/// Math functions that want to call their implementation want to write:
///
/// ```ignore
/// <T as ScalarBackend<N, A>>::function_implementation(arguments)
/// ```
///
/// This results in a compiler error. The compiler understands that `T`
/// implements [`ScalarBackend`] for lengths `2`, `3`, `4`, and alignments
/// [`Aligned`], [`Unaligned`], but is not smart enough to understand that those
/// are all possible cases, and that `T` implements [`ScalarBackend<N, A>`].
///
/// To bypass this, math functions have to match over `N` and `A`, and for each
/// of the 6 possible cases perform unsafe transmutations to convert inputs and
/// outputs, or function pointers, from their generic form to a concrete form.
///
/// The `specialize` macro is a safe wrapper for this pattern that allows math
/// functions to write:
///
/// ```ignore
/// specialize!(<T as ScalarBackend<N, A>>::function_implementation(arguments))
/// ```
///
/// Once the type system is smart enough, `specialize` could be removed. This
/// will improve compile times and lower the chances of soundness bugs
/// appearing.
///
/// Note: If certain function signatures fail to compile with this macro, the
/// [`Specialize`] trait may not be implemented for those signature's types.
///
/// [`ScalarBackend<N, A>`]: crate::ScalarBackend
/// [`ScalarBackend`]: crate::ScalarBackend
macro_rules! specialize {
    (<$T:ty as $Backend:ident<$N:ident, $A:tt>>::$f:ident($($arg:expr),*$(,)?)) => {
        (const {
            $crate::utils::specialize_helper::<
                $N,
                $A,
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
            >(
                <$T as $Backend<2, $crate::Aligned>>::$f,
                <$T as $Backend<3, $crate::Aligned>>::$f,
                <$T as $Backend<4, $crate::Aligned>>::$f,
                <$T as $Backend<2, $crate::Unaligned>>::$f,
                <$T as $Backend<3, $crate::Unaligned>>::$f,
                <$T as $Backend<4, $crate::Unaligned>>::$f,
            )
        })($($arg),*)
    };
    (<$T:ty as $Backend:ident<$N:literal, $A:tt>>::$f:ident($($arg:expr),*$(,)?)) => {
        (const {
            $crate::utils::specialize_n_helper::<
                $N,
                $A,
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
            >(
                <$T as $Backend<$N, $crate::Unaligned>>::$f,
                <$T as $Backend<$N, $crate::Aligned>>::$f,
            )
        })($($arg),*)
    };
    (<$T:ty as $Backend:ident<$A:tt>>::$f:ident($($arg:expr),*$(,)?)) => {
        (const {
            $crate::utils::specialize_helper::<
                2,
                $A,
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
            >(
                <$T as $Backend<$crate::Aligned>>::$f,
                <$T as $Backend<$crate::Aligned>>::$f,
                <$T as $Backend<$crate::Aligned>>::$f,
                <$T as $Backend<$crate::Unaligned>>::$f,
                <$T as $Backend<$crate::Unaligned>>::$f,
                <$T as $Backend<$crate::Unaligned>>::$f,
            )
        })($($arg),*)
    };
    ($Struct:ident::<$N:tt, $T:ident, $A:tt>::$f:ident$(::<$G0:ty>)?($($arg:expr),*$(,)?)) => {
        (const {
            $crate::utils::specialize_helper::<
                $N,
                $A,
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
            >(
                <$Struct::<2, $T, $crate::Aligned>>::$f$(::<$G0>)?,
                <$Struct::<3, $T, $crate::Aligned>>::$f$(::<$G0>)?,
                <$Struct::<4, $T, $crate::Aligned>>::$f$(::<$G0>)?,
                <$Struct::<2, $T, $crate::Unaligned>>::$f$(::<$G0>)?,
                <$Struct::<3, $T, $crate::Unaligned>>::$f$(::<$G0>)?,
                <$Struct::<4, $T, $crate::Unaligned>>::$f$(::<$G0>)?,
            )
        })($($arg),*)
    };

    (@fn()) => {
        fn() -> _
    };
    (@fn($_0:expr)) => {
        fn(_) -> _
    };
    (@fn($_0:expr, $_1:expr)) => {
        fn(_, _) -> _
    };
    (@fn($_0:expr, $_1:expr, $_2:expr)) => {
        fn(_, _, _) -> _
    };
    (@fn($_0:expr, $_1:expr, $_2:expr, $_3:expr)) => {
        fn(_, _, _, _) -> _
    };
    (@fn($_0:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr)) => {
        fn(_, _, _, _, _) -> _
    };
    (@fn($_0:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr, $_5:expr)) => {
        fn(_, _, _, _, _, _) -> _
    };
}

pub(crate) use specialize;

/// A variant of [`specialize`] that only includes lengths 2 and 3, without 4.
macro_rules! specialize_23 {
    (<$T:ty as $Backend:ident<$N:tt, $A:tt>>::$f:ident($($arg:expr),*$(,)?)) => {
        (const {
            $crate::utils::specialize_23_helper::<
                $N,
                $A,
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
            >(
                <$T as $Backend<2, $crate::Aligned>>::$f,
                <$T as $Backend<3, $crate::Aligned>>::$f,
                <$T as $Backend<2, $crate::Unaligned>>::$f,
                <$T as $Backend<3, $crate::Unaligned>>::$f,
            )
        })($($arg),*)
    };
    ($Struct:ident::<$N:tt, $T:ident, $A:tt>::$f:ident$(::<$G0:ty>)?($($arg:expr),*$(,)?)) => {
        (const {
            $crate::utils::specialize_23_helper::<
                $N,
                $A,
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
                $crate::utils::specialize!(@fn($($arg),*)),
            >(
                <$Struct::<2, $T, $crate::Aligned>>::$f$(::<$G0>)?,
                <$Struct::<3, $T, $crate::Aligned>>::$f$(::<$G0>)?,
                <$Struct::<2, $T, $crate::Unaligned>>::$f$(::<$G0>)?,
                <$Struct::<3, $T, $crate::Unaligned>>::$f$(::<$G0>)?,
            )
        })($($arg),*)
    };
}

pub(crate) use specialize_23;

/// Performs the unsafe transmution for [`specialize`].
///
/// The macro call:
///
/// ```ignore
/// specialize!(<T as ScalarBackend<N, A>>::function_implementation(arguments))
/// ```
///
/// Expands to:
///
/// ```ignore
/// (const {
///     specialize_helper::<
///         N,
///         A,
///         fn(_) -> _,
///         fn(_) -> _,
///         fn(_) -> _,
///         fn(_) -> _,
///         fn(_) -> _,
///         fn(_) -> _,
///         fn(_) -> _,
///     >(
///         <T as ScalarBackend<2, Aligned>>::function_implementation,
///         <T as ScalarBackend<3, Aligned>>::function_implementation,
///         <T as ScalarBackend<4, Aligned>>::function_implementation,
///         <T as ScalarBackend<2, Unaligned>>::function_implementation,
///         <T as ScalarBackend<3, Unaligned>>::function_implementation,
///         <T as ScalarBackend<4, Unaligned>>::function_implementation,
///     )
/// })(arguments)
/// ```
///
/// `specialize_helper` then matches over `N` and `A` and transmutes the
/// appropriate function pointer to the output signature.
///
/// `fn(_) -> _` helps type inference understand that passed functions are
/// function pointers, and not function ZSTs which cannot be transmuted soundly.
///
/// The soundness of the transmutation relies on the signatures matching each
/// other. This is staticly guaranteed by the usage of the [`Specialize`] trait.
#[expect(private_bounds)]
pub const fn specialize_helper<const N: usize, A: Alignment, F2, F3, F4, F2U, F3U, F4U, F>(
    f2: F2,
    f3: F3,
    f4: F4,
    f2u: F2U,
    f3u: F3U,
    f4u: F4U,
) -> F
where
    Length<N>: SupportedLength,
    F2: Specialize<F, 2, N, Aligned, A> + Copy,
    F3: Specialize<F, 3, N, Aligned, A> + Copy,
    F4: Specialize<F, 4, N, Aligned, A> + Copy,
    F2U: Specialize<F, 2, N, Unaligned, A> + Copy,
    F3U: Specialize<F, 3, N, Unaligned, A> + Copy,
    F4U: Specialize<F, 4, N, Unaligned, A> + Copy,
{
    match (N, A::IS_ALIGNED) {
        // SAFETY: `T2` is guaranteed to be the same type as `T` as long as `N`
        // is `2` and `A` is `Aligned`. `N` was just checked to be `2` and `A`
        // is guaranteed to be `Aligned` because `A::IS_ALIGNED` is true, and so
        // `T2` and `T` are the same type.
        (2, true) => unsafe { transmute_generic::<F2, F>(f2) },

        // SAFETY: `T3` is guaranteed to be the same type as `T` as long as `N`
        // is `3` and `A` is `Aligned`. `N` was just checked to be `3` and `A`
        // is guaranteed to be `Aligned` because `A::IS_ALIGNED` is true, and so
        // `T3` and `T` are the same type.
        (3, true) => unsafe { transmute_generic::<F3, F>(f3) },

        // SAFETY: `T4` is guaranteed to be the same type as `T` as long as `N`
        // is `4` and `A` is `Aligned`. `N` was just checked to be `4` and `A`
        // is guaranteed to be `Aligned` because `A::IS_ALIGNED` is true, and so
        // `T4` and `T` are the same type.
        (4, true) => unsafe { transmute_generic::<F4, F>(f4) },

        // SAFETY: `T2U` is guaranteed to be the same type as `T` as long as `N`
        // is `2` and `A` is `Unaligned`. `N` was just checked to be `2` and `A`
        // is guaranteed to be `Unaligned` because `A::IS_ALIGNED` is false, and
        // so `T2U` and `T` are the same type.
        (2, false) => unsafe { transmute_generic::<F2U, F>(f2u) },

        // SAFETY: `T3U` is guaranteed to be the same type as `T` as long as `N`
        // is `3` and `A` is `Unaligned`. `N` was just checked to be `3` and `A`
        // is guaranteed to be `Unaligned` because `A::IS_ALIGNED` is false, and
        // so `T3U` and `T` are the same type.
        (3, false) => unsafe { transmute_generic::<F3U, F>(f3u) },

        // SAFETY: `T4U` is guaranteed to be the same type as `T` as long as `N`
        // is `4` and `A` is `Unaligned`. `N` was just checked to be `4` and `A`
        // is guaranteed to be `Unaligned` because `A::IS_ALIGNED` is false, and
        // so `T4U` and `T` are the same type.
        (4, false) => unsafe { transmute_generic::<F4U, F>(f4u) },

        _ => unreachable!(),
    }
}

/// A variant of [`specialize_helper`] that only allows lengths 2 and 3,
/// without 4.
#[expect(private_bounds)]
pub const fn specialize_23_helper<const N: usize, A: Alignment, F2, F3, F2U, F3U, F>(
    f2: F2,
    f3: F3,
    f2u: F2U,
    f3u: F3U,
) -> F
where
    Length<N>: TwoOrThree,
    F2: Specialize<F, 2, N, Aligned, A> + Copy,
    F3: Specialize<F, 3, N, Aligned, A> + Copy,
    F2U: Specialize<F, 2, N, Unaligned, A> + Copy,
    F3U: Specialize<F, 3, N, Unaligned, A> + Copy,
{
    match (N, A::IS_ALIGNED) {
        // SAFETY: `T2` is guaranteed to be the same type as `T` as long as `N`
        // is `2` and `A` is `Aligned`. `N` was just checked to be `2` and `A`
        // is guaranteed to be `Aligned` because `A::IS_ALIGNED` is true, and so
        // `T2` and `T` are the same type.
        (2, true) => unsafe { transmute_generic::<F2, F>(f2) },

        // SAFETY: `T3` is guaranteed to be the same type as `T` as long as `N`
        // is `3` and `A` is `Aligned`. `N` was just checked to be `3` and `A`
        // is guaranteed to be `Aligned` because `A::IS_ALIGNED` is true, and so
        // `T3` and `T` are the same type.
        (3, true) => unsafe { transmute_generic::<F3, F>(f3) },

        // SAFETY: `T2U` is guaranteed to be the same type as `T` as long as `N`
        // is `2` and `A` is `Unaligned`. `N` was just checked to be `2` and `A`
        // is guaranteed to be `Unaligned` because `A::IS_ALIGNED` is false, and
        // so `T2U` and `T` are the same type.
        (2, false) => unsafe { transmute_generic::<F2U, F>(f2u) },

        // SAFETY: `T3U` is guaranteed to be the same type as `T` as long as `N`
        // is `3` and `A` is `Unaligned`. `N` was just checked to be `3` and `A`
        // is guaranteed to be `Unaligned` because `A::IS_ALIGNED` is false, and
        // so `T3U` and `T` are the same type.
        (3, false) => unsafe { transmute_generic::<F3U, F>(f3u) },

        _ => unreachable!(),
    }
}

/// A variant of [`specialize_helper`] that only specializes alignment, with a
/// specific values for `N`.
#[expect(private_bounds)]
pub const fn specialize_n_helper<const N: usize, A: Alignment, Fu, Fa, F>(fu: Fu, fa: Fa) -> F
where
    Length<N>: TwoOrThree,
    Fu: Specialize<F, N, N, Unaligned, A> + Copy,
    Fa: Specialize<F, N, N, Aligned, A> + Copy,
{
    if A::IS_ALIGNED {
        // SAFETY: `Fa` is guaranteed to be the same type as `F` as long as `A`
        // is `Aligned`. Because `A::IS_ALIGNED` is true, `A` is guaranteed to
        // be `Aligned`, so the types are the same.
        unsafe { transmute_generic::<Fa, F>(fa) }
    } else {
        // SAFETY: `Fu` is guaranteed to be the same type as `F` as long as `A`
        // is `Unaligned`. Because `A::IS_ALIGNED` is false, `A` is guaranteed
        // to be `Unaligned`, so the types are the same.
        unsafe { transmute_generic::<Fu, F>(fu) }
    }
}

/// Staticly guarantees the soundness of [`specialize_helper`].
///
/// This trait is implemented when `Self` and `T2` are the same type assuming
/// `N == N2` and `A == A2`.
///
/// # Safety
///
/// If `N == N2` and `A == A2`, `Self == T2` must be correct. An incorrect
/// implementation would make it possible to transmute between incompatible
/// types, causing undefined behaviour.
unsafe trait Specialize<T2, const N: usize, const N2: usize, A: Alignment, A2: Alignment> {}

// SAFETY: `T == T` regardless of `N` and `A`.
unsafe impl<T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<T, N, N2, A, A2> for T
where
    T: Scalar,
{
}

// SAFETY: `T == T` regardless of `N` and `A`.
unsafe impl<'a, 'b, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a mut core::fmt::Formatter<'b>, N, N2, A, A2>
    for &'a mut core::fmt::Formatter<'b>
{
}

// SAFETY: `T == T` regardless of `N` and `A`.
unsafe impl<const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<core::fmt::Result, N, N2, A, A2> for core::fmt::Result
{
}

// SAFETY: `N == N2`, `A == A2` => `Vector<N, T, A> == Vector<N2, T, A2>`.
unsafe impl<T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<Vector<N2, T, A2>, N, N2, A, A2> for Vector<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`
// => `&'a Vector<N, T, A> == &'a Vector<N2, T, A2>`.
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a Vector<N2, T, A2>, N, N2, A, A2> for &'a Vector<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`
// => `&'a mut Vector<N, T, A> == &'a mut Vector<N2, T, A2>`.
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a mut Vector<N2, T, A2>, N, N2, A, A2> for &'a mut Vector<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2` => `Matrix<N, T, A> == Matrix<N2, T, A2>`.
unsafe impl<T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<Matrix<N2, T, A2>, N, N2, A, A2> for Matrix<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`
// => `&'a Matrix<N, T, A> == &'a Matrix<N2, T, A2>`.
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a Matrix<N2, T, A2>, N, N2, A, A2> for &'a Matrix<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`
// => `&'a mut Matrix<N, T, A> == &'a mut Matrix<N2, T, A2>`.
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a mut Matrix<N2, T, A2>, N, N2, A, A2> for &'a mut Matrix<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2` => `Affine<N, T, A> == Affine<N2, T, A2>`.
unsafe impl<T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<Affine<N2, T, A2>, N, N2, A, A2> for Affine<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`
// => `&'a Affine<N, T, A> == &'a Affine<N2, T, A2>`.
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a Affine<N2, T, A2>, N, N2, A, A2> for &'a Affine<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`
// => `&'a mut Affine<N, T, A> == &'a mut Affine<N2, T, A2>`.
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a mut Affine<N2, T, A2>, N, N2, A, A2> for &'a mut Affine<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2` => `Projective<N, T, A> == Projective<N2, T, A2>`
unsafe impl<T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<Projective<N2, T, A2>, N, N2, A, A2> for Projective<N, T, A>
where
    T: Scalar,
    Length<N>: TwoOrThree,
    Length<N2>: TwoOrThree,
{
}

// SAFETY: `N == N2`, `A == A2` => `&'a Projective<N, T, A> == &'a Projective<N2, T, A2>`
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a Projective<N2, T, A2>, N, N2, A, A2> for &'a Projective<N, T, A>
where
    T: Scalar,
    Length<N>: TwoOrThree,
    Length<N2>: TwoOrThree,
{
}

// SAFETY: `N == N2`, `A == A2` => `&'a Projective<N, T, A> == &'a Projective<N2, T, A2>`
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a mut Projective<N2, T, A2>, N, N2, A, A2> for &'a mut Projective<N, T, A>
where
    T: Scalar,
    Length<N>: TwoOrThree,
    Length<N2>: TwoOrThree,
{
}

// SAFETY: `N == N2`, `A == A2` => `Rotor<N, T, A> == Rotor<N2, T, A2>`
unsafe impl<T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<Rotor<N2, T, A2>, N, N2, A, A2> for Rotor<N, T, A>
where
    T: Scalar,
    Length<N>: TwoOrThree,
    Length<N2>: TwoOrThree,
{
}

// SAFETY: `N == N2`, `A == A2` => `&'a Rotor<N, T, A> == &'a Rotor<N2, T, A2>`
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a Rotor<N2, T, A2>, N, N2, A, A2> for &'a Rotor<N, T, A>
where
    T: Scalar,
    Length<N>: TwoOrThree,
    Length<N2>: TwoOrThree,
{
}

// SAFETY: `N == N2`, `A == A2` => `&'a Rotor<N, T, A> == &'a Rotor<N2, T, A2>`
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a mut Rotor<N2, T, A2>, N, N2, A, A2> for &'a mut Rotor<N, T, A>
where
    T: Scalar,
    Length<N>: TwoOrThree,
    Length<N2>: TwoOrThree,
{
}

// SAFETY: `N == N2`, `A == A2` => `Mask<N, T, A> == Mask<N2, T, A2>`.
unsafe impl<T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<Mask<N2, T, A2>, N, N2, A, A2> for Mask<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2` => `&'a Mask<N, T, A> == &'a Mask<N2, T, A2>`.
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a Mask<N2, T, A2>, N, N2, A, A2> for &'a Mask<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`
// => `&'a mut Mask<N, T, A> == &'a mut Mask<N2, T, A2>`.
unsafe impl<'a, T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a mut Mask<N2, T, A2>, N, N2, A, A2> for &'a mut Mask<N, T, A>
where
    T: Scalar,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`, `T == T2` => `[T; N] == [T2; N2]`.
unsafe impl<T, T2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<[T2; N2], N, N2, A, A2> for [T; N]
where
    T: Specialize<T2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `T == T2` => `&[T; N] == &[T2; N]`.
unsafe impl<
    'a,
    T,
    T2,
    const N: usize,
    const N1: usize,
    const N2: usize,
    A: Alignment,
    A2: Alignment,
> Specialize<&'a [T2; N], N1, N2, A, A2> for &'a [T; N]
where
    T: Specialize<T2, N1, N2, A, A2>,
    Length<N1>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `() == ()` regardless of `N` and `A`.
unsafe impl<const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<(), N, N2, A, A2> for ()
where
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `(T,) == (T,)`.
unsafe impl<T, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<(T,), N, N2, A, A2> for (T,)
where
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`, `Ta == Ta2`, `Tb == Tb2` => `(Ta, Tb) == (Ta2, Tb2)`.
unsafe impl<Ta, Ta2, Tb, Tb2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<(Ta2, Tb2), N, N2, A, A2> for (Ta, Tb)
where
    Ta: Specialize<Ta2, N, N2, A, A2>,
    Tb: Specialize<Tb2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`, `T == T2` => `Option<T> == Option<T2>`.
unsafe impl<T, T2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<Option<T2>, N, N2, A, A2> for Option<T>
where
    T: Specialize<T2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

macro_rules! fn_impl {
    ($($P:ident, $P2:ident),*) => {
        // SAFETY: `$P == $P2`, `R == R2` => `fn($P) -> R == fn($P2) -> R2`.
        unsafe impl<$($P, $P2,)* R, R2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
            Specialize<fn($($P2),*) -> R2, N, N2, A, A2> for fn($($P),*) -> R
        where
            $($P: Specialize<$P2, N, N2, A, A2>,)*
            R: Specialize<R2, N, N2, A, A2>,
        {
        }
    };
}
fn_impl!(Pa, Pa2);
fn_impl!(Pa, Pa2, Pb, Pb2);
fn_impl!(Pa, Pa2, Pb, Pb2, Pc, Pc2);
fn_impl!(Pa, Pa2, Pb, Pb2, Pc, Pc2, Pd, Pd2);
fn_impl!(Pa, Pa2, Pb, Pb2, Pc, Pc2, Pd, Pd2, Pe, Pe2);
fn_impl!(Pa, Pa2, Pb, Pb2, Pc, Pc2, Pd, Pd2, Pe, Pe2, Pf, Pf2);
