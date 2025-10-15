use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::{
    Scalar, Simdness, SupportedVecLen, VecLen, Vector,
    vector::{VectorApi, specialized_vector_api},
};

impl_unary_op!(Neg { neg });
impl_unary_op!(Not { not });

impl_binary_op!(Add { add }, AddAssign { add_assign });
impl_binary_op!(Sub { sub }, SubAssign { sub_assign });
impl_binary_op!(Mul { mul }, MulAssign { mul_assign });
impl_binary_op!(Div { div }, DivAssign { div_assign });
impl_binary_op!(Rem { rem }, RemAssign { rem_assign });
impl_binary_op!(Shl { shl }, ShlAssign { shl_assign });
impl_binary_op!(Shr { shr }, ShrAssign { shr_assign });
impl_binary_op!(BitAnd { bitand }, BitAndAssign { bitand_assign });
impl_binary_op!(BitOr { bitor }, BitOrAssign { bitor_assign });
impl_binary_op!(BitXor { bitxor }, BitXorAssign { bitxor_assign });

macro_rules! impl_unary_op {
    ($Op:ident { $op:ident }) => {
        impl<const N: usize, T: Scalar + $Op<Output = T>, S: Simdness> $Op for Vector<N, T, S>
        where
            VecLen<N>: SupportedVecLen,
        {
            type Output = Self;

            specialized_vector_api! {
                VectorApi for <N, T, S>:

                fn $op(self) -> Self;
            }
        }
    };
}

use impl_unary_op;

macro_rules! impl_binary_op {
    ($Op:ident { $op:ident }, $OpAssign:ident { $op_assign:ident }) => {
        impl<const N: usize, T: Scalar + $Op<Output = T>, S: Simdness> $Op for Vector<N, T, S>
        where
            VecLen<N>: SupportedVecLen,
        {
            type Output = Self;

            specialized_vector_api! {
                VectorApi for <N, T, S>:

                fn $op(self, rhs: Self) -> Self;
            }
        }

        impl<const N: usize, T: Scalar + $Op<Output = T>, S: Simdness> $OpAssign for Vector<N, T, S>
        where
            VecLen<N>: SupportedVecLen,
        {
            #[inline(always)]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T: Scalar + $Op<Output = T>, S: Simdness> $Op<T> for Vector<N, T, S>
        where
            VecLen<N>: SupportedVecLen,
        {
            type Output = Self;

            #[inline(always)]
            fn $op(self, rhs: T) -> Self {
                self.$op(Vector::splat(rhs))
            }
        }

        impl<const N: usize, T: Scalar + $Op<Output = T>, S: Simdness> $OpAssign<T>
            for Vector<N, T, S>
        where
            VecLen<N>: SupportedVecLen,
        {
            #[inline(always)]
            fn $op_assign(&mut self, rhs: T) {
                *self = self.$op(rhs);
            }
        }
    };
}

use impl_binary_op;
