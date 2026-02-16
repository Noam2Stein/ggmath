use assert_impl_trait::assert_impl;
use fixp::FixedPoint;
use ggmath::{
    Alignment, Length, Scalar, SupportedLength, Vector,
    constants::{Max, Min, Zero},
};

assert_impl!(
    for<const N: usize, const FRAC_BITS: usize, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        FixedPoint<i8, FRAC_BITS>: Scalar<Repr = i8> + Zero + Min + Max,
        FixedPoint<i16, FRAC_BITS>: Scalar<Repr = i16> + Zero + Min + Max,
        FixedPoint<i32, FRAC_BITS>: Scalar<Repr = i32> + Zero + Min + Max,
        FixedPoint<i64, FRAC_BITS>: Scalar<Repr = i64> + Zero + Min + Max,
        FixedPoint<i128, FRAC_BITS>: Scalar<Repr = i128> + Zero + Min + Max,
        FixedPoint<isize, FRAC_BITS>:
            Scalar<Repr = <isize as Scalar>::Repr> + Zero + Min + Max,
        FixedPoint<u8, FRAC_BITS>: Scalar<Repr = i8> + Zero + Min + Max,
        FixedPoint<u16, FRAC_BITS>: Scalar<Repr = i16> + Zero + Min + Max,
        FixedPoint<u32, FRAC_BITS>: Scalar<Repr = i32> + Zero + Min + Max,
        FixedPoint<u64, FRAC_BITS>: Scalar<Repr = i64> + Zero + Min + Max,
        FixedPoint<u128, FRAC_BITS>: Scalar<Repr = i128> + Zero + Min + Max,
        FixedPoint<usize, FRAC_BITS>:
            Scalar<Repr = <usize as Scalar>::Repr> + Zero + Min + Max,

        Vector<N, FixedPoint<i8, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<i16, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<i32, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<i64, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<i128, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<isize, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<u8, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<u16, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<u32, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<u64, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<u128, FRAC_BITS>, A>:,
        Vector<N, FixedPoint<usize, FRAC_BITS>, A>:,
    }
);
