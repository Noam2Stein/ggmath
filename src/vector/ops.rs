use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::{
    Scalar, SupportedVecLen, VecLen, Vector,
    vector::{ScalarBackend, specialize},
};

impl_unary_op!(Neg { neg => vec_neg });
impl_unary_op!(Not { not => vec_not });

impl_binary_op!(Add { add => vec_add } + AddAssign { add_assign });
impl_binary_op!(Sub { sub => vec_sub } + SubAssign { sub_assign });
impl_binary_op!(Mul { mul => vec_mul } + MulAssign { mul_assign });
impl_binary_op!(Div { div => vec_div } + DivAssign { div_assign });
impl_binary_op!(Rem { rem => vec_rem } + RemAssign { rem_assign });
impl_binary_op!(Shl { shl => vec_shl } + ShlAssign { shl_assign });
impl_binary_op!(Shr { shr => vec_shr } + ShrAssign { shr_assign });
impl_binary_op!(BitAnd { bitand => vec_bitand } + BitAndAssign { bitand_assign });
impl_binary_op!(BitOr { bitor => vec_bitor } + BitOrAssign { bitor_assign });
impl_binary_op!(BitXor { bitxor => vec_bitxor } + BitXorAssign { bitxor_assign });

macro_rules! impl_unary_op {
    ($Op:ident { $op:ident => $vec_op:ident }) => {
        impl<const N: usize, T: Scalar + $Op<Output = T>> $Op for Vector<N, T>
        where
            VecLen<N>: SupportedVecLen,
        {
            type Output = Self;

            #[inline(always)]
            fn $op(self) -> Self {
                specialize!(<T as ScalarBackend<N>>::$vec_op(self))
            }
        }
    };
}

use impl_unary_op;

macro_rules! impl_binary_op {
    ($Op:ident { $op:ident => $vec_op:ident } + $OpAssign:ident { $op_assign:ident }) => {
        impl<const N: usize, T: Scalar + $Op<Output = T>> $Op for Vector<N, T>
        where
            VecLen<N>: SupportedVecLen,
        {
            type Output = Self;

            #[inline(always)]
            fn $op(self, rhs: Self) -> Self {
                specialize!(<T as ScalarBackend<N>>::$vec_op(self, rhs))
            }
        }

        impl<const N: usize, T: Scalar + $Op<Output = T>> $OpAssign for Vector<N, T>
        where
            VecLen<N>: SupportedVecLen,
        {
            #[inline(always)]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T: Scalar + $Op<Output = T>> $Op<T> for Vector<N, T>
        where
            VecLen<N>: SupportedVecLen,
        {
            type Output = Self;

            #[inline(always)]
            fn $op(self, rhs: T) -> Self {
                self.$op(Vector::splat(rhs))
            }
        }

        impl<const N: usize, T: Scalar + $Op<Output = T>> $OpAssign<T> for Vector<N, T>
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
