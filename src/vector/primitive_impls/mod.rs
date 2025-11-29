use crate::vector::{Alignment, Length, ScalarBackend, SupportedLength};

mod f32;
mod f64;
mod i32;
mod i64;
mod isize;
mod u32;
mod u64;
mod usize;

////////////////////////////////////////////////////////////////////////////////
// Unoptimizable Scalars
////////////////////////////////////////////////////////////////////////////////

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i8
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [i8; N];
}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i16
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [i16; N];
}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i128
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [i128; N];
}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u8
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [u8; N];
}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u16
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [u16; N];
}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u128
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [u128; N];
}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for bool
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [bool; N];
}

////////////////////////////////////////////////////////////////////////////////
// Helper Macros
////////////////////////////////////////////////////////////////////////////////

macro_rules! safe_arch {
    (
        for $feature:literal:

        $($(#[$attr:meta])* fn $f:ident$(<$(const $PARAM:ident: $PARAM_TY:ty),*$(,)?>)?($($param:ident: $_param_ty:ty),*$(,)?) -> $ret_ty:ty $b:block)*
    ) => {
        $(
            $(#[$attr])*
            fn $f$(<$(const $PARAM: $PARAM_TY),*>)?($($param: $_param_ty),*) -> $ret_ty {
                #[inline]
                #[target_feature(enable = $feature)]
                fn $f$(<$(const $PARAM: $PARAM_TY),*>)?($($param: $_param_ty),*) -> $ret_ty $b

                #[cfg(not(target_feature = $feature))]
                compile_error!(concat!("target feature `", $feature, "` is not enabled"));

                unsafe {
                    $f$(::<$($PARAM),*>)?($($param),*)
                }
            }
        )*
    };
}

use safe_arch;
