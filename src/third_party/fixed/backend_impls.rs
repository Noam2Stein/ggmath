use fixed::{
    FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
    FixedU128,
};

use crate::{Alignment, Length, ScalarBackend, SupportedLength};

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedI8<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedI16<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedI32<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedI64<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedI128<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedU8<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedU16<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedU32<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedU64<Frac> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, Frac, A: Alignment> ScalarBackend<N, A> for FixedU128<Frac> where
    Length<N>: SupportedLength
{
}
