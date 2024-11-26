use std::ops::*;

use super::*;

macro_rules! scalar_op {
    (
        $std_trait:ident $std_fn:ident,
        $assign_std_trait:ident $assign_std_fn:ident
    ) => {
        impl<
                const N: usize,
                T: Scalar + $std_trait<Rhs, Output: Scalar>,
                A: VecAlignment,
                Rhs: Scalar,
            > $std_trait<Rhs> for Vector<N, T, A>
        where
            ScalarCount<N>: VecLen,
        {
            type Output = <Self as $std_trait<Vector<N, Rhs, A>>>::Output;

            fn $std_fn(self, rhs: Rhs) -> Self::Output {
                self.$std_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }

        impl<const N: usize, T: Scalar + $assign_std_trait<Rhs>, A: VecAlignment, Rhs: Scalar>
            $assign_std_trait<Rhs> for Vector<N, T, A>
        where
            ScalarCount<N>: VecLen,
        {
            fn $assign_std_fn(&mut self, rhs: Rhs) {
                self.$assign_std_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }
    };
}

scalar_op!(Mul mul, MulAssign mul_assign);
scalar_op!(Div div, DivAssign div_assign);
scalar_op!(Rem rem, RemAssign rem_assign);
