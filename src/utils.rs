use core::mem::{ManuallyDrop, transmute, transmute_copy};

use crate::{Aligned, Alignment, Length, Mask, Scalar, SupportedLength, Unaligned, Vector};

////////////////////////////////////////////////////////////////////////////////
// Transmute
////////////////////////////////////////////////////////////////////////////////

/// Variant of [`core::mem::transmute`] that ensures memory-layout compatibility
/// at runtime instead of at compile-time.
///
/// This makes it possible to transmute between generic types and dependently
/// sized types.
///
/// # Safety
///
/// Exactly like [`core::mem::transmute`].
#[inline]
pub const unsafe fn transmute_generic<T, U>(value: T) -> U {
    assert!(size_of::<T>() == size_of::<U>());

    // SAFETY: The caller must ensure that the bits of the input are valid for
    // the output type.
    unsafe { transmute_copy::<ManuallyDrop<T>, U>(&ManuallyDrop::new(value)) }
}

/// Transmutes a reference to a reference of a different type.
///
/// # Safety
///
/// Exactly like [`core::mem::transmute`] for references, except that here
/// immediate size and alignment validity is automatically checked, and the
/// lifetime of the reference cannot be accidently changed.
#[inline]
pub const unsafe fn transmute_ref<T, U>(value: &T) -> &U {
    assert!(size_of::<T>() >= size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

    // SAFETY: The output size was just checked to be no more than the input
    // size, the output alignment was just checked to be no more than the input
    // alignment, the lifetimes match because of the function signature, and the
    // caller must ensure that the bits of the input value are valid for the
    // output type.
    unsafe { transmute::<&T, &U>(value) }
}

/// Transmutes a mutable reference to a mutable reference of a different type.
///
/// # Safety
///
/// Exactly like [`core::mem::transmute`] for mutable references, except that
/// here size and alignment validity is automatically checked, and the lifetime
/// cannot be accidently changed.
#[inline]
pub const unsafe fn transmute_mut<T, U>(value: &mut T) -> &mut U {
    assert!(size_of::<T>() >= size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

    // SAFETY: The output size was just checked to be no more than the input
    // size, the output alignment was just checked to be no more than the input
    // alignment, the lifetimes match because of the function signature, the
    // input is mutably borrowed until the output reference is dropped, and the
    // caller must ensure that the bits of the input value are valid for the
    // output type.
    unsafe { transmute::<&mut T, &mut U>(value) }
}

////////////////////////////////////////////////////////////////////////////////
// Specialize
////////////////////////////////////////////////////////////////////////////////

/// Bypasses a limitation of const generics to do specialization.
///
/// `ggmath` allows types that implement [`Scalar`] to override the
/// implementations of math functions for each length and alignment to make
/// optimizations.
///
/// Implementations are overriden via the [`ScalarBackend<N,
/// A>`](crate::ScalarBackend) trait, which has to be implemented for all
/// lengths and both alignments. Math functions that want to call their
/// implementations would ideally want to perform:
///
/// ```ignore
/// <T as ScalarBackend<N, A>>::math_fn(args)
/// ```
///
/// But this approach results in a compiler error. The compiler doesn't know
/// that `T` implements `ScalarBackend` for all cases of `N` and `A`, it only
/// knows that it implements it for `2, 3, 4` and `Aligned, Unaligned`.
///
/// To bypass this, math functions have to match against `N` and `A` to their 6
/// possible cases then use unsafe transmutes to convert inputs and outputs from
/// their generic form to their concrete form.
///
/// This macro is a safe wrapper for this pattern, and allows math functions to
/// write:
///
/// ```ignore
/// specialize!(<T as ScalarBackend<N, A>>::math_function(arguments))
/// ```
///
/// If certain function signatures fails to compile with this macro, its
/// implementation needs to be adjusted to support those function's types.
///
/// When `generic_const_params` stabilizes and the compiler is smart enough to
/// understand `T: ScalarBackend<N, A>`, this macro should be removed.
macro_rules! specialize {
    (<$T:ty as $Backend:ident<$N:tt, $A:tt>>::$f:ident($($arg:expr),*$(,)?)) => {
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
}

pub(crate) use specialize;

#[expect(private_bounds)]
pub const fn specialize_helper<const N: usize, A: Alignment, T2, T3, T4, T2U, T3U, T4U, T>(
    value2: T2,
    value3: T3,
    value4: T4,
    value2u: T2U,
    value3u: T3U,
    value4u: T4U,
) -> T
where
    Length<N>: SupportedLength,
    T2: Specialize<T, 2, N, Aligned, A> + Copy,
    T3: Specialize<T, 3, N, Aligned, A> + Copy,
    T4: Specialize<T, 4, N, Aligned, A> + Copy,
    T2U: Specialize<T, 2, N, Unaligned, A> + Copy,
    T3U: Specialize<T, 3, N, Unaligned, A> + Copy,
    T4U: Specialize<T, 4, N, Unaligned, A> + Copy,
{
    match (N, A::IS_ALIGNED) {
        // SAFETY: `T2` is guaranteed to be the same type as `T` as long as `N`
        // is `2` and `A` is `Aligned`. `N` was just checked to be `2` and `A`
        // is guaranteed to be `Aligned` because `A::IS_ALIGNED` is true, and so
        // `T2` and `T` are the same type.
        (2, true) => unsafe { transmute_generic::<T2, T>(value2) },

        // SAFETY: `T3` is guaranteed to be the same type as `T` as long as `N`
        // is `3` and `A` is `Aligned`. `N` was just checked to be `3` and `A`
        // is guaranteed to be `Aligned` because `A::IS_ALIGNED` is true, and so
        // `T3` and `T` are the same type.
        (3, true) => unsafe { transmute_generic::<T3, T>(value3) },

        // SAFETY: `T4` is guaranteed to be the same type as `T` as long as `N`
        // is `4` and `A` is `Aligned`. `N` was just checked to be `4` and `A`
        // is guaranteed to be `Aligned` because `A::IS_ALIGNED` is true, and so
        // `T4` and `T` are the same type.
        (4, true) => unsafe { transmute_generic::<T4, T>(value4) },

        // SAFETY: `T2U` is guaranteed to be the same type as `T` as long as `N`
        // is `2` and `A` is `Unaligned`. `N` was just checked to be `2` and `A`
        // is guaranteed to be `Unaligned` because `A::IS_ALIGNED` is false, and
        // so `T2U` and `T` are the same type.
        (2, false) => unsafe { transmute_generic::<T2U, T>(value2u) },

        // SAFETY: `T3U` is guaranteed to be the same type as `T` as long as `N`
        // is `3` and `A` is `Unaligned`. `N` was just checked to be `3` and `A`
        // is guaranteed to be `Unaligned` because `A::IS_ALIGNED` is false, and
        // so `T3U` and `T` are the same type.
        (3, false) => unsafe { transmute_generic::<T3U, T>(value3u) },

        // SAFETY: `T4U` is guaranteed to be the same type as `T` as long as `N`
        // is `4` and `A` is `Unaligned`. `N` was just checked to be `4` and `A`
        // is guaranteed to be `Unaligned` because `A::IS_ALIGNED` is false, and
        // so `T4U` and `T` are the same type.
        (4, false) => unsafe { transmute_generic::<T4U, T>(value4u) },

        _ => unreachable!(),
    }
}

/// Ensures that `Self` is the same type as `T2` if `N == N2` and `A == A2`.
///
/// This trait is used to ensure the soundness of [`specialize!`]. Examples of
/// correct usage are:
///
/// If `N == N2` and `A == A2`:
///
/// - `Vector<N, T, A> == Vector<N2, T, A2>`.
/// - `[T; N] == [T; N2]`.
/// - `bool == bool` (this one is true regardless).
/// - `fn([T; N]) -> Vector<N, T, A> == fn([T; N2]) -> Vector<N2, T, A2>`.
///
/// The last example is what the macro uses directly.
///
/// # Safety
///
/// If `N == N2` and `A` being the same type as `A2` are proven to be true,
/// `Self` must be the same type as `T2`.
unsafe trait Specialize<T2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
where
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
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

// SAFETY: `N == N2`, `A == A2`, `R == R2` => `fn() -> R == fn() -> R2`.
unsafe impl<R, R2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<fn() -> R2, N, N2, A, A2> for fn() -> R
where
    R: Specialize<R2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`, `Pa == Pa2`, `R == R2`
// => `fn(Pa) -> R == fn(Pa2) -> R2`.
unsafe impl<Pa, Pa2, R, R2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<fn(Pa2) -> R2, N, N2, A, A2> for fn(Pa) -> R
where
    Pa: Specialize<Pa2, N, N2, A, A2>,
    R: Specialize<R2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`, `Pa == Pa2`, `Pb == Pb2`, `R == R2`
// => `fn(Pa, Pb) -> R == fn(Pa2, Pb2) -> R2`.
unsafe impl<Pa, Pa2, Pb, Pb2, R, R2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<fn(Pa2, Pb2) -> R2, N, N2, A, A2> for fn(Pa, Pb) -> R
where
    Pa: Specialize<Pa2, N, N2, A, A2>,
    Pb: Specialize<Pb2, N, N2, A, A2>,
    R: Specialize<R2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

// SAFETY: `N == N2`, `A == A2`, `Pa == Pa2`, `Pb == Pb2`, `Pc == Pc2`, `R == R2`
// => `fn(Pa, Pb, Pc) -> R == fn(Pa2, Pb2, Pc2) -> R2`.
unsafe impl<
    Pa,
    Pa2,
    Pb,
    Pb2,
    Pc,
    Pc2,
    R,
    R2,
    const N: usize,
    const N2: usize,
    A: Alignment,
    A2: Alignment,
> Specialize<fn(Pa2, Pb2, Pc2) -> R2, N, N2, A, A2> for fn(Pa, Pb, Pc) -> R
where
    Pa: Specialize<Pa2, N, N2, A, A2>,
    Pb: Specialize<Pb2, N, N2, A, A2>,
    Pc: Specialize<Pc2, N, N2, A, A2>,
    R: Specialize<R2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

macro_rules! impl_specialzie {
    ($($T:ty),*$(,)?) => {
        $(
            // `$T == $T`.
            unsafe impl<const N: usize, const N2: usize, A: Alignment, A2: Alignment>
                Specialize<$T, N, N2, A, A2> for $T
            where
                Length<N>: SupportedLength,
                Length<N2>: SupportedLength,
            {
            }
        )*
    };
}
impl_specialzie!(
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    bool,
    (),
);

////////////////////////////////////////////////////////////////////////////////
// Safe Arch
////////////////////////////////////////////////////////////////////////////////

#[allow(unused_macros)]
macro_rules! safe_arch {
    (
        #![target_feature(enable = $feature:literal)]

        $(
            $(#[$meta:meta])* fn $f:ident($($param:ident: $Param:ty),* $(,)?) $(-> $Ret:ty)? $body:block
        )*
    ) => {
        $(
            $(#[$meta])*
            fn $f($($param: $Param),*) $(-> $Ret)? {
                #[inline]
                #[target_feature(enable = $feature)]
                fn $f($($param: $Param),*) $(-> $Ret)? $body

                #[cfg(not(target_feature = $feature))]
                compile_error!(concat!("target feature is not enabled"));

                // SAFETY: If `$feature` is disabled, there will be a compile
                // error.
                unsafe { $f($($param),*) }
            }
        )*
    };
}

#[allow(unused_imports)]
pub(crate) use safe_arch;

////////////////////////////////////////////////////////////////////////////////
// Representation Types
////////////////////////////////////////////////////////////////////////////////

/*
These types are used instead of arrays because for some reason arrays lead to
worse assembly than structs.

When this is fixed, these structs should be removed.
*/

/// Contains two values of type `T`.
///
/// This type is used instead of `[T; 2]` because for some reason arrays lead
/// to worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr2<T>(pub T, pub T);

/// Contains three values of type `T`.
///
/// This type is used instead of `[T; 3]` because for some reason arrays lead
/// to worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr3<T>(pub T, pub T, pub T);

/// Contains four values of type `T`.
///
/// This type is used instead of `[T; 4]` because for some reason arrays lead
/// to worse assembly than structs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Repr4<T>(pub T, pub T, pub T, pub T);
