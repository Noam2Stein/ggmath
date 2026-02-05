#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{
    Alignment, Length, Scalar, ScalarRepr, SupportedLength,
    utils::{Repr2, Repr3, Repr4},
};

unsafe impl ScalarRepr for i32 {
    type VectorRepr<const N: usize, T, A: Alignment>
        = <A as Alignment>::Select<
        <Length<N> as SupportedLength>::Select<Repr2<T>, __m128, __m128>,
        <Length<N> as SupportedLength>::Select<Repr2<T>, Repr3<T>, Repr4<T>>,
    >
    where
        Length<N>: SupportedLength,
        T: Scalar;
}
