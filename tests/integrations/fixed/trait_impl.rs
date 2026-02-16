use assert_impl_trait::assert_impl;
use fixed::{
    FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
    FixedU128,
};
use ggmath::{
    Alignment, Length, Scalar, SupportedLength, Vector,
    constants::{Max, Min, Zero},
};

assert_impl!(
    for<const N: usize, Frac, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        FixedI8<Frac>: Scalar<Repr = i8> + Zero + Min + Max,
        FixedI16<Frac>: Scalar<Repr = i16> + Zero + Min + Max,
        FixedI32<Frac>: Scalar<Repr = i32> + Zero + Min + Max,
        FixedI64<Frac>: Scalar<Repr = i64> + Zero + Min + Max,
        FixedI128<Frac>: Scalar<Repr = i128> + Zero + Min + Max,
        FixedU8<Frac>: Scalar<Repr = i8> + Zero + Min + Max,
        FixedU16<Frac>: Scalar<Repr = i16> + Zero + Min + Max,
        FixedU32<Frac>: Scalar<Repr = i32> + Zero + Min + Max,
        FixedU64<Frac>: Scalar<Repr = i64> + Zero + Min + Max,
        FixedU128<Frac>: Scalar<Repr = i128> + Zero + Min + Max,

        Vector<N, FixedI8<Frac>, A>:,
        Vector<N, FixedI16<Frac>, A>:,
        Vector<N, FixedI32<Frac>, A>:,
        Vector<N, FixedI64<Frac>, A>:,
        Vector<N, FixedI128<Frac>, A>:,
        Vector<N, FixedU8<Frac>, A>:,
        Vector<N, FixedU16<Frac>, A>:,
        Vector<N, FixedU32<Frac>, A>:,
        Vector<N, FixedU64<Frac>, A>:,
        Vector<N, FixedU128<Frac>, A>:,
    }
);
