use crate::Scalar;

// There are currently no SIMD instructions for i128 arithmetic.

impl Scalar<2> for i128 {
    type InnerSimdVectorType = [i128; 2];
}

impl Scalar<3> for i128 {
    type InnerSimdVectorType = [i128; 3];
}

impl Scalar<4> for i128 {
    type InnerSimdVectorType = [i128; 4];
}
