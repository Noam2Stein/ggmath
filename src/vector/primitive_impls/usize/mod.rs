use crate::Scalar;

// TODO: Sync usize vectors with `u16`, `u32` and `u64`.

impl Scalar<2> for usize {
    type InnerSimdVectorType = [usize; 2];
}

impl Scalar<3> for usize {
    type InnerSimdVectorType = [usize; 3];
}

impl Scalar<4> for usize {
    type InnerSimdVectorType = [usize; 4];
}
