use crate::{Scalar, ScalarNegOne, ScalarOne, ScalarZero};

impl Scalar for T {}

impl ScalarZero for T {
    const ZERO: Self = 0;
}

impl ScalarOne for T {
    const ONE: Self = 1;
}

impl ScalarNegOne for T {
    const NEG_ONE: Self = -1;
}
