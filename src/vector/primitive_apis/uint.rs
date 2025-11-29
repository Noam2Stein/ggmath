use crate::vector::{Scalar, ScalarOne, ScalarZero};

impl Scalar for T {}

impl ScalarZero for T {
    const ZERO: Self = 0;
}

impl ScalarOne for T {
    const ONE: Self = 1;
}
