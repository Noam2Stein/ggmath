use std::ops::Mul;

use super::*;

ggmath_proc_macros::vector_interface!(
    ScalarDot<TRhs: Scalar>: ScalarMul<TRhs, Output: ScalarCSum>;

    pub impl:

    fn dot(self, other: Vector<N, TRhs, impl VecAlignment>) -> <T as Mul<TRhs>>::Output {
        (self * other).csum()
    }
);
