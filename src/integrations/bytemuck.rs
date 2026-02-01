use bytemuck::{Pod, Zeroable};

use crate::{Alignment, Length, Scalar, SupportedLength, Vector};

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

/*
TODO: add mask implementations. This depends on the representation rules of
masks which are not defined yet.
*/
