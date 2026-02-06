use bytemuck::{NoUninit, Pod, Zeroable};

use crate::{Alignment, Length, Mask, Scalar, SupportedLength, Vector};

/*
Missing implementations are blocked on:
https://github.com/rust-lang/rust/issues/29864
*/

unsafe impl<const N: usize, T, A: Alignment> Pod for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Pod,
{
}

unsafe impl<const N: usize, T, A: Alignment> Zeroable for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zeroable,
{
}

unsafe impl<const N: usize, T, A: Alignment> NoUninit for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + 'static,
{
}

unsafe impl<const N: usize, T, A: Alignment> Zeroable for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}
