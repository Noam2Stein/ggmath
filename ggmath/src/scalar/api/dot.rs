use std::ops::Mul;

use super::*;

pub trait ScalarDot<TRhs: Scalar>: ScalarMul<TRhs, Output: ScalarCSum> {
    fn vector_dot<const N: usize, A: VecAlignment>(vec: , other: Vector<N, TRhs, impl VecAlignment>) -> <Self as Mul<TRhs>>::Output {
        (self * other).csum()
    }
}

ggmath_proc_macros::vector_interface!(
    ;

    pub impl:

    fn dot(self, other: Vector<N, TRhs, impl VecAlignment>) -> <T as Mul<TRhs>>::Output {
        (self * other).csum()
    }
);
