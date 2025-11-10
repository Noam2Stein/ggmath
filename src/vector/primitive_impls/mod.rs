mod f32;
mod f64;
mod i32;
mod i64;
mod isize;
mod u32;
mod u64;
mod usize;

////////////////////////////////////////////////////////////////////////////////
// Non-SIMD-Compatible Primitives
////////////////////////////////////////////////////////////////////////////////

use crate::{ScalarBackend, SupportedVecLen, VecLen};

impl<const N: usize> ScalarBackend<N> for i8
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [i8; N];
}

impl<const N: usize> ScalarBackend<N> for i16
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [i16; N];
}

impl<const N: usize> ScalarBackend<N> for i128
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [i128; N];
}

impl<const N: usize> ScalarBackend<N> for u8
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [u8; N];
}

impl<const N: usize> ScalarBackend<N> for u16
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [u16; N];
}

impl<const N: usize> ScalarBackend<N> for u128
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [u128; N];
}

impl<const N: usize> ScalarBackend<N> for bool
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [bool; N];
}
