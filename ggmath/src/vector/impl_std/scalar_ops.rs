use std::ops::*;

use super::*;

macro_rules! scalar_op {
    (
        $std_trait:ident $scalar_trait:ident $std_fn:ident,
        $assign_std_trait:ident $assign_scalar_trait:ident $assign_std_fn:ident
    ) => {
        impl<const N: usize, T: $scalar_trait<Rhs>, A: VecAlignment, Rhs: Scalar> $std_trait<Rhs>
            for Vector<N, T, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            type Output = <Self as $std_trait<Vector<N, Rhs, A>>>::Output;
            fn $std_fn(self, rhs: Rhs) -> Self::Output {
                self.$std_fn(Vector::splat(rhs))
            }
        }

        impl<const N: usize, T: $assign_scalar_trait<Rhs>, A: VecAlignment, Rhs: Scalar>
            $assign_std_trait<Rhs> for Vector<N, T, A>
        where
            ScalarCount<N>: VecLen<N>,
        {
            fn $assign_std_fn(&mut self, rhs: Rhs) {
                self.$assign_std_fn(Vector::splat(rhs))
            }
        }
    };
}

scalar_op!(Mul ScalarMul mul, MulAssign ScalarMulAssign mul_assign);
scalar_op!(Div ScalarDiv div, DivAssign ScalarDivAssign div_assign);
scalar_op!(Rem ScalarRem rem, RemAssign ScalarRemAssign rem_assign);
