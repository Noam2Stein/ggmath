use crate::Scalar;

// There are currently no SIMD instructions for u128 arithmetic.

impl Scalar<2> for u128 {
    type InnerSimdVectorType = [u128; 2];
}

impl Scalar<3> for u128 {
    type InnerSimdVectorType = [u128; 3];
}

impl Scalar<4> for u128 {
    type InnerSimdVectorType = [u128; 4];
}
