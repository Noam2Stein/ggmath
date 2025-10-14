use crate::Scalar;

// TODO: Sync isize vectors with `i16`, `i32` and `i64`.

impl Scalar<2> for isize {
    type InnerSimdVectorType = [isize; 2];
}

impl Scalar<3> for isize {
    type InnerSimdVectorType = [isize; 3];
}

impl Scalar<4> for isize {
    type InnerSimdVectorType = [isize; 4];
}
