use core::mem::{ManuallyDrop, transmute, transmute_copy};

use crate::{Aligned, Alignment, Length, Scalar, SupportedLength, Unaligned, Vector};

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

    unsafe { transmute_copy::<ManuallyDrop<T>, U>(&ManuallyDrop::new(value)) }
}

/// Transmutes a reference to a reference of a different type.
///
/// # Safety
///
/// Exactly like [`core::mem::transmute`] for references, except that here
/// memory-layout validity is automatically checked and the lifetime cannot be
/// accidently changed.
#[inline]
pub const unsafe fn transmute_ref<T, U>(value: &T) -> &U {
    assert!(size_of::<T>() >= size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

    unsafe { transmute::<&T, &U>(value) }
}

/// Transmutes a mutable reference to a mutable reference of a different type.
///
/// # Safety
///
/// Exactly like [`core::mem::transmute`] for references, except that here
/// memory-layout validity is automatically checked and the lifetime cannot be
/// accidently changed.
#[inline]
pub const unsafe fn transmute_mut<T, U>(value: &mut T) -> &mut U {
    assert!(size_of::<T>() >= size_of::<U>());
    assert!(align_of::<T>() >= align_of::<U>());

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
    T2: Specialize<T, 2, N, Aligned, A> + Copy,
    T3: Specialize<T, 3, N, Aligned, A> + Copy,
    T4: Specialize<T, 4, N, Aligned, A> + Copy,
    T2U: Specialize<T, 2, N, Unaligned, A> + Copy,
    T3U: Specialize<T, 3, N, Unaligned, A> + Copy,
    T4U: Specialize<T, 4, N, Unaligned, A> + Copy,
    Length<N>: SupportedLength,
{
    unsafe {
        match (N, A::IS_ALIGNED) {
            (2, true) => transmute_generic::<T2, T>(value2),
            (3, true) => transmute_generic::<T3, T>(value3),
            (4, true) => transmute_generic::<T4, T>(value4),
            (2, false) => transmute_generic::<T2U, T>(value2u),
            (3, false) => transmute_generic::<T3U, T>(value3u),
            (4, false) => transmute_generic::<T4U, T>(value4u),
            _ => unreachable!(),
        }
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
/// If `N == N2` and `A == A2` are proven to be true, `Self` must be the same
/// type as `T2`.
unsafe trait Specialize<T2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
where
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

unsafe impl<T: Scalar, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<Vector<N2, T, A2>, N, N2, A, A2> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

unsafe impl<'a, T: Scalar, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a Vector<N2, T, A2>, N, N2, A, A2> for &'a Vector<N, T, A>
where
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

unsafe impl<'a, T: Scalar, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<&'a mut Vector<N2, T, A2>, N, N2, A, A2> for &'a mut Vector<N, T, A>
where
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

unsafe impl<R, R2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<fn() -> R2, N, N2, A, A2> for fn() -> R
where
    R: Specialize<R2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

unsafe impl<Pa, Pa2, R, R2, const N: usize, const N2: usize, A: Alignment, A2: Alignment>
    Specialize<fn(Pa2) -> R2, N, N2, A, A2> for fn(Pa) -> R
where
    Pa: Specialize<Pa2, N, N2, A, A2>,
    R: Specialize<R2, N, N2, A, A2>,
    Length<N>: SupportedLength,
    Length<N2>: SupportedLength,
{
}

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
impl_specialzie!(f32, f64, bool, ());

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
                #[cfg(not(target_feature = $feature))]
                compile_error!(concat!("target feature is not enabled"));

                #[inline]
                #[target_feature(enable = $feature)]
                fn $f($($param: $Param),*) $(-> $Ret)? $body

                unsafe {
                    $f($($param),*)
                }
            }
        )*
    };
}

#[allow(unused_imports)]
pub(crate) use safe_arch;
