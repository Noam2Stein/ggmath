use std::fmt::Debug;

use ggmath::*;

fn main() {
    println!("{:?}", FpVec2::X);
}

/// Fixed point number with 8 fractional bits and 24 integer bits.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FixedPoint(i32);

vector_aliases!(type Fp => FixedPoint);

impl ScalarZero for FixedPoint {
    const ZERO: Self = Self(0);

    scalar_zero_boilerplate! {}
}

impl ScalarOne for FixedPoint {
    const ONE: Self = Self(1 << 8);

    scalar_one_boilerplate! {}
}

impl ScalarNegOne for FixedPoint {
    const NEG_ONE: Self = Self(-(1 << 8));

    scalar_neg_one_boilerplate! {}
}

impl Debug for FixedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0 as f32 / (1 << 8) as f32)
    }
}
