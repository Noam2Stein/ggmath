use crate::Scalar;

// TODO: Determine if char vectors benefit from SIMD.

impl Scalar<2> for char {
    type InnerSimdVectorType = [char; 2];
}

impl Scalar<3> for char {
    type InnerSimdVectorType = [char; 3];
}

impl Scalar<4> for char {
    type InnerSimdVectorType = [char; 4];
}
