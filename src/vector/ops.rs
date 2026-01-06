use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::vector::{
    Alignment, Length, Scalar, ScalarBackend, SupportedLength, Vector, specialize,
};

impl_unary_op!(Neg, neg, vec_neg);
impl_unary_op!(Not, not, vec_not);

impl_binary_op!(Add, add, vec_add, AddAssign, add_assign);
impl_binary_op!(Sub, sub, vec_sub, SubAssign, sub_assign);
impl_binary_op!(Mul, mul, vec_mul, MulAssign, mul_assign);
impl_binary_op!(Div, div, vec_div, DivAssign, div_assign);
impl_binary_op!(Rem, rem, vec_rem, RemAssign, rem_assign);
impl_binary_op!(Shl, shl, vec_shl, ShlAssign, shl_assign);
impl_binary_op!(Shr, shr, vec_shr, ShrAssign, shr_assign);
impl_binary_op!(BitAnd, bitand, vec_bitand, BitAndAssign, bitand_assign);
impl_binary_op!(BitOr, bitor, vec_bitor, BitOrAssign, bitor_assign);
impl_binary_op!(BitXor, bitxor, vec_bitxor, BitXorAssign, bitxor_assign);

macro_rules! impl_unary_op {
    ($Op:ident, $op:ident, $vec_op:ident) => {
        impl<const N: usize, T: Scalar, A: Alignment> $Op for Vector<N, T, A>
        where
            T: $Op<Output = T>,
            Length<N>: SupportedLength,
        {
            type Output = Self;

            #[inline(always)]
            fn $op(self) -> Self {
                specialize!(<T as ScalarBackend<N, A>>::$vec_op(self))
            }
        }
    };
}

use impl_unary_op;

macro_rules! impl_binary_op {
    ($Op:ident, $op:ident, $vec_op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<const N: usize, T: Scalar, A: Alignment> $Op for Vector<N, T, A>
        where
            T: $Op<Output = T>,
            Length<N>: SupportedLength,
        {
            type Output = Self;

            #[inline(always)]
            fn $op(self, rhs: Self) -> Self {
                specialize!(<T as ScalarBackend<N, A>>::$vec_op(self, rhs))
            }
        }

        impl<const N: usize, T: Scalar, A: Alignment> $Op<T> for Vector<N, T, A>
        where
            T: $Op<Output = T>,
            Length<N>: SupportedLength,
        {
            type Output = Self;

            #[inline(always)]
            fn $op(self, rhs: T) -> Self {
                self.$op(Vector::splat(rhs))
            }
        }

        impl<const N: usize, T: Scalar, A: Alignment> $OpAssign for Vector<N, T, A>
        where
            T: $Op<Output = T>,
            Length<N>: SupportedLength,
        {
            #[inline(always)]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T: Scalar, A: Alignment> $OpAssign<T> for Vector<N, T, A>
        where
            T: $Op<Output = T>,
            Length<N>: SupportedLength,
        {
            #[inline(always)]
            fn $op_assign(&mut self, rhs: T) {
                *self = self.$op(rhs);
            }
        }
    };
}

use impl_binary_op;
