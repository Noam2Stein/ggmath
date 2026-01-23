use bytemuck::{Pod, Zeroable};

use crate::{Alignment, Length, Scalar, SupportedLength, Vector};

/*
Missing implementations are blocked on:
https://github.com/rust-lang/rust/issues/29864
*/

unsafe impl<const N: usize, T: Scalar + Pod, A: Alignment> Pod for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

unsafe impl<const N: usize, T: Scalar + Zeroable, A: Alignment> Zeroable for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}
